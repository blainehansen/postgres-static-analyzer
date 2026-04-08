use super::*;
use futures::TryStreamExt;
use crate::aclitem::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgAggregate {
	/// `regproc` `(references pg_proc.oid)` pg_proc OID of the aggregate function
	pub aggfnoid: Qual,
	/// `char`  Aggregate kind: n for “normal” aggregates, o for “ordered-set” aggregates, or h for “hypothetical-set” aggregates
	pub aggkind: PgAggregateAggkind,
	/// `int2`  Number of direct (non-aggregated) arguments of an ordered-set or hypothetical-set aggregate, counting a variadic array as one argument. If equal to pronargs, the aggregate must be variadic and the variadic array describes the aggregated arguments as well as the final direct arguments. Always zero for normal aggregates.
	pub aggnumdirectargs: u16,
	/// `regproc` `(references pg_proc.oid)` Transition function
	pub aggtransfn: Qual,
	/// `regproc` `(references pg_proc.oid)` Final function (zero if none)
	pub aggfinalfn: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Combine function (zero if none)
	pub aggcombinefn: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Serialization function (zero if none)
	pub aggserialfn: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Deserialization function (zero if none)
	pub aggdeserialfn: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Forward transition function for moving-aggregate mode (zero if none)
	pub aggmtransfn: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Inverse transition function for moving-aggregate mode (zero if none)
	pub aggminvtransfn: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Final function for moving-aggregate mode (zero if none)
	pub aggmfinalfn: Option<Qual>,
	/// `bool`  True to pass extra dummy arguments to aggfinalfn
	pub aggfinalextra: bool,
	/// `bool`  True to pass extra dummy arguments to aggmfinalfn
	pub aggmfinalextra: bool,
	/// `char`  Whether aggfinalfn modifies the transition state value: r if it is read-only, s if the aggtransfn cannot be applied after the aggfinalfn, or w if it writes on the value
	pub aggfinalmodify: PgAggregateAggfinalmodify,
	/// `char`  Like aggfinalmodify, but for the aggmfinalfn
	pub aggmfinalmodify: PgAggregateAggmfinalmodify,
	/// `oid` `(references pg_operator.oid)` Associated sort operator (zero if none)
	pub aggsortop: Option<Qual>,
	/// `oid` `(references pg_type.oid)` Data type of the aggregate function's internal transition (state) data
	pub aggtranstype: Qual,
	// aggtransspace int4  Approximate average size (in bytes) of the transition state data, or zero to use a default estimate
	/// `oid` `(references pg_type.oid)` Data type of the aggregate function's internal transition (state) data for moving-aggregate mode (zero if none)
	pub aggmtranstype: Option<Qual>,
	// aggmtransspace int4  Approximate average size (in bytes) of the transition state data for moving-aggregate mode, or zero to use a default estimate
	/// `text`  The initial value of the transition state. This is a text field containing the initial value in its external string representation. If this field is null, the transition state value starts out null.
	pub agginitval: Option<Str>,
	/// `text`  The initial value of the transition state for moving-aggregate mode. This is a text field containing the initial value in its external string representation. If this field is null, the transition state value starts out null.
	pub aggminitval: Option<Str>,
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
		.iter().await?.try_collect()
		.await?;

	Ok(pg_aggregate_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgAm {
	// oid oid  Row identifier
	/// `name`  Name of the access method
	pub amname: Str,
	/// `regproc` `(references pg_proc.oid)` OID of a handler function that is responsible for supplying information about the access method
	pub amhandler: Qual,
	/// `char`  t = table (including materialized views), i = index.
	pub amtype: PgAmAmtype,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
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
				description: pg_am.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_am_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgAmop {
	// oid oid  Row identifier
	/// `oid` `(references pg_opfamily.oid)` The operator family this entry is for
	pub amopfamily: Qual,
	/// `oid` `(references pg_type.oid)` Left-hand input data type of operator
	pub amoplefttype: Qual,
	/// `oid` `(references pg_type.oid)` Right-hand input data type of operator
	pub amoprighttype: Qual,
	/// `int2`  Operator strategy number
	pub amopstrategy: u16,
	/// `char`  Operator purpose, either s for search or o for ordering
	pub amoppurpose: PgAmopAmoppurpose,
	/// `oid` `(references pg_operator.oid)` OID of the operator
	pub amopopr: Qual,
	/// `oid` `(references pg_am.oid)` Index access method operator family is for
	pub amopmethod: Str,
	/// `oid` `(references pg_opfamily.oid)` The B-tree operator family this entry sorts according to, if an ordering operator; zero if a search operator
	pub amopsortfamily: Option<Qual>,
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
		.iter().await?.try_collect()
		.await?;

	Ok(pg_amop_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgAmproc {
	// oid oid  Row identifier
	/// `oid` `(references pg_opfamily.oid)` The operator family this entry is for
	pub amprocfamily: Qual,
	/// `oid` `(references pg_type.oid)` Left-hand input data type of associated operator
	pub amproclefttype: Qual,
	/// `oid` `(references pg_type.oid)` Right-hand input data type of associated operator
	pub amprocrighttype: Qual,
	/// `int2`  Support function number
	pub amprocnum: u16,
	/// `regproc` `(references pg_proc.oid)` OID of the function
	pub amproc: Qual,
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
		.iter().await?.try_collect()
		.await?;

	Ok(pg_amproc_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgAttrdef {
	// oid oid  Row identifier
	/// `oid` `(references pg_class.oid)` The table this column belongs to
	pub adrelid: Qual,
	/// `int2` `(references pg_attribute.attnum)` The number of the column
	pub adnum: u16,
	/// `pg_node_tree`  The column default value, in nodeToString() representation. Use pg_get_expr(adbin, adrelid) to convert it to an SQL expression.
	pub adbin: Str,
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
		.iter().await?.try_collect()
		.await?;

	Ok(pg_attrdef_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgAttribute {
	/// `oid` `(references pg_class.oid)` The table this column belongs to
	pub attrelid: Qual,
	/// `name`  The column name
	pub attname: Str,
	/// `oid` `(references pg_type.oid)` The data type of this column (zero for a dropped column)
	pub atttypid: Qual,
	// attlen int2  A copy of pg_type.typlen of this column's type
	/// `int2`  The number of the column. Ordinary columns are numbered from 1 up. System columns, such as ctid, have (arbitrary) negative numbers.
	pub attnum: u16,
	// attcacheoff int4  Always -1 in storage, but when loaded into a row descriptor in memory this might be updated to cache the offset of the attribute within the row
	/// `int4`  atttypmod records type-specific data supplied at table creation time (for example, the maximum length of a varchar column). It is passed to type-specific input functions and length coercion functions. The value will generally be -1 for types that do not need atttypmod.
	pub atttypmod: Option<u32>,
	/// `int2`  Number of dimensions, if the column is an array type; otherwise 0. (Presently, the number of dimensions of an array is not enforced, so any nonzero value effectively means “it's an array”.)
	pub attndims: u16,
	// attbyval bool  A copy of pg_type.typbyval of this column's type
	// attalign char  A copy of pg_type.typalign of this column's type
	// attstorage char  Normally a copy of pg_type.typstorage of this column's type. For TOAST-able data types, this can be altered after column creation to control storage policy.
	/// `char`  The current compression method of the column. Typically this is '\0' to specify use of the current default setting (see default_toast_compression). Otherwise, 'p' selects pglz compression, while 'l' selects LZ4 compression. However, this field is ignored whenever attstorage does not allow compression.
	pub attcompression: Option<PgAttributeAttcompression>,
	/// `bool`  This represents a not-null constraint.
	pub attnotnull: bool,
	/// `bool`  This column has a default expression or generation expression, in which case there will be a corresponding entry in the pg_attrdef catalog that actually defines the expression. (Check attgenerated to determine whether this is a default or a generation expression.)
	pub atthasdef: bool,
	// atthasmissing bool  This column has a value which is used where the column is entirely missing from the row, as happens when a column is added with a non-volatile DEFAULT value after the row is created. The actual value used is stored in the attmissingval column.
	/// `char`  If a zero byte (''), then not an identity column. Otherwise, a = generated always, d = generated by default.
	pub attidentity: Option<PgAttributeAttidentity>,
	/// `char`  If a zero byte (''), then not a generated column. Otherwise, s = stored. (Other values might be added in the future.)
	pub attgenerated: Option<PgAttributeAttgenerated>,
	/// `bool`  This column has been dropped and is no longer valid. A dropped column is still physically present in the table, but is ignored by the parser and so cannot be accessed via SQL.
	pub attisdropped: bool,
	/// `bool`  This column is defined locally in the relation. Note that a column can be locally defined and inherited simultaneously.
	pub attislocal: bool,
	/// `int2`  The number of direct ancestors this column has. A column with a nonzero number of ancestors cannot be dropped nor renamed.
	pub attinhcount: u16,
	/// `oid` `(references pg_collation.oid)` The defined collation of the column, or zero if the column is not of a collatable data type
	pub attcollation: Option<Qual>,
	/// `int2`  attstattarget controls the level of detail of statistics accumulated for this column by ANALYZE. A zero value indicates that no statistics should be collected. A null value says to use the system default statistics target. The exact meaning of positive values is data type-dependent. For scalar data types, attstattarget is both the target number of “most common values” to collect, and the target number of histogram bins to create.
	pub attstattarget: Option<u16>,
	/// `aclitem[]`  Column-level access privileges, if any have been granted specifically on this column
	pub attacl: Option<Vec<aclitem::TableColumnAclItem>>,
	/// `text[]`  Attribute-level options, as “keyword=value” strings
	pub attoptions: Option<Vec<Str>>,
	/// `text[]`  Attribute-level foreign data wrapper options, as “keyword=value” strings
	pub attfdwoptions: Option<Vec<Str>>,
	// attmissingval anyarray  This column has a one element array containing the value used when the column is entirely missing from the row, as happens when the column is added with a non-volatile DEFAULT value after the row is created. The value is only used when atthasmissing is true. If there is no value the column is null.
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
	/// `text`  The seclabel from pg_seclabel
	pub seclabel: Option<Str>,
	/// `text`  The provider from pg_seclabel
	pub seclabel_provider: Option<Str>,
	/// `aclitem[]`  The initial access privileges from pg_init_privs.
	pub initprivs: Option<Vec<aclitem::TableColumnAclItem>>,
	/// `char`  A code defining the type of initial privilege of this object from pg_init_privs. 'i' if set by initdb, 'e' if set by CREATE EXTENSION.
	pub initprivs_type: Option<PgAttributeInitprivsType>,
}

pg_char_enum!(PgAttributeAttcompression { 'p' => PGLZ, 'l'=> LZ4 });
pg_char_enum!(PgAttributeAttidentity { 'a' => GeneratedAlways, 'd' => GenertedByDefault });
pg_char_enum!(PgAttributeAttgenerated { 's' => Stored });
pg_char_enum!(PgAttributeInitprivsType { 'i' => InitDb, 'e' => CreateExtension });

pub async fn reflect_pg_attribute(
	client: &PgClient
) -> Result<Vec<PgAttribute>, postgres::Error> {
	let pg_attribute_coll = reflect_crate::queries::reflect_gen::reflect_pg_attribute().bind(client)
		.map(|pg_attribute| {
			PgAttribute {
				attrelid: Qual::parse(pg_attribute.attrelid),
				attname: pg_attribute.attname.into(),
				atttypid: Qual::parse(pg_attribute.atttypid),
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
				description: pg_attribute.description.map(Into::into),
				seclabel: pg_attribute.seclabel.map(Into::into),
				seclabel_provider: pg_attribute.seclabel_provider.map(Into::into),
				initprivs: pg_attribute.initprivs.map(|initprivs| initprivs.map(|acl| aclitem(acl, &TableColumnGrantParser)).collect()),
				initprivs_type: pg_attribute.initprivs_type.map(PgAttributeInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_attribute_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgRoles {
	/// `name`  Role name
	pub rolname: Str,
	/// `bool`  Role has superuser privileges
	pub rolsuper: bool,
	/// `bool`  Role automatically inherits privileges of roles it is a member of
	pub rolinherit: bool,
	/// `bool`  Role can create more roles
	pub rolcreaterole: bool,
	/// `bool`  Role can create databases
	pub rolcreatedb: bool,
	/// `bool`  Role can log in. That is, this role can be given as the initial session authorization identifier
	pub rolcanlogin: bool,
	/// `bool`  Role is a replication role. A replication role can initiate replication connections and create and drop replication slots.
	pub rolreplication: bool,
	/// `int4`  For roles that can log in, this sets maximum number of concurrent connections this role can make. -1 means no limit.
	pub rolconnlimit: Option<u32>,
	// rolpassword text  Not the password (always reads as ********)
	/// `timestamptz`  Password expiry time (only used for password authentication); null if no expiration
	pub rolvaliduntil: Option<chrono::DateTime<chrono::FixedOffset>>,
	/// `bool`  Role bypasses every row-level security policy, see Section 5.9 for more information.
	pub rolbypassrls: bool,
	/// `text[]`  Role-specific defaults for run-time configuration variables
	pub rolconfig: Option<Vec<Str>>,
	// oid oid (references pg_authid.oid) ID of role
	/// `text`  The comment from pg_shdescription
	pub description: Option<Str>,
	/// `text`  The seclabel from pg_shseclabel
	pub seclabel: Option<Str>,
	/// `text`  The provider from pg_shseclabel
	pub seclabel_provider: Option<Str>,
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
				description: pg_roles.description.map(Into::into),
				seclabel: pg_roles.seclabel.map(Into::into),
				seclabel_provider: pg_roles.seclabel_provider.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_roles_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgAuthMembers {
	// oid oid  Row identifier
	/// `oid` `(references pg_authid.oid)` ID of a role that has a member
	pub roleid: Str,
	/// `oid` `(references pg_authid.oid)` ID of a role that is a member of roleid
	pub member: Str,
	/// `oid` `(references pg_authid.oid)` ID of the role that granted this membership
	pub grantor: Str,
	/// `bool`  True if member can grant membership in roleid to others
	pub admin_option: bool,
	/// `bool`  True if the member automatically inherits the privileges of the granted role
	pub inherit_option: bool,
	/// `bool`  True if the member can SET ROLE to the granted role
	pub set_option: bool,
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
		.iter().await?.try_collect()
		.await?;

	Ok(pg_auth_members_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgCast {
	// oid oid  Row identifier
	/// `oid` `(references pg_type.oid)` OID of the source data type
	pub castsource: Qual,
	/// `oid` `(references pg_type.oid)` OID of the target data type
	pub casttarget: Qual,
	/// `oid` `(references pg_proc.oid)` The OID of the function to use to perform this cast. Zero is stored if the cast method doesn't require a function.
	pub castfunc: Option<Qual>,
	/// `char`  Indicates what contexts the cast can be invoked in. e means only as an explicit cast (using CAST or :: syntax). a means implicitly in assignment to a target column, as well as explicitly. i means implicitly in expressions, as well as the other cases.
	pub castcontext: PgCastCastcontext,
	/// `char`  Indicates how the cast is performed. f means that the function specified in the castfunc field is used. i means that the input/output functions are used. b means that the types are binary-coercible, thus no conversion is required.
	pub castmethod: PgCastCastmethod,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
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
				description: pg_cast.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_cast_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgClass {
	/// `oid`  Row identifier
	pub oid: Qual,
	/// `name`  Name of the table, index, view, etc.
	pub relname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this relation
	pub relnamespace: Str,
	/// `oid` `(references pg_type.oid)` The OID of the data type that corresponds to this table's row type, if any; zero for indexes, sequences, and toast tables, which have no pg_type entry
	pub reltype: Option<Qual>,
	/// `oid` `(references pg_type.oid)` For typed tables, the OID of the underlying composite type; zero for all other relations
	pub reloftype: Option<Qual>,
	/// `oid` `(references pg_authid.oid)` Owner of the relation
	pub relowner: Str,
	/// `oid` `(references pg_am.oid)` The access method used to access this table or index. Not meaningful if the relation is a sequence or has no on-disk file, except for partitioned tables, where, if set, it takes precedence over default_table_access_method when determining the access method to use for partitions created when one is not specified in the creation command.
	pub relam: Option<Str>,
	// relfilenode oid  Name of the on-disk file of this relation; zero means this is a “mapped” relation whose disk file name is determined by low-level state
	// reltablespace oid (references pg_tablespace.oid) The tablespace in which this relation is stored. If zero, the database's default tablespace is implied. Not meaningful if the relation has no on-disk file, except for partitioned tables, where this is the tablespace in which partitions will be created when one is not specified in the creation command.
	// relpages int4  Size of the on-disk representation of this table in pages (of size BLCKSZ). This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX.
	// reltuples float4  Number of live rows in the table. This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX. If the table has never yet been vacuumed or analyzed, reltuples contains -1 indicating that the row count is unknown.
	// relallvisible int4  Number of pages that are marked all-visible in the table's visibility map. This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX.
	// reltoastrelid oid (references pg_class.oid) OID of the TOAST table associated with this table, zero if none. The TOAST table stores large attributes “out of line” in a secondary table.
	// relhasindex bool  True if this is a table and it has (or recently had) any indexes
	/// `bool`  True if this table is shared across all databases in the cluster. Only certain system catalogs (such as pg_database) are shared.
	pub relisshared: bool,
	/// `char`  p = permanent table/sequence, u = unlogged table/sequence, t = temporary table/sequence
	pub relpersistence: PgClassRelpersistence,
	/// `char`  r = ordinary table, i = index, S = sequence, t = TOAST table, v = view, m = materialized view, c = composite type, f = foreign table, p = partitioned table, I = partitioned index
	pub relkind: PgClassRelkind,
	/// `int2`  Number of user columns in the relation (system columns not counted). There must be this many corresponding entries in pg_attribute. See also pg_attribute.attnum.
	pub relnatts: u16,
	/// `int2`  Number of CHECK constraints on the table; see pg_constraint catalog
	pub relchecks: u16,
	// relhasrules bool  True if table has (or once had) rules; see pg_rewrite catalog
	// relhastriggers bool  True if table has (or once had) triggers; see pg_trigger catalog
	// relhassubclass bool  True if table or index has (or once had) any inheritance children or partitions
	/// `bool`  True if table has row-level security enabled; see pg_policy catalog
	pub relrowsecurity: bool,
	/// `bool`  True if row-level security (when enabled) will also apply to table owner; see pg_policy catalog
	pub relforcerowsecurity: bool,
	// relispopulated bool  True if relation is populated (this is true for all relations other than some materialized views)
	/// `char`  Columns used to form “replica identity” for rows: d = default (primary key, if any), n = nothing, f = all columns, i = index with indisreplident set (same as nothing if the index used has been dropped)
	pub relreplident: PgClassRelreplident,
	/// `bool`  True if table or index is a partition
	pub relispartition: bool,
	// relrewrite oid (references pg_class.oid) For new relations being written during a DDL operation that requires a table rewrite, this contains the OID of the original relation; otherwise zero. That state is only visible internally; this field should never contain anything other than zero for a user-visible relation.
	// relfrozenxid xid  All transaction IDs before this one have been replaced with a permanent (“frozen”) transaction ID in this table. This is used to track whether the table needs to be vacuumed in order to prevent transaction ID wraparound or to allow pg_xact to be shrunk. Zero (InvalidTransactionId) if the relation is not a table.
	// relminmxid xid  All multixact IDs before this one have been replaced by a transaction ID in this table. This is used to track whether the table needs to be vacuumed in order to prevent multixact ID wraparound or to allow pg_multixact to be shrunk. Zero (InvalidMultiXactId) if the relation is not a table.
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	pub relacl: Option<Vec<aclitem::TableAclItem>>,
	/// `text[]`  Access-method-specific options, as “keyword=value” strings
	pub reloptions: Option<Vec<Str>>,
	/// `pg_node_tree`  If table is a partition (see relispartition), internal representation of the partition bound
	pub relpartbound: Option<Str>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
	/// `text`  The seclabel from pg_seclabel
	pub seclabel: Option<Str>,
	/// `text`  The provider from pg_seclabel
	pub seclabel_provider: Option<Str>,
	/// `aclitem[]`  The initial access privileges from pg_init_privs.
	pub initprivs: Option<Vec<aclitem::TableAclItem>>,
	/// `char`  A code defining the type of initial privilege of this object from pg_init_privs. 'i' if set by initdb, 'e' if set by CREATE EXTENSION.
	pub initprivs_type: Option<PgClassInitprivsType>,
}
impl_qual_hash_and_equivalent!(PgClass);

pg_char_enum!(PgClassRelpersistence { 'p' => Permanent, 'u' => Unlogged, 't' => Temporary });
pg_char_enum!(PgClassRelkind { 'r' => Ordinary, 'i' => Index, 'S' => Sequence, 't' => Toast, 'v' => View, 'm' => MaterializedView, 'c' => CompositeType, 'f' => ForeignTable, 'p' => PartitionedTable, 'I' => PartitionedIndex });
pg_char_enum!(PgClassRelreplident { 'd' => Default, 'n' => Nothing, 'f' => AllColumns, 'i' => Index  });
pg_char_enum!(PgClassInitprivsType { 'i' => InitDb, 'e' => CreateExtension });

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
				description: pg_class.description.map(Into::into),
				seclabel: pg_class.seclabel.map(Into::into),
				seclabel_provider: pg_class.seclabel_provider.map(Into::into),
				initprivs: pg_class.initprivs.map(|initprivs| initprivs.map(|acl| aclitem(acl, &TableGrantParser)).collect()),
				initprivs_type: pg_class.initprivs_type.map(PgClassInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_class_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgCollation {
	/// `oid`  Row identifier
	pub oid: Qual,
	/// `name`  Collation name (unique per namespace and encoding)
	pub collname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this collation
	pub collnamespace: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the collation
	pub collowner: Str,
	/// `char`  Provider of the collation: d = database default, b = builtin, c = libc, i = icu
	pub collprovider: PgCollationCollprovider,
	/// `bool`  Is the collation deterministic?
	pub collisdeterministic: bool,
	/// `int4`  Encoding in which the collation is applicable, or -1 if it works for any encoding
	pub collencoding: Option<Str>,
	/// `text`  LC_COLLATE for this collation object. If the provider is not libc, collcollate is NULL and colllocale is used instead.
	pub collcollate: Option<Str>,
	/// `text`  LC_CTYPE for this collation object. If the provider is not libc, collctype is NULL and colllocale is used instead.
	pub collctype: Option<Str>,
	/// `text`  Collation provider locale name for this collation object. If the provider is libc, colllocale is NULL; collcollate and collctype are used instead.
	pub colllocale: Option<Str>,
	/// `text`  ICU collation rules for this collation object
	pub collicurules: Option<Str>,
	/// `text`  Provider-specific version of the collation. This is recorded when the collation is created and then checked when it is used, to detect changes in the collation definition that could lead to data corruption.
	pub collversion: Option<Str>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
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
				description: pg_collation.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_collation_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgConstraint {
	// oid oid  Row identifier
	/// `name`  Constraint name (not necessarily unique!)
	pub conname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this constraint
	pub connamespace: Str,
	/// `char`  c = check constraint, f = foreign key constraint, n = not-null constraint (domains only), p = primary key constraint, u = unique constraint, t = constraint trigger, x = exclusion constraint
	pub contype: PgConstraintContype,
	/// `bool`  Is the constraint deferrable?
	pub condeferrable: bool,
	/// `bool`  Is the constraint deferred by default?
	pub condeferred: bool,
	/// `bool`  Has the constraint been validated? Currently, can be false only for foreign keys and CHECK constraints
	pub convalidated: bool,
	/// `oid` `(references pg_class.oid)` The table this constraint is on; zero if not a table constraint
	pub conrelid: Option<Qual>,
	/// `oid` `(references pg_type.oid)` The domain this constraint is on; zero if not a domain constraint
	pub contypid: Option<Qual>,
	/// `oid` `(references pg_class.oid)` The index supporting this constraint, if it's a unique, primary key, foreign key, or exclusion constraint; else zero
	pub conindid: Option<Qual>,
	/// `oid` `(references pg_constraint.oid)` The corresponding constraint of the parent partitioned table, if this is a constraint on a partition; else zero
	pub conparentid: Option<Qual>,
	/// `oid` `(references pg_class.oid)` If a foreign key, the referenced table; else zero
	pub confrelid: Option<Qual>,
	/// `char`  Foreign key update action code: a = no action, r = restrict, c = cascade, n = set null, d = set default
	pub confupdtype: Option<PgConstraintConfupdtype>,
	/// `char`  Foreign key deletion action code: a = no action, r = restrict, c = cascade, n = set null, d = set default
	pub confdeltype: Option<PgConstraintConfdeltype>,
	/// `char`  Foreign key match type: f = full, p = partial, s = simple
	pub confmatchtype: Option<PgConstraintConfmatchtype>,
	/// `bool`  This constraint is defined locally for the relation. Note that a constraint can be locally defined and inherited simultaneously.
	pub conislocal: bool,
	/// `int2`  The number of direct inheritance ancestors this constraint has. A constraint with a nonzero number of ancestors cannot be dropped nor renamed.
	pub coninhcount: u16,
	/// `bool`  This constraint is defined locally for the relation. It is a non-inheritable constraint.
	pub connoinherit: bool,
	/// `int2[]` `(references pg_attribute.attnum)` If a table constraint (including foreign keys, but not constraint triggers), list of the constrained columns
	pub conkey: Option<Vec<u16>>,
	/// `int2[]` `(references pg_attribute.attnum)` If a foreign key, list of the referenced columns
	pub confkey: Option<Vec<u16>>,
	/// `oid[]` `(references pg_operator.oid)` If a foreign key, list of the equality operators for PK = FK comparisons
	pub conpfeqop: Option<Vec<Qual>>,
	/// `oid[]` `(references pg_operator.oid)` If a foreign key, list of the equality operators for PK = PK comparisons
	pub conppeqop: Option<Vec<Qual>>,
	/// `oid[]` `(references pg_operator.oid)` If a foreign key, list of the equality operators for FK = FK comparisons
	pub conffeqop: Option<Vec<Qual>>,
	/// `int2[]` `(references pg_attribute.attnum)` If a foreign key with a SET NULL or SET DEFAULT delete action, the columns that will be updated. If null, all of the referencing columns will be updated.
	pub confdelsetcols: Option<Vec<u16>>,
	/// `oid[]` `(references pg_operator.oid)` If an exclusion constraint, list of the per-column exclusion operators
	pub conexclop: Option<Vec<Qual>>,
	/// `pg_node_tree`  If a check constraint, an internal representation of the expression. (It's recommended to use pg_get_constraintdef() to extract the definition of a check constraint.)
	pub conbin: Option<Str>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
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
				description: pg_constraint.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_constraint_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgConversion {
	// oid oid  Row identifier
	/// `name`  Conversion name (unique within a namespace)
	pub conname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this conversion
	pub connamespace: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the conversion
	pub conowner: Str,
	/// `int4`  Source encoding ID (pg_encoding_to_char() can translate this number to the encoding name)
	pub conforencoding: Str,
	/// `int4`  Destination encoding ID (pg_encoding_to_char() can translate this number to the encoding name)
	pub contoencoding: Str,
	/// `regproc` `(references pg_proc.oid)` Conversion function
	pub conproc: Qual,
	/// `bool`  True if this is the default conversion
	pub condefault: bool,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
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
				description: pg_conversion.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_conversion_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgDatabase {
	// oid oid  Row identifier
	/// `name`  Database name
	pub datname: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the database, usually the user who created it
	pub datdba: Str,
	/// `int4`  Character encoding for this database (pg_encoding_to_char() can translate this number to the encoding name)
	pub encoding: Str,
	/// `char`  Locale provider for this database: b = builtin, c = libc, i = icu
	pub datlocprovider: PgDatabaseDatlocprovider,
	/// `bool`  If true, then this database can be cloned by any user with CREATEDB privileges; if false, then only superusers or the owner of the database can clone it.
	pub datistemplate: bool,
	/// `bool`  If false then no one can connect to this database. This is used to protect the template0 database from being altered.
	pub datallowconn: bool,
	// dathasloginevt bool  Indicates that there are login event triggers defined for this database. This flag is used to avoid extra lookups on the pg_event_trigger table during each backend startup. This flag is used internally by PostgreSQL and should not be manually altered or read for monitoring purposes.
	/// `int4`  Sets maximum number of concurrent connections that can be made to this database. -1 means no limit, -2 indicates the database is invalid.
	pub datconnlimit: Option<u32>,
	// datfrozenxid xid  All transaction IDs before this one have been replaced with a permanent (“frozen”) transaction ID in this database. This is used to track whether the database needs to be vacuumed in order to prevent transaction ID wraparound or to allow pg_xact to be shrunk. It is the minimum of the per-table pg_class.relfrozenxid values.
	// datminmxid xid  All multixact IDs before this one have been replaced with a transaction ID in this database. This is used to track whether the database needs to be vacuumed in order to prevent multixact ID wraparound or to allow pg_multixact to be shrunk. It is the minimum of the per-table pg_class.relminmxid values.
	// dattablespace oid (references pg_tablespace.oid) The default tablespace for the database. Within this database, all tables for which pg_class.reltablespace is zero will be stored in this tablespace; in particular, all the non-shared system catalogs will be there.
	/// `text`  LC_COLLATE for this database
	pub datcollate: Option<Str>,
	/// `text`  LC_CTYPE for this database
	pub datctype: Option<Str>,
	/// `text`  Collation provider locale name for this database. If the provider is libc, datlocale is NULL; datcollate and datctype are used instead.
	pub datlocale: Option<Str>,
	/// `text`  ICU collation rules for this database
	pub daticurules: Option<Str>,
	/// `text`  Provider-specific version of the collation. This is recorded when the database is created and then checked when it is used, to detect changes in the collation definition that could lead to data corruption.
	pub datcollversion: Option<Str>,
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	pub datacl: Option<Vec<aclitem::DbAclItem>>,
	/// `text`  The comment from pg_shdescription
	pub description: Option<Str>,
	/// `text`  The seclabel from pg_shseclabel
	pub seclabel: Option<Str>,
	/// `text`  The provider from pg_shseclabel
	pub seclabel_provider: Option<Str>,
}
impl_name_hash_and_equivalent!(PgDatabase, datname);

pg_char_enum!(PgDatabaseDatlocprovider { 'b' => Builtin, 'c' => Libc, 'i' => Icu });

pub async fn reflect_pg_database(
	client: &PgClient
) -> Result<PgDatabase, postgres::Error> {
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
				description: pg_database.description.map(Into::into),
				seclabel: pg_database.seclabel.map(Into::into),
				seclabel_provider: pg_database.seclabel_provider.map(Into::into),
			}
		})
		.one()
		.await?;

	Ok(pg_database_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgDefaultAcl {
	// oid oid  Row identifier
	/// `oid` `(references pg_authid.oid)` The OID of the role associated with this entry
	pub defaclrole: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace associated with this entry, or zero if none
	pub defaclnamespace: Option<Str>,
	/// `char`  Type of object this entry is for: r = relation (table, view), S = sequence, f = function, T = type, n = schema
	pub defaclobjtype: PgDefaultAclDefaclobjtype,
	/// `aclitem[]`  Access privileges that this type of object should have on creation
	pub defaclacl: Option<Vec<aclitem::AclDefaultAclItem>>,
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
		.iter().await?.try_collect()
		.await?;

	Ok(pg_default_acl_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgEventTrigger {
	// oid oid  Row identifier
	/// `name`  Trigger name (must be unique)
	pub evtname: Str,
	/// `name`  Identifies the event for which this trigger fires
	pub evtevent: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the event trigger
	pub evtowner: Str,
	/// `oid` `(references pg_proc.oid)` The function to be called
	pub evtfoid: Qual,
	/// `char`  Controls in which session_replication_role modes the event trigger fires. O = trigger fires in “origin” and “local” modes, D = trigger is disabled, R = trigger fires in “replica” mode, A = trigger fires always.
	pub evtenabled: PgEventTriggerEvtenabled,
	/// `text[]`  Command tags for which this trigger will fire. If NULL, the firing of this trigger is not restricted on the basis of the command tag.
	pub evttags: Option<Vec<Str>>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
	/// `text`  The seclabel from pg_seclabel
	pub seclabel: Option<Str>,
	/// `text`  The provider from pg_seclabel
	pub seclabel_provider: Option<Str>,
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
				description: pg_event_trigger.description.map(Into::into),
				seclabel: pg_event_trigger.seclabel.map(Into::into),
				seclabel_provider: pg_event_trigger.seclabel_provider.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_event_trigger_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgExtension {
	// oid oid  Row identifier
	/// `name`  Name of the extension
	pub extname: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the extension
	pub extowner: Str,
	/// `oid` `(references pg_namespace.oid)` Schema containing the extension's exported objects
	pub extnamespace: Str,
	/// `bool`  True if extension can be relocated to another schema
	pub extrelocatable: bool,
	/// `text`  Version name for the extension
	pub extversion: Str,
	/// `oid[]` `(references pg_class.oid)` Array of regclass OIDs for the extension's configuration table(s), or NULL if none
	pub extconfig: Option<Vec<Qual>>,
	/// `text[]`  Array of WHERE-clause filter conditions for the extension's configuration table(s), or NULL if none
	pub extcondition: Option<Vec<Str>>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
}

pub async fn reflect_pg_extension(
	client: &PgClient
) -> Result<Vec<PgExtension>, postgres::Error> {
	let pg_extension_coll = reflect_crate::queries::reflect_gen::reflect_pg_extension().bind(client)
		.map(|pg_extension| {
			PgExtension {
				extname: pg_extension.extname.into(),
				extowner: pg_extension.extowner.into(),
				extnamespace: pg_extension.extnamespace.into(),
				extrelocatable: pg_extension.extrelocatable,
				extversion: pg_extension.extversion.into(),
				extconfig: pg_extension.extconfig.map(|items| items.map(Qual::parse).collect()),
				extcondition: pg_extension.extcondition.map(|items| items.map(Into::into).collect()),
				description: pg_extension.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_extension_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgForeignDataWrapper {
	// oid oid  Row identifier
	/// `name`  Name of the foreign-data wrapper
	pub fdwname: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the foreign-data wrapper
	pub fdwowner: Str,
	/// `oid` `(references pg_proc.oid)` References a handler function that is responsible for supplying execution routines for the foreign-data wrapper. Zero if no handler is provided
	pub fdwhandler: Option<Qual>,
	/// `oid` `(references pg_proc.oid)` References a validator function that is responsible for checking the validity of the options given to the foreign-data wrapper, as well as options for foreign servers and user mappings using the foreign-data wrapper. Zero if no validator is provided
	pub fdwvalidator: Option<Qual>,
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	pub fdwacl: Option<Vec<aclitem::ForeignDataWrapperAclItem>>,
	/// `text[]`  Foreign-data wrapper specific options, as “keyword=value” strings
	pub fdwoptions: Option<Vec<Str>>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
	/// `aclitem[]`  The initial access privileges from pg_init_privs.
	pub initprivs: Option<Vec<aclitem::ForeignDataWrapperAclItem>>,
	/// `char`  A code defining the type of initial privilege of this object from pg_init_privs. 'i' if set by initdb, 'e' if set by CREATE EXTENSION.
	pub initprivs_type: Option<PgForeignDataWrapperInitprivsType>,
}

pg_char_enum!(PgForeignDataWrapperInitprivsType { 'i' => InitDb, 'e' => CreateExtension });

pub async fn reflect_pg_foreign_data_wrapper(
	client: &PgClient
) -> Result<Vec<PgForeignDataWrapper>, postgres::Error> {
	let pg_foreign_data_wrapper_coll = reflect_crate::queries::reflect_gen::reflect_pg_foreign_data_wrapper().bind(client)
		.map(|pg_foreign_data_wrapper| {
			PgForeignDataWrapper {
				fdwname: pg_foreign_data_wrapper.fdwname.into(),
				fdwowner: pg_foreign_data_wrapper.fdwowner.into(),
				fdwhandler: Qual::maybe_parse(pg_foreign_data_wrapper.fdwhandler),
				fdwvalidator: Qual::maybe_parse(pg_foreign_data_wrapper.fdwvalidator),
				fdwacl: pg_foreign_data_wrapper.fdwacl.map(|fdwacl| fdwacl.map(|acl| aclitem(acl, &ForeignDataWrapperGrantParser)).collect()),
				fdwoptions: pg_foreign_data_wrapper.fdwoptions.map(|items| items.map(Into::into).collect()),
				description: pg_foreign_data_wrapper.description.map(Into::into),
				initprivs: pg_foreign_data_wrapper.initprivs.map(|initprivs| initprivs.map(|acl| aclitem(acl, &ForeignDataWrapperGrantParser)).collect()),
				initprivs_type: pg_foreign_data_wrapper.initprivs_type.map(PgForeignDataWrapperInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_foreign_data_wrapper_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgForeignServer {
	// oid oid  Row identifier
	/// `name`  Name of the foreign server
	pub srvname: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the foreign server
	pub srvowner: Str,
	/// `oid` `(references pg_foreign_data_wrapper.oid)` OID of the foreign-data wrapper of this foreign server
	pub srvfdw: Str,
	/// `text`  Type of the server (optional)
	pub srvtype: Option<Str>,
	/// `text`  Version of the server (optional)
	pub srvversion: Option<Str>,
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	pub srvacl: Option<Vec<aclitem::ForeignServerAclItem>>,
	/// `text[]`  Foreign server specific options, as “keyword=value” strings
	pub srvoptions: Option<Vec<Str>>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
	/// `aclitem[]`  The initial access privileges from pg_init_privs.
	pub initprivs: Option<Vec<aclitem::ForeignServerAclItem>>,
	/// `char`  A code defining the type of initial privilege of this object from pg_init_privs. 'i' if set by initdb, 'e' if set by CREATE EXTENSION.
	pub initprivs_type: Option<PgForeignServerInitprivsType>,
}

pg_char_enum!(PgForeignServerInitprivsType { 'i' => InitDb, 'e' => CreateExtension });

pub async fn reflect_pg_foreign_server(
	client: &PgClient
) -> Result<Vec<PgForeignServer>, postgres::Error> {
	let pg_foreign_server_coll = reflect_crate::queries::reflect_gen::reflect_pg_foreign_server().bind(client)
		.map(|pg_foreign_server| {
			PgForeignServer {
				srvname: pg_foreign_server.srvname.into(),
				srvowner: pg_foreign_server.srvowner.into(),
				srvfdw: pg_foreign_server.srvfdw.into(),
				srvtype: pg_foreign_server.srvtype.map(Into::into),
				srvversion: pg_foreign_server.srvversion.map(Into::into),
				srvacl: pg_foreign_server.srvacl.map(|srvacl| srvacl.map(|acl| aclitem(acl, &ForeignServerGrantParser)).collect()),
				srvoptions: pg_foreign_server.srvoptions.map(|items| items.map(Into::into).collect()),
				description: pg_foreign_server.description.map(Into::into),
				initprivs: pg_foreign_server.initprivs.map(|initprivs| initprivs.map(|acl| aclitem(acl, &ForeignServerGrantParser)).collect()),
				initprivs_type: pg_foreign_server.initprivs_type.map(PgForeignServerInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_foreign_server_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgForeignTable {
	/// `oid` `(references pg_class.oid)` The OID of the pg_class entry for this foreign table
	pub ftrelid: Qual,
	/// `oid` `(references pg_foreign_server.oid)` OID of the foreign server for this foreign table
	pub ftserver: Str,
	/// `text[]`  Foreign table options, as “keyword=value” strings
	pub ftoptions: Option<Vec<Str>>,
}

pub async fn reflect_pg_foreign_table(
	client: &PgClient
) -> Result<Vec<PgForeignTable>, postgres::Error> {
	let pg_foreign_table_coll = reflect_crate::queries::reflect_gen::reflect_pg_foreign_table().bind(client)
		.map(|pg_foreign_table| {
			PgForeignTable {
				ftrelid: Qual::parse(pg_foreign_table.ftrelid),
				ftserver: pg_foreign_table.ftserver.into(),
				ftoptions: pg_foreign_table.ftoptions.map(|items| items.map(Into::into).collect()),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_foreign_table_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgIndex {
	/// `oid` `(references pg_class.oid)` The OID of the pg_class entry for this index
	pub indexrelid: Qual,
	/// `oid` `(references pg_class.oid)` The OID of the pg_class entry for the table this index is for
	pub indrelid: Qual,
	/// `int2`  The total number of columns in the index (duplicates pg_class.relnatts); this number includes both key and included attributes
	pub indnatts: u16,
	/// `int2`  The number of key columns in the index, not counting any included columns, which are merely stored and do not participate in the index semantics
	pub indnkeyatts: u16,
	/// `bool`  If true, this is a unique index
	pub indisunique: bool,
	/// `bool`  This value is only used for unique indexes. If false, this unique index will consider null values distinct (so the index can contain multiple null values in a column, the default PostgreSQL behavior). If it is true, it will consider null values to be equal (so the index can only contain one null value in a column).
	pub indnullsnotdistinct: bool,
	/// `bool`  If true, this index represents the primary key of the table (indisunique should always be true when this is true)
	pub indisprimary: bool,
	/// `bool`  If true, this index supports an exclusion constraint
	pub indisexclusion: bool,
	/// `bool`  If true, the uniqueness check is enforced immediately on insertion (irrelevant if indisunique is not true)
	pub indimmediate: bool,
	/// `bool`  If true, the table was last clustered on this index
	pub indisclustered: bool,
	// indisvalid bool  If true, the index is currently valid for queries. False means the index is possibly incomplete: it must still be modified by INSERT/UPDATE operations, but it cannot safely be used for queries. If it is unique, the uniqueness property is not guaranteed true either.
	// indcheckxmin bool  If true, queries must not use the index until the xmin of this pg_index row is below their TransactionXmin event horizon, because the table may contain broken HOT chains with incompatible rows that they can see
	// indisready bool  If true, the index is currently ready for inserts. False means the index must be ignored by INSERT/UPDATE operations.
	// indislive bool  If false, the index is in process of being dropped, and should be ignored for all purposes (including HOT-safety decisions)
	/// `bool`  If true this index has been chosen as “replica identity” using ALTER TABLE ... REPLICA IDENTITY USING INDEX ...
	pub indisreplident: bool,
	/// `int2vector` `(references pg_attribute.attnum)` This is an array of indnatts values that indicate which table columns this index indexes. For example, a value of 1 3 would mean that the first and the third table columns make up the index entries. Key columns come before non-key (included) columns. A zero in this array indicates that the corresponding index attribute is an expression over the table columns, rather than a simple column reference.
	pub indkey: Vec<u16>,
	/// `oidvector` `(references pg_collation.oid)` For each column in the index key (indnkeyatts values), this contains the OID of the collation to use for the index, or zero if the column is not of a collatable data type.
	pub indcollation: Vec<Option<Qual>>,
	/// `oidvector` `(references pg_opclass.oid)` For each column in the index key (indnkeyatts values), this contains the OID of the operator class to use. See pg_opclass for details.
	pub indclass: Vec<Qual>,
	/// `int2vector`  This is an array of indnkeyatts values that store per-column flag bits. The meaning of the bits is defined by the index's access method.
	pub indoption: Vec<i16>,
	/// `pg_node_tree`  Expression trees (in nodeToString() representation) for index attributes that are not simple column references. This is a list with one element for each zero entry in indkey. Null if all index attributes are simple references.
	pub indexprs: Option<Str>,
	/// `pg_node_tree`  Expression tree (in nodeToString() representation) for partial index predicate. Null if not a partial index.
	pub indpred: Option<Str>,
}

pub async fn reflect_pg_index(
	client: &PgClient
) -> Result<Vec<PgIndex>, postgres::Error> {
	let pg_index_coll = reflect_crate::queries::reflect_gen::reflect_pg_index().bind(client)
		.map(|pg_index| {
			PgIndex {
				indexrelid: Qual::parse(pg_index.indexrelid),
				indrelid: Qual::parse(pg_index.indrelid),
				indnatts: pg_index.indnatts.unsigned_abs(),
				indnkeyatts: pg_index.indnkeyatts.unsigned_abs(),
				indisunique: pg_index.indisunique,
				indnullsnotdistinct: pg_index.indnullsnotdistinct,
				indisprimary: pg_index.indisprimary,
				indisexclusion: pg_index.indisexclusion,
				indimmediate: pg_index.indimmediate,
				indisclustered: pg_index.indisclustered,
				indisreplident: pg_index.indisreplident,
				indkey: pg_index.indkey.map(i16::unsigned_abs).collect(),
				indcollation: pg_index.indcollation.map(Qual::maybe_parse).collect(),
				indclass: pg_index.indclass.map(Qual::parse).collect(),
				indoption: pg_index.indoption.collect(),
				indexprs: pg_index.indexprs.map(Into::into),
				indpred: pg_index.indpred.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_index_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgInherits {
	/// `oid` `(references pg_class.oid)` The OID of the child table or index
	pub inhrelid: Qual,
	/// `oid` `(references pg_class.oid)` The OID of the parent table or index
	pub inhparent: Qual,
	/// `int4`  If there is more than one direct parent for a child table (multiple inheritance), this number tells the order in which the inherited columns are to be arranged. The count starts at 1. Indexes cannot have multiple inheritance, since they can only inherit when using declarative partitioning.
	pub inhseqno: u32,
	// inhdetachpending bool  true for a partition that is in the process of being detached; false otherwise.
}

pub async fn reflect_pg_inherits(
	client: &PgClient
) -> Result<Vec<PgInherits>, postgres::Error> {
	let pg_inherits_coll = reflect_crate::queries::reflect_gen::reflect_pg_inherits().bind(client)
		.map(|pg_inherits| {
			PgInherits {
				inhrelid: Qual::parse(pg_inherits.inhrelid),
				inhparent: Qual::parse(pg_inherits.inhparent),
				inhseqno: pg_inherits.inhseqno.unsigned_abs(),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_inherits_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgLanguage {
	// oid oid  Row identifier
	/// `name`  Name of the language
	pub lanname: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the language
	pub lanowner: Str,
	/// `bool`  This is false for internal languages (such as SQL) and true for user-defined languages. Currently, pg_dump still uses this to determine which languages need to be dumped, but this might be replaced by a different mechanism in the future.
	pub lanispl: bool,
	/// `bool`  True if this is a trusted language, which means that it is believed not to grant access to anything outside the normal SQL execution environment. Only superusers can create functions in untrusted languages.
	pub lanpltrusted: bool,
	/// `oid` `(references pg_proc.oid)` For noninternal languages this references the language handler, which is a special function that is responsible for executing all functions that are written in the particular language. Zero for internal languages.
	pub lanplcallfoid: Option<Qual>,
	/// `oid` `(references pg_proc.oid)` This references a function that is responsible for executing “inline” anonymous code blocks (DO blocks). Zero if inline blocks are not supported.
	pub laninline: Option<Qual>,
	/// `oid` `(references pg_proc.oid)` This references a language validator function that is responsible for checking the syntax and validity of new functions when they are created. Zero if no validator is provided.
	pub lanvalidator: Option<Qual>,
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	pub lanacl: Option<Vec<aclitem::LanguageAclItem>>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
	/// `text`  The seclabel from pg_seclabel
	pub seclabel: Option<Str>,
	/// `text`  The provider from pg_seclabel
	pub seclabel_provider: Option<Str>,
	/// `aclitem[]`  The initial access privileges from pg_init_privs.
	pub initprivs: Option<Vec<aclitem::LanguageAclItem>>,
	/// `char`  A code defining the type of initial privilege of this object from pg_init_privs. 'i' if set by initdb, 'e' if set by CREATE EXTENSION.
	pub initprivs_type: Option<PgLanguageInitprivsType>,
}
impl_name_hash_and_equivalent!(PgLanguage, lanname);

pg_char_enum!(PgLanguageInitprivsType { 'i' => InitDb, 'e' => CreateExtension });

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
				description: pg_language.description.map(Into::into),
				seclabel: pg_language.seclabel.map(Into::into),
				seclabel_provider: pg_language.seclabel_provider.map(Into::into),
				initprivs: pg_language.initprivs.map(|initprivs| initprivs.map(|acl| aclitem(acl, &LanguageGrantParser)).collect()),
				initprivs_type: pg_language.initprivs_type.map(PgLanguageInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_language_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgNamespace {
	// oid oid  Row identifier
	/// `name`  Name of the namespace
	pub nspname: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the namespace
	pub nspowner: Str,
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	pub nspacl: Option<Vec<aclitem::SchemaAclItem>>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
	/// `text`  The seclabel from pg_seclabel
	pub seclabel: Option<Str>,
	/// `text`  The provider from pg_seclabel
	pub seclabel_provider: Option<Str>,
	/// `aclitem[]`  The initial access privileges from pg_init_privs.
	pub initprivs: Option<Vec<aclitem::SchemaAclItem>>,
	/// `char`  A code defining the type of initial privilege of this object from pg_init_privs. 'i' if set by initdb, 'e' if set by CREATE EXTENSION.
	pub initprivs_type: Option<PgNamespaceInitprivsType>,
}
impl_name_hash_and_equivalent!(PgNamespace, nspname);

pg_char_enum!(PgNamespaceInitprivsType { 'i' => InitDb, 'e' => CreateExtension });

pub async fn reflect_pg_namespace(
	client: &PgClient
) -> Result<Set<PgNamespace>, postgres::Error> {
	let pg_namespace_coll = reflect_crate::queries::reflect_gen::reflect_pg_namespace().bind(client)
		.map(|pg_namespace| {
			PgNamespace {
				nspname: pg_namespace.nspname.into(),
				nspowner: pg_namespace.nspowner.into(),
				nspacl: pg_namespace.nspacl.map(|nspacl| nspacl.map(|acl| aclitem(acl, &SchemaGrantParser)).collect()),
				description: pg_namespace.description.map(Into::into),
				seclabel: pg_namespace.seclabel.map(Into::into),
				seclabel_provider: pg_namespace.seclabel_provider.map(Into::into),
				initprivs: pg_namespace.initprivs.map(|initprivs| initprivs.map(|acl| aclitem(acl, &SchemaGrantParser)).collect()),
				initprivs_type: pg_namespace.initprivs_type.map(PgNamespaceInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_namespace_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgOpclass {
	// oid oid  Row identifier
	/// `oid` `(references pg_am.oid)` Index access method operator class is for
	pub opcmethod: Str,
	/// `name`  Name of this operator class
	pub opcname: Str,
	/// `oid` `(references pg_namespace.oid)` Namespace of this operator class
	pub opcnamespace: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the operator class
	pub opcowner: Str,
	/// `oid` `(references pg_opfamily.oid)` Operator family containing the operator class
	pub opcfamily: Qual,
	/// `oid` `(references pg_type.oid)` Data type that the operator class indexes
	pub opcintype: Qual,
	/// `bool`  True if this operator class is the default for opcintype
	pub opcdefault: bool,
	/// `oid` `(references pg_type.oid)` Type of data stored in index, or zero if same as opcintype
	pub opckeytype: Option<Qual>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
}

pub async fn reflect_pg_opclass(
	client: &PgClient
) -> Result<Vec<PgOpclass>, postgres::Error> {
	let pg_opclass_coll = reflect_crate::queries::reflect_gen::reflect_pg_opclass().bind(client)
		.map(|pg_opclass| {
			PgOpclass {
				opcmethod: pg_opclass.opcmethod.into(),
				opcname: pg_opclass.opcname.into(),
				opcnamespace: pg_opclass.opcnamespace.into(),
				opcowner: pg_opclass.opcowner.into(),
				opcfamily: Qual::parse(pg_opclass.opcfamily),
				opcintype: Qual::parse(pg_opclass.opcintype),
				opcdefault: pg_opclass.opcdefault,
				opckeytype: Qual::maybe_parse(pg_opclass.opckeytype),
				description: pg_opclass.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_opclass_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgOperator {
	/// `oid`  Row identifier
	pub oid: Qual,
	/// `name`  Name of the operator
	pub oprname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this operator
	pub oprnamespace: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the operator
	pub oprowner: Str,
	/// `char`  b = infix operator (“both”), or l = prefix operator (“left”)
	pub oprkind: PgOperatorOprkind,
	/// `bool`  This operator supports merge joins
	pub oprcanmerge: bool,
	/// `bool`  This operator supports hash joins
	pub oprcanhash: bool,
	/// `oid` `(references pg_type.oid)` Type of the left operand (zero for a prefix operator)
	pub oprleft: Option<Qual>,
	/// `oid` `(references pg_type.oid)` Type of the right operand
	pub oprright: Qual,
	/// `oid` `(references pg_type.oid)` Type of the result (zero for a not-yet-defined “shell” operator)
	pub oprresult: Option<Qual>,
	/// `oid` `(references pg_operator.oid)` Commutator of this operator (zero if none)
	pub oprcom: Option<Qual>,
	/// `oid` `(references pg_operator.oid)` Negator of this operator (zero if none)
	pub oprnegate: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Function that implements this operator (zero for a not-yet-defined “shell” operator)
	pub oprcode: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Restriction selectivity estimation function for this operator (zero if none)
	pub oprrest: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Join selectivity estimation function for this operator (zero if none)
	pub oprjoin: Option<Qual>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
}
impl_qual_hash_and_equivalent!(PgOperator);

pg_char_enum!(PgOperatorOprkind { 'b' => InfixOperatorBoth, 'l' => PrefixOperatorLeft });

pub async fn reflect_pg_operator(
	client: &PgClient
) -> Result<Set<PgOperator>, postgres::Error> {
	let pg_operator_coll = reflect_crate::queries::reflect_gen::reflect_pg_operator().bind(client)
		.map(|pg_operator| {
			PgOperator {
				oid: Qual::parse(pg_operator.oid),
				oprname: pg_operator.oprname.into(),
				oprnamespace: pg_operator.oprnamespace.into(),
				oprowner: pg_operator.oprowner.into(),
				oprkind: PgOperatorOprkind::pg_from_char(pg_operator.oprkind),
				oprcanmerge: pg_operator.oprcanmerge,
				oprcanhash: pg_operator.oprcanhash,
				oprleft: Qual::maybe_parse(pg_operator.oprleft),
				oprright: Qual::parse(pg_operator.oprright),
				oprresult: Qual::maybe_parse(pg_operator.oprresult),
				oprcom: Qual::maybe_parse(pg_operator.oprcom),
				oprnegate: Qual::maybe_parse(pg_operator.oprnegate),
				oprcode: Qual::maybe_parse(pg_operator.oprcode),
				oprrest: Qual::maybe_parse(pg_operator.oprrest),
				oprjoin: Qual::maybe_parse(pg_operator.oprjoin),
				description: pg_operator.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_operator_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgOpfamily {
	// oid oid  Row identifier
	/// `oid` `(references pg_am.oid)` Index access method operator family is for
	pub opfmethod: Str,
	/// `name`  Name of this operator family
	pub opfname: Str,
	/// `oid` `(references pg_namespace.oid)` Namespace of this operator family
	pub opfnamespace: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the operator family
	pub opfowner: Str,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
}

pub async fn reflect_pg_opfamily(
	client: &PgClient
) -> Result<Vec<PgOpfamily>, postgres::Error> {
	let pg_opfamily_coll = reflect_crate::queries::reflect_gen::reflect_pg_opfamily().bind(client)
		.map(|pg_opfamily| {
			PgOpfamily {
				opfmethod: pg_opfamily.opfmethod.into(),
				opfname: pg_opfamily.opfname.into(),
				opfnamespace: pg_opfamily.opfnamespace.into(),
				opfowner: pg_opfamily.opfowner.into(),
				description: pg_opfamily.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_opfamily_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgParameterAcl {
	// oid oid  Row identifier
	/// `text`  The name of a configuration parameter for which privileges are granted
	pub parname: Str,
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	pub paracl: Option<Vec<aclitem::ParameterAclItem>>,
	/// `aclitem[]`  The initial access privileges from pg_init_privs.
	pub initprivs: Option<Vec<aclitem::ParameterAclItem>>,
	/// `char`  A code defining the type of initial privilege of this object from pg_init_privs. 'i' if set by initdb, 'e' if set by CREATE EXTENSION.
	pub initprivs_type: Option<PgParameterAclInitprivsType>,
}

pg_char_enum!(PgParameterAclInitprivsType { 'i' => InitDb, 'e' => CreateExtension });

pub async fn reflect_pg_parameter_acl(
	client: &PgClient
) -> Result<Vec<PgParameterAcl>, postgres::Error> {
	let pg_parameter_acl_coll = reflect_crate::queries::reflect_gen::reflect_pg_parameter_acl().bind(client)
		.map(|pg_parameter_acl| {
			PgParameterAcl {
				parname: pg_parameter_acl.parname.into(),
				paracl: pg_parameter_acl.paracl.map(|paracl| paracl.map(|acl| aclitem(acl, &ParameterGrantParser)).collect()),
				initprivs: pg_parameter_acl.initprivs.map(|initprivs| initprivs.map(|acl| aclitem(acl, &ParameterGrantParser)).collect()),
				initprivs_type: pg_parameter_acl.initprivs_type.map(PgParameterAclInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_parameter_acl_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgPartitionedTable {
	/// `oid` `(references pg_class.oid)` The OID of the pg_class entry for this partitioned table
	pub partrelid: Qual,
	/// `char`  Partitioning strategy; h = hash partitioned table, l = list partitioned table, r = range partitioned table
	pub partstrat: PgPartitionedTablePartstrat,
	/// `int2`  The number of columns in the partition key
	pub partnatts: u16,
	/// `oid` `(references pg_class.oid)` The OID of the pg_class entry for the default partition of this partitioned table, or zero if this partitioned table does not have a default partition
	pub partdefid: Option<Qual>,
	/// `int2vector` `(references pg_attribute.attnum)` This is an array of partnatts values that indicate which table columns are part of the partition key. For example, a value of 1 3 would mean that the first and the third table columns make up the partition key. A zero in this array indicates that the corresponding partition key column is an expression, rather than a simple column reference.
	pub partattrs: Vec<u16>,
	/// `oidvector` `(references pg_opclass.oid)` For each column in the partition key, this contains the OID of the operator class to use. See pg_opclass for details.
	pub partclass: Vec<Qual>,
	/// `oidvector` `(references pg_collation.oid)` For each column in the partition key, this contains the OID of the collation to use for partitioning, or zero if the column is not of a collatable data type.
	pub partcollation: Vec<Option<Qual>>,
	/// `pg_node_tree`  Expression trees (in nodeToString() representation) for partition key columns that are not simple column references. This is a list with one element for each zero entry in partattrs. Null if all partition key columns are simple references.
	pub partexprs: Option<Str>,
}

pg_char_enum!(PgPartitionedTablePartstrat { 'h' => HashPartitionedTable, 'l' => ListPartitionedTable, 'r' => RangePartitionedTable });

pub async fn reflect_pg_partitioned_table(
	client: &PgClient
) -> Result<Vec<PgPartitionedTable>, postgres::Error> {
	let pg_partitioned_table_coll = reflect_crate::queries::reflect_gen::reflect_pg_partitioned_table().bind(client)
		.map(|pg_partitioned_table| {
			PgPartitionedTable {
				partrelid: Qual::parse(pg_partitioned_table.partrelid),
				partstrat: PgPartitionedTablePartstrat::pg_from_char(pg_partitioned_table.partstrat),
				partnatts: pg_partitioned_table.partnatts.unsigned_abs(),
				partdefid: Qual::maybe_parse(pg_partitioned_table.partdefid),
				partattrs: pg_partitioned_table.partattrs.map(i16::unsigned_abs).collect(),
				partclass: pg_partitioned_table.partclass.map(Qual::parse).collect(),
				partcollation: pg_partitioned_table.partcollation.map(Qual::maybe_parse).collect(),
				partexprs: pg_partitioned_table.partexprs.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_partitioned_table_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgPolicy {
	// oid oid  Row identifier
	/// `name`  The name of the policy
	pub polname: Str,
	/// `oid` `(references pg_class.oid)` The table to which the policy applies
	pub polrelid: Qual,
	/// `char`  The command type to which the policy is applied: r for SELECT, a for INSERT, w for UPDATE, d for DELETE, or * for all
	pub polcmd: PgPolicyPolcmd,
	/// `bool`  Is the policy permissive or restrictive?
	pub polpermissive: bool,
	/// `oid[]` `(references pg_authid.oid)` The roles to which the policy is applied; zero means PUBLIC (and normally appears alone in the array)
	pub polroles: Vec<Option<Str>>,
	/// `pg_node_tree`  The expression tree to be added to the security barrier qualifications for queries that use the table
	pub polqual: Option<Str>,
	/// `pg_node_tree`  The expression tree to be added to the WITH CHECK qualifications for queries that attempt to add rows to the table
	pub polwithcheck: Option<Str>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
}

pg_char_enum!(PgPolicyPolcmd { 'r' => Select, 'a' => Insert, 'w' => Update, 'd' => Delete, '*' => All });

pub async fn reflect_pg_policy(
	client: &PgClient
) -> Result<Vec<PgPolicy>, postgres::Error> {
	let pg_policy_coll = reflect_crate::queries::reflect_gen::reflect_pg_policy().bind(client)
		.map(|pg_policy| {
			PgPolicy {
				polname: pg_policy.polname.into(),
				polrelid: Qual::parse(pg_policy.polrelid),
				polcmd: PgPolicyPolcmd::pg_from_char(pg_policy.polcmd),
				polpermissive: pg_policy.polpermissive,
				polroles: pg_policy.polroles.map(|item| item.map(Into::into)).collect(),
				polqual: pg_policy.polqual.map(Into::into),
				polwithcheck: pg_policy.polwithcheck.map(Into::into),
				description: pg_policy.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_policy_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgPublication {
	// oid oid  Row identifier
	/// `name`  Name of the publication
	pub pubname: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the publication
	pub pubowner: Str,
	/// `bool`  If true, this publication automatically includes all tables in the database, including any that will be created in the future.
	pub puballtables: bool,
	/// `bool`  If true, INSERT operations are replicated for tables in the publication.
	pub pubinsert: bool,
	/// `bool`  If true, UPDATE operations are replicated for tables in the publication.
	pub pubupdate: bool,
	/// `bool`  If true, DELETE operations are replicated for tables in the publication.
	pub pubdelete: bool,
	/// `bool`  If true, TRUNCATE operations are replicated for tables in the publication.
	pub pubtruncate: bool,
	/// `bool`  If true, operations on a leaf partition are replicated using the identity and schema of its topmost partitioned ancestor mentioned in the publication instead of its own.
	pub pubviaroot: bool,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
	/// `text`  The seclabel from pg_seclabel
	pub seclabel: Option<Str>,
	/// `text`  The provider from pg_seclabel
	pub seclabel_provider: Option<Str>,
}
impl_name_hash_and_equivalent!(PgPublication, pubname);

pub async fn reflect_pg_publication(
	client: &PgClient
) -> Result<Set<PgPublication>, postgres::Error> {
	let pg_publication_coll = reflect_crate::queries::reflect_gen::reflect_pg_publication().bind(client)
		.map(|pg_publication| {
			PgPublication {
				pubname: pg_publication.pubname.into(),
				pubowner: pg_publication.pubowner.into(),
				puballtables: pg_publication.puballtables,
				pubinsert: pg_publication.pubinsert,
				pubupdate: pg_publication.pubupdate,
				pubdelete: pg_publication.pubdelete,
				pubtruncate: pg_publication.pubtruncate,
				pubviaroot: pg_publication.pubviaroot,
				description: pg_publication.description.map(Into::into),
				seclabel: pg_publication.seclabel.map(Into::into),
				seclabel_provider: pg_publication.seclabel_provider.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_publication_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgPublicationNamespace {
	// oid oid  Row identifier
	/// `oid` `(references pg_publication.oid)` Reference to publication
	pub pnpubid: Str,
	/// `oid` `(references pg_namespace.oid)` Reference to schema
	pub pnnspid: Str,
}

pub async fn reflect_pg_publication_namespace(
	client: &PgClient
) -> Result<Vec<PgPublicationNamespace>, postgres::Error> {
	let pg_publication_namespace_coll = reflect_crate::queries::reflect_gen::reflect_pg_publication_namespace().bind(client)
		.map(|pg_publication_namespace| {
			PgPublicationNamespace {
				pnpubid: pg_publication_namespace.pnpubid.into(),
				pnnspid: pg_publication_namespace.pnnspid.into(),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_publication_namespace_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgPublicationRel {
	// oid oid  Row identifier
	/// `oid` `(references pg_publication.oid)` Reference to publication
	pub prpubid: Str,
	/// `oid` `(references pg_class.oid)` Reference to relation
	pub prrelid: Qual,
	/// `pg_node_tree`  Expression tree (in nodeToString() representation) for the relation's publication qualifying condition. Null if there is no publication qualifying condition.
	pub prqual: Option<Str>,
	/// `int2vector` `(references pg_attribute.attnum)` This is an array of values that indicates which table columns are part of the publication. For example, a value of 1 3 would mean that the first and the third table columns are published. A null value indicates that all columns are published.
	pub prattrs: Option<Vec<u16>>,
}

pub async fn reflect_pg_publication_rel(
	client: &PgClient
) -> Result<Vec<PgPublicationRel>, postgres::Error> {
	let pg_publication_rel_coll = reflect_crate::queries::reflect_gen::reflect_pg_publication_rel().bind(client)
		.map(|pg_publication_rel| {
			PgPublicationRel {
				prpubid: pg_publication_rel.prpubid.into(),
				prrelid: Qual::parse(pg_publication_rel.prrelid),
				prqual: pg_publication_rel.prqual.map(Into::into),
				prattrs: pg_publication_rel.prattrs.map(|items| items.map(i16::unsigned_abs).collect()),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_publication_rel_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgRange {
	/// `oid` `(references pg_type.oid)` OID of the range type
	pub rngtypid: Qual,
	/// `oid` `(references pg_type.oid)` OID of the element type (subtype) of this range type
	pub rngsubtype: Qual,
	/// `oid` `(references pg_type.oid)` OID of the multirange type for this range type
	pub rngmultitypid: Qual,
	/// `oid` `(references pg_collation.oid)` OID of the collation used for range comparisons, or zero if none
	pub rngcollation: Option<Qual>,
	/// `oid` `(references pg_opclass.oid)` OID of the subtype's operator class used for range comparisons
	pub rngsubopc: Qual,
	/// `regproc` `(references pg_proc.oid)` OID of the function to convert a range value into canonical form, or zero if none
	pub rngcanonical: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` OID of the function to return the difference between two element values as double precision, or zero if none
	pub rngsubdiff: Option<Qual>,
}

pub async fn reflect_pg_range(
	client: &PgClient
) -> Result<Vec<PgRange>, postgres::Error> {
	let pg_range_coll = reflect_crate::queries::reflect_gen::reflect_pg_range().bind(client)
		.map(|pg_range| {
			PgRange {
				rngtypid: Qual::parse(pg_range.rngtypid),
				rngsubtype: Qual::parse(pg_range.rngsubtype),
				rngmultitypid: Qual::parse(pg_range.rngmultitypid),
				rngcollation: Qual::maybe_parse(pg_range.rngcollation),
				rngsubopc: Qual::parse(pg_range.rngsubopc),
				rngcanonical: Qual::maybe_parse(pg_range.rngcanonical),
				rngsubdiff: Qual::maybe_parse(pg_range.rngsubdiff),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_range_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgRules {
	/// `name` `(references pg_namespace.nspname)` Name of schema containing table
	pub schemaname: Str,
	/// `name` `(references pg_class.relname)` Name of table the rule is for
	pub tablename: Qual,
	/// `name` `(references pg_rewrite.rulename)` Name of rule
	pub rulename: Str,
	/// `text`  Rule definition (a reconstructed creation command)
	pub definition: Str,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
}

pub async fn reflect_pg_rules(
	client: &PgClient
) -> Result<Vec<PgRules>, postgres::Error> {
	let pg_rules_coll = reflect_crate::queries::reflect_gen::reflect_pg_rules().bind(client)
		.map(|pg_rules| {
			PgRules {
				schemaname: pg_rules.schemaname.into(),
				tablename: Qual::parse(pg_rules.tablename),
				rulename: pg_rules.rulename.into(),
				definition: pg_rules.definition.into(),
				description: pg_rules.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_rules_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgViews {
	/// `name` `(references pg_namespace.nspname)` Name of schema containing view
	pub schemaname: Str,
	/// `name` `(references pg_class.relname)` Name of view
	pub viewname: Qual,
	/// `name` `(references pg_authid.rolname)` Name of view's owner
	pub viewowner: Str,
	/// `text`  View definition (a reconstructed SELECT query)
	pub definition: Str,
}

pub async fn reflect_pg_views(
	client: &PgClient
) -> Result<Vec<PgViews>, postgres::Error> {
	let pg_views_coll = reflect_crate::queries::reflect_gen::reflect_pg_views().bind(client)
		.map(|pg_views| {
			PgViews {
				schemaname: pg_views.schemaname.into(),
				viewname: Qual::parse(pg_views.viewname),
				viewowner: pg_views.viewowner.into(),
				definition: pg_views.definition.into(),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_views_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgMatviews {
	/// `name` `(references pg_namespace.nspname)` Name of schema containing materialized view
	pub schemaname: Str,
	/// `name` `(references pg_class.relname)` Name of materialized view
	pub matviewname: Qual,
	/// `name` `(references pg_authid.rolname)` Name of materialized view's owner
	pub matviewowner: Str,
	// tablespace name (references pg_tablespace.spcname) Name of tablespace containing materialized view (null if default for database)
	// hasindexes bool  True if materialized view has (or recently had) any indexes
	// ispopulated bool  True if materialized view is currently populated
	/// `text`  Materialized view definition (a reconstructed SELECT query)
	pub definition: Str,
}

pub async fn reflect_pg_matviews(
	client: &PgClient
) -> Result<Vec<PgMatviews>, postgres::Error> {
	let pg_matviews_coll = reflect_crate::queries::reflect_gen::reflect_pg_matviews().bind(client)
		.map(|pg_matviews| {
			PgMatviews {
				schemaname: pg_matviews.schemaname.into(),
				matviewname: Qual::parse(pg_matviews.matviewname),
				matviewowner: pg_matviews.matviewowner.into(),
				definition: pg_matviews.definition.into(),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_matviews_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgSequence {
	/// `oid` `(references pg_class.oid)` The OID of the pg_class entry for this sequence
	pub seqrelid: Qual,
	/// `oid` `(references pg_type.oid)` Data type of the sequence
	pub seqtypid: Qual,
	/// `int8`  Start value of the sequence
	pub seqstart: i64,
	/// `int8`  Increment value of the sequence
	pub seqincrement: i64,
	/// `int8`  Maximum value of the sequence
	pub seqmax: i64,
	/// `int8`  Minimum value of the sequence
	pub seqmin: i64,
	/// `int8`  Cache size of the sequence
	pub seqcache: i64,
	/// `bool`  Whether the sequence cycles
	pub seqcycle: bool,
}

pub async fn reflect_pg_sequence(
	client: &PgClient
) -> Result<Vec<PgSequence>, postgres::Error> {
	let pg_sequence_coll = reflect_crate::queries::reflect_gen::reflect_pg_sequence().bind(client)
		.map(|pg_sequence| {
			PgSequence {
				seqrelid: Qual::parse(pg_sequence.seqrelid),
				seqtypid: Qual::parse(pg_sequence.seqtypid),
				seqstart: pg_sequence.seqstart,
				seqincrement: pg_sequence.seqincrement,
				seqmax: pg_sequence.seqmax,
				seqmin: pg_sequence.seqmin,
				seqcache: pg_sequence.seqcache,
				seqcycle: pg_sequence.seqcycle,
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_sequence_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgStatisticExt {
	// oid oid  Row identifier
	/// `oid` `(references pg_class.oid)` Table containing the columns described by this object
	pub stxrelid: Qual,
	/// `name`  Name of the statistics object
	pub stxname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this statistics object
	pub stxnamespace: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the statistics object
	pub stxowner: Str,
	/// `int2vector` `(references pg_attribute.attnum)` An array of attribute numbers, indicating which table columns are covered by this statistics object; for example a value of 1 3 would mean that the first and the third table columns are covered
	pub stxkeys: Vec<u16>,
	/// `int2`  stxstattarget controls the level of detail of statistics accumulated for this statistics object by ANALYZE. A zero value indicates that no statistics should be collected. A null value says to use the maximum of the statistics targets of the referenced columns, if set, or the system default statistics target. Positive values of stxstattarget determine the target number of “most common values” to collect.
	pub stxstattarget: Option<u16>,
	/// `char[]`  An array containing codes for the enabled statistics kinds; valid values are: d for n-distinct statistics, f for functional dependency statistics, m for most common values (MCV) list statistics, and e for expression statistics
	pub stxkind: Vec<PgStatisticExtStxkind>,
	/// `pg_node_tree`  Expression trees (in nodeToString() representation) for statistics object attributes that are not simple column references. This is a list with one element per expression. Null if all statistics object attributes are simple references.
	pub stxexprs: Option<Str>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
}

pg_char_enum!(PgStatisticExtStxkind { 'd' => NDistinct, 'f' => FunctionalDependency, 'm' => MostCommonValuesList, 'e' => Expression });

pub async fn reflect_pg_statistic_ext(
	client: &PgClient
) -> Result<Vec<PgStatisticExt>, postgres::Error> {
	let pg_statistic_ext_coll = reflect_crate::queries::reflect_gen::reflect_pg_statistic_ext().bind(client)
		.map(|pg_statistic_ext| {
			PgStatisticExt {
				stxrelid: Qual::parse(pg_statistic_ext.stxrelid),
				stxname: pg_statistic_ext.stxname.into(),
				stxnamespace: pg_statistic_ext.stxnamespace.into(),
				stxowner: pg_statistic_ext.stxowner.into(),
				stxkeys: pg_statistic_ext.stxkeys.map(i16::unsigned_abs).collect(),
				stxstattarget: pg_statistic_ext.stxstattarget.map(i16::unsigned_abs),
				stxkind: pg_statistic_ext.stxkind.map(PgStatisticExtStxkind::pg_from_char).collect(),
				stxexprs: pg_statistic_ext.stxexprs.map(Into::into),
				description: pg_statistic_ext.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_statistic_ext_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgSubscription {
	// oid oid  Row identifier
	// subdbid oid (references pg_database.oid) OID of the database that the subscription resides in
	// subskiplsn pg_lsn  Finish LSN of the transaction whose changes are to be skipped, if a valid LSN; otherwise 0/0.
	/// `name`  Name of the subscription
	pub subname: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the subscription
	pub subowner: Str,
	/// `bool`  If true, the subscription is enabled and should be replicating
	pub subenabled: bool,
	/// `bool`  If true, the subscription will request that the publisher send data in binary format
	pub subbinary: bool,
	/// `char`  Controls how to handle the streaming of in-progress transactions: f = disallow streaming of in-progress transactions, t = spill the changes of in-progress transactions to disk and apply at once after the transaction is committed on the publisher and received by the subscriber, p = apply changes directly using a parallel apply worker if available (same as t if no worker is available)
	pub substream: PgSubscriptionSubstream,
	/// `char`  State codes for two-phase mode: d = disabled, p = pending enablement, e = enabled
	pub subtwophasestate: PgSubscriptionSubtwophasestate,
	/// `bool`  If true, the subscription will be disabled if one of its workers detects an error
	pub subdisableonerr: bool,
	/// `bool`  If true, the subscription will be required to specify a password for authentication
	pub subpasswordrequired: bool,
	/// `bool`  If true, the subscription will be run with the permissions of the subscription owner
	pub subrunasowner: bool,
	/// `bool`  If true, the associated replication slots (i.e. the main slot and the table sync slots) in the upstream database are enabled to be synchronized to the standbys
	pub subfailover: bool,
	/// `text`  Connection string to the upstream database
	pub subconninfo: Str,
	/// `name`  Name of the replication slot in the upstream database (also used for the local replication origin name); null represents NONE
	pub subslotname: Option<Str>,
	/// `text`  The synchronous_commit setting for the subscription's workers to use
	pub subsynccommit: Str,
	/// `text[]`  Array of subscribed publication names. These reference publications defined in the upstream database. For more on publications see Section 29.1.
	pub subpublications: Vec<Str>,
	/// `text`  The origin value must be either none or any. The default is any. If none, the subscription will request the publisher to only send changes that don't have an origin. If any, the publisher sends changes regardless of their origin.
	pub suborigin: Option<Str>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
	/// `text`  The seclabel from pg_seclabel
	pub seclabel: Option<Str>,
	/// `text`  The provider from pg_seclabel
	pub seclabel_provider: Option<Str>,
}

pg_char_enum!(PgSubscriptionSubstream { 'f' => DisallowStreamingInProgress, 't' => SpillApplyAfterCommitted, 'p' => ApplyDirectlyParallel });
pg_char_enum!(PgSubscriptionSubtwophasestate { 'd' => Disabled, 'p' => PendingEnablement, 'e' => Enabled });

pub async fn reflect_pg_subscription(
	client: &PgClient
) -> Result<Vec<PgSubscription>, postgres::Error> {
	let pg_subscription_coll = reflect_crate::queries::reflect_gen::reflect_pg_subscription().bind(client)
		.map(|pg_subscription| {
			PgSubscription {
				subname: pg_subscription.subname.into(),
				subowner: pg_subscription.subowner.into(),
				subenabled: pg_subscription.subenabled,
				subbinary: pg_subscription.subbinary,
				substream: PgSubscriptionSubstream::pg_from_char(pg_subscription.substream),
				subtwophasestate: PgSubscriptionSubtwophasestate::pg_from_char(pg_subscription.subtwophasestate),
				subdisableonerr: pg_subscription.subdisableonerr,
				subpasswordrequired: pg_subscription.subpasswordrequired,
				subrunasowner: pg_subscription.subrunasowner,
				subfailover: pg_subscription.subfailover,
				subconninfo: pg_subscription.subconninfo.into(),
				subslotname: pg_subscription.subslotname.map(Into::into),
				subsynccommit: pg_subscription.subsynccommit.into(),
				subpublications: pg_subscription.subpublications.map(Into::into).collect(),
				suborigin: pg_subscription.suborigin.map(Into::into),
				description: pg_subscription.description.map(Into::into),
				seclabel: pg_subscription.seclabel.map(Into::into),
				seclabel_provider: pg_subscription.seclabel_provider.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_subscription_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgTransform {
	// oid oid  Row identifier
	/// `oid` `(references pg_type.oid)` OID of the data type this transform is for
	pub trftype: Qual,
	/// `oid` `(references pg_language.oid)` OID of the language this transform is for
	pub trflang: Str,
	/// `regproc` `(references pg_proc.oid)` The OID of the function to use when converting the data type for input to the procedural language (e.g., function parameters). Zero is stored if the default behavior should be used.
	pub trffromsql: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` The OID of the function to use when converting output from the procedural language (e.g., return values) to the data type. Zero is stored if the default behavior should be used.
	pub trftosql: Option<Qual>,
}

pub async fn reflect_pg_transform(
	client: &PgClient
) -> Result<Vec<PgTransform>, postgres::Error> {
	let pg_transform_coll = reflect_crate::queries::reflect_gen::reflect_pg_transform().bind(client)
		.map(|pg_transform| {
			PgTransform {
				trftype: Qual::parse(pg_transform.trftype),
				trflang: pg_transform.trflang.into(),
				trffromsql: Qual::maybe_parse(pg_transform.trffromsql),
				trftosql: Qual::maybe_parse(pg_transform.trftosql),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_transform_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgTrigger {
	// oid oid  Row identifier
	/// `oid` `(references pg_class.oid)` The table this trigger is on
	pub tgrelid: Qual,
	/// `oid` `(references pg_trigger.oid)` Parent trigger that this trigger is cloned from (this happens when partitions are created or attached to a partitioned table); zero if not a clone
	pub tgparentid: Option<Str>,
	/// `name`  Trigger name (must be unique among triggers of same table)
	pub tgname: Str,
	/// `oid` `(references pg_proc.oid)` The function to be called
	pub tgfoid: Qual,
	/// `int2`  Bit mask identifying trigger firing conditions
	pub tgtype: i16,
	/// `char`  Controls in which session_replication_role modes the trigger fires. O = trigger fires in “origin” and “local” modes, D = trigger is disabled, R = trigger fires in “replica” mode, A = trigger fires always.
	pub tgenabled: PgTriggerTgenabled,
	/// `bool`  True if trigger is internally generated (usually, to enforce the constraint identified by tgconstraint)
	pub tgisinternal: bool,
	/// `oid` `(references pg_class.oid)` The table referenced by a referential integrity constraint (zero if trigger is not for a referential integrity constraint)
	pub tgconstrrelid: Option<Qual>,
	/// `oid` `(references pg_class.oid)` The index supporting a unique, primary key, referential integrity, or exclusion constraint (zero if trigger is not for one of these types of constraint)
	pub tgconstrindid: Option<Qual>,
	/// `oid` `(references pg_constraint.oid)` The pg_constraint entry associated with the trigger (zero if trigger is not for a constraint)
	pub tgconstraint: Option<Qual>,
	/// `bool`  True if constraint trigger is deferrable
	pub tgdeferrable: bool,
	/// `bool`  True if constraint trigger is initially deferred
	pub tginitdeferred: bool,
	/// `int2`  Number of argument strings passed to trigger function
	pub tgnargs: u16,
	/// `int2vector` `(references pg_attribute.attnum)` Column numbers, if trigger is column-specific; otherwise an empty array
	pub tgattr: Vec<u16>,
	/// `bytea`  Argument strings to pass to trigger, each NULL-terminated
	pub tgargs: Vec<u8>,
	/// `pg_node_tree`  Expression tree (in nodeToString() representation) for the trigger's WHEN condition, or null if none
	pub tgqual: Option<Str>,
	/// `name`  REFERENCING clause name for OLD TABLE, or null if none
	pub tgoldtable: Option<Str>,
	/// `name`  REFERENCING clause name for NEW TABLE, or null if none
	pub tgnewtable: Option<Str>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
	/// `text`  The seclabel from pg_seclabel
	pub seclabel: Option<Str>,
	/// `text`  The provider from pg_seclabel
	pub seclabel_provider: Option<Str>,
}

pg_char_enum!(PgTriggerTgenabled { 'O' => OriginAndLocalMode, 'D' => Disabled, 'R' => ReplicaMode, 'A' => Always });

pub async fn reflect_pg_trigger(
	client: &PgClient
) -> Result<Vec<PgTrigger>, postgres::Error> {
	let pg_trigger_coll = reflect_crate::queries::reflect_gen::reflect_pg_trigger().bind(client)
		.map(|pg_trigger| {
			PgTrigger {
				tgrelid: Qual::parse(pg_trigger.tgrelid),
				tgparentid: pg_trigger.tgparentid.map(Into::into),
				tgname: pg_trigger.tgname.into(),
				tgfoid: Qual::parse(pg_trigger.tgfoid),
				tgtype: pg_trigger.tgtype,
				tgenabled: PgTriggerTgenabled::pg_from_char(pg_trigger.tgenabled),
				tgisinternal: pg_trigger.tgisinternal,
				tgconstrrelid: Qual::maybe_parse(pg_trigger.tgconstrrelid),
				tgconstrindid: Qual::maybe_parse(pg_trigger.tgconstrindid),
				tgconstraint: Qual::maybe_parse(pg_trigger.tgconstraint),
				tgdeferrable: pg_trigger.tgdeferrable,
				tginitdeferred: pg_trigger.tginitdeferred,
				tgnargs: pg_trigger.tgnargs.unsigned_abs(),
				tgattr: pg_trigger.tgattr.map(i16::unsigned_abs).collect(),
				tgargs: pg_trigger.tgargs.into(),
				tgqual: pg_trigger.tgqual.map(Into::into),
				tgoldtable: pg_trigger.tgoldtable.map(Into::into),
				tgnewtable: pg_trigger.tgnewtable.map(Into::into),
				description: pg_trigger.description.map(Into::into),
				seclabel: pg_trigger.seclabel.map(Into::into),
				seclabel_provider: pg_trigger.seclabel_provider.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_trigger_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgTsConfig {
	/// `oid`  Row identifier
	pub oid: Qual,
	/// `name`  Text search configuration name
	pub cfgname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this configuration
	pub cfgnamespace: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the configuration
	pub cfgowner: Str,
	// cfgparser oid (references pg_ts_parser.oid) The OID of the text search parser for this configuration
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
	/// `text`  The seclabel from pg_seclabel
	pub seclabel: Option<Str>,
	/// `text`  The provider from pg_seclabel
	pub seclabel_provider: Option<Str>,
}
impl_qual_hash_and_equivalent!(PgTsConfig);

pub async fn reflect_pg_ts_config(
	client: &PgClient
) -> Result<Set<PgTsConfig>, postgres::Error> {
	let pg_ts_config_coll = reflect_crate::queries::reflect_gen::reflect_pg_ts_config().bind(client)
		.map(|pg_ts_config| {
			PgTsConfig {
				oid: Qual::parse(pg_ts_config.oid),
				cfgname: pg_ts_config.cfgname.into(),
				cfgnamespace: pg_ts_config.cfgnamespace.into(),
				cfgowner: pg_ts_config.cfgowner.into(),
				description: pg_ts_config.description.map(Into::into),
				seclabel: pg_ts_config.seclabel.map(Into::into),
				seclabel_provider: pg_ts_config.seclabel_provider.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_ts_config_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgTsConfigMap {
	/// `oid` `(references pg_ts_config.oid)` The OID of the pg_ts_config entry owning this map entry
	pub mapcfg: Qual,
	/// `int4`  A token type emitted by the configuration's parser
	pub maptokentype: i32,
	/// `int4`  Order in which to consult this entry (lower mapseqnos first)
	pub mapseqno: i32,
	/// `oid` `(references pg_ts_dict.oid)` The OID of the text search dictionary to consult
	pub mapdict: Qual,
}

pub async fn reflect_pg_ts_config_map(
	client: &PgClient
) -> Result<Vec<PgTsConfigMap>, postgres::Error> {
	let pg_ts_config_map_coll = reflect_crate::queries::reflect_gen::reflect_pg_ts_config_map().bind(client)
		.map(|pg_ts_config_map| {
			PgTsConfigMap {
				mapcfg: Qual::parse(pg_ts_config_map.mapcfg),
				maptokentype: pg_ts_config_map.maptokentype,
				mapseqno: pg_ts_config_map.mapseqno,
				mapdict: Qual::parse(pg_ts_config_map.mapdict),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_ts_config_map_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgTsDict {
	/// `oid`  Row identifier
	pub oid: Qual,
	/// `name`  Text search dictionary name
	pub dictname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this dictionary
	pub dictnamespace: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the dictionary
	pub dictowner: Str,
	// dicttemplate oid (references pg_ts_template.oid) The OID of the text search template for this dictionary
	/// `text`  Initialization option string for the template
	pub dictinitoption: Option<Str>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
	/// `text`  The seclabel from pg_seclabel
	pub seclabel: Option<Str>,
	/// `text`  The provider from pg_seclabel
	pub seclabel_provider: Option<Str>,
}
impl_qual_hash_and_equivalent!(PgTsDict);

pub async fn reflect_pg_ts_dict(
	client: &PgClient
) -> Result<Set<PgTsDict>, postgres::Error> {
	let pg_ts_dict_coll = reflect_crate::queries::reflect_gen::reflect_pg_ts_dict().bind(client)
		.map(|pg_ts_dict| {
			PgTsDict {
				oid: Qual::parse(pg_ts_dict.oid),
				dictname: pg_ts_dict.dictname.into(),
				dictnamespace: pg_ts_dict.dictnamespace.into(),
				dictowner: pg_ts_dict.dictowner.into(),
				dictinitoption: pg_ts_dict.dictinitoption.map(Into::into),
				description: pg_ts_dict.description.map(Into::into),
				seclabel: pg_ts_dict.seclabel.map(Into::into),
				seclabel_provider: pg_ts_dict.seclabel_provider.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_ts_dict_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgTsParser {
	// oid oid  Row identifier
	/// `name`  Text search parser name
	pub prsname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this parser
	pub prsnamespace: Str,
	/// `regproc` `(references pg_proc.oid)` OID of the parser's startup function
	pub prsstart: Qual,
	/// `regproc` `(references pg_proc.oid)` OID of the parser's next-token function
	pub prstoken: Qual,
	/// `regproc` `(references pg_proc.oid)` OID of the parser's shutdown function
	pub prsend: Qual,
	/// `regproc` `(references pg_proc.oid)` OID of the parser's headline function (zero if none)
	pub prsheadline: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` OID of the parser's lextype function
	pub prslextype: Qual,
}

pub async fn reflect_pg_ts_parser(
	client: &PgClient
) -> Result<Vec<PgTsParser>, postgres::Error> {
	let pg_ts_parser_coll = reflect_crate::queries::reflect_gen::reflect_pg_ts_parser().bind(client)
		.map(|pg_ts_parser| {
			PgTsParser {
				prsname: pg_ts_parser.prsname.into(),
				prsnamespace: pg_ts_parser.prsnamespace.into(),
				prsstart: Qual::parse(pg_ts_parser.prsstart),
				prstoken: Qual::parse(pg_ts_parser.prstoken),
				prsend: Qual::parse(pg_ts_parser.prsend),
				prsheadline: Qual::maybe_parse(pg_ts_parser.prsheadline),
				prslextype: Qual::parse(pg_ts_parser.prslextype),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_ts_parser_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgTsTemplate {
	// oid oid  Row identifier
	/// `name`  Text search template name
	pub tmplname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this template
	pub tmplnamespace: Str,
	/// `regproc` `(references pg_proc.oid)` OID of the template's initialization function (zero if none)
	pub tmplinit: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` OID of the template's lexize function
	pub tmpllexize: Qual,
}

pub async fn reflect_pg_ts_template(
	client: &PgClient
) -> Result<Vec<PgTsTemplate>, postgres::Error> {
	let pg_ts_template_coll = reflect_crate::queries::reflect_gen::reflect_pg_ts_template().bind(client)
		.map(|pg_ts_template| {
			PgTsTemplate {
				tmplname: pg_ts_template.tmplname.into(),
				tmplnamespace: pg_ts_template.tmplnamespace.into(),
				tmplinit: Qual::maybe_parse(pg_ts_template.tmplinit),
				tmpllexize: Qual::parse(pg_ts_template.tmpllexize),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_ts_template_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgType {
	/// `oid`  Row identifier
	pub oid: Qual,
	/// `name`  Data type name
	pub typname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this type
	pub typnamespace: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the type
	pub typowner: Str,
	/// `int2`  For a fixed-size type, typlen is the number of bytes in the internal representation of the type. But for a variable-length type, typlen is negative. -1 indicates a “varlena” type (one that has a length word), -2 indicates a null-terminated C string.
	pub typlen: i16,
	/// `bool`  typbyval determines whether internal routines pass a value of this type by value or by reference. typbyval had better be false if typlen is not 1, 2, or 4 (or 8 on machines where Datum is 8 bytes). Variable-length types are always passed by reference. Note that typbyval can be false even if the length would allow pass-by-value.
	pub typbyval: bool,
	/// `char`  typtype is b for a base type, c for a composite type (e.g., a table's row type), d for a domain, e for an enum type, p for a pseudo-type, r for a range type, or m for a multirange type. See also typrelid and typbasetype.
	pub typtype: PgTypeTyptype,
	// typcategory char  typcategory is an arbitrary classification of data types that is used by the parser to determine which implicit casts should be “preferred”. See Table 51.65.
	/// `bool`  True if the type is a preferred cast target within its typcategory
	pub typispreferred: bool,
	/// `bool`  True if the type is defined, false if this is a placeholder entry for a not-yet-defined type. When typisdefined is false, nothing except the type name, namespace, and OID can be relied on.
	pub typisdefined: bool,
	/// `char`  Character that separates two values of this type when parsing array input. Note that the delimiter is associated with the array element data type, not the array data type.
	pub typdelim: i8,
	/// `oid` `(references pg_class.oid)` If this is a composite type (see typtype), then this column points to the pg_class entry that defines the corresponding table. (For a free-standing composite type, the pg_class entry doesn't really represent a table, but it is needed anyway for the type's pg_attribute entries to link to.) Zero for non-composite types.
	pub typrelid: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Subscripting handler function's OID, or zero if this type doesn't support subscripting. Types that are “true” array types have typsubscript = array_subscript_handler, but other types may have other handler functions to implement specialized subscripting behavior.
	pub typsubscript: Option<Qual>,
	/// `oid` `(references pg_type.oid)` If typelem is not zero then it identifies another row in pg_type, defining the type yielded by subscripting. This should be zero if typsubscript is zero. However, it can be zero when typsubscript isn't zero, if the handler doesn't need typelem to determine the subscripting result type. Note that a typelem dependency is considered to imply physical containment of the element type in this type; so DDL changes on the element type might be restricted by the presence of this type.
	pub typelem: Option<Qual>,
	/// `oid` `(references pg_type.oid)` If typarray is not zero then it identifies another row in pg_type, which is the “true” array type having this type as element
	pub typarray: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Input conversion function (text format)
	pub typinput: Qual,
	/// `regproc` `(references pg_proc.oid)` Output conversion function (text format)
	pub typoutput: Qual,
	/// `regproc` `(references pg_proc.oid)` Input conversion function (binary format), or zero if none
	pub typreceive: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Output conversion function (binary format), or zero if none
	pub typsend: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Type modifier input function, or zero if type does not support modifiers
	pub typmodin: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Type modifier output function, or zero to use the standard format
	pub typmodout: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Custom ANALYZE function, or zero to use the standard function
	pub typanalyze: Option<Qual>,
	/// `char`  typalign is the alignment required when storing a value of this type. It applies to storage on disk as well as most representations of the value inside PostgreSQL. When multiple values are stored consecutively, such as in the representation of a complete row on disk, padding is inserted before a datum of this type so that it begins on the specified boundary. The alignment reference is the beginning of the first datum in the sequence. Possible values are: c = char alignment, i.e., no alignment needed. s = short alignment (2 bytes on most machines). i = int alignment (4 bytes on most machines). d = double alignment (8 bytes on many machines, but by no means all).
	pub typalign: PgTypeTypalign,
	/// `char`  typstorage tells for varlena types (those with typlen = -1) if the type is prepared for toasting and what the default strategy for attributes of this type should be. Possible values are: p (plain): Values must always be stored plain (non-varlena types always use this value). e (external): Values can be stored in a secondary “TOAST” relation (if relation has one, see pg_class.reltoastrelid). m (main): Values can be compressed and stored inline. x (extended): Values can be compressed and/or moved to a secondary relation. x is the usual choice for toast-able types. Note that m values can also be moved out to secondary storage, but only as a last resort (e and x values are moved first).
	pub typstorage: PgTypeTypstorage,
	/// `bool`  typnotnull represents a not-null constraint on a type. Used for domains only.
	pub typnotnull: bool,
	/// `oid` `(references pg_type.oid)` If this is a domain (see typtype), then typbasetype identifies the type that this one is based on. Zero if this type is not a domain.
	pub typbasetype: Option<Qual>,
	/// `int4`  Domains use typtypmod to record the typmod to be applied to their base type (-1 if base type does not use a typmod). -1 if this type is not a domain.
	pub typtypmod: Option<u32>,
	/// `int4`  typndims is the number of array dimensions for a domain over an array (that is, typbasetype is an array type). Zero for types other than domains over array types.
	pub typndims: u32,
	/// `oid` `(references pg_collation.oid)` typcollation specifies the collation of the type. If the type does not support collations, this will be zero. A base type that supports collations will have a nonzero value here, typically DEFAULT_COLLATION_OID. A domain over a collatable type can have a collation OID different from its base type's, if one was specified for the domain.
	pub typcollation: Option<Qual>,
	/// `pg_node_tree`  If typdefaultbin is not null, it is the nodeToString() representation of a default expression for the type. This is only used for domains.
	pub typdefaultbin: Option<Str>,
	/// `text`  typdefault is null if the type has no associated default value. If typdefaultbin is not null, typdefault must contain a human-readable version of the default expression represented by typdefaultbin. If typdefaultbin is null and typdefault is not, then typdefault is the external representation of the type's default value, which can be fed to the type's input converter to produce a constant.
	pub typdefault: Option<Str>,
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	pub typacl: Option<Vec<aclitem::TypeAclItem>>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
	/// `text`  The seclabel from pg_seclabel
	pub seclabel: Option<Str>,
	/// `text`  The provider from pg_seclabel
	pub seclabel_provider: Option<Str>,
	/// `aclitem[]`  The initial access privileges from pg_init_privs.
	pub initprivs: Option<Vec<aclitem::TypeAclItem>>,
	/// `char`  A code defining the type of initial privilege of this object from pg_init_privs. 'i' if set by initdb, 'e' if set by CREATE EXTENSION.
	pub initprivs_type: Option<PgTypeInitprivsType>,
}
impl_qual_hash_and_equivalent!(PgType);

pg_char_enum!(PgTypeTyptype { 'b' => Base, 'c' => Composite, 'd' => Domain, 'e' => Enum, 'p' => Pseudo, 'r' => Range, 'm' => Multirange });
pg_char_enum!(PgTypeTypalign { 'c' => Char, 's' => Short, 'i' => Int, 'd' => Double });
pg_char_enum!(PgTypeTypstorage { 'p' => Plain, 'e' => External, 'm' => Main, 'x' => Extended });
pg_char_enum!(PgTypeInitprivsType { 'i' => InitDb, 'e' => CreateExtension });

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
				description: pg_type.description.map(Into::into),
				seclabel: pg_type.seclabel.map(Into::into),
				seclabel_provider: pg_type.seclabel_provider.map(Into::into),
				initprivs: pg_type.initprivs.map(|initprivs| initprivs.map(|acl| aclitem(acl, &TypeGrantParser)).collect()),
				initprivs_type: pg_type.initprivs_type.map(PgTypeInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_type_coll)
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgUserMappings {
	// umid oid (references pg_user_mapping.oid) OID of the user mapping
	// srvid oid (references pg_foreign_server.oid) The OID of the foreign server that contains this mapping
	/// `name` `(references pg_foreign_server.srvname)` Name of the foreign server
	pub srvname: Str,
	/// `oid` `(references pg_authid.oid)` OID of the local role being mapped, or zero if the user mapping is public
	pub umuser: Option<Str>,
	/// `name`  Name of the local user to be mapped
	pub usename: Str,
	/// `text[]`  User mapping specific options, as “keyword=value” strings
	pub umoptions: Option<Vec<Str>>,
}

pub async fn reflect_pg_user_mappings(
	client: &PgClient
) -> Result<Vec<PgUserMappings>, postgres::Error> {
	let pg_user_mappings_coll = reflect_crate::queries::reflect_gen::reflect_pg_user_mappings().bind(client)
		.map(|pg_user_mappings| {
			PgUserMappings {
				srvname: pg_user_mappings.srvname.into(),
				umuser: pg_user_mappings.umuser.map(Into::into),
				usename: pg_user_mappings.usename.into(),
				umoptions: pg_user_mappings.umoptions.map(|items| items.map(Into::into).collect()),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_user_mappings_coll)
}

