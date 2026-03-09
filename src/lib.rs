pub(crate) use smol_str::SmolStr as Str;

pub(crate) use reflect_crate::tokio_postgres::{self as postgres, /*Config as PgConfig,*/ Client as PgClient};

pub type Set<T> = hashbrown::HashSet<T>;
pub type Map<T> = hashbrown::HashMap<Str, T>;
pub(crate) use hashbrown::HashMap;

mod reflect;
#[cfg(test)]
mod reflect_test;

pub use reflect::{reflect_pg_state, reflect_pg_class};

mod aclitem;

//  ($type:ty, $field:ident) => {
macro_rules! impl_hash_and_equivalent {
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

macro_rules! pg_enum {
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

pg_enum!(FunctionKind { 'f' => Function, 'p' => Procedure, 'a' => Aggregate, 'w' => Window });

// provolatile tells whether the function's result depends only on its input arguments, or is affected by outside factors. It is i for “immutable” functions, which always deliver the same result for the same inputs. It is s for “stable” functions, whose results (for fixed inputs) do not change within a scan. It is v for “volatile” functions, whose results might change at any time. (Use v also for functions with side-effects, so that calls to them cannot get optimized away.)
pg_enum!(FunctionVolatilty { 'i' => Immutable, 's' => Stable, 'v' => Volatile });

// encoded as i for IN arguments, o for OUT arguments, b for INOUT arguments, v for VARIADIC arguments, t for TABLE arguments
pg_enum!(ArgMode { 'i' => In, 'o' => Out, 'b' => InOut, 'v' => Variadic, 't' => Table });

// p = permanent table/sequence, u = unlogged table/sequence, t = temporary table/sequence
pg_enum!(ClassPersistence { 'p' => Permanant, 'u' => Unlogged, 't' => Temporary });

// r = ordinary table, i = index, S = sequence, t = TOAST table, v = view, m = materialized view, c = composite type, f = foreign table, p = partitioned table, I = partitioned index
pg_enum!(ClassKind { 'r' => Table, 'i' => Index, 'S' => Sequence, 't' => Toast, 'v' => View, 'm' => MaterializedView, 'c' => CompositeType, 'f' => ForeignTable, 'p' => PartitionedTable, 'I' => PartitionedIndex });

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ref {
	pub schema_name: Str,
	pub name: Str,
}

#[derive(Debug)]
pub struct PgState {
	pub pg_class: Set<PgClass>,
}


// `pg_aggregate`: https://www.postgresql.org/docs/17/catalog-pg-aggregate.html


// `pg_am`: https://www.postgresql.org/docs/17/catalog-pg-am.html


// `pg_amop`: https://www.postgresql.org/docs/17/catalog-pg-amop.html


// `pg_amproc`: https://www.postgresql.org/docs/17/catalog-pg-amproc.html


// `pg_attrdef`: https://www.postgresql.org/docs/17/catalog-pg-attrdef.html


// `pg_attribute`: https://www.postgresql.org/docs/17/catalog-pg-attribute.html


// `pg_authid`: https://www.postgresql.org/docs/17/catalog-pg-authid.html


// `pg_auth_members`: https://www.postgresql.org/docs/17/catalog-pg-auth-members.html


// `pg_cast`: https://www.postgresql.org/docs/17/catalog-pg-cast.html


// `pg_class`: https://www.postgresql.org/docs/17/catalog-pg-class.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PgClass {
	// relname name -- Name of the table, index, view, etc.
	relname: Str,
	// relnamespace oid -- The OID of the namespace that contains this relation
	relnamespace: Str,
	// reltype oid -- The OID of the data type that corresponds to this table's row type, if any; zero for indexes, sequences, and toast tables, which have no pg_type entry
	reltype: Option<Ref>,
	// reloftype oid -- For typed tables, the OID of the underlying composite type; zero for all other relations
	reloftype: Option<Ref>,
	// relowner oid -- Owner of the relation
	relowner: Str,
	// relam oid -- The access method used to access this table or index. Not meaningful if the relation is a sequence or has no on-disk file, except for partitioned tables, where, if set, it takes precedence over default_table_access_method when determining the access method to use for partitions created when one is not specified in the creation command.
	// relam: Option<Str>,
	// relfilenode oid -- Name of the on-disk file of this relation; zero means this is a “mapped” relation whose disk file name is determined by low-level state
	// reltablespace oid -- The tablespace in which this relation is stored. If zero, the database's default tablespace is implied. Not meaningful if the relation has no on-disk file, except for partitioned tables, where this is the tablespace in which partitions will be created when one is not specified in the creation command.
	reltablespace: Option<Str>,
	// relpages int4 -- Size of the on-disk representation of this table in pages (of size BLCKSZ). This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX.
	// reltuples float4 -- Number of live rows in the table. This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX. If the table has never yet been vacuumed or analyzed, reltuples contains -1 indicating that the row count is unknown.
	// relallvisible int4 -- Number of pages that are marked all-visible in the table's visibility map. This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX.
	// reltoastrelid oid -- OID of the TOAST table associated with this table, zero if none. The TOAST table stores large attributes “out of line” in a secondary table.
	// relhasindex bool -- True if this is a table and it has (or recently had) any indexes
	// relisshared bool -- True if this table is shared across all databases in the cluster. Only certain system catalogs (such as pg_database) are shared.
	relisshared: bool,
	// relpersistence char -- p = permanent table/sequence, u = unlogged table/sequence, t = temporary table/sequence
	relpersistence: ClassPersistence,
	// relkind char -- r = ordinary table, i = index, S = sequence, t = TOAST table, v = view, m = materialized view, c = composite type, f = foreign table, p = partitioned table, I = partitioned index
	relkind: ClassKind,
	// relnatts int2 -- Number of user columns in the relation (system columns not counted). There must be this many corresponding entries in pg_attribute. See also pg_attribute.attnum.
	// relchecks int2 -- Number of CHECK constraints on the table; see pg_constraint catalog
	// relhasrules bool -- True if table has (or once had) rules; see pg_rewrite catalog
	// relhastriggers bool -- True if table has (or once had) triggers; see pg_trigger catalog
	// relhassubclass bool -- True if table or index has (or once had) any inheritance children or partitions
	// relrowsecurity bool -- True if table has row-level security enabled; see pg_policy catalog
	relrowsecurity: bool,
	// relforcerowsecurity bool -- True if row-level security (when enabled) will also apply to table owner; see pg_policy catalog
	relforcerowsecurity: bool,
	// relispopulated bool -- True if relation is populated (this is true for all relations other than some materialized views)
	// relreplident char -- Columns used to form “replica identity” for rows: d = default (primary key, if any), n = nothing, f = all columns, i = index with indisreplident set (same as nothing if the index used has been dropped)
	// relispartition bool -- True if table or index is a partition
	relispartition: bool,
	// relrewrite oid -- For new relations being written during a DDL operation that requires a table rewrite, this contains the OID of the original relation; otherwise zero. That state is only visible internally; this field should never contain anything other than zero for a user-visible relation.
	// relfrozenxid xid -- All transaction IDs before this one have been replaced with a permanent (“frozen”) transaction ID in this table. This is used to track whether the table needs to be vacuumed in order to prevent transaction ID wraparound or to allow pg_xact to be shrunk. Zero (InvalidTransactionId) if the relation is not a table.
	// relminmxid xid -- All multixact IDs before this one have been replaced by a transaction ID in this table. This is used to track whether the table needs to be vacuumed in order to prevent multixact ID wraparound or to allow pg_multixact to be shrunk. Zero (InvalidMultiXactId) if the relation is not a table.
	// relacl aclitem[] -- Access privileges; see Section 5.8 for details
	relacl: Vec<aclitem::TableAclItem>,
	// reloptions text[] -- Access-method-specific options, as “keyword=value” strings
	reloptions: Vec<Str>,
	// relpartbound pg_node_tree -- If table is a partition (see relispartition), internal representation of the partition bound
	// relpartbound: Option<Str>,
}
impl_hash_and_equivalent!(PgClass, relname);


// `pg_collation`: https://www.postgresql.org/docs/17/catalog-pg-collation.html


// `pg_constraint`: https://www.postgresql.org/docs/17/catalog-pg-constraint.html


// `pg_conversion`: https://www.postgresql.org/docs/17/catalog-pg-conversion.html


// `pg_database`: https://www.postgresql.org/docs/17/catalog-pg-database.html


// `pg_db_role_setting`: https://www.postgresql.org/docs/17/catalog-pg-db-role-setting.html


// `pg_default_acl`: https://www.postgresql.org/docs/17/catalog-pg-default-acl.html


// `pg_depend`: https://www.postgresql.org/docs/17/catalog-pg-depend.html


// `pg_description`: https://www.postgresql.org/docs/17/catalog-pg-description.html


// `pg_enum`: https://www.postgresql.org/docs/17/catalog-pg-enum.html


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


