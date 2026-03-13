import { DOMParser, Element } from "jsr:@b-fuze/deno-dom@^0.1.56"
import { RawTable, RawColumn } from "./gen_pg_info.utils.ts"

async function fetchColumns(url: string): Promise<RawColumn[]> {
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


	const fields: RawColumn[] = []
	for (const row of targetTable.querySelectorAll("tbody tr")) {
		const td = row.querySelector("td.catalog_table_entry")
		if (td === null) throw new Error("no catalog_table_entry")

		const colDef = td.querySelector("p.column_definition")
		if (colDef === null) throw new Error("no column_definition")

		const fieldEl = colDef.querySelector("code.structfield")
		const typeEl = colDef.querySelector("code.type")
		if (fieldEl === null || typeEl === null) throw new Error("no structfield or type")

		const name = fieldEl.textContent.trim()
		const typ = typeEl.textContent.trim()

		const colDefText = (colDef.textContent ?? "").trim().replace(/\s+/g, " ")
		const ref = colDefText
			.replace(name, "")
			.replace(typ, "")
			.trim()

		const desc = Array.from(td.querySelectorAll("p"))
			.filter((p) => !p.classList.contains("column_definition"))
			.map((p) => (p.textContent ?? "").trim().replace(/\s+/g, " "))
			.join(" ")

		fields.push({ name, typ, ref, desc })
	}

	return fields
}


const pages: [string, string][] = [
	["pg_aggregate", "https://www.postgresql.org/docs/17/catalog-pg-aggregate.html"],
	["pg_am", "https://www.postgresql.org/docs/17/catalog-pg-am.html"],
	["pg_amop", "https://www.postgresql.org/docs/17/catalog-pg-amop.html"],
	["pg_amproc", "https://www.postgresql.org/docs/17/catalog-pg-amproc.html"],
	["pg_attrdef", "https://www.postgresql.org/docs/17/catalog-pg-attrdef.html"],
	["pg_attribute", "https://www.postgresql.org/docs/17/catalog-pg-attribute.html"],
	// ["pg_authid", "https://www.postgresql.org/docs/17/catalog-pg-authid.html"],
	["pg_roles", "https://www.postgresql.org/docs/17/view-pg-roles.html"],
	["pg_auth_members", "https://www.postgresql.org/docs/17/catalog-pg-auth-members.html"],
	["pg_cast", "https://www.postgresql.org/docs/17/catalog-pg-cast.html"],
	["pg_class", "https://www.postgresql.org/docs/17/catalog-pg-class.html"],
	["pg_collation", "https://www.postgresql.org/docs/17/catalog-pg-collation.html"],
	["pg_constraint", "https://www.postgresql.org/docs/17/catalog-pg-constraint.html"],
	["pg_conversion", "https://www.postgresql.org/docs/17/catalog-pg-conversion.html"],
	["pg_database", "https://www.postgresql.org/docs/17/catalog-pg-database.html"],
	["pg_db_role_setting", "https://www.postgresql.org/docs/17/catalog-pg-db-role-setting.html"],
	["pg_default_acl", "https://www.postgresql.org/docs/17/catalog-pg-default-acl.html"],
	["pg_depend", "https://www.postgresql.org/docs/17/catalog-pg-depend.html"],
	// ugh really complicated, comments point to *any oid*, and specify what *system catalog* the pointed to object lives in
	["pg_description", "https://www.postgresql.org/docs/17/catalog-pg-description.html"],
	["pg_enum", "https://www.postgresql.org/docs/17/catalog-pg-enum.html"],
	["pg_event_trigger", "https://www.postgresql.org/docs/17/catalog-pg-event-trigger.html"],
	["pg_extension", "https://www.postgresql.org/docs/17/catalog-pg-extension.html"],
	["pg_foreign_data_wrapper", "https://www.postgresql.org/docs/17/catalog-pg-foreign-data-wrapper.html"],
	["pg_foreign_server", "https://www.postgresql.org/docs/17/catalog-pg-foreign-server.html"],
	["pg_foreign_table", "https://www.postgresql.org/docs/17/catalog-pg-foreign-table.html"],
	["pg_index", "https://www.postgresql.org/docs/17/catalog-pg-index.html"],
	["pg_inherits", "https://www.postgresql.org/docs/17/catalog-pg-inherits.html"],
	// this one's complicated, it's for grants to objects created by extensions?
	["pg_init_privs", "https://www.postgresql.org/docs/17/catalog-pg-init-privs.html"],
	["pg_language", "https://www.postgresql.org/docs/17/catalog-pg-language.html"],
	// ["pg_largeobject", "https://www.postgresql.org/docs/17/catalog-pg-largeobject.html"],
	// ["pg_largeobject_metadata", "https://www.postgresql.org/docs/17/catalog-pg-largeobject-metadata.html"],
	["pg_namespace", "https://www.postgresql.org/docs/17/catalog-pg-namespace.html"],
	["pg_opclass", "https://www.postgresql.org/docs/17/catalog-pg-opclass.html"],
	["pg_operator", "https://www.postgresql.org/docs/17/catalog-pg-operator.html"],
	["pg_opfamily", "https://www.postgresql.org/docs/17/catalog-pg-opfamily.html"],
	["pg_parameter_acl", "https://www.postgresql.org/docs/17/catalog-pg-parameter-acl.html"],
	["pg_partitioned_table", "https://www.postgresql.org/docs/17/catalog-pg-partitioned-table.html"],
	["pg_policy", "https://www.postgresql.org/docs/17/catalog-pg-policy.html"],
	["pg_proc", "https://www.postgresql.org/docs/17/catalog-pg-proc.html"],
	["pg_publication", "https://www.postgresql.org/docs/17/catalog-pg-publication.html"],
	["pg_publication_namespace", "https://www.postgresql.org/docs/17/catalog-pg-publication-namespace.html"],
	["pg_publication_rel", "https://www.postgresql.org/docs/17/catalog-pg-publication-rel.html"],
	["pg_range", "https://www.postgresql.org/docs/17/catalog-pg-range.html"],
	["pg_replication_origin", "https://www.postgresql.org/docs/17/catalog-pg-replication-origin.html"],
	["pg_rewrite", "https://www.postgresql.org/docs/17/catalog-pg-rewrite.html"],
	["pg_seclabel", "https://www.postgresql.org/docs/17/catalog-pg-seclabel.html"],
	["pg_sequence", "https://www.postgresql.org/docs/17/catalog-pg-sequence.html"],
	["pg_shdepend", "https://www.postgresql.org/docs/17/catalog-pg-shdepend.html"],
	["pg_shdescription", "https://www.postgresql.org/docs/17/catalog-pg-shdescription.html"],
	["pg_shseclabel", "https://www.postgresql.org/docs/17/catalog-pg-shseclabel.html"],
	// ["pg_statistic", "https://www.postgresql.org/docs/17/catalog-pg-statistic.html"],
	["pg_statistic_ext", "https://www.postgresql.org/docs/17/catalog-pg-statistic-ext.html"],
	// ["pg_statistic_ext_data", "https://www.postgresql.org/docs/17/catalog-pg-statistic-ext-data.html"],
	["pg_subscription", "https://www.postgresql.org/docs/17/catalog-pg-subscription.html"],
	// ["pg_subscription_rel", "https://www.postgresql.org/docs/17/catalog-pg-subscription-rel.html"],
	// ["pg_tablespace", "https://www.postgresql.org/docs/17/catalog-pg-tablespace.html"],
	["pg_transform", "https://www.postgresql.org/docs/17/catalog-pg-transform.html"],
	["pg_trigger", "https://www.postgresql.org/docs/17/catalog-pg-trigger.html"],
	["pg_ts_config", "https://www.postgresql.org/docs/17/catalog-pg-ts-config.html"],
	["pg_ts_config_map", "https://www.postgresql.org/docs/17/catalog-pg-ts-config-map.html"],
	["pg_ts_dict", "https://www.postgresql.org/docs/17/catalog-pg-ts-dict.html"],
	["pg_ts_parser", "https://www.postgresql.org/docs/17/catalog-pg-ts-parser.html"],
	["pg_ts_template", "https://www.postgresql.org/docs/17/catalog-pg-ts-template.html"],
	["pg_type", "https://www.postgresql.org/docs/17/catalog-pg-type.html"],
	["pg_user_mapping", "https://www.postgresql.org/docs/17/catalog-pg-user-mapping.html"],
]

const resolved: RawTable[] = await Promise.all(
	pages.map(async ([tableName, url]): Promise<RawTable> => {
		const columns = await fetchColumns(url)
		return { tableName, url, columns }
	}),
)

await Deno.writeTextFile("./extract_pg_info.local.json", JSON.stringify(resolved, null, '\t'))



// | pg_catalog Table/View    | Purely Runtime / Data? | Manipulation Mechanism                                                                                                                                                                                                                                                      |
// | ------------------------ | ---------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
// | pg_aggregate             | No                     | CREATE AGGREGATE                                                                                                                                                                                                                                                            |
// | pg_am                    | No                     | CREATE ACCESS METHOD                                                                                                                                                                                                                                                        |
// | pg_amop                  | No                     | CREATE OPERATOR CLASS                                                                                                                                                                                                                                                       |
// | pg_amproc                | No                     | CREATE OPERATOR CLASS                                                                                                                                                                                                                                                       |
// | pg_attrdef               | No                     | CREATE TABLE / ALTER TABLE ... SET DEFAULT                                                                                                                                                                                                                                  |
// | pg_attribute             | No                     | CREATE TABLE / ALTER TABLE                                                                                                                                                                                                                                                  |
// | pg_authid                | No                     | CREATE ROLE / CREATE USER                                                                                                                                                                                                                                                   |
// | pg_roles (view)          | No                     | View over pg_authid                                                                                                                                                                                                                                                         |
// | pg_auth_members          | No                     | GRANT ROLE                                                                                                                                                                                                                                                                  |
// | pg_cast                  | No                     | CREATE CAST                                                                                                                                                                                                                                                                 |
// | pg_class                 | No (Mixed)             | Primarily schema (CREATE TABLE/INDEX/VIEW), but the server caches runtime statistics like reltuples and relpages here during VACUUM or ANALYZE.                                                                                                                             |
// | pg_collation             | No                     | CREATE COLLATION                                                                                                                                                                                                                                                            |
// | pg_constraint            | No                     | ALTER TABLE ... ADD CONSTRAINT                                                                                                                                                                                                                                              |
// | pg_conversion            | No                     | CREATE CONVERSION                                                                                                                                                                                                                                                           |
// | pg_database              | No                     | CREATE DATABASE                                                                                                                                                                                                                                                             |
// | pg_db_role_setting       | No                     | ALTER ROLE / ALTER DATABASE ... SET                                                                                                                                                                                                                                         |
// | pg_default_acl           | No                     | ALTER DEFAULT PRIVILEGES                                                                                                                                                                                                                                                    |
// | pg_depend                | No                     | Updated internally by the server when objects are created/dropped to track dependencies.                                                                                                                                                                                    |
// | pg_description           | No                     | COMMENT ON                                                                                                                                                                                                                                                                  |
// | pg_enum                  | No                     | CREATE TYPE / ALTER TYPE ... ADD VALUE                                                                                                                                                                                                                                      |
// | pg_event_trigger         | No                     | CREATE EVENT TRIGGER                                                                                                                                                                                                                                                        |
// | pg_extension             | No                     | CREATE EXTENSION                                                                                                                                                                                                                                                            |
// | pg_foreign_data_wrapper  | No                     | CREATE FOREIGN DATA WRAPPER                                                                                                                                                                                                                                                 |
// | pg_foreign_server        | No                     | CREATE SERVER                                                                                                                                                                                                                                                               |
// | pg_foreign_table         | No                     | CREATE FOREIGN TABLE                                                                                                                                                                                                                                                        |
// | pg_index                 | No (Mixed)             | Primarily schema (CREATE INDEX), but like pg_class, it holds cached statistic properties for the index.                                                                                                                                                                     |
// | pg_inherits              | No                     | CREATE TABLE ... INHERITS                                                                                                                                                                                                                                                   |
// | pg_init_privs            | No                     | Extension initialization system (Internal).                                                                                                                                                                                                                                 |
// | pg_language              | No                     | CREATE LANGUAGE                                                                                                                                                                                                                                                             |
// | pg_largeobject           | Yes (User Data)        | Holds actual blob user data . Manipulated by client functions like lo_create(), lo_write(), and lo_put() rather than standard DDL statements .                                                                                                                              |
// | pg_largeobject_metadata  | Yes (User Metadata)    | Tracks ownership and permissions for the blobs in pg_largeobject . Records are created implicitly via lo_create() and modified via GRANT / REVOKE .                                                                                                                         |
// | pg_namespace             | No                     | CREATE SCHEMA                                                                                                                                                                                                                                                               |
// | pg_opclass               | No                     | CREATE OPERATOR CLASS                                                                                                                                                                                                                                                       |
// | pg_operator              | No                     | CREATE OPERATOR                                                                                                                                                                                                                                                             |
// | pg_opfamily              | No                     | CREATE OPERATOR FAMILY                                                                                                                                                                                                                                                      |
// | pg_parameter_acl         | No                     | GRANT ON PARAMETER                                                                                                                                                                                                                                                          |
// | pg_partitioned_table     | No                     | CREATE TABLE ... PARTITION BY                                                                                                                                                                                                                                               |
// | pg_policy                | No                     | CREATE POLICY                                                                                                                                                                                                                                                               |
// | pg_proc                  | No                     | CREATE FUNCTION / CREATE PROCEDURE                                                                                                                                                                                                                                          |
// | pg_publication           | No                     | CREATE PUBLICATION                                                                                                                                                                                                                                                          |
// | pg_publication_namespace | No                     | CREATE PUBLICATION ... FOR ALL TABLES IN SCHEMA                                                                                                                                                                                                                             |
// | pg_publication_rel       | No                     | CREATE PUBLICATION ... FOR TABLE                                                                                                                                                                                                                                            |
// | pg_range                 | No                     | CREATE TYPE ... AS RANGE                                                                                                                                                                                                                                                    |
// | pg_replication_origin    | No                     | Schema mapping for origins created via pg_replication_origin_create(). (Actual replication progress LSNs are kept in shared memory/disk, not this catalog).                                                                                                                 |
// | pg_rewrite               | No                     | CREATE RULE                                                                                                                                                                                                                                                                 |
// | pg_seclabel              | No                     | SECURITY LABEL ON                                                                                                                                                                                                                                                           |
// | pg_sequence              | No                     | Holds sequence schema parameters (CREATE SEQUENCE); actual current values are stored in the sequence relation itself.                                                                                                                                                       |
// | pg_shdepend              | No                     | Server tracks shared dependencies internally.                                                                                                                                                                                                                               |
// | pg_shdescription         | No                     | COMMENT ON (for shared objects like databases/roles).                                                                                                                                                                                                                       |
// | pg_shseclabel            | No                     | SECURITY LABEL ON (for shared objects).                                                                                                                                                                                                                                     |
// | pg_statistic             | Yes (Runtime)          | Populated and continuously updated entirely by the running server via autovacuum workers or manual ANALYZE commands . No CREATE statements insert directly into this table .                                                                                                |
// | pg_statistic_ext         | No                     | Defines extended stats objects via CREATE STATISTICS.                                                                                                                                                                                                                       |
// | pg_statistic_ext_data    | Yes (Runtime)          | Populated by the server (ANALYZE) based on the tracking definitions configured in pg_statistic_ext .                                                                                                                                                                        |
// | pg_subscription          | No                     | CREATE SUBSCRIPTION                                                                                                                                                                                                                                                         |
// | pg_subscription_rel      | Yes (State)            | Holds the synchronization state of individual tables in a subscription . Rows are initialized via CREATE SUBSCRIPTION or ALTER SUBSCRIPTION ... REFRESH PUBLICATION, but the state codes (srsubstate) are continually updated by the running replication worker processes . |
// | pg_tablespace            | No                     | CREATE TABLESPACE                                                                                                                                                                                                                                                           |
// | pg_transform             | No                     | CREATE TRANSFORM                                                                                                                                                                                                                                                            |
// | pg_trigger               | No                     | CREATE TRIGGER                                                                                                                                                                                                                                                              |
// | pg_ts_config             | No                     | CREATE TEXT SEARCH CONFIGURATION                                                                                                                                                                                                                                            |
// | pg_ts_config_map         | No                     | ALTER TEXT SEARCH CONFIGURATION ... ADD MAPPING                                                                                                                                                                                                                             |
// | pg_ts_dict               | No                     | CREATE TEXT SEARCH DICTIONARY                                                                                                                                                                                                                                               |
// | pg_ts_parser             | No                     | CREATE TEXT SEARCH PARSER                                                                                                                                                                                                                                                   |
// | pg_ts_template           | No                     | CREATE TEXT SEARCH TEMPLATE                                                                                                                                                                                                                                                 |
// | pg_type                  | No                     | CREATE TYPE                                                                                                                                                                                                                                                                 |
// | pg_user_mapping          | No                     | CREATE USER MAPPING                                                                                                                                                                                                                                                         |
