import { parse } from "jsr:@std/jsonc@^1.0.2"
import { DOMParser, Element } from "jsr:@b-fuze/deno-dom@^0.1.56"
import { z } from "jsr:@zod/zod@^4.3.6"

function toPascalCase(str: string) {
	return str.replace(/(^\w|_\w)/g, (match) => match.replace('_', '').toUpperCase())
}

const Field = z.strictObject({
	colName: z.string(),
	colType: z.string(),
	refPart: z.string(),
	description: z.string(),
})
type Field = z.infer<typeof Field>

async function fetchFields(url: string): Promise<Field[]> {
	const response = await fetch(url)
	if (!response.ok)
		throw new Error(`HTTP ${response.status}: ${response.statusText}`)

	const html = await response.text()
	const doc = new DOMParser().parseFromString(html, "text/html")
	if (doc === null)
		throw new Error("Failed to parse HTML document.")

	// Find the first table.table that contains catalog_table_entry cells
	let targetTable: Element | null = null
	for (const table of doc.querySelectorAll("table.table")) {
		if (table.querySelector("td.catalog_table_entry") !== null) {
			targetTable = table
			break
		}
	}
	if (!targetTable)
		throw new Error("No catalog table found on the page.")


	const fields: Field[] = []
	for (const row of targetTable.querySelectorAll("tbody tr")) {
		const td = row.querySelector("td.catalog_table_entry")
		if (td === null) throw new Error("no catalog_table_entry")

		const colDef = td.querySelector("p.column_definition")
		if (colDef === null) throw new Error("no column_definition")

		const fieldEl = colDef.querySelector("code.structfield")
		const typeEl = colDef.querySelector("code.type")
		if (fieldEl === null || typeEl === null) throw new Error("no structfield or type")

		const colName = fieldEl.textContent.trim()
		const colType = typeEl.textContent.trim()

		const colDefText = (colDef.textContent ?? "").trim().replace(/\s+/g, " ")
		const refPart = colDefText
			.replace(colName, "")
			.replace(colType, "")
			.trim()

		// Collect all <p> elements that are NOT the column_definition paragraph
		const description = Array.from(td.querySelectorAll("p"))
			.filter((p) => !p.classList.contains("column_definition"))
			.map((p) => (p.textContent ?? "").trim().replace(/\s+/g, " "))
			.join(" ")

		fields.push({ colName, colType, refPart, description })
	}

	return fields
}


const pages: [string, string][] = [
	// ["pg_aggregate", "https://www.postgresql.org/docs/17/catalog-pg-aggregate.html"],
	// ["pg_am", "https://www.postgresql.org/docs/17/catalog-pg-am.html"],
	// ["pg_amop", "https://www.postgresql.org/docs/17/catalog-pg-amop.html"],
	// ["pg_amproc", "https://www.postgresql.org/docs/17/catalog-pg-amproc.html"],
	// ["pg_attrdef", "https://www.postgresql.org/docs/17/catalog-pg-attrdef.html"],
	["pg_attribute", "https://www.postgresql.org/docs/17/catalog-pg-attribute.html"],
	// // ["pg_authid", "https://www.postgresql.org/docs/17/catalog-pg-authid.html"],
	// ["pg_roles", "https://www.postgresql.org/docs/17/view-pg-roles.html"],
	// ["pg_auth_members", "https://www.postgresql.org/docs/17/catalog-pg-auth-members.html"],
	// ["pg_cast", "https://www.postgresql.org/docs/17/catalog-pg-cast.html"],
	// ["pg_class", "https://www.postgresql.org/docs/17/catalog-pg-class.html"],
	// ["pg_collation", "https://www.postgresql.org/docs/17/catalog-pg-collation.html"],
	// ["pg_constraint", "https://www.postgresql.org/docs/17/catalog-pg-constraint.html"],
	// ["pg_conversion", "https://www.postgresql.org/docs/17/catalog-pg-conversion.html"],
	// ["pg_database", "https://www.postgresql.org/docs/17/catalog-pg-database.html"],
	// ["pg_db_role_setting", "https://www.postgresql.org/docs/17/catalog-pg-db-role-setting.html"],
	// ["pg_default_acl", "https://www.postgresql.org/docs/17/catalog-pg-default-acl.html"],
	// ["pg_depend", "https://www.postgresql.org/docs/17/catalog-pg-depend.html"],
	// ["pg_description", "https://www.postgresql.org/docs/17/catalog-pg-description.html"],
	// ["pg_char_enum", "https://www.postgresql.org/docs/17/catalog-pg-enum.html"],
	// ["pg_event_trigger", "https://www.postgresql.org/docs/17/catalog-pg-event-trigger.html"],
	// ["pg_extension", "https://www.postgresql.org/docs/17/catalog-pg-extension.html"],
	// ["pg_foreign_data_wrapper", "https://www.postgresql.org/docs/17/catalog-pg-foreign-data-wrapper.html"],
	// ["pg_foreign_server", "https://www.postgresql.org/docs/17/catalog-pg-foreign-server.html"],
	// ["pg_foreign_table", "https://www.postgresql.org/docs/17/catalog-pg-foreign-table.html"],
	// ["pg_index", "https://www.postgresql.org/docs/17/catalog-pg-index.html"],
	// ["pg_inherits", "https://www.postgresql.org/docs/17/catalog-pg-inherits.html"],
	// ["pg_init_privs", "https://www.postgresql.org/docs/17/catalog-pg-init-privs.html"],
	// ["pg_language", "https://www.postgresql.org/docs/17/catalog-pg-language.html"],
	// ["pg_largeobject", "https://www.postgresql.org/docs/17/catalog-pg-largeobject.html"],
	// ["pg_largeobject_metadata", "https://www.postgresql.org/docs/17/catalog-pg-largeobject-metadata.html"],
	// ["pg_namespace", "https://www.postgresql.org/docs/17/catalog-pg-namespace.html"],
	// ["pg_opclass", "https://www.postgresql.org/docs/17/catalog-pg-opclass.html"],
	// ["pg_operator", "https://www.postgresql.org/docs/17/catalog-pg-operator.html"],
	// ["pg_opfamily", "https://www.postgresql.org/docs/17/catalog-pg-opfamily.html"],
	// ["pg_parameter_acl", "https://www.postgresql.org/docs/17/catalog-pg-parameter-acl.html"],
	// ["pg_partitioned_table", "https://www.postgresql.org/docs/17/catalog-pg-partitioned-table.html"],
	// ["pg_policy", "https://www.postgresql.org/docs/17/catalog-pg-policy.html"],
	// ["pg_proc", "https://www.postgresql.org/docs/17/catalog-pg-proc.html"],
	// ["pg_publication", "https://www.postgresql.org/docs/17/catalog-pg-publication.html"],
	// ["pg_publication_namespace", "https://www.postgresql.org/docs/17/catalog-pg-publication-namespace.html"],
	// ["pg_publication_rel", "https://www.postgresql.org/docs/17/catalog-pg-publication-rel.html"],
	// ["pg_range", "https://www.postgresql.org/docs/17/catalog-pg-range.html"],
	// ["pg_replication_origin", "https://www.postgresql.org/docs/17/catalog-pg-replication-origin.html"],
	// ["pg_rewrite", "https://www.postgresql.org/docs/17/catalog-pg-rewrite.html"],
	// ["pg_seclabel", "https://www.postgresql.org/docs/17/catalog-pg-seclabel.html"],
	// ["pg_sequence", "https://www.postgresql.org/docs/17/catalog-pg-sequence.html"],
	// ["pg_shdepend", "https://www.postgresql.org/docs/17/catalog-pg-shdepend.html"],
	// ["pg_shdescription", "https://www.postgresql.org/docs/17/catalog-pg-shdescription.html"],
	// ["pg_shseclabel", "https://www.postgresql.org/docs/17/catalog-pg-shseclabel.html"],
	// ["pg_statistic", "https://www.postgresql.org/docs/17/catalog-pg-statistic.html"],
	// ["pg_statistic_ext", "https://www.postgresql.org/docs/17/catalog-pg-statistic-ext.html"],
	// ["pg_statistic_ext_data", "https://www.postgresql.org/docs/17/catalog-pg-statistic-ext-data.html"],
	// ["pg_subscription", "https://www.postgresql.org/docs/17/catalog-pg-subscription.html"],
	// ["pg_subscription_rel", "https://www.postgresql.org/docs/17/catalog-pg-subscription-rel.html"],
	// ["pg_tablespace", "https://www.postgresql.org/docs/17/catalog-pg-tablespace.html"],
	// ["pg_transform", "https://www.postgresql.org/docs/17/catalog-pg-transform.html"],
	// ["pg_trigger", "https://www.postgresql.org/docs/17/catalog-pg-trigger.html"],
	// ["pg_ts_config", "https://www.postgresql.org/docs/17/catalog-pg-ts-config.html"],
	// ["pg_ts_config_map", "https://www.postgresql.org/docs/17/catalog-pg-ts-config-map.html"],
	// ["pg_ts_dict", "https://www.postgresql.org/docs/17/catalog-pg-ts-dict.html"],
	// ["pg_ts_parser", "https://www.postgresql.org/docs/17/catalog-pg-ts-parser.html"],
	// ["pg_ts_template", "https://www.postgresql.org/docs/17/catalog-pg-ts-template.html"],
	// ["pg_type", "https://www.postgresql.org/docs/17/catalog-pg-type.html"],
	// ["pg_user_mapping", "https://www.postgresql.org/docs/17/catalog-pg-user-mapping.html"],
]


const resolved = await Promise.all(
	pages.map(async ([tableName, url]) => {
		const fields = await fetchFields(url)
		return { tableName, url, fields }
	}),
)

const s = JSON.stringify
const jsonFields = resolved.map(({ tableName, url, fields }) => {
	const formattedFields = fields.map(({ colName: name, colType: typ, description: desc, refPart: ref }) => {
		const query =
			(typ === 'name') ? `${name}::text`
			: (ref === "(references pg_authid.oid)") ? `pg_get_userbyid(${name})::text`
			: (ref === "(references pg_namespace.oid)") ? `pg_namespace.nspname::text as ${name}`
			: (ref === "(references pg_type.oid)") ? `${name}_typ_sch.nspname::text as ${name}_schema_name, ${name}_typ.typname::text as ${name}_name`
			: (/^\(references \w+\.oid\)$/.test(ref)) ? `${name}_TODO_sch.nspname::text as ${name}_schema_name, ${name}_TODO.TODO::text as ${name}_name`
			: null

		const ty =
			(typ === 'name' || typ === 'text') ? "Str"
			: (ref === "(references pg_namespace.oid)") ? "Str"
			: (ref === "(references pg_authid.oid)") ? "Str"
			: (/^\(references \w+\.oid\)$/.test(ref)) ? "Option<Ref>"
			: null

		return `\t\t// ${name} ${typ} ${desc} ${ref}\n\t\t{ "name": ${s(name)}, "query": ${s(query)}, "type": ${s(ty)} }`
	}).join(",\n")
	return `\t// ${url}\n\t{ "name": "${tableName}", "fields": [\n${formattedFields}\n\t] }`
}).join(",\n\n\n")

// Deno.writeTextFile("./extract_pg_info.local.jsonc", `[\n${jsonFields}\n]`)



// async function fetchDecisions() {
// 	const text = await Deno.readTextFile("./extract_pg_info.jsonc");
// 	return z.array(z.strictObject({
// 		name: z.string(),
// 		fields: z.array(FieldDecision)
// 	})).parse(parse(text))
// }

// const FieldDecision = z.strictObject({
// 	name: z.string(),
// 	query: z.string().nullable(), type: z.string().nullable(),
// 	joins: z.array(z.string()).optional(),
// })
// type FieldDecision = z.infer<typeof FieldDecision>

function formatAll(tableName: string, fields: Field[]) {
	const structName = toPascalCase(tableName)

	const nullableQueryFields: string[] = []
	const formattedQueryFields: string[] = []
	const formattedStructFields: string[] = []
	const formattedReflectFields: string[] = []
	const formattedTestFields: string[] = []
	const allJoins: string[] = []
	for (const { colName, colType, description, refPart } of fields) {
		if (!type) return null

		const nullable = type.startsWith("Option<")

		const ref =
			(type === "Str") ? `${tableName}.${name}.into()`
			: (type === "Option<Str>") ? `${tableName}.${name}.map(Into::into)`

			: (type === "Vec<Str>") ? `${tableName}.${name}.map(Into::into).collect()`
			: (type === "Option<Vec<Str>>") ? `${tableName}.${name}.map(|items| items.map(Into::into).collect())`

			: (type === "Ref") ? `make_ref(${tableName}.${name}_schema_name, ${tableName}.${name}_name)`
			: (type === "Option<Ref>") ? `maybe_ref(${tableName}.${name}_schema_name, ${tableName}.${name}_name)`

			: (type === "u8") ? `${tableName}.${name}.unsigned_abs()`
			: (type === "Option<u8>") ? `${tableName}.${name}.map(i8::unsigned_abs)`
			: (type === "u16") ? `${tableName}.${name}.unsigned_abs()`
			: (type === "Option<u16>") ? `${tableName}.${name}.map(i16::unsigned_abs)`
			: (type === "u32") ? `${tableName}.${name}.unsigned_abs()`
			: (type === "Option<u32>") ? `${tableName}.${name}.map(i32::unsigned_abs)`

			: `${tableName}.${name}`

		const actualTest =
			nullable ? "None"
			: "()"

		if (nullable) nullableQueryFields.push(name)
		formattedQueryFields.push(`\t${query ?? name}`)
		formattedStructFields.push(`\t${name}: ${type},`)
		formattedReflectFields.push(`\t\t\t\t${name}: ${ref},`)
		formattedTestFields.push(`\t\t\t\t${name}: ${actualTest},`)

		if (joins)
			Array.prototype.push.apply(allJoins, joins)
	}
	const joinsPortion = !allJoins.length ? '' : '\n' + allJoins.map(j => `\t${j}`).join("\n")

	const query = `\
--! reflect_${tableName} : (${nullableQueryFields.map(f => `${f}?`).join(", ")})
select
${formattedQueryFields.join(",\n")}
from
	${tableName}${joinsPortion}
;`

	const struct = `\
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ${structName} {
${formattedStructFields.join("\n")}
}`

	const reflect = `\
pub async fn reflect_${tableName}(
	client: &PgClient
) -> Result<Set<${structName}}>, postgres::Error> {
	let ${tableName}_set = reflect_crate::queries::main::reflect_${tableName}().bind(client)
		.map(|${tableName}| {
			${structName} {
${formattedReflectFields.join("\n")}
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(${tableName}_set)
}`

	const test = `\
#[tokio::test]
async fn test_reflect_${tableName}() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let ${tableName} = reflect::reflect_${tableName}(&client).await?;
		assert!(${tableName}.is_empty());

		client.batch_execute(r#"
		"#).await?;
		let ${tableName} = reflect::reflect_${tableName}(&client).await?;
		assert_eq!(${tableName}, Set::from([
			${structName} {
${formattedTestFields.join("\n")}
			},
		]));

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}`

	return { query, struct, reflect, test }
}

const resolvedDecisions: NonNullable<ReturnType<typeof formatAll>>[] = []
for (const { name: tableName, fields } of (await fetchDecisions())) {
	const formatted = formatAll(tableName, fields)
	if (!formatted) continue
	resolvedDecisions.push(formatted)
}

const totalQuery = resolvedDecisions.map(({ query }) => `${query}\n\n`).join('\n')
const totalStruct = resolvedDecisions.map(({ struct }) => `${struct}\n\n`).join('\n')
const totalReflect = resolvedDecisions.map(({ reflect }) => `${reflect}\n\n`).join('\n')
const totalTest = resolvedDecisions.map(({ test }) => `${test}\n\n`).join('\n')

await Promise.all([
	Deno.writeTextFile("./extract_pg_info.local.query.txt", totalQuery),
	Deno.writeTextFile("./extract_pg_info.local.struct.txt", totalStruct),
	Deno.writeTextFile("./extract_pg_info.local.reflect.txt", totalReflect),
	Deno.writeTextFile("./extract_pg_info.local.test.txt", totalTest),
])
