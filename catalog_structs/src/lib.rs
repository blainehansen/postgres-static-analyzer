//! Struct definitions for all the [postgres catalog tables](https://www.postgresql.org/docs/17/catalogs.html) that are [**DDL only**](https://en.wikipedia.org/wiki/Data_definition_language), meaning only the tables and columns that describe the "schema" of the database are included. No `oid`s, no transient server state or like clustering or tablespaces etc, and of course no actual table data.
//!
//! `oid`s pointing to other tables are translated either to strings (as [`Str`]) if they aren't contained in a [namespace](https://www.postgresql.org/docs/17/catalog-pg-namespace.html) (confusingly created with [`create schema`](https://www.postgresql.org/docs/17/sql-createschema.html)), or a "qualified name" struct [`Qual`] for those that are.

pub use smol_str::SmolStr as Str;
pub use ordered_float;

pub type Set<T> = hashbrown::HashSet<T>;
pub type Map<T> = hashbrown::HashMap<Str, T>;
// pub(crate) use hashbrown::HashMap;

mod struct_gen;
pub use struct_gen::*;

pub mod aclitem;

/// The qualified name of an object that exists in a [`namespace`](https://www.postgresql.org/docs/17/catalog-pg-namespace.html).
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct Qual {
	pub schema_name: Str,
	pub name: Str,
}
impl Qual {
	// pub(crate) fn make(schema_name: &str, name: &str) -> Qual {
	// 	Qual { schema_name: schema_name.into(), name: name.into() }
	// }

	// pub(crate) fn maybe(schema_name: Option<&str>, name: Option<&str>) -> Option<Qual> {
	// 	match (schema_name, name) {
	// 		(Some(schema_name), Some(name)) => Some(Qual::make(schema_name, name)),
	// 		_ => None,
	// 	}
	// }

	/// Parse a qualifed name as would be given by the sql `quote_ident(namespace_name) || '.' || quote_ident(object_name)`
	pub fn parse(qualified: &str) -> Qual {
		if let Ok((_, (schema_name, name))) = aclitem::parse_qualified(qualified) {
			return Qual { schema_name, name }
		}

		Qual { schema_name: "pg_catalog".into(), name: qualified.into() }
	}

	pub fn parse_func(qualified: &str) -> Qual {
		if let Ok((_, (schema_name, name))) = aclitem::parse_qualified_func(qualified) {
			return Qual { schema_name, name }
		}

		Qual { schema_name: "pg_catalog".into(), name: qualified.into() }
	}

	/// Optionally call `parse`
	pub fn maybe_parse(qualified: Option<&str>) -> Option<Qual> {
		qualified.map(Qual::parse)
	}
}
// impl<T: AsRef<str>> hashbrown::Equivalent<Qual> for (&T, &T) {
// 	fn equivalent(&self, key: &Qual) -> bool {
// 		key.schema_name == self.0.as_ref() && key.name == self.1.as_ref()
// 	}
// }

/// A large wrapper struct that holds the results of all the other reflections.
/// Only includes information for the single [database](https://www.postgresql.org/docs/17/sql-createdatabase.html), which is why `pg_database` isn't a collection.
/// Objects that are "cluster shared" such as roles are those for the entire cluster.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DbState {
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
	pub pg_index: Vec<PgIndex>,
	pub pg_inherits: Vec<PgInherits>,
	// pub pg_init_privs: Vec<PgInitPrivs>,
	pub pg_language: Set<PgLanguage>,
	pub pg_namespace: Set<PgNamespace>,
	pub pg_opclass: Vec<PgOpclass>,
	pub pg_operator: Set<PgOperator>,
	pub pg_opfamily: Vec<PgOpfamily>,
	pub pg_parameter_acl: Vec<PgParameterAcl>,
	pub pg_partitioned_table: Vec<PgPartitionedTable>,
	pub pg_policy: Vec<PgPolicy>,
	pub pg_proc: Set<PgProc>,
	pub pg_publication: Set<PgPublication>,
	pub pg_publication_namespace: Vec<PgPublicationNamespace>,
	pub pg_publication_rel: Vec<PgPublicationRel>,
	pub pg_range: Vec<PgRange>,
	// pub pg_replication_origin: Vec<PgReplicationOrigin>,
	// pub pg_rewrite: Vec<PgRewrite>,
	pub pg_rules: Vec<PgRules>,
	pub pg_views: Vec<PgViews>,
	pub pg_matviews: Vec<PgMatviews>,
	// pub pg_seclabel: Vec<PgSeclabel>,
	pub pg_sequence: Vec<PgSequence>,
	// pub pg_shdepend: Vec<PgShdepend>,
	// pub pg_shdescription: Vec<PgShdescription>,
	// pub pg_shseclabel: Vec<PgShseclabel>,
	pub pg_statistic_ext: Vec<PgStatisticExt>,
	pub pg_subscription: Vec<PgSubscription>,
	pub pg_transform: Vec<PgTransform>,
	pub pg_trigger: Vec<PgTrigger>,
	pub pg_ts_config: Set<PgTsConfig>,
	pub pg_ts_config_map: Vec<PgTsConfigMap>,
	pub pg_ts_dict: Set<PgTsDict>,
	pub pg_ts_parser: Vec<PgTsParser>,
	pub pg_ts_template: Vec<PgTsTemplate>,
	pub pg_type: Set<PgType>,
	pub pg_user_mappings: Vec<PgUserMappings>,
}

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
pub(crate) use impl_name_hash_and_equivalent;

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
pub(crate) use impl_qual_hash_and_equivalent;

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

macro_rules! pg_char_enum {
	($name:ident { $($char:literal => $variant:ident),* $(,)? }) => {
		#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
		pub enum $name {
			$($variant),*
		}

		impl $name {
			pub fn pg_from_char(c: i8) -> $name {
				match c as u8 as char {
					$($char => $name::$variant,)*
					_ => panic!(
						"unknown {} variant: {}",
						stringify!($name),
						c as u8 as char
					),
				}
			}
		}
	};
}
pub(crate) use pg_char_enum;

// `pg_aggregate`: https://www.postgresql.org/docs/17/catalog-pg-aggregate.html

// `pg_am`: https://www.postgresql.org/docs/17/catalog-pg-am.html

// `pg_amop`: https://www.postgresql.org/docs/17/catalog-pg-amop.html

// `pg_amproc`: https://www.postgresql.org/docs/17/catalog-pg-amproc.html

// `pg_attrdef`: https://www.postgresql.org/docs/17/catalog-pg-attrdef.html

// `pg_attribute`: https://www.postgresql.org/docs/17/catalog-pg-attribute.html

// `pg_authid`: https://www.postgresql.org/docs/17/catalog-pg-authid.html
// `pg_roles`: https://www.postgresql.org/docs/17/view-pg-roles.html

// `pg_auth_members`: https://www.postgresql.org/docs/17/catalog-pg-auth-members.html

// `pg_cast`: https://www.postgresql.org/docs/17/catalog-pg-cast.html

// `pg_class`: https://www.postgresql.org/docs/17/catalog-pg-class.html

// `pg_collation`: https://www.postgresql.org/docs/17/catalog-pg-collation.html

// `pg_constraint`: https://www.postgresql.org/docs/17/catalog-pg-constraint.html

// `pg_conversion`: https://www.postgresql.org/docs/17/catalog-pg-conversion.html

// `pg_database`: https://www.postgresql.org/docs/17/catalog-pg-database.html

/// The DDL-only contents of [`pg_db_role_setting`](https://www.postgresql.org/docs/17/catalog-pg-db-role-setting.html)
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgDbRoleSetting {
	/// `oid` (references pg_database.oid) The OID of the database the setting is applicable to, or zero if not database-specific
	pub setdatabase: Option<()>,
	/// `oid` `(references pg_authid.oid)` The OID of the role the setting is applicable to, or zero if not role-specific
	pub setrole: Option<Str>,
	/// `text[]`  Defaults for run-time configuration variables
	pub setconfig: Option<Vec<Str>>,
}

// `pg_default_acl`: https://www.postgresql.org/docs/17/catalog-pg-default-acl.html

// `pg_depend`: https://www.postgresql.org/docs/17/catalog-pg-depend.html

// `pg_description`: https://www.postgresql.org/docs/17/catalog-pg-description.html

/// The DDL-only contents of [`pg_enum`](https://www.postgresql.org/docs/17/catalog-pg-enum.html)
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgEnum {
	// `oid`  Row identifier
	/// enumtypid `oid` `(references pg_type.oid)` The OID of the pg_type entry owning this enum value
	pub enumtypid: Qual,
	/// enumlabel `name`  The textual label for this enum value
	pub enumlabels: Vec<Str>,
	// enumsortorder `float4`  The sort position of this enum value within its enum type
}
impl_qual_hash_and_equivalent!(PgEnum, enumtypid);

// `pg_event_trigger`: https://www.postgresql.org/docs/17/catalog-pg-event-trigger.html

// `pg_extension`: https://www.postgresql.org/docs/17/catalog-pg-extension.html

// `pg_foreign_data_wrapper`: https://www.postgresql.org/docs/17/catalog-pg-foreign-data-wrapper.html

// `pg_foreign_server`: https://www.postgresql.org/docs/17/catalog-pg-foreign-server.html

// `pg_foreign_table`: https://www.postgresql.org/docs/17/catalog-pg-foreign-table.html

// `pg_index`: https://www.postgresql.org/docs/17/catalog-pg-index.html

// `pg_inherits`: https://www.postgresql.org/docs/17/catalog-pg-inherits.html

// `pg_init_privs`: https://www.postgresql.org/docs/17/catalog-pg-init-privs.html

// `pg_language`: https://www.postgresql.org/docs/17/catalog-pg-language.html

// `pg_namespace`: https://www.postgresql.org/docs/17/catalog-pg-namespace.html

// `pg_opclass`: https://www.postgresql.org/docs/17/catalog-pg-opclass.html

// `pg_operator`: https://www.postgresql.org/docs/17/catalog-pg-operator.html

// `pg_opfamily`: https://www.postgresql.org/docs/17/catalog-pg-opfamily.html

// `pg_parameter_acl`: https://www.postgresql.org/docs/17/catalog-pg-parameter-acl.html

// `pg_partitioned_table`: https://www.postgresql.org/docs/17/catalog-pg-partitioned-table.html

// `pg_policy`: https://www.postgresql.org/docs/17/catalog-pg-policy.html

/// The DDL-only contents of [`pg_proc`](https://www.postgresql.org/docs/17/catalog-pg-proc.html)
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PgProc {
	/// `oid`  Row identifier
	pub oid: Qual,
	/// `name`  Name of the function
	pub proname: Str,
	/// `oid` `(references pg_namespace.oid)` The OID of the namespace that contains this function
	pub pronamespace: Str,
	/// `oid` `(references pg_authid.oid)` Owner of the function
	pub proowner: Str,
	/// `oid` `(references pg_language.oid)` Implementation language or call interface of this function
	pub prolang: Str,
	/// `float4`  Estimated execution cost (in units of cpu_operator_cost); if proretset, this is cost per row returned
	pub procost: Option<ordered_float::NotNan<f32>>,
	/// `float4`  Estimated number of result rows (zero if not proretset)
	pub prorows: Option<ordered_float::NotNan<f32>>,
	/// `oid` `(references pg_type.oid)` Data type of the variadic array parameter's elements, or zero if the function does not have a variadic parameter
	pub provariadic: Option<Qual>,
	/// `regproc` `(references pg_proc.oid)` Planner support function for this function (see Section 36.11), or zero if none
	pub prosupport: Option<Qual>,
	/// `char`  f for a normal function, p for a procedure, a for an aggregate function, or w for a window function
	pub prokind: PgProcProkind,
	/// `bool`  Function is a security definer (i.e., a “setuid” function)
	pub prosecdef: bool,
	/// `bool`  The function has no side effects. No information about the arguments is conveyed except via the return value. Any function that might throw an error depending on the values of its arguments is not leak-proof.
	pub proleakproof: bool,
	/// `bool`  Function returns null if any call argument is null. In that case the function won't actually be called at all. Functions that are not “strict” must be prepared to handle null inputs.
	pub proisstrict: bool,
	/// `bool`  Function returns a set (i.e., multiple values of the specified data type)
	pub proretset: bool,
	/// `char`  provolatile tells whether the function's result depends only on its input arguments, or is affected by outside factors. It is i for “immutable” functions, which always deliver the same result for the same inputs. It is s for “stable” functions, whose results (for fixed inputs) do not change within a scan. It is v for “volatile” functions, whose results might change at any time. (Use v also for functions with side-effects, so that calls to them cannot get optimized away.)
	pub provolatile: PgProcProvolatile,
	/// `char`  proparallel tells whether the function can be safely run in parallel mode. It is s for functions which are safe to run in parallel mode without restriction. It is r for functions which can be run in parallel mode, but their execution is restricted to the parallel group leader; parallel worker processes cannot invoke these functions. It is u for functions which are unsafe in parallel mode; the presence of such a function forces a serial execution plan.
	pub proparallel: PgProcProparallel,
	/// `int2`  Number of input arguments
	pub pronargs: u16,
	/// `int2`  Number of arguments that have defaults
	pub pronargdefaults: u16,
	/// `oid` `(references pg_type.oid)` Data type of the return value
	pub prorettype: Qual,
	// `oidvector` `(references pg_type.oid)` An array of the data types of the function arguments. This includes only input arguments (including INOUT and VARIADIC arguments), and thus represents the call signature of the function.
	pub proargtypes: Vec<Qual>,
	/// `oid[]` `(references pg_type.oid)` An array of the data types of the function arguments. This includes all arguments (including OUT and INOUT arguments); however, if all the arguments are IN arguments, this field will be null. Note that subscripting is 1-based, whereas for historical reasons proargtypes is subscripted from 0.
	pub proallargtypes: Option<Vec<Qual>>,
	/// `char[]`  An array of the modes of the function arguments, encoded as i for IN arguments, o for OUT arguments, b for INOUT arguments, v for VARIADIC arguments, t for TABLE arguments. If all the arguments are IN arguments, this field will be null. Note that subscripts correspond to positions of proallargtypes not proargtypes.
	pub proargmodes: Option<Vec<PgProcProargmodes>>,
	/// `text[]`  An array of the names of the function arguments. Arguments without a name are set to empty strings in the array. If none of the arguments have a name, this field will be null. Note that subscripts correspond to positions of proallargtypes not proargtypes.
	pub proargnames: Option<Vec<Str>>,
	/// `pg_node_tree`  Expression trees (in nodeToString() representation) for default values. This is a list with pronargdefaults elements, corresponding to the last N input arguments (i.e., the last N proargtypes positions). If none of the arguments have defaults, this field will be null.
	pub proargdefaults: Option<Vec<Option<Str>>>,
	/// `oid[]` `(references pg_type.oid)` An array of the argument/result data type(s) for which to apply transforms (from the function's TRANSFORM clause). Null if none.
	pub protrftypes: Option<Vec<Qual>>,
	/// `text`  This tells the function handler how to invoke the function. It might be the actual source code of the function for interpreted languages, a link symbol, a file name, or just about anything else, depending on the implementation language/call convention.
	pub prosrc: Option<Str>,
	/// `text`  Additional information about how to invoke the function. Again, the interpretation is language-specific.
	pub probin: Option<Str>,
	/// pg_node_tree  Pre-parsed SQL function body. This is used for SQL-language functions when the body is given in SQL-standard notation rather than as a string literal. It's null in other cases.
	pub prosqlbody: Option<Str>,
	/// `text[]`  Function's local settings for run-time configuration variables
	pub proconfig: Option<Vec<Str>>,
	/// `aclitem[]`  Access privileges; see Section 5.8 for details
	pub proacl: Option<Vec<aclitem::FunctionAclItem>>,
	/// `text`  The comment from pg_description
	pub description: Option<Str>,
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

// `pg_publication`: https://www.postgresql.org/docs/17/catalog-pg-publication.html

// `pg_publication_namespace`: https://www.postgresql.org/docs/17/catalog-pg-publication-namespace.html

// `pg_publication_rel`: https://www.postgresql.org/docs/17/catalog-pg-publication-rel.html

// `pg_range`: https://www.postgresql.org/docs/17/catalog-pg-range.html

// `pg_replication_origin`: https://www.postgresql.org/docs/17/catalog-pg-replication-origin.html

// `pg_rewrite`: https://www.postgresql.org/docs/17/catalog-pg-rewrite.html

// `pg_rules`: https://www.postgresql.org/docs/17/view-pg-rules.html
// `pg_views`: https://www.postgresql.org/docs/17/view-pg-views.html
// `pg_matviews`: https://www.postgresql.org/docs/17/view-pg-matviews.html

// `pg_seclabel`: https://www.postgresql.org/docs/17/catalog-pg-seclabel.html

// `pg_sequence`: https://www.postgresql.org/docs/17/catalog-pg-sequence.html

// `pg_shdepend`: https://www.postgresql.org/docs/17/catalog-pg-shdepend.html

// `pg_shdescription`: https://www.postgresql.org/docs/17/catalog-pg-shdescription.html

// `pg_shseclabel`: https://www.postgresql.org/docs/17/catalog-pg-shseclabel.html

// `pg_statistic_ext`: https://www.postgresql.org/docs/17/catalog-pg-statistic-ext.html

// `pg_subscription`: https://www.postgresql.org/docs/17/catalog-pg-subscription.html

// `pg_transform`: https://www.postgresql.org/docs/17/catalog-pg-transform.html

// `pg_trigger`: https://www.postgresql.org/docs/17/catalog-pg-trigger.html

// `pg_ts_config`: https://www.postgresql.org/docs/17/catalog-pg-ts-config.html

// `pg_ts_config_map`: https://www.postgresql.org/docs/17/catalog-pg-ts-config-map.html

// `pg_ts_dict`: https://www.postgresql.org/docs/17/catalog-pg-ts-dict.html

// `pg_ts_parser`: https://www.postgresql.org/docs/17/catalog-pg-ts-parser.html

// `pg_ts_template`: https://www.postgresql.org/docs/17/catalog-pg-ts-template.html

// `pg_type`: https://www.postgresql.org/docs/17/catalog-pg-type.html

// `pg_user_mappings`: https://www.postgresql.org/docs/17/catalog-pg-user-mapping.html
