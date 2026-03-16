import { parse as tomlParse } from "jsr:@std/toml@^1.0.11"
import { z } from "jsr:@zod/zod@^4.3.6"
import { dedent } from "npm:ts-dedent@^2.2.0"

import { TableDecision, toPascalCase } from "./gen_pg_info.utils.ts"

const decisions = await Deno.readTextFile("./gen_pg_info_decisions.toml")
	.then(t => z.record(z.string(), TableDecision).parse(tomlParse(t)))

const formatted = Object.entries(decisions).map(([tableName, tableDecision]) => formatTable(tableName, tableDecision))

const queryText = formatted.map(({ query }) => `${query}\n\n`).join('\n')
const reflectText = formatted.map(({ reflect }) => `${reflect}\n\n`).join('\n')

await Promise.all([
	Deno.writeTextFile("./reflect_queries/reflect_gen.sql", queryText),
	Deno.writeTextFile("./src/reflect_gen.rs", reflectText),
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
	const allPgEnums: string[] = []
	for (const [name, column] of Object.entries(columns)) {
		if ('skip' in column) {
			const { typ, ref, desc } = column
			formattedQueryColumns.push(`\t\t\t-- ${name} ${typ} ${ref} ${desc}`)
			formattedStructColumns.push(`\t\t\t// ${name} ${typ} ${ref} ${desc}`)
			formattedReflectColumns.push(`\t\t\t\t\t\t// ${name} ${typ} ${ref} ${desc}`)
			continue
		}

		const { sel, ty, exp, joins, filters, pgEnum, typ, ref, desc } = column
		const nullable = ty.startsWith('Option<')
		const actualExp = exp ?? `${tableName}.${name}`

		if (nullable) nullableQueryColumns.push(name)
		formattedQueryColumns.push(`\t\t\t${sel ?? name} -- ${typ} ${ref} ${desc}`)
		formattedStructColumns.push(`\t\t\t${name}: ${ty}, // ${typ} ${ref} ${desc}`)
		formattedReflectColumns.push(`\t\t\t\t\t\t${name}: ${actualExp}, // ${typ} ${ref} ${desc}`)

		if (joins && joins.length)
			Array.prototype.push.apply(allJoins, joins)
		if (filters && filters.length)
			Array.prototype.push.apply(allFilters, filters)
		if (pgEnum)
			allPgEnums.push(pgEnum)
	}

	const joinsPortion = !allJoins.length ? '' : '\n' + allJoins.map(j => `\t\t\t${j}`).join("\n")
	const filtersPortion = !allFilters.length ? '' : 'where ' + ('\n' + allFilters.map(f => `\t\t\t${f}`).join("\nand "))
	const query = dedent(`
		--! reflect_${tableName} : (${nullableQueryColumns.map(f => `${f}?`).join(", ")})
		select
			${formattedQueryColumns.join(",\n")}
		from
			${tableName}${joinsPortion}${filtersPortion}
		;
	`)

	const implHashPortion =
		hashCol === true ? `\n\t\timpl_qual_hash_and_equivalent!(${structName});`
		: hashCol ? `\n\t\timpl_name_hash_and_equivalent!(${structName}, ${hashCol});`
		: ''
	const pgEnumsPortion = !allPgEnums.length ? '' : '\n\n' + allPgEnums.map(j => `\t\tpg_char_enum!(${j});`).join("\n")
	const collection = hashCol ? 'Set' : 'Vec'
	const reflect = dedent(`
		#[derive(Debug, PartialEq, Eq, Clone)]
		pub struct ${structName} {
			${formattedStructColumns.join("\n")}
		}${implHashPortion}${pgEnumsPortion}

		pub async fn reflect_${tableName}(
			client: &PgClient
		) -> Result<${collection}<${structName}>, postgres::Error> {
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

	return { query, reflect }
}
