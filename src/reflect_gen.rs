use super::*;
use futures::TryStreamExt;use crate::aclitem::*;

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
pub struct PgDatabase {
	// oid oid  Row identifier
	/// `name`  Database name
	datname: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the database, usually the user who created it
	datdba: Str,
	/// `int4`  Character encoding for this database (pg_encoding_to_char() can translate this number to the encoding name)
	encoding: Str,
	/// `char`  Locale provider for this database: b = builtin, c = libc, i = icu
	datlocprovider: PgDatabaseDatlocprovider,
	/// `bool`  If true, then this database can be cloned by any user with CREATEDB privileges; if false, then only superusers or the owner of the database can clone it.
	datistemplate: bool,
	/// `bool`  If false then no one can connect to this database. This is used to protect the template0 database from being altered.
	datallowconn: bool,
	// dathasloginevt bool  Indicates that there are login event triggers defined for this database. This flag is used to avoid extra lookups on the pg_event_trigger table during each backend startup. This flag is used internally by PostgreSQL and should not be manually altered or read for monitoring purposes.
	/// `int4`  Sets maximum number of concurrent connections that can be made to this database. -1 means no limit, -2 indicates the database is invalid.
	datconnlimit: Option<u32>,
	// datfrozenxid xid  All transaction IDs before this one have been replaced with a permanent (“frozen”) transaction ID in this database. This is used to track whether the database needs to be vacuumed in order to prevent transaction ID wraparound or to allow pg_xact to be shrunk. It is the minimum of the per-table pg_class.relfrozenxid values.
	// datminmxid xid  All multixact IDs before this one have been replaced with a transaction ID in this database. This is used to track whether the database needs to be vacuumed in order to prevent multixact ID wraparound or to allow pg_multixact to be shrunk. It is the minimum of the per-table pg_class.relminmxid values.
	// dattablespace oid (references pg_tablespace.oid) The default tablespace for the database. Within this database, all tables for which pg_class.reltablespace is zero will be stored in this tablespace; in particular, all the non-shared system catalogs will be there.
	/// `text`  LC_COLLATE for this database
	datcollate: Option<Str>,
	/// `text`  LC_CTYPE for this database
	datctype: Option<Str>,
	/// `text`  Collation provider locale name for this database. If the provider is libc, datlocale is NULL; datcollate and datctype are used instead.
	datlocale: Option<Str>,
	/// `text`  ICU collation rules for this database
	daticurules: Option<Str>,
	/// `text`  Provider-specific version of the collation. This is recorded when the database is created and then checked when it is used, to detect changes in the collation definition that could lead to data corruption.
	datcollversion: Option<Str>,
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	datacl: Option<Vec<aclitem::DbAclItem>>,
}
impl_name_hash_and_equivalent!(PgDatabase, datname);

pg_char_enum!(PgDatabaseDatlocprovider { 'b' => Builtin, 'c' => Libc, 'i' => Icu });

pub async fn reflect_pg_database(
	client: &PgClient
) -> Result<Set<PgDatabase>, postgres::Error> {
	let pg_database_coll = reflect_crate::queries::reflect_gen::reflect_pg_database().bind(client)
		.map(|pg_database| {
			PgDatabase {
				datname: pg_database.datname.into(),
				datdba: pg_database.datdba.into(),
				encoding: pg_database.encoding.into(),
				datlocprovider: PgDatabaseDatlocprovider::pg_from_char(pg_database.datlocprovider),
				datistemplate: pg_database.datistemplate,
				datallowconn: pg_database.datallowconn,
				datconnlimit: pg_database.datconnlimit.map(i32::unsigned_abs),
				datcollate: pg_database.datcollate.map(Into::into),
				datctype: pg_database.datctype.map(Into::into),
				datlocale: pg_database.datlocale.map(Into::into),
				daticurules: pg_database.daticurules.map(Into::into),
				datcollversion: pg_database.datcollversion.map(Into::into),
				datacl: pg_database.datacl.map(|datacl| datacl.map(|acl| aclitem(acl, &DbGrantParser)).collect()),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_database_coll)
}

