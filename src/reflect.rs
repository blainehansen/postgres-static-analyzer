use crate::*;
use crate::aclitem::*;
use futures::TryStreamExt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PgState {
	pub pg_aggregate: Vec<PgAggregate>,
	pub pg_am: Set<PgAm>,
	pub pg_amop: Vec<PgAmop>,
	pub pg_amproc: Vec<PgAmproc>,
	pub pg_attrdef: Vec<PgAttrdef>,
	pub pg_attribute: Vec<PgAttribute>,
	pub pg_roles: Set<PgRoles>,
	pub pg_auth_members: Vec<PgAuthMembers>,
	pub pg_cast: Vec<PgCast>,
	pub pg_class: Set<PgClass>,
	pub pg_collation: Vec<PgCollation>,
	pub pg_constraint: Vec<PgConstraint>,
	pub pg_conversion: Vec<PgConversion>,
	pub pg_database: PgDatabase,
	pub pg_db_role_setting: Vec<PgDbRoleSetting>,
	pub pg_default_acl: Vec<PgDefaultAcl>,
	// pub pg_depend: PgDepend,
	// pub pg_description: PgDescription,
	pub pg_enum: Set<PgEnum>,
	pub pg_event_trigger: Vec<PgEventTrigger>,
	pub pg_extension: Vec<PgExtension>,
	pub pg_foreign_data_wrapper: Vec<PgForeignDataWrapper>,
	pub pg_foreign_server: Vec<PgForeignServer>,
	pub pg_foreign_table: Vec<PgForeignTable>,
	// pub pg_index: PgIndex,
	// pub pg_inherits: PgInherits,
	// pub pg_init_privs: PgInitPrivs,
	pub pg_language: Set<PgLanguage>,
	pub pg_namespace: Set<PgNamespace>,
	pub pg_opclass: Vec<PgOpclass>,
	// pub pg_operator: PgOperator,
	// pub pg_opfamily: PgOpfamily,
	// pub pg_parameter_acl: PgParameterAcl,
	// pub pg_partitioned_table: PgPartitionedTable,
	pub pg_policy: Vec<PgPolicy>,
	pub pg_proc: Set<PgProc>,
	pub pg_publication: Set<PgPublication>,
	// pub pg_publication_namespace: PgPublicationNamespace,
	// pub pg_publication_rel: PgPublicationRel,
	// pub pg_range: PgRange,
	// pub pg_replication_origin: PgReplicationOrigin,
	// pub pg_rewrite: PgRewrite,
	// pub pg_seclabel: PgSeclabel,
	// pub pg_sequence: PgSequence,
	// pub pg_shdepend: PgShdepend,
	// pub pg_shdescription: PgShdescription,
	// pub pg_shseclabel: PgShseclabel,
	// pub pg_statistic_ext: PgStatisticExt,
	// pub pg_subscription: PgSubscription,
	// pub pg_transform: PgTransform,
	// pub pg_trigger: PgTrigger,
	// pub pg_ts_config: PgTsConfig,
	// pub pg_ts_config_map: PgTsConfigMap,
	// pub pg_ts_dict: PgTsDict,
	// pub pg_ts_parser: PgTsParser,
	// pub pg_ts_template: PgTsTemplate,
	pub pg_type: Set<PgType>,
	pub pg_user_mappings: Vec<PgUserMappings>,
}

pub async fn reflect_pg_state(
	client: &PgClient
) -> Result<PgState, postgres::Error> {
	client.batch_execute(include_str!("../reflect_fns.sql")).await?;

	let (
		pg_aggregate,
		pg_am,
		pg_amop,
		pg_amproc,
		pg_attrdef,
		pg_attribute,
		pg_roles,
		pg_auth_members,
		pg_cast,
		pg_class,
		pg_collation,
		pg_constraint,
		pg_conversion,
		pg_database,
		pg_db_role_setting,
		pg_default_acl,
		// pg_depend,
		// pg_description,
		pg_enum,
		pg_event_trigger,
		pg_extension,
		pg_foreign_data_wrapper,
		pg_foreign_server,
		pg_foreign_table,
		// pg_index,
		// pg_inherits,
		// pg_init_privs,
		pg_language,
		pg_namespace,
		pg_opclass,
		// pg_operator,
		// pg_opfamily,
		// pg_parameter_acl,
		// pg_partitioned_table,
		pg_policy,
		pg_proc,
		pg_publication,
		// pg_publication_namespace,
		// pg_publication_rel,
		// pg_range,
		// pg_replication_origin,
		// pg_rewrite,
		// pg_seclabel,
		// pg_sequence,
		// pg_shdepend,
		// pg_shdescription,
		// pg_shseclabel,
		// pg_statistic_ext,
		// pg_subscription,
		// pg_transform,
		// pg_trigger,
		// pg_ts_config,
		// pg_ts_config_map,
		// pg_ts_dict,
		// pg_ts_parser,
		// pg_ts_template,
		pg_type,
		pg_user_mappings,
	) = tokio::try_join!(
		reflect_pg_aggregate(client),
		reflect_pg_am(client),
		reflect_pg_amop(client),
		reflect_pg_amproc(client),
		reflect_pg_attrdef(client),
		reflect_pg_attribute(client),
		reflect_pg_roles(client),
		reflect_pg_auth_members(client),
		reflect_pg_cast(client),
		reflect_pg_class(client),
		reflect_pg_collation(client),
		reflect_pg_constraint(client),
		reflect_pg_conversion(client),
		reflect_pg_database(client),
		reflect_pg_db_role_setting(client),
		reflect_pg_default_acl(client),
		// reflect_pg_depend(client),
		// reflect_pg_description(client),
		reflect_pg_enum(client),
		reflect_pg_event_trigger(client),
		reflect_pg_extension(client),
		reflect_pg_foreign_data_wrapper(client),
		reflect_pg_foreign_server(client),
		reflect_pg_foreign_table(client),
		// reflect_pg_index(client),
		// reflect_pg_inherits(client),
		// reflect_pg_init_privs(client),
		reflect_pg_language(client),
		reflect_pg_namespace(client),
		reflect_pg_opclass(client),
		// reflect_pg_operator(client),
		// reflect_pg_opfamily(client),
		// reflect_pg_parameter_acl(client),
		// reflect_pg_partitioned_table(client),
		reflect_pg_policy(client),
		reflect_pg_proc(client),
		reflect_pg_publication(client),
		// reflect_pg_publication_namespace(client),
		// reflect_pg_publication_rel(client),
		// reflect_pg_range(client),
		// reflect_pg_replication_origin(client),
		// reflect_pg_rewrite(client),
		// reflect_pg_seclabel(client),
		// reflect_pg_sequence(client),
		// reflect_pg_shdepend(client),
		// reflect_pg_shdescription(client),
		// reflect_pg_shseclabel(client),
		// reflect_pg_statistic_ext(client),
		// reflect_pg_subscription(client),
		// reflect_pg_transform(client),
		// reflect_pg_trigger(client),
		// reflect_pg_ts_config(client),
		// reflect_pg_ts_config_map(client),
		// reflect_pg_ts_dict(client),
		// reflect_pg_ts_parser(client),
		// reflect_pg_ts_template(client),
		reflect_pg_type(client),
		reflect_pg_user_mappings(client),
	)?;

	Ok(PgState {
		pg_aggregate,
		pg_am,
		pg_amop,
		pg_amproc,
		pg_attrdef,
		pg_attribute,
		pg_roles,
		pg_auth_members,
		pg_cast,
		pg_class,
		pg_collation,
		pg_constraint,
		pg_conversion,
		pg_database,
		pg_db_role_setting,
		pg_default_acl,
		// pg_depend,
		// pg_description,
		pg_enum,
		pg_event_trigger,
		pg_extension,
		pg_foreign_data_wrapper,
		pg_foreign_server,
		pg_foreign_table,
		// pg_index,
		// pg_inherits,
		// pg_init_privs,
		pg_language,
		pg_namespace,
		pg_opclass,
		// pg_operator,
		// pg_opfamily,
		// pg_parameter_acl,
		// pg_partitioned_table,
		pg_policy,
		pg_proc,
		pg_publication,
		// pg_publication_namespace,
		// pg_publication_rel,
		// pg_range,
		// pg_replication_origin,
		// pg_rewrite,
		// pg_seclabel,
		// pg_sequence,
		// pg_shdepend,
		// pg_shdescription,
		// pg_shseclabel,
		// pg_statistic_ext,
		// pg_subscription,
		// pg_transform,
		// pg_trigger,
		// pg_ts_config,
		// pg_ts_config_map,
		// pg_ts_dict,
		// pg_ts_parser,
		// pg_ts_template,
		pg_type,
		pg_user_mappings,
	})
}


#[macro_export]
macro_rules! impl_name_hash_and_equivalent {
	($type:ty, $field:ident) => {
		impl std::hash::Hash for $type {
			fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
				self.$field.hash(state);
			}
		}

		impl hashbrown::Equivalent<$type> for str {
			fn equivalent(&self, key: &$type) -> bool {
				key.$field == *self
			}
		}

		impl hashbrown::Equivalent<$type> for Str {
			fn equivalent(&self, key: &$type) -> bool {
				key.$field == *self
			}
		}
	};
}

#[macro_export]
macro_rules! impl_qual_hash_and_equivalent {
	($type:ty) => {
		impl std::hash::Hash for $type {
			fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
				self.oid.schema_name.hash(state);
				self.oid.name.hash(state);
			}
		}

		impl hashbrown::Equivalent<$type> for (&str, &str) {
			fn equivalent(&self, key: &$type) -> bool {
				key.oid.schema_name == *self.0 && key.oid.name == *self.1
			}
		}

		impl hashbrown::Equivalent<$type> for (&Str, &Str) {
			fn equivalent(&self, key: &$type) -> bool {
				key.oid.schema_name == *self.0 && key.oid.name == *self.1
			}
		}
	};
	($type:ty, $field:ident) => {
		impl std::hash::Hash for $type {
			fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
				self.$field.schema_name.hash(state);
				self.$field.name.hash(state);
			}
		}

		impl hashbrown::Equivalent<$type> for (&str, &str) {
			fn equivalent(&self, key: &$type) -> bool {
				key.$field.schema_name == *self.0 && key.$field.name == *self.1
			}
		}

		impl hashbrown::Equivalent<$type> for (&Str, &Str) {
			fn equivalent(&self, key: &$type) -> bool {
				key.$field.schema_name == *self.0 && key.$field.name == *self.1
			}
		}
	};
}


// #[macro_export]
// macro_rules! impl_pg_from_str {
// 	($type:ident, $($variant:ident),+ $(,)?) => {
// 		impl $type {
// 			fn pg_from_str(s: &str) -> $type {
// 				match s {
// 					$(stringify!($variant) => $type::$variant,)+
// 					_ => panic!("Postgres returned unexpected {} variant: {}", stringify!($type), s),
// 				}
// 			}
// 		}
// 	};
// }

#[macro_export]
macro_rules! pg_char_enum {
	($name:ident { $($char:literal => $variant:ident),* $(,)? }) => {
		#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
		pub enum $name {
			$($variant),*
		}

		impl $name {
			fn pg_from_char(c: i8) -> $name {
				match c as u8 as char {
					$($char => $name::$variant,)*
					_ => panic!(
						"Postgres returned an unknown {} variant: {}",
						stringify!($name),
						c as u8 as char
					),
				}
			}
		}
	};
}

// `pg_aggregate`: https://www.postgresql.org/docs/17/catalog-pg-aggregate.html
use reflect_gen::{PgAggregate, reflect_pg_aggregate};

// `pg_am`: https://www.postgresql.org/docs/17/catalog-pg-am.html
use reflect_gen::{PgAm, reflect_pg_am};

// `pg_amop`: https://www.postgresql.org/docs/17/catalog-pg-amop.html
use reflect_gen::{PgAmop, reflect_pg_amop};

// `pg_amproc`: https://www.postgresql.org/docs/17/catalog-pg-amproc.html
use reflect_gen::{PgAmproc, reflect_pg_amproc};

// `pg_attrdef`: https://www.postgresql.org/docs/17/catalog-pg-attrdef.html
use reflect_gen::{PgAttrdef, reflect_pg_attrdef};

// `pg_attribute`: https://www.postgresql.org/docs/17/catalog-pg-attribute.html
use reflect_gen::{PgAttribute, reflect_pg_attribute};

// `pg_authid`: https://www.postgresql.org/docs/17/catalog-pg-authid.html
// `pg_roles`: https://www.postgresql.org/docs/17/view-pg-roles.html
use reflect_gen::{PgRoles, reflect_pg_roles};

// `pg_auth_members`: https://www.postgresql.org/docs/17/catalog-pg-auth-members.html
use reflect_gen::{PgAuthMembers, reflect_pg_auth_members};

// `pg_cast`: https://www.postgresql.org/docs/17/catalog-pg-cast.html
use reflect_gen::{PgCast, reflect_pg_cast};

// `pg_class`: https://www.postgresql.org/docs/17/catalog-pg-class.html
use reflect_gen::{PgClass, reflect_pg_class};

// `pg_collation`: https://www.postgresql.org/docs/17/catalog-pg-collation.html
use reflect_gen::{PgCollation, reflect_pg_collation};

// `pg_constraint`: https://www.postgresql.org/docs/17/catalog-pg-constraint.html
use reflect_gen::{PgConstraint, reflect_pg_constraint};

// `pg_conversion`: https://www.postgresql.org/docs/17/catalog-pg-conversion.html
use reflect_gen::{PgConversion, reflect_pg_conversion};

// `pg_database`: https://www.postgresql.org/docs/17/catalog-pg-database.html
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
) -> Result<PgDatabase, postgres::Error> {
	let pg_database = reflect_crate::queries::reflect::reflect_pg_database().bind(client)
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
		.one()
		.await?;

	Ok(pg_database)
}

// `pg_db_role_setting`: https://www.postgresql.org/docs/17/catalog-pg-db-role-setting.html
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgDbRoleSetting {
	setdatabase: Option<()>, // oid (references pg_database.oid) The OID of the database the setting is applicable to, or zero if not database-specific
	/// `oid` `(references pg_authid.oid)` The OID of the role the setting is applicable to, or zero if not role-specific
	setrole: Option<Str>,
	/// `text[]`  Defaults for run-time configuration variables
	setconfig: Option<Vec<Str>>,
}

pub async fn reflect_pg_db_role_setting(
	client: &PgClient
) -> Result<Vec<PgDbRoleSetting>, postgres::Error> {
	let pg_db_role_setting_coll = reflect_crate::queries::reflect::reflect_pg_db_role_setting().bind(client)
		.map(|pg_db_role_setting| {
			PgDbRoleSetting {
				setdatabase: if pg_db_role_setting.setdatabase { Some(()) } else { None },
				setrole: pg_db_role_setting.setrole.map(Into::into),
				setconfig: pg_db_role_setting.setconfig.map(|items| items.map(Into::into).collect()),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_db_role_setting_coll)
}

// `pg_default_acl`: https://www.postgresql.org/docs/17/catalog-pg-default-acl.html
use reflect_gen::{PgDefaultAcl, reflect_pg_default_acl};

// `pg_depend`: https://www.postgresql.org/docs/17/catalog-pg-depend.html
// use reflect_gen::{PgDepend, reflect_pg_depend};

// `pg_description`: https://www.postgresql.org/docs/17/catalog-pg-description.html
// use reflect_gen::{PgDescription, reflect_pg_description};

// `pg_enum`: https://www.postgresql.org/docs/17/catalog-pg-enum.html
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgEnum {
	// `oid`  Row identifier
	/// enumtypid `oid` `(references pg_type.oid)` The OID of the pg_type entry owning this enum value
	enumtypid: Qual,
	/// enumlabel `name`  The textual label for this enum value
	enumlabels: Vec<Str>,
	// enumsortorder `float4`  The sort position of this enum value within its enum type
}
impl_qual_hash_and_equivalent!(PgEnum, enumtypid);

pub async fn reflect_pg_enum(
	client: &PgClient
) -> Result<Set<PgEnum>, postgres::Error> {
	let pg_enum_coll = reflect_crate::queries::reflect::reflect_pg_enum().bind(client)
		.map(|pg_enum| {
			PgEnum {
				enumtypid: Qual::parse(pg_enum.enumtypid),
				enumlabels: pg_enum.enumlabels.map(Into::into).collect(),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_enum_coll)
}

// `pg_event_trigger`: https://www.postgresql.org/docs/17/catalog-pg-event-trigger.html
use reflect_gen::{PgEventTrigger, reflect_pg_event_trigger};

// `pg_extension`: https://www.postgresql.org/docs/17/catalog-pg-extension.html
use reflect_gen::{PgExtension, reflect_pg_extension};

// `pg_foreign_data_wrapper`: https://www.postgresql.org/docs/17/catalog-pg-foreign-data-wrapper.html
use reflect_gen::{PgForeignDataWrapper, reflect_pg_foreign_data_wrapper};

// `pg_foreign_server`: https://www.postgresql.org/docs/17/catalog-pg-foreign-server.html
use reflect_gen::{PgForeignServer, reflect_pg_foreign_server};

// `pg_foreign_table`: https://www.postgresql.org/docs/17/catalog-pg-foreign-table.html
use reflect_gen::{PgForeignTable, reflect_pg_foreign_table};

// `pg_index`: https://www.postgresql.org/docs/17/catalog-pg-index.html
// use reflect_gen::{PgIndex, reflect_pg_index};

// `pg_inherits`: https://www.postgresql.org/docs/17/catalog-pg-inherits.html
// use reflect_gen::{PgInherits, reflect_pg_inherits};

// `pg_init_privs`: https://www.postgresql.org/docs/17/catalog-pg-init-privs.html
// use reflect_gen::{PgInitPrivs, reflect_pg_init_privs};

// `pg_language`: https://www.postgresql.org/docs/17/catalog-pg-language.html
use reflect_gen::{PgLanguage, reflect_pg_language};

// `pg_namespace`: https://www.postgresql.org/docs/17/catalog-pg-namespace.html
use reflect_gen::{PgNamespace, reflect_pg_namespace};

// `pg_opclass`: https://www.postgresql.org/docs/17/catalog-pg-opclass.html
use reflect_gen::{PgOpclass, reflect_pg_opclass};

// `pg_operator`: https://www.postgresql.org/docs/17/catalog-pg-operator.html
// use reflect_gen::{PgOperator, reflect_pg_operator};

// `pg_opfamily`: https://www.postgresql.org/docs/17/catalog-pg-opfamily.html
// use reflect_gen::{PgOpfamily, reflect_pg_opfamily};

// `pg_parameter_acl`: https://www.postgresql.org/docs/17/catalog-pg-parameter-acl.html
// use reflect_gen::{PgParameterAcl, reflect_pg_parameter_acl};

// `pg_partitioned_table`: https://www.postgresql.org/docs/17/catalog-pg-partitioned-table.html
// use reflect_gen::{PgPartitionedTable, reflect_pg_partitioned_table};

// `pg_policy`: https://www.postgresql.org/docs/17/catalog-pg-policy.html
use reflect_gen::{PgPolicy, reflect_pg_policy};

// `pg_proc`: https://www.postgresql.org/docs/17/catalog-pg-proc.html
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgProc {
	/// `oid`  Row identifier
	oid: Qual,
	/// `name`  Name of the function
	proname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this function
	pronamespace: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the function
	proowner: Str,
	/// `oid` `(references pg_language.oid)` Implementation language or call interface of this function
	prolang: Str,
	/// `float4`  Estimated execution cost (in units of cpu_operator_cost); if proretset, this is cost per row returned
	procost: Option<Str>,
	/// `float4`  Estimated number of result rows (zero if not proretset)
	prorows: Option<Str>,
	/// `oid` `(references pg_type.oid)` Data type of the variadic array parameter's elements, or zero if the function does not have a variadic parameter
	provariadic: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Planner support function for this function (see Section 36.11), or zero if none
	prosupport: Option<Qual>,
	/// `char`  f for a normal function, p for a procedure, a for an aggregate function, or w for a window function
	prokind: PgProcProkind,
	/// `bool`  Function is a security definer (i.e., a “setuid” function)
	prosecdef: bool,
	/// `bool`  The function has no side effects. No information about the arguments is conveyed except via the return value. Any function that might throw an error depending on the values of its arguments is not leak-proof.
	proleakproof: bool,
	/// `bool`  Function returns null if any call argument is null. In that case the function won't actually be called at all. Functions that are not “strict” must be prepared to handle null inputs.
	proisstrict: bool,
	/// `bool`  Function returns a set (i.e., multiple values of the specified data type)
	proretset: bool,
	/// `char`  provolatile tells whether the function's result depends only on its input arguments, or is affected by outside factors. It is i for “immutable” functions, which always deliver the same result for the same inputs. It is s for “stable” functions, whose results (for fixed inputs) do not change within a scan. It is v for “volatile” functions, whose results might change at any time. (Use v also for functions with side-effects, so that calls to them cannot get optimized away.)
	provolatile: PgProcProvolatile,
	/// `char`  proparallel tells whether the function can be safely run in parallel mode. It is s for functions which are safe to run in parallel mode without restriction. It is r for functions which can be run in parallel mode, but their execution is restricted to the parallel group leader; parallel worker processes cannot invoke these functions. It is u for functions which are unsafe in parallel mode; the presence of such a function forces a serial execution plan.
	proparallel: PgProcProparallel,
	/// `int2`  Number of input arguments
	pronargs: u16,
	/// `int2`  Number of arguments that have defaults
	pronargdefaults: u16,
	/// `oid` `(references pg_type.oid)` Data type of the return value
	prorettype: Qual,
	// `oidvector` `(references pg_type.oid)` An array of the data types of the function arguments. This includes only input arguments (including INOUT and VARIADIC arguments), and thus represents the call signature of the function.
	proargtypes: Vec<Qual>,
	/// `oid[]` `(references pg_type.oid)` An array of the data types of the function arguments. This includes all arguments (including OUT and INOUT arguments); however, if all the arguments are IN arguments, this field will be null. Note that subscripting is 1-based, whereas for historical reasons proargtypes is subscripted from 0.
	proallargtypes: Option<Vec<Qual>>,
	/// `char[]`  An array of the modes of the function arguments, encoded as i for IN arguments, o for OUT arguments, b for INOUT arguments, v for VARIADIC arguments, t for TABLE arguments. If all the arguments are IN arguments, this field will be null. Note that subscripts correspond to positions of proallargtypes not proargtypes.
	proargmodes: Option<Vec<PgProcProargmodes>>,
	/// `text[]`  An array of the names of the function arguments. Arguments without a name are set to empty strings in the array. If none of the arguments have a name, this field will be null. Note that subscripts correspond to positions of proallargtypes not proargtypes.
	proargnames: Option<Vec<Str>>,
	/// `pg_node_tree`  Expression trees (in nodeToString() representation) for default values. This is a list with pronargdefaults elements, corresponding to the last N input arguments (i.e., the last N proargtypes positions). If none of the arguments have defaults, this field will be null.
	proargdefaults: Option<Vec<Option<Str>>>,
	/// `oid[]` `(references pg_type.oid)` An array of the argument/result data type(s) for which to apply transforms (from the function's TRANSFORM clause). Null if none.
	protrftypes: Option<Vec<Qual>>,
	/// `text`  This tells the function handler how to invoke the function. It might be the actual source code of the function for interpreted languages, a link symbol, a file name, or just about anything else, depending on the implementation language/call convention.
	prosrc: Option<Str>,
	/// `text`  Additional information about how to invoke the function. Again, the interpretation is language-specific.
	probin: Option<Str>,
	/// pg_node_tree  Pre-parsed SQL function body. This is used for SQL-language functions when the body is given in SQL-standard notation rather than as a string literal. It's null in other cases.
	prosqlbody: Option<Str>,
	/// `text[]`  Function's local settings for run-time configuration variables
	proconfig: Option<Vec<Str>>,
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	proacl: Option<Vec<aclitem::FunctionAclItem>>,
}
impl_qual_hash_and_equivalent!(PgProc);

// f for a normal function, p for a procedure, a for an aggregate function, or w for a window function
pg_char_enum!(PgProcProkind { 'f' => NormalFunction, 'p' => Procedure, 'a' => AggregateFunction, 'w' => WindowFunction });
// It is i for “immutable” functions, which always deliver the same result for the same inputs. It is s for “stable” functions, whose results (for fixed inputs) do not change within a scan. It is v for “volatile” functions, whose results might change at any time
pg_char_enum!(PgProcProvolatile { 'i' => Immutable, 's' => Stable, 'v' => Volatile });
// It is s for functions which are safe to run in parallel mode without restriction. It is r for functions which can be run in parallel mode, but their execution is restricted to the parallel group leader; parallel worker processes cannot invoke these functions. It is u for functions which are unsafe in parallel mode; the presence of such a function forces a serial execution plan.
pg_char_enum!(PgProcProparallel { 's' => SafeWithoutRestriction, 'r' => RestrictedToGroupLeader, 'u' => Unsafe });
// i for IN arguments, o for OUT arguments, b for INOUT arguments, v for VARIADIC arguments, t for TABLE arguments
pg_char_enum!(PgProcProargmodes { 'i' => In, 'o' => Out, 'b' => Inout, 'v' => Variadic, 't' => Table });

pub async fn reflect_pg_proc(
	client: &PgClient
) -> Result<Set<PgProc>, postgres::Error> {
	let pg_proc_coll = reflect_crate::queries::reflect::reflect_pg_proc().bind(client)
		.map(|pg_proc| {
			PgProc {
				oid: Qual::parse(pg_proc.oid),
				proname: pg_proc.proname.into(),
				pronamespace: pg_proc.pronamespace.into(),
				proowner: pg_proc.proowner.into(),
				prolang: pg_proc.prolang.into(),
				procost: pg_proc.procost.map(Into::into),
				prorows: pg_proc.prorows.map(Into::into),
				provariadic: Qual::maybe_parse(pg_proc.provariadic),
				prosupport: Qual::maybe_parse(pg_proc.prosupport),
				prokind: PgProcProkind::pg_from_char(pg_proc.prokind),
				prosecdef: pg_proc.prosecdef,
				proleakproof: pg_proc.proleakproof,
				proisstrict: pg_proc.proisstrict,
				proretset: pg_proc.proretset,
				provolatile: PgProcProvolatile::pg_from_char(pg_proc.provolatile),
				proparallel: PgProcProparallel::pg_from_char(pg_proc.proparallel),
				pronargs: pg_proc.pronargs.unsigned_abs(),
				pronargdefaults: pg_proc.pronargdefaults.unsigned_abs(),
				prorettype: Qual::parse(pg_proc.prorettype),
		    proargtypes: pg_proc.proargtypes.map(Qual::parse).collect(),
				proallargtypes: pg_proc.proallargtypes.map(|items| items.map(Qual::parse).collect()),
		    proargmodes: pg_proc.proargmodes.map(|items| items.map(PgProcProargmodes::pg_from_char).collect()),
				proargnames: pg_proc.proargnames.map(|items| items.map(Into::into).collect()),
		    proargdefaults: pg_proc.proargdefaults.map(|items| items.map(|item| item.map(Into::into)).collect()),
				protrftypes: pg_proc.protrftypes.map(|items| items.map(Qual::parse).collect()),
				prosrc: pg_proc.prosrc.map(Into::into),
				probin: pg_proc.probin.map(Into::into),
		    prosqlbody: pg_proc.prosqlbody.map(Into::into),
				proconfig: pg_proc.proconfig.map(|items| items.map(Into::into).collect()),
				proacl: pg_proc.proacl.map(|proacl| proacl.map(|acl| aclitem(acl, &FunctionGrantParser)).collect()),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_proc_coll)
}

// `pg_publication`: https://www.postgresql.org/docs/17/catalog-pg-publication.html
use reflect_gen::{PgPublication, reflect_pg_publication};

// `pg_publication_namespace`: https://www.postgresql.org/docs/17/catalog-pg-publication-namespace.html
// use reflect_gen::{PgPublicationNamespace, reflect_pg_publication_namespace};

// `pg_publication_rel`: https://www.postgresql.org/docs/17/catalog-pg-publication-rel.html
// use reflect_gen::{PgPublicationRel, reflect_pg_publication_rel};

// `pg_range`: https://www.postgresql.org/docs/17/catalog-pg-range.html
// use reflect_gen::{PgRange, reflect_pg_range};

// `pg_replication_origin`: https://www.postgresql.org/docs/17/catalog-pg-replication-origin.html
// use reflect_gen::{PgReplicationOrigin, reflect_pg_replication_origin};

// `pg_rewrite`: https://www.postgresql.org/docs/17/catalog-pg-rewrite.html
// use reflect_gen::{PgRewrite, reflect_pg_rewrite};

// `pg_seclabel`: https://www.postgresql.org/docs/17/catalog-pg-seclabel.html
// use reflect_gen::{PgSeclabel, reflect_pg_seclabel};

// `pg_sequence`: https://www.postgresql.org/docs/17/catalog-pg-sequence.html
// use reflect_gen::{PgSequence, reflect_pg_sequence};

// `pg_shdepend`: https://www.postgresql.org/docs/17/catalog-pg-shdepend.html
// use reflect_gen::{PgShdepend, reflect_pg_shdepend};

// `pg_shdescription`: https://www.postgresql.org/docs/17/catalog-pg-shdescription.html
// use reflect_gen::{PgShdescription, reflect_pg_shdescription};

// `pg_shseclabel`: https://www.postgresql.org/docs/17/catalog-pg-shseclabel.html
// use reflect_gen::{PgShseclabel, reflect_pg_shseclabel};

// `pg_statistic_ext`: https://www.postgresql.org/docs/17/catalog-pg-statistic-ext.html
// use reflect_gen::{PgStatisticExt, reflect_pg_statistic_ext};

// `pg_subscription`: https://www.postgresql.org/docs/17/catalog-pg-subscription.html
// use reflect_gen::{PgSubscription, reflect_pg_subscription};

// `pg_transform`: https://www.postgresql.org/docs/17/catalog-pg-transform.html
// use reflect_gen::{PgTransform, reflect_pg_transform};

// `pg_trigger`: https://www.postgresql.org/docs/17/catalog-pg-trigger.html
// use reflect_gen::{PgTrigger, reflect_pg_trigger};

// `pg_ts_config`: https://www.postgresql.org/docs/17/catalog-pg-ts-config.html
// use reflect_gen::{PgTsConfig, reflect_pg_ts_config};

// `pg_ts_config_map`: https://www.postgresql.org/docs/17/catalog-pg-ts-config-map.html
// use reflect_gen::{PgTsConfigMap, reflect_pg_ts_config_map};

// `pg_ts_dict`: https://www.postgresql.org/docs/17/catalog-pg-ts-dict.html
// use reflect_gen::{PgTsDict, reflect_pg_ts_dict};

// `pg_ts_parser`: https://www.postgresql.org/docs/17/catalog-pg-ts-parser.html
// use reflect_gen::{PgTsParser, reflect_pg_ts_parser};

// `pg_ts_template`: https://www.postgresql.org/docs/17/catalog-pg-ts-template.html
// use reflect_gen::{PgTsTemplate, reflect_pg_ts_template};

// `pg_type`: https://www.postgresql.org/docs/17/catalog-pg-type.html
use reflect_gen::{PgType, reflect_pg_type};

// `pg_user_mappings`: https://www.postgresql.org/docs/17/catalog-pg-user-mapping.html
use reflect_gen::{PgUserMappings, reflect_pg_user_mappings};
