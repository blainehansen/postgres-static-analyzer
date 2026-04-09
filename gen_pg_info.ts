import { parse as tomlParse } from "jsr:@std/toml@^1.0.11"
import { z } from "jsr:@zod/zod@^4.3.6"
import { dedent } from "npm:ts-dedent@^2.2.0"

import { TableDecision, toPascalCase } from "./gen_pg_info.utils.ts"

const decisions = await Deno.readTextFile("./gen_pg_info_decisions.toml")
	.then(t => z.record(z.string(), TableDecision).parse(tomlParse(t)))

const formatted = Object.entries(decisions).map(([tableName, tableDecision]) => formatTable(tableName, tableDecision))

const structText = 'use super::*;\n\n'
	+ formatted.map(({ struct }) => `${struct}\n\n`).join('\n')

const queryText = formatted.map(({ query }) => `${query}\n\n`).join('\n')
const reflectText = 'use super::*;\nuse futures::TryStreamExt;\nuse crate::aclitem::*;\n\n'
	+ formatted.map(({ reflect }) => `${reflect}\n\n`).join('\n')

await Promise.all([
	Deno.writeTextFile("./catalog_structs/src/struct_gen.rs", structText),
	Deno.writeTextFile("./reflect/queries/query_gen.sql", queryText),
	Deno.writeTextFile("./reflect/src/reflect_gen.rs", reflectText),
])


function formatTable(
	tableName: string, { columns, hashCol }: TableDecision,
) {
	const structName = toPascalCase(tableName)

	const nullableQueryColumns: string[] = []
	const formattedQueryColumns: string[] = []
	const formattedStructColumns: string[] = []
	const formattedReflectColumns: string[] = []
	const allJoins: string[] = []
	const allFilters: string[] = []
	const allPgEnums: [string, string][] = []
	for (const [name, column] of Object.entries(columns)) {
		if ('skip' in column) {
			const { typ, ref, desc } = column
			formattedQueryColumns.push(`-- ${name} ${typ} ${ref} ${desc}`)
			formattedStructColumns.push(`// ${name} ${typ} ${ref} ${desc}`)
			// formattedReflectColumns.push(`// ${name} ${typ} ${ref} ${desc}`)
			continue
		}

		const { sel, ty, exp, joins, filters, pgEnum, typ, ref, desc } = column
		const nullable = ty.startsWith('Option<')
		const actualSel = sel ?? `${tableName}.${name}`
		const actualExp = exp ?? `${tableName}.${name}`

		if (nullable)
			nullableQueryColumns.push(`${name}?`)
		if (ty.startsWith("Vec<Option<"))
			nullableQueryColumns.push(`${name}[?]`)
		formattedQueryColumns.push(`${actualSel} as ${name} -- ${typ} ${ref} ${desc}`)
		const prettyRef = ref ? `\`${ref}\`` : ref
		formattedStructColumns.push(`/// \`${typ}\` ${prettyRef} ${desc}\n\t\t\tpub ${name}: ${ty},`)
		formattedReflectColumns.push(`${name}: ${actualExp},`)

		if (joins && joins.length)
			Array.prototype.push.apply(allJoins, joins)
		if (filters && filters.length)
			Array.prototype.push.apply(allFilters, filters)
		if (pgEnum)
			allPgEnums.push([toPascalCase(name), pgEnum])
	}

	const joinsPortion = !allJoins.length ? '' : '\n' + allJoins.map(j => `\t\t\t${j}`).join("\n")
	const filtersPortion = !allFilters.length ? '' : '\n\t\twhere ' + ('\n\t\t\t' + allFilters.join("\n\t\t\tand "))

	const lastRealIndex = Math.max(...formattedQueryColumns.flatMap((c, i) => c.startsWith('--') ? [] : [i]))
	const finalFormattedQueryColumns = formattedQueryColumns.map((c, i) => {
		const commentIndex = c.indexOf('--') - 1
		return (c.startsWith('--') || i === lastRealIndex) ? c :`${c.slice(0, commentIndex)}, ${c.slice(commentIndex + 1)}`
	}).join("\n\t\t\t")

	const query = dedent(`
		--! reflect_${tableName} : (${nullableQueryColumns.join(", ")})
		select
			${finalFormattedQueryColumns}
		from
			${tableName}${joinsPortion}${filtersPortion}
		;
	`)

	const implHashPortion =
		hashCol === true ? `\n\t\timpl_qual_hash_and_equivalent!(${structName});`
		: hashCol ? `\n\t\timpl_name_hash_and_equivalent!(${structName}, ${hashCol});`
		: ''
	const pgEnumsPortion =
		!allPgEnums.length ? ''
		: '\n\n' + allPgEnums.map(([fieldStructName, members]) => `\t\tpg_char_enum!(${structName}${fieldStructName} { ${members} });`).join("\n")
	const collection = hashCol ? 'Set' : 'Vec'
	const returnPortion = tableName === "pg_database" ? "PgDatabase" : `${collection}<${structName}>`
	const collectPortion = tableName === "pg_database" ? ".one()" : ".iter().await?.try_collect()"

	const struct = dedent(`
		#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
		pub struct ${structName} {
			${formattedStructColumns.join("\n\t\t\t")}
		}${implHashPortion}${pgEnumsPortion}
	`)

	const reflect = dedent(`
		pub async fn reflect_${tableName}(
			client: &PgClient
		) -> Result<${returnPortion}, postgres::Error> {
			let ${tableName}_coll = queries_crate::queries::query_gen::reflect_${tableName}().bind(client)
				.map(|${tableName}| {
					${structName} {
						${formattedReflectColumns.join("\n\t\t\t\t\t\t")}
					}
				})
				${collectPortion}
				.await?;

			Ok(${tableName}_coll)
		}
	`)

	return { query, struct, reflect }
}
