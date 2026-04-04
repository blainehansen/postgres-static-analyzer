import { parse as tomlParse, stringify as tomlStringify } from "jsr:@std/toml@^1.0.11"
import { z } from "jsr:@zod/zod@^4.3.6"
import { RawTable, TableDecision, ColumnDecision, TableOverride, ColumnOverride, commonPrefix, refToReg, aclitemMapping, ignoredTables, toPascalCase } from "./gen_pg_info.utils.ts"
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

	const review = tableOverride === 'review'
	const actualOverride = review ? {} : tableOverride
	console.log(tableName)

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

	const decidedColumns: Dict<ColumnDecision> = {}
	let foundHashColumn: string | true | undefined = undefined
	for (const { name, typ, ref, desc } of columns) {
		const columnOverride = (actualOverride ?? {})[name]
		const [hashColumn, decision] = await decideColumn(tableName, name, typ, ref, desc, columnOverride)
		if (review) {
			console.log('name:', name)
			console.log('hashColumn:', hashColumn)
			console.log('decision:', decision)
			const input = prompt('look good?')
			if (input) Deno.exit(1)
			console.log('')
		}
		decidedColumns[name] = decision
		foundHashColumn ??= hashColumn
	}

	// https://www.postgresql.org/docs/17/sql-comment.html
	const shdescriptionTables = new Set([
		"pg_database",
		"pg_roles",
	])
	if (shdescriptionTables.has(tableName)) {
		decidedColumns["description"] = {
			typ: "text", ref: "", desc: "The comment from pg_shdescription",
			sel: `pg_shdescription.description`, ty: "Option<Str>", exp: `${tableName}.description.map(Into::into)`,
			joins: [`left join pg_shdescription on pg_shdescription.objoid = ${tableName}.oid`],
		}
	}
	const descriptionTables = new Set([
		"pg_class",
		"pg_namespace",
		"pg_am",
		// "pg_aggregate",
		// "pg_cast",
		// "pg_collation",
		// "pg_constraint",
		// "pg_conversion",
		// "pg_extension",
		// "pg_event_trigger",
		"pg_proc",
		// "pg_operator",
		// "pg_opclass",
		// "pg_opfamily",
		// "pg_policy",
		// "pg_language",
		// "pg_publication",
		// "pg_foreign_data_wrapper",
		// "pg_foreign_server",
		// "pg_statistic_ext",
		// "pg_subscription",
		// "pg_ts_config",
		// "pg_ts_dict",
		// "pg_trigger",
		// "pg_type",
	])
	if (descriptionTables.has(tableName)) {
		decidedColumns["description"] = {
			typ: "text", ref: "", desc: "The comment from pg_description",
			sel: `pg_description.description`, ty: "Option<Str>", exp: `${tableName}.description.map(Into::into)`,
			joins: [`left join pg_description on pg_description.objoid = ${tableName}.oid and pg_description.objsubid = 0`],
		}
	}
	else if (tableName === "pg_attribute") {
		decidedColumns["description"] = {
			typ: "text", ref: "", desc: "The comment from pg_description",
			sel: `pg_description.description`, ty: "Option<Str>", exp: `${tableName}.description.map(Into::into)`,
			joins: [`left join pg_description on pg_description.objoid = pg_attribute.attrelid and pg_description.objsubid = pg_attribute.attnum`],
		}
	}
	// else if (tableName === "pg_rules") {
	// 	// TODO need to join to pg_rewrite
	// 	decidedColumns["description"] = {
	// 		typ: "text", ref: "", desc: "The comment from pg_description",
	// 		sel: `pg_description.description`, ty: "Option<Str>", exp: `${tableName}.description.map(Into::into)`,
	// 		joins: [`left join pg_description on pg_description.objoid = pg_attribute.attrelid and pg_description.objsubid = pg_attribute.attnum`],
	// 	}
	// }

	return { columns: decidedColumns, hashCol: foundHashColumn }
}

async function decideColumn(
	tableName: string,
	name: string, typ: string, ref: string, desc: string,
	override: ColumnOverride | undefined
): Promise<[string | true | undefined, ColumnDecision]> {
	if (override === 'skip')
		return [undefined, { typ, ref, desc, skip: true }]
	if (tableName === "pg_attribute" && desc.includes(" copy of "))
		return [undefined, { typ, ref, desc, skip: true }]
	if (tableName === "pg_class" && desc.includes(" (or once had) "))
		return [undefined, { typ, ref, desc, skip: true }]

	const { nullable: ovNullable, zero: ovZero } = override ?? {}
	if (override) {
		delete override.nullable
		delete override.zero
	}

	if (override && override.ty)
		return [undefined, { typ, ref, desc, ...override, ty: override.ty }]

	const nameUniqueTables = new Set([
		"pg_database",
		"pg_am",
		"pg_roles",
		"pg_publication",
		"pg_language",
		"pg_namespace",
	])
	if (typ === "name" && nameUniqueTables.has(tableName)) {
		const sel = `${tableName}.${name}::text`
		const [ty, exp] = makeStr(tableName, name, false)
		return [name, { typ, ref, desc, sel, ty, exp, filters: override?.filters }]
	}
	if (typ === "name") {
		const nullable = /null/i.test(desc)
		const sel = `${tableName}.${name}::text`
		const [ty, exp] = makeStr(tableName, name, nullable)
		return [undefined, { typ, ref, desc, sel, ty, exp, filters: override?.filters }]
	}
	if (name === "oid" && !(tableName in refToReg))
		return [undefined, { typ, ref, desc, skip: true }]
	if (typ === "xid")
		return [undefined, { typ, ref, desc, skip: true }]
	if (name === "oid" && (tableName in refToReg)) {
		const reg = refToReg[tableName]
		if (!reg) throw ''
		const sel = `${tableName}.${name}::${reg}::text`
		const ty = "Qual"
		const exp = `Qual::parse(${tableName}.${name})`
		const hashCol = tableName !== "pg_collation" ? true : undefined
		return [hashCol, { typ, ref, desc, sel, ty, exp }]
	}
	if (typ === "regproc") {
		const zeroable = ovZero ?? /zero/i.test(desc)
		const nullable = ovNullable ?? zeroable ?? false
		const sel = zeroable
			? `case when ${name} = 0 then null else ${name}::regproc::text end`
			: `${name}::regproc::text`
		const ty = nullable ? `Option<Qual>` : `Qual`
		const exp = `${nullable ? 'Qual::maybe_parse' : 'Qual::parse'}(${tableName}.${name})`
		return [undefined, { typ, ref, desc, sel, ty, exp, filters: override?.filters }]
	}
	if (typ === "oid" && ref === "(references pg_authid.oid)") {
		const zeroable = ovZero ?? /zero/i.test(desc)
		const nullable = ovNullable ?? zeroable ?? false
		const sel = zeroable
			? `case when ${tableName}.${name} = 0 then null else pg_get_userbyid(${tableName}.${name})::text end`
			: `pg_get_userbyid(${tableName}.${name})::text`
		const [ty, exp] = makeStr(tableName, name, nullable)
		return [undefined, { typ, ref, desc, sel, ty, exp, filters: override?.filters }]
	}
	if (typ === "oid[]" && ref === "(references pg_authid.oid)") {
		const sel = `pg_temp.format_role_oid_array(${tableName}.${name})`
		const ty = "Vec<Option<Str>>"
		const exp = `${tableName}.${name}.map(|item| item.map(Into::into)).collect()`
		return [undefined, { typ, ref, desc, ...override, sel, ty, exp }]
	}
	if (typ === "oid" && ref === "(references pg_namespace.oid)") {
		const zeroable = ovZero ?? /zero/i.test(desc)
		const nullable = ovNullable ?? zeroable ?? false
		const sel = zeroable
			? `case when ${tableName}.${name} = 0 then null else ${tableName}.${name}::regnamespace::text end`
			: `${tableName}.${name}::regnamespace::text`
		const [ty, exp] = makeStr(tableName, name, nullable)
		// TODO figure out if this is always a safe default
		// const filters = ["not starts_with(nspname, 'pg_temp')", "not starts_with(nspname, 'pg_toast')"]
		return [undefined, { typ, ref, desc, sel, ty, exp, filters: override?.filters }]
	}
	if (typ === "oid" && ref === "(references pg_database.oid)") {
		throw new Error(`handle ${name} manually please, it references pg_database.oid`)
	}

	if (typ === "oid" && ref === "(references pg_am.oid)") {
		const nullable = ovNullable ?? false

		const joinTableName = `${name}_pg_am`
		const sel = `${joinTableName}.amname::text`
		const [ty, exp] = makeStr(tableName, name, nullable)

		const leftPortion = nullable ? 'left ' : ''
		const joins = [
			leftPortion + `join pg_am as ${joinTableName} on ${tableName}.${name} = ${joinTableName}.oid`,
		]
		return [undefined, { typ, ref, desc, ...override, sel, ty, exp, joins }]
	}
	// if (typ === "oid" && ref === "(references pg_publication.oid)") {
	// 	const nullable = ovNullable ?? false

	// 	const joinTableName = `${name}_pg_publication`
	// 	const sel = `${joinTableName}.pubname::text`
	// 	const [ty, exp] = makeStr(tableName, name, nullable)

	// 	const leftPortion = nullable ? 'left ' : ''
	// 	const joins = [
	// 		leftPortion + `join pg_publication as ${joinTableName} on ${tableName}.${name} = ${joinTableName}.oid`,
	// 	]
	// 	return [undefined, { typ, ref, desc, ...override, sel, ty, exp, joins }]
	// }
	const noNamespaceTables = new Set([
		"pg_am",
		"pg_amop",
		"pg_publication",
		"pg_trigger",
	])

	const genericReferences = ref.match(/\(references (\w+)\.oid\)/)
	const genericReferencesTable = genericReferences && genericReferences[1]
	if (typ === "oid" && genericReferencesTable && ignoredTables.has(genericReferencesTable))
		return [undefined, { typ, ref, desc, skip: true }]
	if (typ === "oid" && genericReferencesTable && (genericReferencesTable in refToReg)) {
		const reg = refToReg[genericReferencesTable as keyof typeof refToReg]
		if (!reg) throw ''
		const zeroable = ovZero ?? /zero/i.test(desc)
		const nullable = ovNullable ?? zeroable ?? false
		const sel = zeroable
			? `case when ${tableName}.${name} = 0 then null else ${tableName}.${name}::${reg}::text end`
			: `${tableName}.${name}::${reg}::text`
		const ty = nullable ? "Option<Qual>" : "Qual"
		const exp = `${nullable ? 'Qual::maybe_parse' : 'Qual::parse'}(${tableName}.${name})`
		return [undefined, { typ, ref, desc, ...override, sel, ty, exp }]
	}
	if (typ === "oid[]" && genericReferencesTable && (genericReferencesTable in refToReg)) {
		const reg = refToReg[genericReferencesTable as keyof typeof refToReg]
		if (!reg) throw ''
		const sel = `${tableName}.${name}::${reg}[]::text[]`
		const ty = "Option<Vec<Qual>>"
		const exp = `${tableName}.${name}.map(|items| items.map(Qual::parse).collect())`
		return [undefined, { typ, ref, desc, ...override, sel, ty, exp }]
	}
	if (typ === "oid" && genericReferencesTable && (noNamespaceTables.has(genericReferencesTable))) {
		const nullable = ovNullable ?? false

		const joinTableName = `${name}_${genericReferencesTable}`

		const prefix = getTablePrefix(genericReferencesTable)
		const sel = `${joinTableName}.${prefix}name::text`
		const [ty, exp] = makeStr(tableName, name, nullable)

		const leftPortion = nullable ? 'left ' : ''
		const joins = [
			leftPortion + `join ${genericReferencesTable} as ${joinTableName} on ${tableName}.${name} = ${joinTableName}.oid`,
		]
		return [undefined, { typ, ref, desc, ...override, sel, ty, exp, joins }]
	}
	if (typ === "oid" && genericReferencesTable) {
		const zeroable = ovZero ?? /zero/i.test(desc)
		const nullable = ovNullable ?? zeroable ?? false

		const joinNamespaceName = `${name}_pg_namespace`
		const joinTableName = `${name}_${genericReferencesTable}`

		const prefix = getTablePrefix(genericReferencesTable)
		const sel = `quote_ident(${joinNamespaceName}.nspname) || '.' || quote_ident(${joinTableName}.${prefix}name)`
		const ty = nullable ? `Option<Qual>` : `Qual`
		const exp = `Qual::${nullable ? 'maybe_parse' : 'parse'}(${tableName}.${name})`

		const leftPortion = nullable ? 'left ' : ''
		const joins = [
			leftPortion + `join ${genericReferencesTable} as ${joinTableName} on ${tableName}.${name} = ${joinTableName}.oid`,
			leftPortion + `join pg_namespace as ${joinNamespaceName} on ${joinTableName}.${prefix}namespace = ${joinNamespaceName}.oid`,
		]

		return [undefined, { typ, ref, desc, ...override, sel, ty, exp, joins }]
	}

	if (typ === "char") {
		const ty = `${toPascalCase(tableName)}${toPascalCase(name)}`
		const exp = `${ty}::pg_from_char(${tableName}.${name})`
		const pgEnum = 'TODO'
		const t = `\n# ${desc}\n${tableName}.${name} = { ty="${ty}", exp="${exp}", pgEnum="${pgEnum}" }`
		await Deno.writeTextFile("./gen_pg_info_decisions.pre.toml", t, { append: true })
		return [undefined, { typ, ref, desc, /*sel,*/ ty, exp, pgEnum }]
	}
	if (typ === "char[]") {
		const ty = `${toPascalCase(tableName)}${toPascalCase(name)}`
		const exp = `${tableName}.${name}.map(|items| items.map(${ty}::pg_from_char).collect())`
		const pgEnum = 'TODO'
		const t = `\n# ${desc}\n${tableName}.${name} = { ty="${ty}", exp="${exp}", pgEnum="${pgEnum}" }`
		await Deno.writeTextFile("./gen_pg_info_decisions.pre.toml", t, { append: true })
		return [undefined, { typ, ref, desc, /*sel,*/ ty, exp, pgEnum }]
	}

	if (typ === "bool") {
		return [undefined, { typ, ref, desc, ...override, /*sel,*/ ty: "bool", /*exp,*/ }]
	}
	if (typ === "text") {
		const nullable = ovNullable ?? (/optional/i.test(desc) || /null/i.test(desc))
		const [ty, exp] = makeStr(tableName, name, nullable)
		return [undefined, { typ, ref, desc, /*sel,*/ ty, exp, filters: override?.filters }]
	}
	if (typ === "text[]" && desc.includes("keyword=value") || desc.includes("configuration variables")) {
		const ty = "Option<Vec<Str>>"
		const exp = `${tableName}.${name}.map(|items| items.map(Into::into).collect())`
		return [undefined, { typ, ref, desc, /*sel,*/ ty, exp }]
	}
	if (typ === "text[]") {
		const nullable = ovNullable ?? /null/i.test(desc)
		const ty = nullable ? "Option<Vec<Str>>" : "Vec<Str>"
		const exp = nullable
			? `${tableName}.${name}.map(|items| items.map(Into::into).collect())`
			: `${tableName}.${name}.map(Into::into).collect()`
		return [undefined, { typ, ref, desc, /*sel,*/ ty, exp }]
	}
	if (typ === "aclitem[]") {
		const aclPrefix = aclitemMapping[tableName]; if (!aclPrefix) throw `no acl for ${tableName}`
		const sel = `${name}::text[]`
		const ty = `Option<Vec<aclitem::${aclPrefix}AclItem>>`
		const exp = `${tableName}.${name}.map(|${name}| ${name}.map(|acl| aclitem(acl, &${aclPrefix}GrantParser)).collect())`
		return [undefined, { typ, ref, desc, sel, ty, exp }]
	}
	if (name === "attnum" || (typ === "int2" && ref === "(references pg_attribute.attnum)")) {
		const [ty, exp] = makeAssumedU(tableName, name, 16, false)
		return [undefined, { typ, ref, desc, /*sel,*/ ty, exp, filters: [`${name} > 0`] }]
	}
	if (typ === "int2[]" && ref === "(references pg_attribute.attnum)") {
		const ty = `Option<Vec<u16>>`
		const exp = `${tableName}.${name}.map(|items| items.map(i16::unsigned_abs).collect())`
		return [undefined, { typ, ref, desc, /*sel,*/ ty, exp, filters: [`(${tableName}.${name} is null or not (0 >= any(${tableName}.${name})))`] }]
	}
	if (typ === "int2vector" && ref === "(references pg_attribute.attnum)") {
		const nullable = ovNullable ?? /null/i.test(desc)
		const ty = nullable
			? 'Option<Vec<u16>>'
			: 'Vec<u16>'
		const exp = nullable
			? `${tableName}.${name}.map(|items| items.map(i16::unsigned_abs).collect())`
			: `${tableName}.${name}.map(i16::unsigned_abs).collect()`
		return [undefined, { typ, ref, desc, /*sel,*/ ty, exp }]
	}
	if (name.endsWith("encoding")) {
		const negativeable = /-1/.test(desc)
		const sel = negativeable
			? `case when ${name} < 0 then null else pg_encoding_to_char(${name})::text end`
			: `pg_encoding_to_char(${name})::text`
		const [ty, exp] = makeStr(tableName, name, negativeable)
		return [undefined, { typ, ref, desc, sel, ty, exp }]
	}

	// these two integer categories both assume unsigned. any others need to be overridden
	if (typ === "int2") {
		const nullable = /null/i.test(desc)
		const [ty, exp] = makeAssumedU(tableName, name, 16, nullable)
		return [undefined, { typ, ref, desc, /*sel,*/ ty, exp }]
	}
	if (typ === "int4" && !ref) {
		const negativeable = /-1/.test(desc)
		const sel = negativeable ? `case when ${name} < 0 then null else ${name} end` : undefined
		const [ty, exp] = makeAssumedU(tableName, name, 32, negativeable)
		return [undefined, { typ, ref, desc, sel, ty, exp }]
	}
	if (typ === "int8" && !ref) {
		const ty = "i64"
		return [undefined, { typ, ref, desc, /*sel,*/ ty, /*exp*/ }]
	}
	if (typ === "bytea" && !ref) {
		const ty = "Vec<u8>"
		const exp = `${tableName}.${name}.into()`
		return [undefined, { typ, ref, desc, /*sel,*/ ty, exp }]
	}

	if (typ === "float4") {
		const sel = `${name}::text`
		const [ty, exp] = makeStr(tableName, name, true)
		return [undefined, { typ, ref, desc, sel, ty, exp }]
	}

	const { decision } = await ask.select({
		name: "decision",
		message: `don't know how to handle ${tableName}.${name}: ${typ} ${ref} ${desc}`,
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
		return [undefined, { typ, ref, desc, skip: true }]
	}
	if (decision === "handle") {
		const t = `# ${typ} ${ref} ${desc}\n${tableName}.columns.${name} = { sel="", ty="", expr="", pgEnum="", joins=[], filters=[], zero=false, nullable=false }`
		await Deno.writeTextFile("./gen_pg_info_decisions.pre.toml", t, { append: true })
		Deno.exit(0)
	}

	Deno.exit(1)
}

// const toml = resolved.map(({ fields, tableName }) => {
// 	return `[${tableName}]\n` + fields.map(({ name, typ, ref, desc }) => `[${tableName}.${name}]\n# ${typ} ${ref} ${desc}`).join('\n')
// }).join('\n\n')



// | Classification | Conceptual Meaning   | String Patterns / Keywords          | Example from your list                        |
// | -------------- | -------------------- | ----------------------------------- | --------------------------------------------- |
// | Cast to Null   | Absence of relation  | zero if none, else zero, zero if no | "Final function (zero if none)"               |
// | Cast to Null   | Inapplicable context | zero for a, zero if not a           | "zero for a dropped column"                   |
// | Cast to Null   | Fallback to default  | zero to use, or zero if             | "zero to use a default estimate"              |
// | Cast to Null   | Unknown state        | zero value means.*unknown           | "zero value means the number... is unknown"   |
// | Keep as Zero   | Quantity or index    | Number of, counting from            | "Number of dimensions"                        |
// | Keep as Zero   | Explicit contrast    | zero.*null value                    | "A zero value indicates... A null value says" |
// | Keep as Zero   | Byte characters      | zero byte                           | "If a zero byte ('')"                         |

// function makeQual(tableName: string, name: string, nullable: boolean): [string, string, string, string] {
// 	const sel = `${joinNamespaceName}.nspname::text as ${name}_schema_name, ${joinTableName}.relname::text as ${name}_table_name`
// 	const ty = nullable ? `Option<Qual>` : `Qual`
// 	const exp = `make_ref(${tableName}.${name})`

// 	return [sel, ty, exp, '']
// }

function makeStr(tableName: string, name: string, nullable: boolean): [string, string] {
	return nullable
		? ["Option<Str>", `${tableName}.${name}.map(Into::into)`]
		: ["Str", `${tableName}.${name}.into()`]
}

function makeAssumedU(tableName: string, name: string, size: 8 | 16 | 32, nullable: boolean): [string, string] {
	const u = `u${size}`
	const i = `i${size}`
	return nullable
		? [`Option<${u}>`, `${tableName}.${name}.map(${i}::unsigned_abs)`]
		: [u, `${tableName}.${name}.unsigned_abs()`]
}


function generateBlanks(tableName: string, columns: RawTable['columns']) {
	const formattedQueryColumns: string[] = []
	const formattedStructColumns: string[] = []
	const formattedReflectColumns: string[] = []

	for (const { name, typ, ref, desc } of columns) {
		formattedQueryColumns.push(`${name} TODO as ${name} -- ${typ} ${ref} ${desc}`)
		const prettyRef = ref ? `\`${ref}\`` : ref
		formattedStructColumns.push(`/// \`${typ}\` ${prettyRef} ${desc}\n\t\t\t${name}: TODO,`)
		formattedReflectColumns.push(`${name}: ${tableName}.${name}, // ${typ} ${ref} ${desc}`)
	}

	const blankQuery = dedent(`
		--! reflect_${tableName} : (TODO nullable)
		select
			${formattedQueryColumns.join(",\n\t\t\t")}
		from
			${tableName}
			TODO join
		TODO where
		;
	`)

	const structName = toPascalCase(tableName)
	const blankReflect = dedent(`
		#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
		pub struct ${structName} {
			${formattedStructColumns.join("\n\t\t\t")}
		}
		TODO_impl_hash!(${structName})

		pub async fn reflect_${tableName}(
			client: &PgClient
		) -> Result<TODOSetVec<${structName}>, postgres::Error> {
			let ${tableName}_coll = reflect_crate::queries::reflect_gen::reflect_${tableName}().bind(client)
				.map(|${tableName}| {
					${structName} {
						${formattedReflectColumns.join("\n\t\t\t\t\t\t")}
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
