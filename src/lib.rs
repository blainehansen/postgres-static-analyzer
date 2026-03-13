pub(crate) use smol_str::SmolStr as Str;

pub(crate) use reflect_crate::tokio_postgres::{self as postgres, /*Config as PgConfig,*/ Client as PgClient};

pub type Set<T> = hashbrown::HashSet<T>;
pub type Map<T> = hashbrown::HashMap<Str, T>;
pub(crate) use hashbrown::HashMap;

mod reflect;
#[cfg(test)]
mod reflect_test;

pub use reflect::{reflect_pg_state, reflect_pg_roles, reflect_pg_class};

mod aclitem;

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
}

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
		#[derive(Debug, PartialEq, Eq, Clone)]
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

pg_char_enum!(FunctionKind { 'f' => Function, 'p' => Procedure, 'a' => Aggregate, 'w' => Window });

// provolatile tells whether the function's result depends only on its input arguments, or is affected by outside factors. It is i for “immutable” functions, which always deliver the same result for the same inputs. It is s for “stable” functions, whose results (for fixed inputs) do not change within a scan. It is v for “volatile” functions, whose results might change at any time. (Use v also for functions with side-effects, so that calls to them cannot get optimized away.)
pg_char_enum!(FunctionVolatilty { 'i' => Immutable, 's' => Stable, 'v' => Volatile });

// encoded as i for IN arguments, o for OUT arguments, b for INOUT arguments, v for VARIADIC arguments, t for TABLE arguments
pg_char_enum!(ArgMode { 'i' => In, 'o' => Out, 'b' => InOut, 'v' => Variadic, 't' => Table });

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Qual {
	pub schema_name: Str,
	pub name: Str,
}
// impl<T: AsRef<str>> hashbrown::Equivalent<Qual> for (&T, &T) {
// 	fn equivalent(&self, key: &Qual) -> bool {
// 		key.schema_name == self.0.as_ref() && key.name == self.1.as_ref()
// 	}
// }

#[derive(Debug)]
pub struct PgState {
	pub pg_class: Set<PgClass>,
}


// `pg_aggregate`: https://www.postgresql.org/docs/17/catalog-pg-aggregate.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PgAggregate {
	// aggfnoid regproc -- pg_proc OID of the aggregate function
	aggfnoid: Qual,
	// aggkind char -- Aggregate kind: n for “normal” aggregates, o for “ordered-set” aggregates, or h for “hypothetical-set” aggregates
	// aggnumdirectargs int2 -- Number of direct (non-aggregated) arguments of an ordered-set or hypothetical-set aggregate, counting a variadic array as one argument. If equal to pronargs, the aggregate must be variadic and the variadic array describes the aggregated arguments as well as the final direct arguments. Always zero for normal aggregates.
	// aggtransfn regproc -- Transition function
	// aggfinalfn regproc -- Final function (zero if none)
	// aggcombinefn regproc -- Combine function (zero if none)
	// aggserialfn regproc -- Serialization function (zero if none)
	// aggdeserialfn regproc -- Deserialization function (zero if none)
	// aggmtransfn regproc -- Forward transition function for moving-aggregate mode (zero if none)
	// aggminvtransfn regproc -- Inverse transition function for moving-aggregate mode (zero if none)
	// aggmfinalfn regproc -- Final function for moving-aggregate mode (zero if none)
	// aggfinalextra bool -- True to pass extra dummy arguments to aggfinalfn
	// aggmfinalextra bool -- True to pass extra dummy arguments to aggmfinalfn
	// aggfinalmodify char -- Whether aggfinalfn modifies the transition state value: r if it is read-only, s if the aggtransfn cannot be applied after the aggfinalfn, or w if it writes on the value
	// aggmfinalmodify char -- Like aggfinalmodify, but for the aggmfinalfn
	// aggsortop oid -- Associated sort operator (zero if none)
	// aggtranstype oid -- Data type of the aggregate function's internal transition (state) data
	// aggtransspace int4 -- Approximate average size (in bytes) of the transition state data, or zero to use a default estimate
	// aggmtranstype oid -- Data type of the aggregate function's internal transition (state) data for moving-aggregate mode (zero if none)
	// aggmtransspace int4 -- Approximate average size (in bytes) of the transition state data for moving-aggregate mode, or zero to use a default estimate
	// agginitval text -- The initial value of the transition state. This is a text field containing the initial value in its external string representation. If this field is null, the transition state value starts out null.
	// aggminitval text -- The initial value of the transition state for moving-aggregate mode. This is a text field containing the initial value in its external string representation. If this field is null, the transition state value starts out null.
}

// `pg_am`: https://www.postgresql.org/docs/17/catalog-pg-am.html


// `pg_amop`: https://www.postgresql.org/docs/17/catalog-pg-amop.html


// `pg_amproc`: https://www.postgresql.org/docs/17/catalog-pg-amproc.html


// `pg_attrdef`: https://www.postgresql.org/docs/17/catalog-pg-attrdef.html


// `pg_attribute`: https://www.postgresql.org/docs/17/catalog-pg-attribute.html


// `pg_authid`: https://www.postgresql.org/docs/17/catalog-pg-authid.html
// `pg_roles`: https://www.postgresql.org/docs/17/view-pg-roles.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PgRoles {
	pub rolname: Str, // pg_get_userbyid(oid)::text as rolname, -- name  Role name
	pub rolsuper: bool, // rolsuper, -- bool  Role has superuser privileges
	pub rolinherit: bool, // rolinherit, -- bool  Role automatically inherits privileges of roles it is a member of
	pub rolcreaterole: bool, // rolcreaterole, -- bool  Role can create more roles
	pub rolcreatedb: bool, // rolcreatedb, -- bool  Role can create databases
	pub rolcanlogin: bool, // rolcanlogin, -- bool  Role can log in. That is, this role can be given as the initial session authorization identifier
	pub rolreplication: bool, // rolreplication, -- bool  Role is a replication role. A replication role can initiate replication connections and create and drop replication slots.
	pub rolconnlimit: Option<u32>, // case when rolconnlimit < 0 then null else rolconnlimit end as rolconnlimit, -- int4  For roles that can log in, this sets maximum number of concurrent connections this role can make. -1 means no limit.
	pub rolvaliduntil: Option<chrono::DateTime<chrono::FixedOffset>>, // rolvaliduntil, -- timestamptz  Password expiry time (only used for password authentication); null if no expiration
	pub rolbypassrls: bool, // rolbypassrls, -- bool  Role bypasses every row-level security policy, see Section 5.9 for more information.
	pub rolconfig: Option<Vec<Str>>, // rolconfig -- text[]  Role-specific defaults for run-time configuration variables
}
impl_name_hash_and_equivalent!(PgRoles, rolname);

// `pg_auth_members`: https://www.postgresql.org/docs/17/catalog-pg-auth-members.html


// `pg_cast`: https://www.postgresql.org/docs/17/catalog-pg-cast.html


// `pg_class`: https://www.postgresql.org/docs/17/catalog-pg-class.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PgClass {
	relname: Str, // relname::text, -- name  Name of the table, index, view, etc.
	relnamespace: Str, // pg_namespace.nspname::text as relnamespace, -- oid (references pg_namespace.oid) The OID of the namespace that contains this relation
	reltype: Option<Qual>, // reltype_typ_sch.nspname::text as reltype_schema_name, reltype_typ.typname::text as reltype_name, -- oid (references pg_type.oid) The OID of the data type that corresponds to this table's row type, if any; zero for indexes, sequences, and toast tables, which have no pg_type entry
	reloftype: Option<Qual>, // reloftype_typ_sch.nspname::text as reloftype_schema_name, reloftype_typ.typname::text as reloftype_name, -- oid (references pg_type.oid) For typed tables, the OID of the underlying composite type; zero for all other relations
	relowner: Str, // pg_get_userbyid(relowner)::text as relowner, -- oid (references pg_authid.oid) Owner of the relation
	// -- relam -- oid (references pg_am.oid) The access method used to access this table or index. Not meaningful if the relation is a sequence or has no on-disk file, except for partitioned tables, where, if set, it takes precedence over default_table_access_method when determining the access method to use for partitions created when one is not specified in the creation command.
	// -- relfilenode -- oid  Name of the on-disk file of this relation; zero means this is a “mapped” relation whose disk file name is determined by low-level state
	reltablespace: Option<Str>, // pg_tablespace.spcname::text as reltablespace, -- oid (references pg_tablespace.oid) The tablespace in which this relation is stored. If zero, the database's default tablespace is implied. Not meaningful if the relation has no on-disk file, except for partitioned tables, where this is the tablespace in which partitions will be created when one is not specified in the creation command.
	// -- relpages -- int4  Size of the on-disk representation of this table in pages (of size BLCKSZ). This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX.
	// -- reltuples -- float4  Number of live rows in the table. This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX. If the table has never yet been vacuumed or analyzed, reltuples contains -1 indicating that the row count is unknown.
	// -- relallvisible -- int4  Number of pages that are marked all-visible in the table's visibility map. This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX.
	// -- reltoastrelid -- oid (references pg_class.oid) OID of the TOAST table associated with this table, zero if none. The TOAST table stores large attributes “out of line” in a secondary table.
	// -- relhasindex -- bool  True if this is a table and it has (or recently had) any indexes
	relisshared: bool, // relisshared, -- bool  True if this table is shared across all databases in the cluster. Only certain system catalogs (such as pg_database) are shared.
	relpersistence: ClassPersistence, // relpersistence, -- char  p = permanent table/sequence, u = unlogged table/sequence, t = temporary table/sequence
	relkind: ClassKind, // relkind, -- char  r = ordinary table, i = index, S = sequence, t = TOAST table, v = view, m = materialized view, c = composite type, f = foreign table, p = partitioned table, I = partitioned index
	// -- relnatts -- int2  Number of user columns in the relation (system columns not counted). There must be this many corresponding entries in pg_attribute. See also pg_attribute.attnum.
	// -- relchecks -- int2  Number of CHECK constraints on the table; see pg_constraint catalog
	// -- relhasrules -- bool  True if table has (or once had) rules; see pg_rewrite catalog
	// -- relhastriggers -- bool  True if table has (or once had) triggers; see pg_trigger catalog
	// -- relhassubclass -- bool  True if table or index has (or once had) any inheritance children or partitions
	relrowsecurity: bool, // relrowsecurity, -- bool  True if table has row-level security enabled; see pg_policy catalog
	relforcerowsecurity: bool, // relforcerowsecurity, -- bool  True if row-level security (when enabled) will also apply to table owner; see pg_policy catalog
	// -- relispopulated -- bool  True if relation is populated (this is true for all relations other than some materialized views)
	// -- relreplident -- char  Columns used to form “replica identity” for rows: d = default (primary key, if any), n = nothing, f = all columns, i = index with indisreplident set (same as nothing if the index used has been dropped)
	relispartition: bool, // relispartition, -- bool  True if table or index is a partition
	// -- relrewrite -- oid (references pg_class.oid) For new relations being written during a DDL operation that requires a table rewrite, this contains the OID of the original relation; otherwise zero. That state is only visible internally; this field should never contain anything other than zero for a user-visible relation.
	// -- relfrozenxid -- xid  All transaction IDs before this one have been replaced with a permanent (“frozen”) transaction ID in this table. This is used to track whether the table needs to be vacuumed in order to prevent transaction ID wraparound or to allow pg_xact to be shrunk. Zero (InvalidTransactionId) if the relation is not a table.
	// -- relminmxid -- xid  All multixact IDs before this one have been replaced by a transaction ID in this table. This is used to track whether the table needs to be vacuumed in order to prevent multixact ID wraparound or to allow pg_multixact to be shrunk. Zero (InvalidMultiXactId) if the relation is not a table.
	relacl: Option<Vec<aclitem::TableAclItem>>, // relacl::text[] as relacl, -- aclitem[]  Access privileges; see Section 5.8 for details
	reloptions: Option<Vec<Str>>, // reloptions::text[], -- text[]  Access-method-specific options, as “keyword=value” strings
	relpartbound: Option<Str>, // pg_get_expr(relpartbound, pg_class.oid) as relpartbound -- pg_node_tree  If table is a partition (see relispartition), internal representation of the partition bound
}
impl_name_hash_and_equivalent!(PgClass, relname);

// p = permanent table/sequence, u = unlogged table/sequence, t = temporary table/sequence
pg_char_enum!(ClassPersistence { 'p' => Permanant, 'u' => Unlogged, 't' => Temporary });

// r = ordinary table, i = index, S = sequence, t = TOAST table, v = view, m = materialized view, c = composite type, f = foreign table, p = partitioned table, I = partitioned index
pg_char_enum!(ClassKind { 'r' => Table, 'i' => Index, 'S' => Sequence, 't' => Toast, 'v' => View, 'm' => MaterializedView, 'c' => CompositeType, 'f' => ForeignTable, 'p' => PartitionedTable, 'I' => PartitionedIndex });


// `pg_collation`: https://www.postgresql.org/docs/17/catalog-pg-collation.html


// `pg_constraint`: https://www.postgresql.org/docs/17/catalog-pg-constraint.html


// `pg_conversion`: https://www.postgresql.org/docs/17/catalog-pg-conversion.html


// `pg_database`: https://www.postgresql.org/docs/17/catalog-pg-database.html


// `pg_db_role_setting`: https://www.postgresql.org/docs/17/catalog-pg-db-role-setting.html


// `pg_default_acl`: https://www.postgresql.org/docs/17/catalog-pg-default-acl.html


// `pg_depend`: https://www.postgresql.org/docs/17/catalog-pg-depend.html


// `pg_description`: https://www.postgresql.org/docs/17/catalog-pg-description.html


// `pg_char_enum`: https://www.postgresql.org/docs/17/catalog-pg-enum.html


// `pg_event_trigger`: https://www.postgresql.org/docs/17/catalog-pg-event-trigger.html


// `pg_extension`: https://www.postgresql.org/docs/17/catalog-pg-extension.html


// `pg_foreign_data_wrapper`: https://www.postgresql.org/docs/17/catalog-pg-foreign-data-wrapper.html


// `pg_foreign_server`: https://www.postgresql.org/docs/17/catalog-pg-foreign-server.html


// `pg_foreign_table`: https://www.postgresql.org/docs/17/catalog-pg-foreign-table.html


// `pg_index`: https://www.postgresql.org/docs/17/catalog-pg-index.html


// `pg_inherits`: https://www.postgresql.org/docs/17/catalog-pg-inherits.html


// `pg_init_privs`: https://www.postgresql.org/docs/17/catalog-pg-init-privs.html


// `pg_language`: https://www.postgresql.org/docs/17/catalog-pg-language.html


// `pg_largeobject`: https://www.postgresql.org/docs/17/catalog-pg-largeobject.html


// `pg_largeobject_metadata`: https://www.postgresql.org/docs/17/catalog-pg-largeobject-metadata.html


// `pg_namespace`: https://www.postgresql.org/docs/17/catalog-pg-namespace.html


// `pg_opclass`: https://www.postgresql.org/docs/17/catalog-pg-opclass.html


// `pg_operator`: https://www.postgresql.org/docs/17/catalog-pg-operator.html


// `pg_opfamily`: https://www.postgresql.org/docs/17/catalog-pg-opfamily.html


// `pg_parameter_acl`: https://www.postgresql.org/docs/17/catalog-pg-parameter-acl.html


// `pg_partitioned_table`: https://www.postgresql.org/docs/17/catalog-pg-partitioned-table.html


// `pg_policy`: https://www.postgresql.org/docs/17/catalog-pg-policy.html


// `pg_proc`: https://www.postgresql.org/docs/17/catalog-pg-proc.html


// `pg_publication`: https://www.postgresql.org/docs/17/catalog-pg-publication.html


// `pg_publication_namespace`: https://www.postgresql.org/docs/17/catalog-pg-publication-namespace.html


// `pg_publication_rel`: https://www.postgresql.org/docs/17/catalog-pg-publication-rel.html


// `pg_range`: https://www.postgresql.org/docs/17/catalog-pg-range.html


// `pg_replication_origin`: https://www.postgresql.org/docs/17/catalog-pg-replication-origin.html


// `pg_rewrite`: https://www.postgresql.org/docs/17/catalog-pg-rewrite.html


// `pg_seclabel`: https://www.postgresql.org/docs/17/catalog-pg-seclabel.html


// `pg_sequence`: https://www.postgresql.org/docs/17/catalog-pg-sequence.html


// `pg_shdepend`: https://www.postgresql.org/docs/17/catalog-pg-shdepend.html


// `pg_shdescription`: https://www.postgresql.org/docs/17/catalog-pg-shdescription.html


// `pg_shseclabel`: https://www.postgresql.org/docs/17/catalog-pg-shseclabel.html


// `pg_statistic`: https://www.postgresql.org/docs/17/catalog-pg-statistic.html


// `pg_statistic_ext`: https://www.postgresql.org/docs/17/catalog-pg-statistic-ext.html


// `pg_statistic_ext_data`: https://www.postgresql.org/docs/17/catalog-pg-statistic-ext-data.html


// `pg_subscription`: https://www.postgresql.org/docs/17/catalog-pg-subscription.html


// `pg_subscription_rel`: https://www.postgresql.org/docs/17/catalog-pg-subscription-rel.html


// `pg_tablespace`: https://www.postgresql.org/docs/17/catalog-pg-tablespace.html


// `pg_transform`: https://www.postgresql.org/docs/17/catalog-pg-transform.html


// `pg_trigger`: https://www.postgresql.org/docs/17/catalog-pg-trigger.html


// `pg_ts_config`: https://www.postgresql.org/docs/17/catalog-pg-ts-config.html


// `pg_ts_config_map`: https://www.postgresql.org/docs/17/catalog-pg-ts-config-map.html


// `pg_ts_dict`: https://www.postgresql.org/docs/17/catalog-pg-ts-dict.html


// `pg_ts_parser`: https://www.postgresql.org/docs/17/catalog-pg-ts-parser.html


// `pg_ts_template`: https://www.postgresql.org/docs/17/catalog-pg-ts-template.html


// `pg_type`: https://www.postgresql.org/docs/17/catalog-pg-type.html


// `pg_user_mapping`: https://www.postgresql.org/docs/17/catalog-pg-user-mapping.html


