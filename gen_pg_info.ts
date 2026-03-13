import { parse as tomlParse } from "jsr:@std/toml@^1.0.11"
import { z } from "jsr:@zod/zod@^4.3.6"
import { dedent } from "npm:ts-dedent@^2.2.0"

import { TableDecision, toPascalCase } from "./gen_pg_info.utils.ts"

const decisions = await Deno.readTextFile("./gen_pg_info_decisions.toml")
	.then(t => z.record(z.string(), TableDecision).parse(tomlParse(t)))

const formatted = Object.entries(decisions).map(([tableName, tableDecision]) => formatTable(tableName, tableDecision))

const reflectQueriesText = formatted.map(({ query }) => `${query}\n\n`).join('\n')
const pgStructsText = formatted.map(({ struct }) => `${struct}\n\n`).join('\n')
const pgStructsReflectText = formatted.map(({ reflect }) => `${reflect}\n\n`).join('\n')
// const pgStructsReflectTestText = formatted.map(({ test }) => `${test}\n\n`).join('\n')

await Promise.all([
	Deno.writeTextFile("./reflect_queries/main.local.sql", reflectQueriesText),
	Deno.writeTextFile("./lib/pg_structs.local.rs", pgStructsText),
	Deno.writeTextFile("./lib/pg_structs_reflect.local.rs", pgStructsReflectText),
	// Deno.writeTextFile("./lib/pg_structs_reflect_test.local.rs", pgStructsReflectTestText),
])


function formatTable(
	tableName: string, { totalQuery, columns, hashCol }: TableDecision,
) {
	const structName = toPascalCase(tableName)

	const nullableQueryColumns: string[] = []
	const formattedQueryColumns: string[] = []
	const formattedStructColumns: string[] = []
	const formattedReflectColumns: string[] = []
	// const formattedTestColumns: string[] = []
	const allJoins: string[] = []
	const allFilters: string[] = []
	const allPgEnums: string[] = []
	for (const [name, column] of Object.entries(columns)) {
		if ('skip' in column) {
			formattedQueryColumns.push(`\t\t\t-- ${name}`)
			formattedStructColumns.push(`\t\t\t// ${name},`)
			formattedReflectColumns.push(`\t\t\t\t\t\t// ${name},`)
			// formattedTestColumns.push(`\t\t\t\t\t\t//${name},`)
			continue
		}

		const { sel, ty, exp, /*test,*/ joins, filters, pgEnum } = column
		const nullable = ty.startsWith('Option<')
		const actualExp = exp ?? `${tableName}.${name}`

		if (nullable) nullableQueryColumns.push(name)
		formattedQueryColumns.push(`\t\t\t${sel ?? name}`)
		formattedStructColumns.push(`\t\t\t${name}: ${ty},`)
		formattedReflectColumns.push(`\t\t\t\t\t\t${name}: ${actualExp},`)
		// formattedTestColumns.push(`\t\t\t\t\t\t${name}: ${test},`)

		if (joins && joins.length)
			Array.prototype.push.apply(allJoins, joins)
		if (filters && filters.length)
			Array.prototype.push.apply(allFilters, filters)
		if (pgEnum)
			allPgEnums.push(pgEnum)
	}

	const joinsPortion = !allJoins.length ? '' : '\n' + allJoins.map(j => `\t\t\t${j}`).join("\n")
	const filtersPortion = !allFilters.length ? '' : 'where ' + ('\n' + allFilters.map(f => `\t\t\t${f}`).join("\nand "))
	const query = totalQuery ?? dedent(`
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
	const pgEnumsPortion = !allPgEnums.length ? '' : '\n' + allPgEnums.map(j => `\t\t${j};`).join("\n")
	const struct = dedent(`
		#[derive(Debug, PartialEq, Eq, Clone)]
		pub struct ${structName} {
			${formattedStructColumns.join("\n")}
		}${implHashPortion}${pgEnumsPortion}
	`)

	const collection = hashCol ? 'Set' : 'Vec'
	const reflect = dedent(`
		pub async fn reflect_${tableName}(
			client: &PgClient
		) -> Result<${collection}<${structName}>, postgres::Error> {
			let ${tableName}_coll = reflect_crate::queries::main::reflect_${tableName}().bind(client)
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

	// const test = dedent(`
	// 	#[tokio::test]
	// 	async fn test_reflect_${tableName}() -> anyhow::Result<()> {
	// 		temp_container_utils::with_temp_postgres_client(async |_, _, client| {
	// 			let ${tableName} = reflect::reflect_${tableName}(&client).await?;
	// 			assert!(${tableName}.is_empty());

	// 			client.batch_execute(r#"
	// 			"#).await?;
	// 			let ${tableName} = reflect::reflect_${tableName}(&client).await?;
	// 			assert_eq!(${tableName}, Set::from([
	// 				${structName} {
	// 					${formattedTestColumns.join("\n")}
	// 				},
	// 			]));

	// 			Ok::<_, postgres::Error>(())
	// 		}).await??;

	// 		Ok(())
	// 	}
	// `)

	return { query, struct, reflect, /*test*/ }
}
