import { z } from "jsr:@zod/zod@^4.3.6"

export const refToReg = {
	pg_class: "regclass",	// relation name	pg_type
	pg_collation: "regcollation",	// collation name	"POSIX"
	pg_ts_config: "regconfig",	// text search configuration	english
	pg_ts_dict: "regdictionary",	// text search dictionary	simple
	pg_namespace: "regnamespace",	// namespace name	pg_catalog
	// pg_operator: "regoper",	// operator name	+
	pg_operator: "regoperator",	// operator with argument types	*(integer, integer) or -(NONE, integer)
	// pg_proc: "regproc",	// function name	sum
	pg_proc: "regprocedure",	// function with argument types	sum(int4)
	// pg_authid: "regrole",	// role name	smithee
	pg_type: "regtype",	// data type name	integer
}

export const RawColumn = z.strictObject({
	name: z.string(),
	typ: z.string(),
	ref: z.string(),
	desc: z.string(),
})
export type RawColumn = z.infer<typeof RawColumn>

export const RawTable = z.strictObject({
	tableName: z.string(),
	url: z.string(),
	columns: z.array(RawColumn),
})
export type RawTable = z.infer<typeof RawTable>


export const SkipDecision = z.strictObject({
	skip: z.literal(true),
})
export type SkipDecision = z.infer<typeof SkipDecision>

export const RealColumnDecision = z.strictObject({
	sel: z.string().optional(),
	ty: z.string(),
	exp: z.string().optional(),
	// test: z.string(),
	joins: z.array(z.string()).optional(),
	filters: z.array(z.string()).optional(),
	pgEnum: z.string().optional(),
})
export type RealColumnDecision = z.infer<typeof RealColumnDecision>

export const ColumnDecision = z.union([SkipDecision, RealColumnDecision])
export type ColumnDecision = z.infer<typeof ColumnDecision>

export const TableDecision = z.strictObject({
	totalQuery: z.string().optional(),
	columns: z.record(z.string(), ColumnDecision),
	hashCol: z.union([z.string(), z.literal(true)]).optional(),
})
export type TableDecision = z.infer<typeof TableDecision>

export const ColumnOverride = z.union([SkipDecision, z.intersection(
	RealColumnDecision.partial(),
	z.strictObject({ zero: z.literal(true), nullable: z.literal(true) }).partial(),
)])
export type ColumnOverride = z.infer<typeof ColumnOverride>
export const TableOverride = z.strictObject({
	columns: z.record(z.string(), ColumnOverride).optional(),
	hashCol: z.union([z.string(), z.literal(true)]).optional(),
})
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
