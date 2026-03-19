use super::*;
use futures::TryStreamExt;
use crate::aclitem::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgAggregate {
	/// `regproc` `(references pg_proc.oid)` pg_proc OID of the aggregate function
	aggfnoid: Qual,
	/// `char`  Aggregate kind: n for “normal” aggregates, o for “ordered-set” aggregates, or h for “hypothetical-set” aggregates
	aggkind: PgAggregateAggkind,
	/// `int2`  Number of direct (non-aggregated) arguments of an ordered-set or hypothetical-set aggregate, counting a variadic array as one argument. If equal to pronargs, the aggregate must be variadic and the variadic array describes the aggregated arguments as well as the final direct arguments. Always zero for normal aggregates.
	aggnumdirectargs: u16,
	/// `regproc` `(references pg_proc.oid)` Transition function
	aggtransfn: Qual,
	/// `regproc` `(references pg_proc.oid)` Final function (zero if none)
	aggfinalfn: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Combine function (zero if none)
	aggcombinefn: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Serialization function (zero if none)
	aggserialfn: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Deserialization function (zero if none)
	aggdeserialfn: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Forward transition function for moving-aggregate mode (zero if none)
	aggmtransfn: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Inverse transition function for moving-aggregate mode (zero if none)
	aggminvtransfn: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Final function for moving-aggregate mode (zero if none)
	aggmfinalfn: Option<Qual>,
	/// `bool`  True to pass extra dummy arguments to aggfinalfn
	aggfinalextra: bool,
	/// `bool`  True to pass extra dummy arguments to aggmfinalfn
	aggmfinalextra: bool,
	/// `char`  Whether aggfinalfn modifies the transition state value: r if it is read-only, s if the aggtransfn cannot be applied after the aggfinalfn, or w if it writes on the value
	aggfinalmodify: PgAggregateAggfinalmodify,
	/// `char`  Like aggfinalmodify, but for the aggmfinalfn
	aggmfinalmodify: PgAggregateAggmfinalmodify,
	/// `oid` `(references pg_operator.oid)` Associated sort operator (zero if none)
	aggsortop: Option<Qual>,
	/// `oid` `(references pg_type.oid)` Data type of the aggregate function's internal transition (state) data
	aggtranstype: Qual,
	// aggtransspace int4  Approximate average size (in bytes) of the transition state data, or zero to use a default estimate
	/// `oid` `(references pg_type.oid)` Data type of the aggregate function's internal transition (state) data for moving-aggregate mode (zero if none)
	aggmtranstype: Option<Qual>,
	// aggmtransspace int4  Approximate average size (in bytes) of the transition state data for moving-aggregate mode, or zero to use a default estimate
	/// `text`  The initial value of the transition state. This is a text field containing the initial value in its external string representation. If this field is null, the transition state value starts out null.
	agginitval: Option<Str>,
	/// `text`  The initial value of the transition state for moving-aggregate mode. This is a text field containing the initial value in its external string representation. If this field is null, the transition state value starts out null.
	aggminitval: Option<Str>,
}

pg_char_enum!(PgAggregateAggkind { 'n' => Normal, 'o' => OrderedSet, 'h' => HypotheticalSet });
pg_char_enum!(PgAggregateAggfinalmodify { 'r' => ReadOnly, 's' => CannotApply, 'w' => Writes });
pg_char_enum!(PgAggregateAggmfinalmodify { 'r' => ReadOnly, 's' => CannotApply, 'w' => Writes });

pub async fn reflect_pg_aggregate(
	client: &PgClient
) -> Result<Vec<PgAggregate>, postgres::Error> {
	let pg_aggregate_coll = reflect_crate::queries::reflect_gen::reflect_pg_aggregate().bind(client)
		.map(|pg_aggregate| {
			PgAggregate {
				aggfnoid: Qual::parse(pg_aggregate.aggfnoid),
				aggkind: PgAggregateAggkind::pg_from_char(pg_aggregate.aggkind),
				aggnumdirectargs: pg_aggregate.aggnumdirectargs.unsigned_abs(),
				aggtransfn: Qual::parse(pg_aggregate.aggtransfn),
				aggfinalfn: Qual::maybe_parse(pg_aggregate.aggfinalfn),
				aggcombinefn: Qual::maybe_parse(pg_aggregate.aggcombinefn),
				aggserialfn: Qual::maybe_parse(pg_aggregate.aggserialfn),
				aggdeserialfn: Qual::maybe_parse(pg_aggregate.aggdeserialfn),
				aggmtransfn: Qual::maybe_parse(pg_aggregate.aggmtransfn),
				aggminvtransfn: Qual::maybe_parse(pg_aggregate.aggminvtransfn),
				aggmfinalfn: Qual::maybe_parse(pg_aggregate.aggmfinalfn),
				aggfinalextra: pg_aggregate.aggfinalextra,
				aggmfinalextra: pg_aggregate.aggmfinalextra,
				aggfinalmodify: PgAggregateAggfinalmodify::pg_from_char(pg_aggregate.aggfinalmodify),
				aggmfinalmodify: PgAggregateAggmfinalmodify::pg_from_char(pg_aggregate.aggmfinalmodify),
				aggsortop: Qual::maybe_parse(pg_aggregate.aggsortop),
				aggtranstype: Qual::parse(pg_aggregate.aggtranstype),
				aggmtranstype: Qual::maybe_parse(pg_aggregate.aggmtranstype),
				agginitval: pg_aggregate.agginitval.map(Into::into),
				aggminitval: pg_aggregate.aggminitval.map(Into::into),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_aggregate_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgAm {
	// oid oid  Row identifier
	/// `name`  Name of the access method
	amname: Str,
	/// `regproc` `(references pg_proc.oid)` OID of a handler function that is responsible for supplying information about the access method
	amhandler: Qual,
	/// `char`  t = table (including materialized views), i = index.
	amtype: PgAmAmtype,
}
impl_name_hash_and_equivalent!(PgAm, amname);

pg_char_enum!(PgAmAmtype { 't' => Table, 'i' => Index });

pub async fn reflect_pg_am(
	client: &PgClient
) -> Result<Set<PgAm>, postgres::Error> {
	let pg_am_coll = reflect_crate::queries::reflect_gen::reflect_pg_am().bind(client)
		.map(|pg_am| {
			PgAm {
				amname: pg_am.amname.into(),
				amhandler: Qual::parse(pg_am.amhandler),
				amtype: PgAmAmtype::pg_from_char(pg_am.amtype),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_am_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgAmop {
	// oid oid  Row identifier
	/// `oid` `(references pg_opfamily.oid)` The operator family this entry is for
	amopfamily: Qual,
	/// `oid` `(references pg_type.oid)` Left-hand input data type of operator
	amoplefttype: Qual,
	/// `oid` `(references pg_type.oid)` Right-hand input data type of operator
	amoprighttype: Qual,
	/// `int2`  Operator strategy number
	amopstrategy: u16,
	/// `char`  Operator purpose, either s for search or o for ordering
	amoppurpose: PgAmopAmoppurpose,
	/// `oid` `(references pg_operator.oid)` OID of the operator
	amopopr: Qual,
	/// `oid` `(references pg_am.oid)` Index access method operator family is for
	amopmethod: Str,
	/// `oid` `(references pg_opfamily.oid)` The B-tree operator family this entry sorts according to, if an ordering operator; zero if a search operator
	amopsortfamily: Option<Qual>,
}

pg_char_enum!(PgAmopAmoppurpose { 's' => Search, 'o' => Ordering });

pub async fn reflect_pg_amop(
	client: &PgClient
) -> Result<Vec<PgAmop>, postgres::Error> {
	let pg_amop_coll = reflect_crate::queries::reflect_gen::reflect_pg_amop().bind(client)
		.map(|pg_amop| {
			PgAmop {
				amopfamily: Qual::parse(pg_amop.amopfamily),
				amoplefttype: Qual::parse(pg_amop.amoplefttype),
				amoprighttype: Qual::parse(pg_amop.amoprighttype),
				amopstrategy: pg_amop.amopstrategy.unsigned_abs(),
				amoppurpose: PgAmopAmoppurpose::pg_from_char(pg_amop.amoppurpose),
				amopopr: Qual::parse(pg_amop.amopopr),
				amopmethod: pg_amop.amopmethod.into(),
				amopsortfamily: Qual::maybe_parse(pg_amop.amopsortfamily),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_amop_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgAmproc {
	// oid oid  Row identifier
	/// `oid` `(references pg_opfamily.oid)` The operator family this entry is for
	amprocfamily: Qual,
	/// `oid` `(references pg_type.oid)` Left-hand input data type of associated operator
	amproclefttype: Qual,
	/// `oid` `(references pg_type.oid)` Right-hand input data type of associated operator
	amprocrighttype: Qual,
	/// `int2`  Support function number
	amprocnum: u16,
	/// `regproc` `(references pg_proc.oid)` OID of the function
	amproc: Qual,
}

pub async fn reflect_pg_amproc(
	client: &PgClient
) -> Result<Vec<PgAmproc>, postgres::Error> {
	let pg_amproc_coll = reflect_crate::queries::reflect_gen::reflect_pg_amproc().bind(client)
		.map(|pg_amproc| {
			PgAmproc {
				amprocfamily: Qual::parse(pg_amproc.amprocfamily),
				amproclefttype: Qual::parse(pg_amproc.amproclefttype),
				amprocrighttype: Qual::parse(pg_amproc.amprocrighttype),
				amprocnum: pg_amproc.amprocnum.unsigned_abs(),
				amproc: Qual::parse(pg_amproc.amproc),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_amproc_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgClass {
	/// `oid`  Row identifier
	oid: Qual,
	/// `name`  Name of the table, index, view, etc.
	relname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this relation
	relnamespace: Str,
	/// `oid` `(references pg_type.oid)` The OID of the data type that corresponds to this table's row type, if any; zero for indexes, sequences, and toast tables, which have no pg_type entry
	reltype: Option<Qual>,
	/// `oid` `(references pg_type.oid)` For typed tables, the OID of the underlying composite type; zero for all other relations
	reloftype: Option<Qual>,
	/// `oid` `(references pg_authid.oid)` Owner of the relation
	relowner: Str,
	/// `oid` `(references pg_am.oid)` The access method used to access this table or index. Not meaningful if the relation is a sequence or has no on-disk file, except for partitioned tables, where, if set, it takes precedence over default_table_access_method when determining the access method to use for partitions created when one is not specified in the creation command.
	relam: Option<Str>,
	// relfilenode oid  Name of the on-disk file of this relation; zero means this is a “mapped” relation whose disk file name is determined by low-level state
	// reltablespace oid (references pg_tablespace.oid) The tablespace in which this relation is stored. If zero, the database's default tablespace is implied. Not meaningful if the relation has no on-disk file, except for partitioned tables, where this is the tablespace in which partitions will be created when one is not specified in the creation command.
	// relpages int4  Size of the on-disk representation of this table in pages (of size BLCKSZ). This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX.
	// reltuples float4  Number of live rows in the table. This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX. If the table has never yet been vacuumed or analyzed, reltuples contains -1 indicating that the row count is unknown.
	// relallvisible int4  Number of pages that are marked all-visible in the table's visibility map. This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX.
	// reltoastrelid oid (references pg_class.oid) OID of the TOAST table associated with this table, zero if none. The TOAST table stores large attributes “out of line” in a secondary table.
	// relhasindex bool  True if this is a table and it has (or recently had) any indexes
	/// `bool`  True if this table is shared across all databases in the cluster. Only certain system catalogs (such as pg_database) are shared.
	relisshared: bool,
	/// `char`  p = permanent table/sequence, u = unlogged table/sequence, t = temporary table/sequence
	relpersistence: PgClassRelpersistence,
	/// `char`  r = ordinary table, i = index, S = sequence, t = TOAST table, v = view, m = materialized view, c = composite type, f = foreign table, p = partitioned table, I = partitioned index
	relkind: PgClassRelkind,
	/// `int2`  Number of user columns in the relation (system columns not counted). There must be this many corresponding entries in pg_attribute. See also pg_attribute.attnum.
	relnatts: u16,
	/// `int2`  Number of CHECK constraints on the table; see pg_constraint catalog
	relchecks: u16,
	// relhasrules bool  True if table has (or once had) rules; see pg_rewrite catalog
	// relhastriggers bool  True if table has (or once had) triggers; see pg_trigger catalog
	// relhassubclass bool  True if table or index has (or once had) any inheritance children or partitions
	/// `bool`  True if table has row-level security enabled; see pg_policy catalog
	relrowsecurity: bool,
	/// `bool`  True if row-level security (when enabled) will also apply to table owner; see pg_policy catalog
	relforcerowsecurity: bool,
	// relispopulated bool  True if relation is populated (this is true for all relations other than some materialized views)
	/// `char`  Columns used to form “replica identity” for rows: d = default (primary key, if any), n = nothing, f = all columns, i = index with indisreplident set (same as nothing if the index used has been dropped)
	relreplident: PgClassRelreplident,
	/// `bool`  True if table or index is a partition
	relispartition: bool,
	// relrewrite oid (references pg_class.oid) For new relations being written during a DDL operation that requires a table rewrite, this contains the OID of the original relation; otherwise zero. That state is only visible internally; this field should never contain anything other than zero for a user-visible relation.
	// relfrozenxid xid  All transaction IDs before this one have been replaced with a permanent (“frozen”) transaction ID in this table. This is used to track whether the table needs to be vacuumed in order to prevent transaction ID wraparound or to allow pg_xact to be shrunk. Zero (InvalidTransactionId) if the relation is not a table.
	// relminmxid xid  All multixact IDs before this one have been replaced by a transaction ID in this table. This is used to track whether the table needs to be vacuumed in order to prevent multixact ID wraparound or to allow pg_multixact to be shrunk. Zero (InvalidMultiXactId) if the relation is not a table.
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	relacl: Option<Vec<aclitem::TableAclItem>>,
	/// `text[]`  Access-method-specific options, as “keyword=value” strings
	reloptions: Option<Vec<Str>>,
	/// `pg_node_tree`  If table is a partition (see relispartition), internal representation of the partition bound
	relpartbound: Option<Str>,
}
impl_qual_hash_and_equivalent!(PgClass);

pg_char_enum!(PgClassRelpersistence { 'p' => Permanent, 'u' => Unlogged, 't' => Temporary });
pg_char_enum!(PgClassRelkind { 'r' => Ordinary, 'i' => Index, 'S' => Sequence, 't' => Toast, 'v' => View, 'm' => MaterializedView, 'c' => CompositeType, 'f' => ForeignTable, 'p' => PartitionedTable, 'I' => PartitionedIndex });
pg_char_enum!(PgClassRelreplident { 'd' => Default, 'n' => Nothing, 'f' => AllColumns, 'i' => Index  });

pub async fn reflect_pg_class(
	client: &PgClient
) -> Result<Set<PgClass>, postgres::Error> {
	let pg_class_coll = reflect_crate::queries::reflect_gen::reflect_pg_class().bind(client)
		.map(|pg_class| {
			PgClass {
				oid: Qual::parse(pg_class.oid),
				relname: pg_class.relname.into(),
				relnamespace: pg_class.relnamespace.into(),
				reltype: Qual::maybe_parse(pg_class.reltype),
				reloftype: Qual::maybe_parse(pg_class.reloftype),
				relowner: pg_class.relowner.into(),
				relam: pg_class.relam.map(Into::into),
				relisshared: pg_class.relisshared,
				relpersistence: PgClassRelpersistence::pg_from_char(pg_class.relpersistence),
				relkind: PgClassRelkind::pg_from_char(pg_class.relkind),
				relnatts: pg_class.relnatts.unsigned_abs(),
				relchecks: pg_class.relchecks.unsigned_abs(),
				relrowsecurity: pg_class.relrowsecurity,
				relforcerowsecurity: pg_class.relforcerowsecurity,
				relreplident: PgClassRelreplident::pg_from_char(pg_class.relreplident),
				relispartition: pg_class.relispartition,
				relacl: pg_class.relacl.map(|relacl| relacl.map(|acl| aclitem(acl, &TableGrantParser)).collect()),
				reloptions: pg_class.reloptions.map(|items| items.map(Into::into).collect()),
				relpartbound: pg_class.relpartbound.map(Into::into),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_class_coll)
}

