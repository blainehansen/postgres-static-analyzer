import { parse as tomlParse, stringify as tomlStringify } from "jsr:@std/toml@^1.0.11"
import { z } from "jsr:@zod/zod@^4.3.6"
import { toPascalCase, RawTable, TableDecision, ColumnDecision, TableOverride, ColumnOverride, commonPrefix, RealColumnDecision, refToReg } from "./gen_pg_info.utils.ts";
import { Ask } from "jsr:@sallai/ask@^2.0.2"

const ask = new Ask()

type Dict<T> = { [key: string]: T }

const [rawTables, tableOverrides] = await Promise.all([
	Deno.readTextFile("./extract_pg_info.local.json")
		.then(t => JSON.parse(t)).then(o => z.array(RawTable).parse(o)),
	Deno.readTextFile("./gen_pg_info_decisions.overrides.toml")
		.then(t => tomlParse(t)).then(o => z.record(z.string(), TableOverride).parse(o)),
])

function getTable(tableName: string) {
	const table = rawTables.find(t => t.tableName === tableName)
	if (!table) throw `no table ${tableName}`
	return table
}
function getTablePrefix(tableName: string) {
	const table = getTable(tableName)
	const prefix = commonPrefix(table.url, table.columns.map(c => c.name))
	if (!prefix) throw `no prefix for ${tableName}`
	return prefix
}

const finalDecisions: Dict<TableDecision> = {}
for (const rawTable of rawTables) {
	finalDecisions[rawTable.tableName] = await decideTable(rawTable)
}

await Deno.writeTextFile("./gen_pg_info_decisions.toml", tomlStringify(finalDecisions, ))


async function decideTable({ tableName, columns }: RawTable): Promise<TableDecision> {
	const tableOverride = tableOverrides[tableName]

	const totalQuery = tableOverride?.totalQuery

	const decidedColumns: Dict<ColumnDecision> = {}
	let foundHashColumn: string | true | undefined = undefined
	for (const { name, typ, ref, desc } of columns) {
		if (name === "oid" && !(tableName in refToReg)) continue

		if (name === "oid" && (tableName in refToReg)) foundHashColumn = true
		// TODO how do I find the
		// if (typ === "name" && name !== "conname") foundHashColumn = name

		const columnOverride = (tableOverride?.columns ?? {})[name]
		decidedColumns[name] = await decideColumn(tableName, name, typ, ref, desc, columnOverride)
	}

	const hashCol = tableOverride?.hashCol ?? foundHashColumn
	return { totalQuery, columns: decidedColumns, hashCol }
}

async function decideColumn(
	tableName: string,
	name: string, typ: string, ref: string, desc: string,
	override: ColumnOverride | undefined
): Promise<ColumnDecision> {
	if (override && 'skip' in override)
		return { skip: true }
	if (tableName === "pg_attribute" && desc.includes(" copy of "))
		return { skip: true }

	if (override && override.ty) {
		delete override.nullable
		delete override.zero
		return { ...override, ty: override.ty  }
	}

	function ov(decision: RealColumnDecision): RealColumnDecision {
		if (override) {
			delete override.nullable
			delete override.zero
		}
		return { ...override, ...decision  }
	}

	if (typ === "oid" && (tableName in refToReg)) {
		const reg = refToReg[tableName as keyof typeof refToReg]
		const sel = `${name}::${reg} as ${name}`
		return { sel, ty: "bool", /*exp,*/ /*test,*/ }
	}
	if (typ === "bool") {
		// const [ty, exp, ] = makeStr(tableName, name, false)
		return ov({ /*sel,*/ ty: "bool", /*exp,*/ /*test,*/ })
	}
	if (typ === "name") {
		const sel = `${name}::text as ${name}`
		const [ty, exp, ] = makeStr(tableName, name, false)
		return { sel, ty, exp, /*test,*/ }
	}
	if (typ === "text") {
		const nullable = /null/i.test(desc)
		const [ty, exp, ] = makeStr(tableName, name, nullable)
		return { /*sel,*/ ty, exp, /*test,*/ }
	}
	if (typ === "text[]" && desc.includes("keyword=value") || desc.includes("configuration variables")) {
		const ty = "Option<Vec<Str>>"
		const exp = `${tableName}.${name}.map(|items| items.map(Into::into).collect())`
		return { /*sel,*/ ty, exp, /*test,*/ }
	}
	if (typ === "aclitem[]") {
		const aclPrefix = aclitemMapping[tableName]; if (!aclPrefix) throw `no acl for ${tableName}`
		const sel = `${name}::text[] as ${name}`
		const ty = `Option<Vec<aclitem::${aclPrefix}AclItem>>`
		const exp = `${tableName}.${name}.map(|${name}| ${name}.map(|acl| aclitem(acl, &${aclPrefix}GrantParser)).collect())`
		return { sel, ty, exp, /*test,*/ }
	}
	if (name === "attnum" || ref === "(references pg_attribute.attnum)") {
		const [ty, exp, ] = makeAssumedU(tableName, name, 16, false)
		// TODO filter?
		return { /*sel,*/ ty, exp, /*test,*/ filters: [`${name} > 0`] }
	}

	if (typ === "int4" && !ref) {
		const zeroable = override?.zero ?? /zero/i.test(desc)
		const negativeable = /-1/i.test(desc)
		const nullable = override?.nullable ?? zeroable ?? negativeable ?? false
		const sel =
			negativeable ? `case when ${name} >= 0 then ${name} else null end`
			: zeroable ? `case when ${name} > 0 then ${name} else null end`
			: undefined
		const [ty, exp, ] = makeAssumedU(tableName, name, 32, nullable)

		return { sel, ty, exp, /*test,*/ }
	}

	if (ref === "(references pg_authid.oid)") {
		const zeroable = override?.zero ?? /zero/i.test(desc)
		const nullable = override?.nullable ?? zeroable ?? false
		const sel = zeroable
			? `case when ${name} = 0 then null else pg_get_userbyid(${name})::text end as ${name}`
			: `pg_get_userbyid(${name})::text as ${name}`
		const [ty, exp, ] = makeStr(tableName, name, nullable)
		return { sel, ty, exp, /*test,*/ filters: override?.filters }
	}

	if (ref === "(references pg_class.oid)") {
		const zeroable = override?.zero ?? /zero/i.test(desc)
		const nullable = override?.nullable ?? zeroable ?? false
		const ty = nullable ? `Option<Ref>` : `Ref`
		const sel = `${name}::regclass as ${name}`
		const exp = `${nullable ? 'maybe_parse_ref' : 'make_parse_ref'}(${tableName}.${name})`
		return { sel, ty, exp, /*test,*/ }
	}
	if (typ === "oid" && ref === "(references pg_type.oid)") {
		const zeroable = override?.zero ?? /zero/i.test(desc)
		const nullable = override?.nullable ?? zeroable ?? false

		const joinTableName = `${name}_pg_type`
		const joinNamespaceName = `${name}_pg_namespace`

		const ty = nullable ? `Option<Ref>` : `Ref`
		const sel = `${joinNamespaceName}.nspname::text as ${name}_nsp_name, ${joinTableName}.typname::text as ${name}_name`
		const exp = `${nullable ? 'maybe_ref' : 'make_ref'}(${tableName}.${name}_nsp_name, ${tableName}.${name}_name)`

		const leftPortion = nullable ? 'left ' : ''
		const joins = [
			leftPortion + `join pg_type as ${joinTableName} on ${tableName}.${name} = ${joinTableName}.oid`,
			leftPortion + `join pg_namespace as ${joinNamespaceName} on ${joinTableName}.typnamespace = ${joinNamespaceName}.oid`,
		]
		return { sel, ty, exp, /*test,*/ joins }
	}
	// if (typ === "pg_node_tree") {
	// 	`pg_get_expr(adbin, adrelid)`
	// }

	// const referencesMatch = ref.match(/^\(references (\w+)\.oid\)$/)
	// const references = referencesMatch && referencesMatch[1]
	// if (references && references !== "pg_authid" && references !== "pg_namespace") {
	// 	joinTableName = `${name}_${references}`
	// 	joinNamespaceName = `${name}_${references}_sch`

	// }

	console.log(`don't know how to handle ${tableName}.${name}: ${typ} ${ref} ${desc}`)
	const { decision } = await ask.select({
		name: "decision",
		message: "What would you like to do?",
		choices: [
			{ message: "skip", value: "skip" },
			{ message: "override", value: "override" },
		],
		default: "skip",
	} as const)
	console.log(decision)

	if (decision === "skip") {
		const t = `${tableName}.columns.${name}.skip = true`
		await Deno.writeTextFile("./gen_pg_info_decisions.overrides.toml", t, { append: true })
		return { skip: true }
	}
	if (decision === "override") {
		const t = `# ${typ} ${ref} ${desc}\n${tableName}.columns.${name} = { sel="", ty="", expr="", pgEnum="", joins=[], filters=[], zero=false, nullable=false }`
		await Deno.writeTextFile("./gen_pg_info_decisions.overrides.toml", t, { append: true })
		throw "overriding"
	}

	throw ""
	// return { skip: true }
}

// const referencesNamespace = fields.some(field => field.ref === "(references pg_namespace.oid)")
// const referencesDatabase = fields.some(field => field.ref === "(references pg_database.oid)")



// const toml = resolved.map(({ fields, tableName }) => {
// 	return `[${tableName}]\n` + fields.map(({ name, typ, ref, desc }) => `[${tableName}.${name}]\n# ${typ} ${ref} ${desc}`).join('\n')
// }).join('\n\n')


// [pg_default_acl.defaclacl]
// [pg_init_privs.initprivs]
const aclitemMapping: Dict<string> = {
	"pg_database": "Db",
	"pg_domain": "Domain",
	"pg_proc": "Function",
	"pg_foreign_data_wrapper": "ForeignDataWrapper",
	"pg_foreign_server": "ForeignServer",
	"pg_language": "Language",
	"pg_largeobject_metadata": "LargeObject",
	"pg_parameter_acl": "Parameter",
	"pg_namespace": "Schema",
	"pg_sequence": "Sequence",
	"pg_class": "Table",
	"pg_attribute": "TableColumn",
	"pg_tablespace": "Tablespace",
	"pg_type": "Type",
}



// function decideZero(typ: string, name: string, desc: string) {
// 	if (typ === "oid" && /zero/i.test(desc))
// 		return true

// 	// | Classification | Conceptual Meaning   | String Patterns / Keywords          | Example from your list                        |
// 	// | -------------- | -------------------- | ----------------------------------- | --------------------------------------------- |
// 	// | Cast to Null   | Absence of relation  | zero if none, else zero, zero if no | "Final function (zero if none)"               |
// 	// | Cast to Null   | Inapplicable context | zero for a, zero if not a           | "zero for a dropped column"                   |
// 	// | Cast to Null   | Fallback to default  | zero to use, or zero if             | "zero to use a default estimate"              |
// 	// | Cast to Null   | Unknown state        | zero value means.*unknown           | "zero value means the number... is unknown"   |
// 	// | Keep as Zero   | Quantity or index    | Number of, counting from            | "Number of dimensions"                        |
// 	// | Keep as Zero   | Explicit contrast    | zero.*null value                    | "A zero value indicates... A null value says" |
// 	// | Keep as Zero   | Byte characters      | zero byte                           | "If a zero byte ('')"                         |
// 	return false
// }

// function makeRef(tableName: string, name: string, nullable: boolean): [string, string, string, string] {
// 	const sel = `${joinNamespaceName}.nspname::text as ${name}_schema_name, ${joinTableName}.relname::text as ${name}_table_name`
// 	const ty = nullable ? `Option<Ref>` : `Ref`
// 	const exp = `make_ref(${tableName}.${name})`

// 	return [sel, ty, exp, '']
// }

function makeStr(tableName: string, name: string, nullable: boolean): [string, string, string] {
	return nullable
		? ["Option<Str>", `${tableName}.${name}.map(Into::into)`, "None"]
		: ["Str", `${tableName}.${name}.into()`, "()"]
}

function makeAssumedU(tableName: string, name: string, size: 8 | 16 | 32, nullable: boolean): [string, string, string] {
	// TODO agh if it's zeroable or negative then we need to have a case statement
	const u = `u${size}`
	const i = `i${size}`

	return nullable
		? [`Option<${u}>`, `${tableName}.${name}.map(${i}::unsigned_abs)`, "None"]
		: [u, `${tableName}.${name}.unsigned_abs()`, "()"]
}
