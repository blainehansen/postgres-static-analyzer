import { z } from "jsr:@zod/zod@^4.3.6"

export const refToReg: { [key: string]: string } = {
	pg_class: "regclass",	// relation name	pg_type
	pg_collation: "regcollation",	// collation name	"POSIX"
	pg_ts_config: "regconfig",	// text search configuration	english
	pg_ts_dict: "regdictionary",	// text search dictionary	simple
	// pg_namespace: "regnamespace",	// namespace name	pg_catalog
	// pg_operator: "regoper",	// operator name	+
	pg_operator: "regoperator",	// operator with argument types	*(integer, integer) or -(NONE, integer)
	// pg_proc: "regproc",	// function name	sum
	pg_proc: "regprocedure",	// function with argument types	sum(int4)
	// pg_authid: "regrole",	// role name	smithee
	pg_type: "regtype",	// data type name	integer
}

// [pg_default_acl.defaclacl]
// [pg_init_privs.initprivs]
export const aclitemMapping: { [key: string]: string } = {
	pg_database: "Db",
	pg_domain: "Domain",
	pg_proc: "Function",
	pg_foreign_data_wrapper: "ForeignDataWrapper",
	pg_foreign_server: "ForeignServer",
	pg_language: "Language",
	pg_largeobject_metadata: "LargeObject",
	pg_parameter_acl: "Parameter",
	pg_namespace: "Schema",
	pg_sequence: "Sequence",
	pg_class: "Table",
	pg_attribute: "TableColumn",
	pg_tablespace: "Tablespace",
	pg_type: "Type",
	pg_default_acl: "AclDefault",
}

export const ignoredTables = new Set([
	"pg_largeobject",
	"pg_largeobject_metadata",
	"pg_seclabel",
	"pg_shseclabel",
	"pg_statistic",
	"pg_statistic_ext_data",
	"pg_subscription_rel",
	"pg_tablespace",
	"pg_transform",
	"pg_ts_parser",
	"pg_ts_template",
])

export const ColumnInfo = z.strictObject({
	typ: z.string(),
	ref: z.string(),
	desc: z.string(),
})
export type ColumnInfo = z.infer<typeof ColumnInfo>

export const RawColumn = z.intersection(
	ColumnInfo,
	z.strictObject({ name: z.string() }),
)
export type RawColumn = z.infer<typeof RawColumn>

export const RawTable = z.strictObject({
	tableName: z.string(),
	url: z.string(),
	columns: z.array(RawColumn),
})
export type RawTable = z.infer<typeof RawTable>


export const SkipDecision = z.intersection(ColumnInfo, z.strictObject({ skip: z.literal(true) }))
export type SkipDecision = z.infer<typeof SkipDecision>

export const RealColumnDecision = z.strictObject({
	sel: z.string().optional(),
	ty: z.string(),
	exp: z.string().optional(),
	joins: z.array(z.string()).optional(),
	filters: z.array(z.string()).optional(),
	pgEnum: z.string().optional(),
})
export type RealColumnDecision = z.infer<typeof RealColumnDecision>

export const ColumnDecision = z.union([
	SkipDecision,
	z.intersection(ColumnInfo, RealColumnDecision),
])
export type ColumnDecision = z.infer<typeof ColumnDecision>

export const TableDecision = z.strictObject({
	hashCol: z.union([z.string(), z.literal(true)]).optional(),
	columns: z.record(z.string(), ColumnDecision),
})
export type TableDecision = z.infer<typeof TableDecision>

export const ColumnOverride = z.union([z.literal('skip'), z.intersection(
	RealColumnDecision.partial(),
	z.strictObject({ zero: z.literal(true), nullable: z.literal(true) }).partial(),
)])
export type ColumnOverride = z.infer<typeof ColumnOverride>
export const TableOverride = z.union([
	z.literal('review'), z.literal('manual'), z.literal('todo'),
	z.record(z.string(), ColumnOverride),
])
export type TableOverride = z.infer<typeof TableOverride>


export function commonPrefix(url: string, columns: string[], startingIndex: number = 7) {
	if (columns.length === 0) {
		throw "empty columns "
	}
	const first = columns[0]!
	let index = startingIndex
	for (; index >= 1; index--) {
		const currentChar = first[index]
		const allSame = columns.every(s => s[index] === currentChar)
		if (allSame) break
	}

	if (index === 0) {
		console.log(url)
		return null
	}

	return first.substring(0, index + 1)
}
// const prefix = commonPrefix(url, prefixNames)

export function toPascalCase(str: string) {
	return str.replace(/(^\w|_\w)/g, (match) => match.replace('_', '').toUpperCase())
}
