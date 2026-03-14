import { parse as tomlParse, stringify as tomlStringify } from "jsr:@std/toml@^1.0.11"
import { z } from "jsr:@zod/zod@^4.3.6"
import { RawTable, TableDecision, ColumnDecision, TableOverride, ColumnOverride, commonPrefix, refToReg, toPascalCase } from "./gen_pg_info.utils.ts"
import { Ask } from "jsr:@sallai/ask@^2.0.2"
import { dedent } from "npm:ts-dedent@^2.2.0"

const ask = new Ask()

type Dict<T> = { [key: string]: T }

const [rawTables, tableOverrides] = await Promise.all([
	Deno.readTextFile("./extract_pg_info.local.json")
		.then(t => z.array(RawTable).parse(JSON.parse(t))),
	Deno.readTextFile("./gen_pg_info_decisions.pre.toml")
		.then(t => z.record(z.string(), TableOverride).parse(tomlParse(t))),
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
	const decision = await decideTable(rawTable)
	if (decision === null) continue
	finalDecisions[rawTable.tableName] = decision
}

await Deno.writeTextFile("./gen_pg_info_decisions.toml", tomlStringify(finalDecisions))



async function decideTable({ tableName, columns }: RawTable): Promise<TableDecision | null> {
	const tableOverride = tableOverrides[tableName]
	if (tableOverride === 'todo' || tableOverride === 'manual')
		return null

	if (tableOverride === undefined) {
		const { decision } = await ask.select({
			name: "decision",
			message: `How to handle ${tableName}`,
			choices: [
				{ message: "auto", value: "auto" },
				{ message: "manual", value: "manual" },
			],
			default: "auto",
		} as const)
		console.log(decision)

		if (decision === "manual") {
			const { blankQuery, blankReflect } = generateBlanks(tableName, columns)
			await Promise.all([
				Deno.writeTextFile("./reflect_queries/reflect.sql", blankQuery, { append: true }),
				Deno.writeTextFile("./src/reflect.rs", blankReflect, { append: true }),
			])
			Deno.exit(0)
		}
		if (decision === "auto") {
			// if auto do nothing and start processing the columns, which will themselves append things to the file
		}
	}

	// for each column, print out the final decision made if you can come to one, otherwise prompt what to do
	// - if skip, output that to the toml and move on
	// - if handle, Deno.exit(0)

	const decidedColumns: Dict<ColumnDecision> = {}
	let foundHashColumn: string | true | undefined = undefined
	for (const { name, typ, ref, desc } of columns) {
		if (name === "oid" && !(tableName in refToReg)) continue

		if (name === "oid" && (tableName in refToReg)) foundHashColumn = true
		// TODO how do I find the
		// if (typ === "name" && name !== "conname") foundHashColumn = name

		const columnOverride = (tableOverride ?? {})[name]
		decidedColumns[name] = await decideColumn(tableName, name, typ, ref, desc, columnOverride)
	}

	// const hashCol = tableOverride?.hashCol ?? foundHashColumn
	return { columns: decidedColumns, hashCol: foundHashColumn }
}

async function decideColumn(
	tableName: string,
	name: string, typ: string, ref: string, desc: string,
	override: ColumnOverride | undefined
): Promise<ColumnDecision> {
	if (override === 'skip')
		return { typ, ref, desc, skip: true }
	if (tableName === "pg_attribute" && desc.includes(" copy of "))
		return { typ, ref, desc, skip: true }

	const { nullable: ovNullable, zero: ovZero } = override ?? {}
	if (override) {
		delete override.nullable
		delete override.zero
	}

	if (override && override.ty)
		return { typ, ref, desc, ...override, ty: override.ty }

	if (typ === "oid" && (tableName in refToReg)) {
		const reg = refToReg[tableName as keyof typeof refToReg]
		// TODO do we have to guard the selector itself here?
		const sel = `${name}::${reg} as ${name}`
		const [ty, exp] =
			ovNullable || ovZero || /null/i.test(desc) ? ["Option<Qual>", "Qual"]
			: [`Qual::maybe_parse(${tableName}.${name})`, `Qual::parse(${tableName}.${name})`]
		return { typ, ref, desc, ...override, sel, ty, exp }
	}
	if (typ === "bool") {
		return { typ, ref, desc, ...override, /*sel,*/ ty: "bool", /*exp,*/ }
	}
	if (typ === "name") {
		const sel = `${name}::text as ${name}`
		const [ty, exp, ] = makeStr(tableName, name, false)
		return { typ, ref, desc, sel, ty, exp }
	}
	if (typ === "text") {
		const nullable = /null/i.test(desc)
		const [ty, exp, ] = makeStr(tableName, name, nullable)
		return { typ, ref, desc, /*sel,*/ ty, exp }
	}
	if (typ === "text[]" && desc.includes("keyword=value") || desc.includes("configuration variables")) {
		const ty = "Option<Vec<Str>>"
		const exp = `${tableName}.${name}.map(|items| items.map(Into::into).collect())`
		return { typ, ref, desc, /*sel,*/ ty, exp }
	}
	if (typ === "aclitem[]") {
		const aclPrefix = aclitemMapping[tableName]; if (!aclPrefix) throw `no acl for ${tableName}`
		const sel = `${name}::text[] as ${name}`
		const ty = `Option<Vec<aclitem::${aclPrefix}AclItem>>`
		const exp = `${tableName}.${name}.map(|${name}| ${name}.map(|acl| aclitem(acl, &${aclPrefix}GrantParser)).collect())`
		return { typ, ref, desc, sel, ty, exp }
	}
	if (name === "attnum" || ref === "(references pg_attribute.attnum)") {
		const [ty, exp, ] = makeAssumedU(tableName, name, 16, false)
		// TODO filter?
		return { typ, ref, desc, /*sel,*/ ty, exp, filters: [`${name} > 0`] }
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

		return { typ, ref, desc, sel, ty, exp }
	}

	if (ref === "(references pg_authid.oid)") {
		const zeroable = override?.zero ?? /zero/i.test(desc)
		const nullable = override?.nullable ?? zeroable ?? false
		const sel = zeroable
			? `case when ${name} = 0 then null else pg_get_userbyid(${name})::text end as ${name}`
			: `pg_get_userbyid(${name})::text as ${name}`
		const [ty, exp, ] = makeStr(tableName, name, nullable)
		return { typ, ref, desc, sel, ty, exp, filters: override?.filters }
	}

	if (ref === "(references pg_class.oid)") {
		const zeroable = override?.zero ?? /zero/i.test(desc)
		const nullable = override?.nullable ?? zeroable ?? false
		const ty = nullable ? `Option<Qual>` : `Qual`
		const sel = `${name}::regclass as ${name}`
		const exp = `${nullable ? 'maybe_parse_ref' : 'make_parse_ref'}(${tableName}.${name})`
		return { typ, ref, desc, sel, ty, exp }
	}
	if (typ === "oid" && ref === "(references pg_type.oid)") {
		const zeroable = override?.zero ?? /zero/i.test(desc)
		const nullable = override?.nullable ?? zeroable ?? false

		const joinTableName = `${name}_pg_type`
		const joinNamespaceName = `${name}_pg_namespace`

		const ty = nullable ? `Option<Qual>` : `Qual`
		const sel = `${joinNamespaceName}.nspname::text as ${name}_nsp_name, ${joinTableName}.typname::text as ${name}_name`
		const exp = `${nullable ? 'maybe_ref' : 'make_ref'}(${tableName}.${name}_nsp_name, ${tableName}.${name}_name)`

		const leftPortion = nullable ? 'left ' : ''
		const joins = [
			leftPortion + `join pg_type as ${joinTableName} on ${tableName}.${name} = ${joinTableName}.oid`,
			leftPortion + `join pg_namespace as ${joinNamespaceName} on ${joinTableName}.typnamespace = ${joinNamespaceName}.oid`,
		]
		return { typ, ref, desc, sel, ty, exp, joins }
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
			{ message: "handle", value: "handle" },
		],
		default: "skip",
	} as const)
	console.log(decision)

	if (decision === "skip") {
		const t = `${tableName}.columns.${name}.skip = true`
		await Deno.writeTextFile("./gen_pg_info_decisions.pre.toml", t, { append: true })
		return { typ, ref, desc, skip: true }
	}
	if (decision === "handle") {
		const t = `# ${typ} ${ref} ${desc}\n${tableName}.columns.${name} = { sel="", ty="", expr="", pgEnum="", joins=[], filters=[], zero=false, nullable=false }`
		await Deno.writeTextFile("./gen_pg_info_decisions.pre.toml", t, { append: true })
		Deno.exit(0)
	}

	Deno.exit(1)
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

// function makeQual(tableName: string, name: string, nullable: boolean): [string, string, string, string] {
// 	const sel = `${joinNamespaceName}.nspname::text as ${name}_schema_name, ${joinTableName}.relname::text as ${name}_table_name`
// 	const ty = nullable ? `Option<Qual>` : `Qual`
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


function generateBlanks(tableName: string, columns: RawTable['columns']) {
	const formattedQueryColumns: string[] = []
	const formattedStructColumns: string[] = []
	const formattedReflectColumns: string[] = []

	for (const { name, typ, ref, desc } of columns) {
		formattedQueryColumns.push(`\t\t\t${name} TODO as ${name} -- ${typ} ${ref} ${desc}`)
		formattedStructColumns.push(`\t\t\t${name}: TODO, // ${typ} ${ref} ${desc}`)
		formattedReflectColumns.push(`\t\t\t\t\t\t${name}: ${tableName}.${name}, // ${typ} ${ref} ${desc}`)
	}

	const blankQuery = dedent(`
		--! reflect_${tableName} : (TODO nullable)
		select
			${formattedQueryColumns.join(",\n")}
		from
			${tableName}
			TODO join
		TODO where
		;
	`)

	const structName = toPascalCase(tableName)
	const blankReflect = dedent(`
		#[derive(Debug, PartialEq, Eq, Clone)]
		pub struct ${structName} {
			${formattedStructColumns.join("\n")}
		}
		TODO_impl_hash!(${structName})

		pub async fn reflect_${tableName}(
			client: &PgClient
		) -> Result<TODOSetVec<${structName}>, postgres::Error> {
			let ${tableName}_coll = reflect_crate::queries::reflect_gen::reflect_${tableName}().bind(client)
				.map(|${tableName}| {
					${structName} {
						${formattedReflectColumns.join("\n")}
					}
				})
				.iter()
				.await?
				.try_collect()
				.await?;

			Ok(${tableName}_coll)
		}
	`)

	return { blankQuery, blankReflect }
}
