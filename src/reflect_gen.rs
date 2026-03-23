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
pub struct PgAttrdef {
	// oid oid  Row identifier
	/// `oid` `(references pg_class.oid)` The table this column belongs to
	adrelid: Qual,
	/// `int2` `(references pg_attribute.attnum)` The number of the column
	adnum: u16,
	/// `pg_node_tree`  The column default value, in nodeToString() representation. Use pg_get_expr(adbin, adrelid) to convert it to an SQL expression.
	adbin: Str,
}

pub async fn reflect_pg_attrdef(
	client: &PgClient
) -> Result<Vec<PgAttrdef>, postgres::Error> {
	let pg_attrdef_coll = reflect_crate::queries::reflect_gen::reflect_pg_attrdef().bind(client)
		.map(|pg_attrdef| {
			PgAttrdef {
				adrelid: Qual::parse(pg_attrdef.adrelid),
				adnum: pg_attrdef.adnum.unsigned_abs(),
				adbin: pg_attrdef.adbin.into(),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_attrdef_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgAttribute {
	/// `oid` `(references pg_class.oid)` The table this column belongs to
	attrelid: Qual,
	/// `name`  The column name
	attname: Str,
	/// `oid` `(references pg_type.oid)` The data type of this column (zero for a dropped column)
	atttypid: Option<Qual>,
	// attlen int2  A copy of pg_type.typlen of this column's type
	/// `int2`  The number of the column. Ordinary columns are numbered from 1 up. System columns, such as ctid, have (arbitrary) negative numbers.
	attnum: u16,
	// attcacheoff int4  Always -1 in storage, but when loaded into a row descriptor in memory this might be updated to cache the offset of the attribute within the row
	/// `int4`  atttypmod records type-specific data supplied at table creation time (for example, the maximum length of a varchar column). It is passed to type-specific input functions and length coercion functions. The value will generally be -1 for types that do not need atttypmod.
	atttypmod: Option<u32>,
	/// `int2`  Number of dimensions, if the column is an array type; otherwise 0. (Presently, the number of dimensions of an array is not enforced, so any nonzero value effectively means “it's an array”.)
	attndims: u16,
	// attbyval bool  A copy of pg_type.typbyval of this column's type
	// attalign char  A copy of pg_type.typalign of this column's type
	// attstorage char  Normally a copy of pg_type.typstorage of this column's type. For TOAST-able data types, this can be altered after column creation to control storage policy.
	/// `char`  The current compression method of the column. Typically this is '\0' to specify use of the current default setting (see default_toast_compression). Otherwise, 'p' selects pglz compression, while 'l' selects LZ4 compression. However, this field is ignored whenever attstorage does not allow compression.
	attcompression: Option<PgAttributeAttcompression>,
	/// `bool`  This represents a not-null constraint.
	attnotnull: bool,
	/// `bool`  This column has a default expression or generation expression, in which case there will be a corresponding entry in the pg_attrdef catalog that actually defines the expression. (Check attgenerated to determine whether this is a default or a generation expression.)
	atthasdef: bool,
	// atthasmissing bool  This column has a value which is used where the column is entirely missing from the row, as happens when a column is added with a non-volatile DEFAULT value after the row is created. The actual value used is stored in the attmissingval column.
	/// `char`  If a zero byte (''), then not an identity column. Otherwise, a = generated always, d = generated by default.
	attidentity: Option<PgAttributeAttidentity>,
	/// `char`  If a zero byte (''), then not a generated column. Otherwise, s = stored. (Other values might be added in the future.)
	attgenerated: Option<PgAttributeAttgenerated>,
	/// `bool`  This column has been dropped and is no longer valid. A dropped column is still physically present in the table, but is ignored by the parser and so cannot be accessed via SQL.
	attisdropped: bool,
	/// `bool`  This column is defined locally in the relation. Note that a column can be locally defined and inherited simultaneously.
	attislocal: bool,
	/// `int2`  The number of direct ancestors this column has. A column with a nonzero number of ancestors cannot be dropped nor renamed.
	attinhcount: u16,
	/// `oid` `(references pg_collation.oid)` The defined collation of the column, or zero if the column is not of a collatable data type
	attcollation: Option<Qual>,
	/// `int2`  attstattarget controls the level of detail of statistics accumulated for this column by ANALYZE. A zero value indicates that no statistics should be collected. A null value says to use the system default statistics target. The exact meaning of positive values is data type-dependent. For scalar data types, attstattarget is both the target number of “most common values” to collect, and the target number of histogram bins to create.
	attstattarget: Option<u16>,
	/// `aclitem[]`  Column-level access privileges, if any have been granted specifically on this column
	attacl: Option<Vec<aclitem::TableColumnAclItem>>,
	/// `text[]`  Attribute-level options, as “keyword=value” strings
	attoptions: Option<Vec<Str>>,
	/// `text[]`  Attribute-level foreign data wrapper options, as “keyword=value” strings
	attfdwoptions: Option<Vec<Str>>,
	// attmissingval anyarray  This column has a one element array containing the value used when the column is entirely missing from the row, as happens when the column is added with a non-volatile DEFAULT value after the row is created. The value is only used when atthasmissing is true. If there is no value the column is null.
}

pg_char_enum!(PgAttributeAttcompression { 'p' => PGLZ, 'l'=> LZ4 });
pg_char_enum!(PgAttributeAttidentity { 'a' => GeneratedAlways, 'd' => GenertedByDefault });
pg_char_enum!(PgAttributeAttgenerated { 's' => Stored });

pub async fn reflect_pg_attribute(
	client: &PgClient
) -> Result<Vec<PgAttribute>, postgres::Error> {
	let pg_attribute_coll = reflect_crate::queries::reflect_gen::reflect_pg_attribute().bind(client)
		.map(|pg_attribute| {
			PgAttribute {
				attrelid: Qual::parse(pg_attribute.attrelid),
				attname: pg_attribute.attname.into(),
				atttypid: Qual::maybe_parse(pg_attribute.atttypid),
				attnum: pg_attribute.attnum.unsigned_abs(),
				atttypmod: pg_attribute.atttypmod.map(i32::unsigned_abs),
				attndims: pg_attribute.attndims.unsigned_abs(),
				attcompression: pg_attribute.attcompression.map(PgAttributeAttcompression::pg_from_char),
				attnotnull: pg_attribute.attnotnull,
				atthasdef: pg_attribute.atthasdef,
				attidentity: pg_attribute.attidentity.map(PgAttributeAttidentity::pg_from_char),
				attgenerated: pg_attribute.attgenerated.map(PgAttributeAttgenerated::pg_from_char),
				attisdropped: pg_attribute.attisdropped,
				attislocal: pg_attribute.attislocal,
				attinhcount: pg_attribute.attinhcount.unsigned_abs(),
				attcollation: Qual::maybe_parse(pg_attribute.attcollation),
				attstattarget: pg_attribute.attstattarget.map(i16::unsigned_abs),
				attacl: pg_attribute.attacl.map(|attacl| attacl.map(|acl| aclitem(acl, &TableColumnGrantParser)).collect()),
				attoptions: pg_attribute.attoptions.map(|items| items.map(Into::into).collect()),
				attfdwoptions: pg_attribute.attfdwoptions.map(|items| items.map(Into::into).collect()),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_attribute_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgRoles {
	/// `name`  Role name
	rolname: Str,
	/// `bool`  Role has superuser privileges
	rolsuper: bool,
	/// `bool`  Role automatically inherits privileges of roles it is a member of
	rolinherit: bool,
	/// `bool`  Role can create more roles
	rolcreaterole: bool,
	/// `bool`  Role can create databases
	rolcreatedb: bool,
	/// `bool`  Role can log in. That is, this role can be given as the initial session authorization identifier
	rolcanlogin: bool,
	/// `bool`  Role is a replication role. A replication role can initiate replication connections and create and drop replication slots.
	rolreplication: bool,
	/// `int4`  For roles that can log in, this sets maximum number of concurrent connections this role can make. -1 means no limit.
	rolconnlimit: Option<u32>,
	// rolpassword text  Not the password (always reads as ********)
	/// `timestamptz`  Password expiry time (only used for password authentication); null if no expiration
	rolvaliduntil: Option<chrono::DateTime<chrono::FixedOffset>>,
	/// `bool`  Role bypasses every row-level security policy, see Section 5.9 for more information.
	rolbypassrls: bool,
	/// `text[]`  Role-specific defaults for run-time configuration variables
	rolconfig: Option<Vec<Str>>,
	// oid oid (references pg_authid.oid) ID of role
}
impl_name_hash_and_equivalent!(PgRoles, rolname);

pub async fn reflect_pg_roles(
	client: &PgClient
) -> Result<Set<PgRoles>, postgres::Error> {
	let pg_roles_coll = reflect_crate::queries::reflect_gen::reflect_pg_roles().bind(client)
		.map(|pg_roles| {
			PgRoles {
				rolname: pg_roles.rolname.into(),
				rolsuper: pg_roles.rolsuper,
				rolinherit: pg_roles.rolinherit,
				rolcreaterole: pg_roles.rolcreaterole,
				rolcreatedb: pg_roles.rolcreatedb,
				rolcanlogin: pg_roles.rolcanlogin,
				rolreplication: pg_roles.rolreplication,
				rolconnlimit: pg_roles.rolconnlimit.map(i32::unsigned_abs),
				rolvaliduntil: pg_roles.rolvaliduntil,
				rolbypassrls: pg_roles.rolbypassrls,
				rolconfig: pg_roles.rolconfig.map(|items| items.map(Into::into).collect()),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_roles_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgAuthMembers {
	// oid oid  Row identifier
	/// `oid` `(references pg_authid.oid)` ID of a role that has a member
	roleid: Str,
	/// `oid` `(references pg_authid.oid)` ID of a role that is a member of roleid
	member: Str,
	/// `oid` `(references pg_authid.oid)` ID of the role that granted this membership
	grantor: Str,
	/// `bool`  True if member can grant membership in roleid to others
	admin_option: bool,
	/// `bool`  True if the member automatically inherits the privileges of the granted role
	inherit_option: bool,
	/// `bool`  True if the member can SET ROLE to the granted role
	set_option: bool,
}

pub async fn reflect_pg_auth_members(
	client: &PgClient
) -> Result<Vec<PgAuthMembers>, postgres::Error> {
	let pg_auth_members_coll = reflect_crate::queries::reflect_gen::reflect_pg_auth_members().bind(client)
		.map(|pg_auth_members| {
			PgAuthMembers {
				roleid: pg_auth_members.roleid.into(),
				member: pg_auth_members.member.into(),
				grantor: pg_auth_members.grantor.into(),
				admin_option: pg_auth_members.admin_option,
				inherit_option: pg_auth_members.inherit_option,
				set_option: pg_auth_members.set_option,
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_auth_members_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgCast {
	// oid oid  Row identifier
	/// `oid` `(references pg_type.oid)` OID of the source data type
	castsource: Qual,
	/// `oid` `(references pg_type.oid)` OID of the target data type
	casttarget: Qual,
	/// `oid` `(references pg_proc.oid)` The OID of the function to use to perform this cast. Zero is stored if the cast method doesn't require a function.
	castfunc: Option<Qual>,
	/// `char`  Indicates what contexts the cast can be invoked in. e means only as an explicit cast (using CAST or :: syntax). a means implicitly in assignment to a target column, as well as explicitly. i means implicitly in expressions, as well as the other cases.
	castcontext: PgCastCastcontext,
	/// `char`  Indicates how the cast is performed. f means that the function specified in the castfunc field is used. i means that the input/output functions are used. b means that the types are binary-coercible, thus no conversion is required.
	castmethod: PgCastCastmethod,
}

pg_char_enum!(PgCastCastcontext { 'e' => Explicit, 'a' => ImplicitAssignment, 'i' => Implicit });
pg_char_enum!(PgCastCastmethod { 'f' => Castfunc, 'i' => IOFunc, 'b' => BinaryCoercible });

pub async fn reflect_pg_cast(
	client: &PgClient
) -> Result<Vec<PgCast>, postgres::Error> {
	let pg_cast_coll = reflect_crate::queries::reflect_gen::reflect_pg_cast().bind(client)
		.map(|pg_cast| {
			PgCast {
				castsource: Qual::parse(pg_cast.castsource),
				casttarget: Qual::parse(pg_cast.casttarget),
				castfunc: Qual::maybe_parse(pg_cast.castfunc),
				castcontext: PgCastCastcontext::pg_from_char(pg_cast.castcontext),
				castmethod: PgCastCastmethod::pg_from_char(pg_cast.castmethod),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_cast_coll)
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


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgCollation {
	/// `oid`  Row identifier
	oid: Qual,
	/// `name`  Collation name (unique per namespace and encoding)
	collname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this collation
	collnamespace: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the collation
	collowner: Str,
	/// `char`  Provider of the collation: d = database default, b = builtin, c = libc, i = icu
	collprovider: PgCollationCollprovider,
	/// `bool`  Is the collation deterministic?
	collisdeterministic: bool,
	/// `int4`  Encoding in which the collation is applicable, or -1 if it works for any encoding
	collencoding: Option<Str>,
	/// `text`  LC_COLLATE for this collation object. If the provider is not libc, collcollate is NULL and colllocale is used instead.
	collcollate: Option<Str>,
	/// `text`  LC_CTYPE for this collation object. If the provider is not libc, collctype is NULL and colllocale is used instead.
	collctype: Option<Str>,
	/// `text`  Collation provider locale name for this collation object. If the provider is libc, colllocale is NULL; collcollate and collctype are used instead.
	colllocale: Option<Str>,
	/// `text`  ICU collation rules for this collation object
	collicurules: Option<Str>,
	/// `text`  Provider-specific version of the collation. This is recorded when the collation is created and then checked when it is used, to detect changes in the collation definition that could lead to data corruption.
	collversion: Option<Str>,
}

pg_char_enum!(PgCollationCollprovider { 'd' => DatabaseDefault, 'b' => Builtin, 'c' => Libc, 'i' => Icu });

pub async fn reflect_pg_collation(
	client: &PgClient
) -> Result<Vec<PgCollation>, postgres::Error> {
	let pg_collation_coll = reflect_crate::queries::reflect_gen::reflect_pg_collation().bind(client)
		.map(|pg_collation| {
			PgCollation {
				oid: Qual::parse(pg_collation.oid),
				collname: pg_collation.collname.into(),
				collnamespace: pg_collation.collnamespace.into(),
				collowner: pg_collation.collowner.into(),
				collprovider: PgCollationCollprovider::pg_from_char(pg_collation.collprovider),
				collisdeterministic: pg_collation.collisdeterministic,
				collencoding: pg_collation.collencoding.map(Into::into),
				collcollate: pg_collation.collcollate.map(Into::into),
				collctype: pg_collation.collctype.map(Into::into),
				colllocale: pg_collation.colllocale.map(Into::into),
				collicurules: pg_collation.collicurules.map(Into::into),
				collversion: pg_collation.collversion.map(Into::into),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_collation_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgConstraint {
	// oid oid  Row identifier
	/// `name`  Constraint name (not necessarily unique!)
	conname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this constraint
	connamespace: Str,
	/// `char`  c = check constraint, f = foreign key constraint, n = not-null constraint (domains only), p = primary key constraint, u = unique constraint, t = constraint trigger, x = exclusion constraint
	contype: PgConstraintContype,
	/// `bool`  Is the constraint deferrable?
	condeferrable: bool,
	/// `bool`  Is the constraint deferred by default?
	condeferred: bool,
	/// `bool`  Has the constraint been validated? Currently, can be false only for foreign keys and CHECK constraints
	convalidated: bool,
	/// `oid` `(references pg_class.oid)` The table this constraint is on; zero if not a table constraint
	conrelid: Option<Qual>,
	/// `oid` `(references pg_type.oid)` The domain this constraint is on; zero if not a domain constraint
	contypid: Option<Qual>,
	/// `oid` `(references pg_class.oid)` The index supporting this constraint, if it's a unique, primary key, foreign key, or exclusion constraint; else zero
	conindid: Option<Qual>,
	/// `oid` `(references pg_constraint.oid)` The corresponding constraint of the parent partitioned table, if this is a constraint on a partition; else zero
	conparentid: Option<Qual>,
	/// `oid` `(references pg_class.oid)` If a foreign key, the referenced table; else zero
	confrelid: Option<Qual>,
	/// `char`  Foreign key update action code: a = no action, r = restrict, c = cascade, n = set null, d = set default
	confupdtype: Option<PgConstraintConfupdtype>,
	/// `char`  Foreign key deletion action code: a = no action, r = restrict, c = cascade, n = set null, d = set default
	confdeltype: Option<PgConstraintConfdeltype>,
	/// `char`  Foreign key match type: f = full, p = partial, s = simple
	confmatchtype: Option<PgConstraintConfmatchtype>,
	/// `bool`  This constraint is defined locally for the relation. Note that a constraint can be locally defined and inherited simultaneously.
	conislocal: bool,
	/// `int2`  The number of direct inheritance ancestors this constraint has. A constraint with a nonzero number of ancestors cannot be dropped nor renamed.
	coninhcount: u16,
	/// `bool`  This constraint is defined locally for the relation. It is a non-inheritable constraint.
	connoinherit: bool,
	/// `int2[]` `(references pg_attribute.attnum)` If a table constraint (including foreign keys, but not constraint triggers), list of the constrained columns
	conkey: Option<Vec<u16>>,
	/// `int2[]` `(references pg_attribute.attnum)` If a foreign key, list of the referenced columns
	confkey: Option<Vec<u16>>,
	/// `oid[]` `(references pg_operator.oid)` If a foreign key, list of the equality operators for PK = FK comparisons
	conpfeqop: Option<Vec<Qual>>,
	/// `oid[]` `(references pg_operator.oid)` If a foreign key, list of the equality operators for PK = PK comparisons
	conppeqop: Option<Vec<Qual>>,
	/// `oid[]` `(references pg_operator.oid)` If a foreign key, list of the equality operators for FK = FK comparisons
	conffeqop: Option<Vec<Qual>>,
	/// `int2[]` `(references pg_attribute.attnum)` If a foreign key with a SET NULL or SET DEFAULT delete action, the columns that will be updated. If null, all of the referencing columns will be updated.
	confdelsetcols: Option<Vec<u16>>,
	/// `oid[]` `(references pg_operator.oid)` If an exclusion constraint, list of the per-column exclusion operators
	conexclop: Option<Vec<Qual>>,
	/// `pg_node_tree`  If a check constraint, an internal representation of the expression. (It's recommended to use pg_get_constraintdef() to extract the definition of a check constraint.)
	conbin: Option<Str>,
}

pg_char_enum!(PgConstraintContype { 'c' => Check, 'f' => ForeignKey, 'n' => DomainNotNull, 'p' => PrimaryKey, 'u' => Unique, 't' => Trigger, 'x' => Exclusion });
pg_char_enum!(PgConstraintConfupdtype { 'a' => NoAction, 'r' => Restrict, 'c' => Cascade, 'n' => SetNull, 'd' => SetDefault });
pg_char_enum!(PgConstraintConfdeltype { 'a' => NoAction, 'r' => Restrict, 'c' => Cascade, 'n' => SetNull, 'd' => SetDefault });
pg_char_enum!(PgConstraintConfmatchtype { 'f' => Full, 'p' => Partial, 's' => Simple });

pub async fn reflect_pg_constraint(
	client: &PgClient
) -> Result<Vec<PgConstraint>, postgres::Error> {
	let pg_constraint_coll = reflect_crate::queries::reflect_gen::reflect_pg_constraint().bind(client)
		.map(|pg_constraint| {
			PgConstraint {
				conname: pg_constraint.conname.into(),
				connamespace: pg_constraint.connamespace.into(),
				contype: PgConstraintContype::pg_from_char(pg_constraint.contype),
				condeferrable: pg_constraint.condeferrable,
				condeferred: pg_constraint.condeferred,
				convalidated: pg_constraint.convalidated,
				conrelid: Qual::maybe_parse(pg_constraint.conrelid),
				contypid: Qual::maybe_parse(pg_constraint.contypid),
				conindid: Qual::maybe_parse(pg_constraint.conindid),
				conparentid: Qual::maybe_parse(pg_constraint.conparentid),
				confrelid: Qual::maybe_parse(pg_constraint.confrelid),
				confupdtype: pg_constraint.confupdtype.map(PgConstraintConfupdtype::pg_from_char),
				confdeltype: pg_constraint.confdeltype.map(PgConstraintConfdeltype::pg_from_char),
				confmatchtype: pg_constraint.confmatchtype.map(PgConstraintConfmatchtype::pg_from_char),
				conislocal: pg_constraint.conislocal,
				coninhcount: pg_constraint.coninhcount.unsigned_abs(),
				connoinherit: pg_constraint.connoinherit,
				conkey: pg_constraint.conkey.map(|items| items.map(i16::unsigned_abs).collect()),
				confkey: pg_constraint.confkey.map(|items| items.map(i16::unsigned_abs).collect()),
				conpfeqop: pg_constraint.conpfeqop.map(|items| items.map(Qual::parse).collect()),
				conppeqop: pg_constraint.conppeqop.map(|items| items.map(Qual::parse).collect()),
				conffeqop: pg_constraint.conffeqop.map(|items| items.map(Qual::parse).collect()),
				confdelsetcols: pg_constraint.confdelsetcols.map(|items| items.map(i16::unsigned_abs).collect()),
				conexclop: pg_constraint.conexclop.map(|items| items.map(Qual::parse).collect()),
				conbin: pg_constraint.conbin.map(Into::into),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_constraint_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgConversion {
	// oid oid  Row identifier
	/// `name`  Conversion name (unique within a namespace)
	conname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this conversion
	connamespace: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the conversion
	conowner: Str,
	/// `int4`  Source encoding ID (pg_encoding_to_char() can translate this number to the encoding name)
	conforencoding: Str,
	/// `int4`  Destination encoding ID (pg_encoding_to_char() can translate this number to the encoding name)
	contoencoding: Str,
	/// `regproc` `(references pg_proc.oid)` Conversion function
	conproc: Qual,
	/// `bool`  True if this is the default conversion
	condefault: bool,
}

pub async fn reflect_pg_conversion(
	client: &PgClient
) -> Result<Vec<PgConversion>, postgres::Error> {
	let pg_conversion_coll = reflect_crate::queries::reflect_gen::reflect_pg_conversion().bind(client)
		.map(|pg_conversion| {
			PgConversion {
				conname: pg_conversion.conname.into(),
				connamespace: pg_conversion.connamespace.into(),
				conowner: pg_conversion.conowner.into(),
				conforencoding: pg_conversion.conforencoding.into(),
				contoencoding: pg_conversion.contoencoding.into(),
				conproc: Qual::parse(pg_conversion.conproc),
				condefault: pg_conversion.condefault,
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_conversion_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgDefaultAcl {
	// oid oid  Row identifier
	/// `oid` `(references pg_authid.oid)` The OID of the role associated with this entry
	defaclrole: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace associated with this entry, or zero if none
	defaclnamespace: Option<Str>,
	/// `char`  Type of object this entry is for: r = relation (table, view), S = sequence, f = function, T = type, n = schema
	defaclobjtype: PgDefaultAclDefaclobjtype,
	/// `aclitem[]`  Access privileges that this type of object should have on creation
	defaclacl: Option<Vec<aclitem::AclDefaultAclItem>>,
}

pg_char_enum!(PgDefaultAclDefaclobjtype { 'r' => Relation, 'S' => Sequence, 'f' => Function, 'T' => Type, 'n' => Schema });

pub async fn reflect_pg_default_acl(
	client: &PgClient
) -> Result<Vec<PgDefaultAcl>, postgres::Error> {
	let pg_default_acl_coll = reflect_crate::queries::reflect_gen::reflect_pg_default_acl().bind(client)
		.map(|pg_default_acl| {
			PgDefaultAcl {
				defaclrole: pg_default_acl.defaclrole.into(),
				defaclnamespace: pg_default_acl.defaclnamespace.map(Into::into),
				defaclobjtype: PgDefaultAclDefaclobjtype::pg_from_char(pg_default_acl.defaclobjtype),
				defaclacl: pg_default_acl.defaclacl.map(|defaclacl| defaclacl.map(|acl| aclitem(acl, &AclDefaultGrantParser)).collect()),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_default_acl_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgEventTrigger {
	// oid oid  Row identifier
	/// `name`  Trigger name (must be unique)
	evtname: Str,
	/// `name`  Identifies the event for which this trigger fires
	evtevent: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the event trigger
	evtowner: Str,
	/// `oid` `(references pg_proc.oid)` The function to be called
	evtfoid: Qual,
	/// `char`  Controls in which session_replication_role modes the event trigger fires. O = trigger fires in “origin” and “local” modes, D = trigger is disabled, R = trigger fires in “replica” mode, A = trigger fires always.
	evtenabled: PgEventTriggerEvtenabled,
	/// `text[]`  Command tags for which this trigger will fire. If NULL, the firing of this trigger is not restricted on the basis of the command tag.
	evttags: Option<Vec<Str>>,
}

pg_char_enum!(PgEventTriggerEvtenabled { 'O' => OriginLocal, 'D' => Disabled, 'R' => Replica, 'A' => Always });

pub async fn reflect_pg_event_trigger(
	client: &PgClient
) -> Result<Vec<PgEventTrigger>, postgres::Error> {
	let pg_event_trigger_coll = reflect_crate::queries::reflect_gen::reflect_pg_event_trigger().bind(client)
		.map(|pg_event_trigger| {
			PgEventTrigger {
				evtname: pg_event_trigger.evtname.into(),
				evtevent: pg_event_trigger.evtevent.into(),
				evtowner: pg_event_trigger.evtowner.into(),
				evtfoid: Qual::parse(pg_event_trigger.evtfoid),
				evtenabled: PgEventTriggerEvtenabled::pg_from_char(pg_event_trigger.evtenabled),
				evttags: pg_event_trigger.evttags.map(|items| items.map(Into::into).collect()),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_event_trigger_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgLanguage {
	// oid oid  Row identifier
	/// `name`  Name of the language
	lanname: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the language
	lanowner: Str,
	/// `bool`  This is false for internal languages (such as SQL) and true for user-defined languages. Currently, pg_dump still uses this to determine which languages need to be dumped, but this might be replaced by a different mechanism in the future.
	lanispl: bool,
	/// `bool`  True if this is a trusted language, which means that it is believed not to grant access to anything outside the normal SQL execution environment. Only superusers can create functions in untrusted languages.
	lanpltrusted: bool,
	/// `oid` `(references pg_proc.oid)` For noninternal languages this references the language handler, which is a special function that is responsible for executing all functions that are written in the particular language. Zero for internal languages.
	lanplcallfoid: Option<Qual>,
	/// `oid` `(references pg_proc.oid)` This references a function that is responsible for executing “inline” anonymous code blocks (DO blocks). Zero if inline blocks are not supported.
	laninline: Option<Qual>,
	/// `oid` `(references pg_proc.oid)` This references a language validator function that is responsible for checking the syntax and validity of new functions when they are created. Zero if no validator is provided.
	lanvalidator: Option<Qual>,
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	lanacl: Option<Vec<aclitem::LanguageAclItem>>,
}
impl_name_hash_and_equivalent!(PgLanguage, lanname);

pub async fn reflect_pg_language(
	client: &PgClient
) -> Result<Set<PgLanguage>, postgres::Error> {
	let pg_language_coll = reflect_crate::queries::reflect_gen::reflect_pg_language().bind(client)
		.map(|pg_language| {
			PgLanguage {
				lanname: pg_language.lanname.into(),
				lanowner: pg_language.lanowner.into(),
				lanispl: pg_language.lanispl,
				lanpltrusted: pg_language.lanpltrusted,
				lanplcallfoid: Qual::maybe_parse(pg_language.lanplcallfoid),
				laninline: Qual::maybe_parse(pg_language.laninline),
				lanvalidator: Qual::maybe_parse(pg_language.lanvalidator),
				lanacl: pg_language.lanacl.map(|lanacl| lanacl.map(|acl| aclitem(acl, &LanguageGrantParser)).collect()),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_language_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgNamespace {
	// oid oid  Row identifier
	/// `name`  Name of the namespace
	nspname: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the namespace
	nspowner: Str,
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	nspacl: Option<Vec<aclitem::SchemaAclItem>>,
}
impl_name_hash_and_equivalent!(PgNamespace, nspname);

pub async fn reflect_pg_namespace(
	client: &PgClient
) -> Result<Set<PgNamespace>, postgres::Error> {
	let pg_namespace_coll = reflect_crate::queries::reflect_gen::reflect_pg_namespace().bind(client)
		.map(|pg_namespace| {
			PgNamespace {
				nspname: pg_namespace.nspname.into(),
				nspowner: pg_namespace.nspowner.into(),
				nspacl: pg_namespace.nspacl.map(|nspacl| nspacl.map(|acl| aclitem(acl, &SchemaGrantParser)).collect()),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_namespace_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgType {
	/// `oid`  Row identifier
	oid: Qual,
	/// `name`  Data type name
	typname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this type
	typnamespace: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the type
	typowner: Str,
	/// `int2`  For a fixed-size type, typlen is the number of bytes in the internal representation of the type. But for a variable-length type, typlen is negative. -1 indicates a “varlena” type (one that has a length word), -2 indicates a null-terminated C string.
	typlen: i16,
	/// `bool`  typbyval determines whether internal routines pass a value of this type by value or by reference. typbyval had better be false if typlen is not 1, 2, or 4 (or 8 on machines where Datum is 8 bytes). Variable-length types are always passed by reference. Note that typbyval can be false even if the length would allow pass-by-value.
	typbyval: bool,
	/// `char`  typtype is b for a base type, c for a composite type (e.g., a table's row type), d for a domain, e for an enum type, p for a pseudo-type, r for a range type, or m for a multirange type. See also typrelid and typbasetype.
	typtype: PgTypeTyptype,
	// typcategory char  typcategory is an arbitrary classification of data types that is used by the parser to determine which implicit casts should be “preferred”. See Table 51.65.
	/// `bool`  True if the type is a preferred cast target within its typcategory
	typispreferred: bool,
	/// `bool`  True if the type is defined, false if this is a placeholder entry for a not-yet-defined type. When typisdefined is false, nothing except the type name, namespace, and OID can be relied on.
	typisdefined: bool,
	/// `char`  Character that separates two values of this type when parsing array input. Note that the delimiter is associated with the array element data type, not the array data type.
	typdelim: i8,
	/// `oid` `(references pg_class.oid)` If this is a composite type (see typtype), then this column points to the pg_class entry that defines the corresponding table. (For a free-standing composite type, the pg_class entry doesn't really represent a table, but it is needed anyway for the type's pg_attribute entries to link to.) Zero for non-composite types.
	typrelid: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Subscripting handler function's OID, or zero if this type doesn't support subscripting. Types that are “true” array types have typsubscript = array_subscript_handler, but other types may have other handler functions to implement specialized subscripting behavior.
	typsubscript: Option<Qual>,
	/// `oid` `(references pg_type.oid)` If typelem is not zero then it identifies another row in pg_type, defining the type yielded by subscripting. This should be zero if typsubscript is zero. However, it can be zero when typsubscript isn't zero, if the handler doesn't need typelem to determine the subscripting result type. Note that a typelem dependency is considered to imply physical containment of the element type in this type; so DDL changes on the element type might be restricted by the presence of this type.
	typelem: Option<Qual>,
	/// `oid` `(references pg_type.oid)` If typarray is not zero then it identifies another row in pg_type, which is the “true” array type having this type as element
	typarray: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Input conversion function (text format)
	typinput: Qual,
	/// `regproc` `(references pg_proc.oid)` Output conversion function (text format)
	typoutput: Qual,
	/// `regproc` `(references pg_proc.oid)` Input conversion function (binary format), or zero if none
	typreceive: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Output conversion function (binary format), or zero if none
	typsend: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Type modifier input function, or zero if type does not support modifiers
	typmodin: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Type modifier output function, or zero to use the standard format
	typmodout: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Custom ANALYZE function, or zero to use the standard function
	typanalyze: Option<Qual>,
	/// `char`  typalign is the alignment required when storing a value of this type. It applies to storage on disk as well as most representations of the value inside PostgreSQL. When multiple values are stored consecutively, such as in the representation of a complete row on disk, padding is inserted before a datum of this type so that it begins on the specified boundary. The alignment reference is the beginning of the first datum in the sequence. Possible values are: c = char alignment, i.e., no alignment needed. s = short alignment (2 bytes on most machines). i = int alignment (4 bytes on most machines). d = double alignment (8 bytes on many machines, but by no means all).
	typalign: PgTypeTypalign,
	/// `char`  typstorage tells for varlena types (those with typlen = -1) if the type is prepared for toasting and what the default strategy for attributes of this type should be. Possible values are: p (plain): Values must always be stored plain (non-varlena types always use this value). e (external): Values can be stored in a secondary “TOAST” relation (if relation has one, see pg_class.reltoastrelid). m (main): Values can be compressed and stored inline. x (extended): Values can be compressed and/or moved to a secondary relation. x is the usual choice for toast-able types. Note that m values can also be moved out to secondary storage, but only as a last resort (e and x values are moved first).
	typstorage: PgTypeTypstorage,
	/// `bool`  typnotnull represents a not-null constraint on a type. Used for domains only.
	typnotnull: bool,
	/// `oid` `(references pg_type.oid)` If this is a domain (see typtype), then typbasetype identifies the type that this one is based on. Zero if this type is not a domain.
	typbasetype: Option<Qual>,
	/// `int4`  Domains use typtypmod to record the typmod to be applied to their base type (-1 if base type does not use a typmod). -1 if this type is not a domain.
	typtypmod: Option<u32>,
	/// `int4`  typndims is the number of array dimensions for a domain over an array (that is, typbasetype is an array type). Zero for types other than domains over array types.
	typndims: u32,
	/// `oid` `(references pg_collation.oid)` typcollation specifies the collation of the type. If the type does not support collations, this will be zero. A base type that supports collations will have a nonzero value here, typically DEFAULT_COLLATION_OID. A domain over a collatable type can have a collation OID different from its base type's, if one was specified for the domain.
	typcollation: Option<Qual>,
	/// `pg_node_tree`  If typdefaultbin is not null, it is the nodeToString() representation of a default expression for the type. This is only used for domains.
	typdefaultbin: Option<Str>,
	/// `text`  typdefault is null if the type has no associated default value. If typdefaultbin is not null, typdefault must contain a human-readable version of the default expression represented by typdefaultbin. If typdefaultbin is null and typdefault is not, then typdefault is the external representation of the type's default value, which can be fed to the type's input converter to produce a constant.
	typdefault: Option<Str>,
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	typacl: Option<Vec<aclitem::TypeAclItem>>,
}
impl_qual_hash_and_equivalent!(PgType);

pg_char_enum!(PgTypeTyptype { 'b' => Base, 'c' => Composite, 'd' => Domain, 'e' => Enum, 'p' => Pseudo, 'r' => Range, 'm' => Multirange });
pg_char_enum!(PgTypeTypalign { 'c' => Char, 's' => Short, 'i' => Int, 'd' => Double });
pg_char_enum!(PgTypeTypstorage { 'p' => Plain, 'e' => External, 'm' => Main, 'x' => Extended });

pub async fn reflect_pg_type(
	client: &PgClient
) -> Result<Set<PgType>, postgres::Error> {
	let pg_type_coll = reflect_crate::queries::reflect_gen::reflect_pg_type().bind(client)
		.map(|pg_type| {
			PgType {
				oid: Qual::parse(pg_type.oid),
				typname: pg_type.typname.into(),
				typnamespace: pg_type.typnamespace.into(),
				typowner: pg_type.typowner.into(),
				typlen: pg_type.typlen,
				typbyval: pg_type.typbyval,
				typtype: PgTypeTyptype::pg_from_char(pg_type.typtype),
				typispreferred: pg_type.typispreferred,
				typisdefined: pg_type.typisdefined,
				typdelim: pg_type.typdelim,
				typrelid: Qual::maybe_parse(pg_type.typrelid),
				typsubscript: Qual::maybe_parse(pg_type.typsubscript),
				typelem: Qual::maybe_parse(pg_type.typelem),
				typarray: Qual::maybe_parse(pg_type.typarray),
				typinput: Qual::parse(pg_type.typinput),
				typoutput: Qual::parse(pg_type.typoutput),
				typreceive: Qual::maybe_parse(pg_type.typreceive),
				typsend: Qual::maybe_parse(pg_type.typsend),
				typmodin: Qual::maybe_parse(pg_type.typmodin),
				typmodout: Qual::maybe_parse(pg_type.typmodout),
				typanalyze: Qual::maybe_parse(pg_type.typanalyze),
				typalign: PgTypeTypalign::pg_from_char(pg_type.typalign),
				typstorage: PgTypeTypstorage::pg_from_char(pg_type.typstorage),
				typnotnull: pg_type.typnotnull,
				typbasetype: Qual::maybe_parse(pg_type.typbasetype),
				typtypmod: pg_type.typtypmod.map(i32::unsigned_abs),
				typndims: pg_type.typndims.unsigned_abs(),
				typcollation: Qual::maybe_parse(pg_type.typcollation),
				typdefaultbin: pg_type.typdefaultbin.map(Into::into),
				typdefault: pg_type.typdefault.map(Into::into),
				typacl: pg_type.typacl.map(|typacl| typacl.map(|acl| aclitem(acl, &TypeGrantParser)).collect()),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_type_coll)
}

