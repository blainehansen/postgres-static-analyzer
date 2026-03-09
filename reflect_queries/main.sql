-- `pg_aggregate`: https://www.postgresql.org/docs/17/catalog-pg-aggregate.html


-- `pg_am`: https://www.postgresql.org/docs/17/catalog-pg-am.html


-- `pg_amop`: https://www.postgresql.org/docs/17/catalog-pg-amop.html


-- `pg_amproc`: https://www.postgresql.org/docs/17/catalog-pg-amproc.html


-- `pg_attrdef`: https://www.postgresql.org/docs/17/catalog-pg-attrdef.html


-- `pg_attribute`: https://www.postgresql.org/docs/17/catalog-pg-attribute.html


-- `pg_authid`: https://www.postgresql.org/docs/17/catalog-pg-authid.html
-- `pg_roles`: https://www.postgresql.org/docs/17/view-pg-roles.html
--! reflect_pg_roles : (rolconnlimit?, rolvaliduntil?, rolconfig?)
select
	pg_get_userbyid(oid)::text as rolname, -- name  Role name
	rolsuper, -- bool  Role has superuser privileges
	rolinherit, -- bool  Role automatically inherits privileges of roles it is a member of
	rolcreaterole, -- bool  Role can create more roles
	rolcreatedb, -- bool  Role can create databases
	rolcanlogin, -- bool  Role can log in. That is, this role can be given as the initial session authorization identifier
	rolreplication, -- bool  Role is a replication role. A replication role can initiate replication connections and create and drop replication slots.
	case when rolconnlimit < 0 then null else rolconnlimit end as rolconnlimit, -- int4  For roles that can log in, this sets maximum number of concurrent connections this role can make. -1 means no limit.
	rolvaliduntil, -- timestamptz  Password expiry time (only used for password authentication); null if no expiration
	rolbypassrls, -- bool  Role bypasses every row-level security policy, see Section 5.9 for more information.
	rolconfig -- text[]  Role-specific defaults for run-time configuration variables
from pg_roles
where pg_get_userbyid(pg_roles.oid) not in ('pg_database_owner', 'pg_read_all_data', 'pg_write_all_data', 'pg_monitor', 'pg_read_all_settings', 'pg_read_all_stats', 'pg_stat_scan_tables', 'pg_read_server_files', 'pg_write_server_files', 'pg_execute_server_program', 'pg_signal_backend', 'pg_checkpoint', 'pg_maintain', 'pg_use_reserved_connections', 'pg_create_subscription')
;

-- `pg_auth_members`: https://www.postgresql.org/docs/17/catalog-pg-auth-members.html


-- `pg_cast`: https://www.postgresql.org/docs/17/catalog-pg-cast.html


-- `pg_class`: https://www.postgresql.org/docs/17/catalog-pg-class.html
--! reflect_pg_class : (reltype_schema_name?, reltype_name?, reloftype_schema_name?, reloftype_name?, reltablespace?, relacl?, reloptions?, relpartbound?)
select
	relname::text, -- name  Name of the table, index, view, etc.
	pg_namespace.nspname::text as relnamespace, -- oid (references pg_namespace.oid) The OID of the namespace that contains this relation
	reltype_typ_sch.nspname::text as reltype_schema_name, reltype_typ.typname::text as reltype_name, -- oid (references pg_type.oid) The OID of the data type that corresponds to this table's row type, if any; zero for indexes, sequences, and toast tables, which have no pg_type entry
	reloftype_typ_sch.nspname::text as reloftype_schema_name, reloftype_typ.typname::text as reloftype_name, -- oid (references pg_type.oid) For typed tables, the OID of the underlying composite type; zero for all other relations
	pg_get_userbyid(relowner)::text as relowner, -- oid (references pg_authid.oid) Owner of the relation
	-- relam -- oid (references pg_am.oid) The access method used to access this table or index. Not meaningful if the relation is a sequence or has no on-disk file, except for partitioned tables, where, if set, it takes precedence over default_table_access_method when determining the access method to use for partitions created when one is not specified in the creation command.
	-- relfilenode -- oid  Name of the on-disk file of this relation; zero means this is a “mapped” relation whose disk file name is determined by low-level state
	pg_tablespace.spcname::text as reltablespace, -- oid (references pg_tablespace.oid) The tablespace in which this relation is stored. If zero, the database's default tablespace is implied. Not meaningful if the relation has no on-disk file, except for partitioned tables, where this is the tablespace in which partitions will be created when one is not specified in the creation command.
	-- relpages -- int4  Size of the on-disk representation of this table in pages (of size BLCKSZ). This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX.
	-- reltuples -- float4  Number of live rows in the table. This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX. If the table has never yet been vacuumed or analyzed, reltuples contains -1 indicating that the row count is unknown.
	-- relallvisible -- int4  Number of pages that are marked all-visible in the table's visibility map. This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX.
	-- reltoastrelid -- oid (references pg_class.oid) OID of the TOAST table associated with this table, zero if none. The TOAST table stores large attributes “out of line” in a secondary table.
	-- relhasindex -- bool  True if this is a table and it has (or recently had) any indexes
	relisshared, -- bool  True if this table is shared across all databases in the cluster. Only certain system catalogs (such as pg_database) are shared.
	relpersistence, -- char  p = permanent table/sequence, u = unlogged table/sequence, t = temporary table/sequence
	relkind, -- char  r = ordinary table, i = index, S = sequence, t = TOAST table, v = view, m = materialized view, c = composite type, f = foreign table, p = partitioned table, I = partitioned index
	-- relnatts -- int2  Number of user columns in the relation (system columns not counted). There must be this many corresponding entries in pg_attribute. See also pg_attribute.attnum.
	-- relchecks -- int2  Number of CHECK constraints on the table; see pg_constraint catalog
	-- relhasrules -- bool  True if table has (or once had) rules; see pg_rewrite catalog
	-- relhastriggers -- bool  True if table has (or once had) triggers; see pg_trigger catalog
	-- relhassubclass -- bool  True if table or index has (or once had) any inheritance children or partitions
	relrowsecurity, -- bool  True if table has row-level security enabled; see pg_policy catalog
	relforcerowsecurity, -- bool  True if row-level security (when enabled) will also apply to table owner; see pg_policy catalog
	-- relispopulated -- bool  True if relation is populated (this is true for all relations other than some materialized views)
	-- relreplident -- char  Columns used to form “replica identity” for rows: d = default (primary key, if any), n = nothing, f = all columns, i = index with indisreplident set (same as nothing if the index used has been dropped)
	relispartition, -- bool  True if table or index is a partition
	-- relrewrite -- oid (references pg_class.oid) For new relations being written during a DDL operation that requires a table rewrite, this contains the OID of the original relation; otherwise zero. That state is only visible internally; this field should never contain anything other than zero for a user-visible relation.
	-- relfrozenxid -- xid  All transaction IDs before this one have been replaced with a permanent (“frozen”) transaction ID in this table. This is used to track whether the table needs to be vacuumed in order to prevent transaction ID wraparound or to allow pg_xact to be shrunk. Zero (InvalidTransactionId) if the relation is not a table.
	-- relminmxid -- xid  All multixact IDs before this one have been replaced by a transaction ID in this table. This is used to track whether the table needs to be vacuumed in order to prevent multixact ID wraparound or to allow pg_multixact to be shrunk. Zero (InvalidMultiXactId) if the relation is not a table.
	relacl::text[] as relacl, -- aclitem[]  Access privileges; see Section 5.8 for details
	reloptions::text[], -- text[]  Access-method-specific options, as “keyword=value” strings
	pg_get_expr(relpartbound, pg_class.oid) as relpartbound -- pg_node_tree  If table is a partition (see relispartition), internal representation of the partition bound

from
	pg_catalog.pg_class

	join pg_catalog.pg_namespace on pg_class.relnamespace = pg_namespace.oid

	left join pg_catalog.pg_type as reltype_typ on pg_class.reltype = reltype_typ.oid
	left join pg_catalog.pg_namespace as reltype_typ_sch on reltype_typ.typnamespace = reltype_typ_sch.oid

	left join pg_catalog.pg_type as reloftype_typ on pg_class.reloftype = reloftype_typ.oid
	left join pg_catalog.pg_namespace as reloftype_typ_sch on reloftype_typ.typnamespace = reloftype_typ_sch.oid

	left join pg_catalog.pg_tablespace on pg_class.reltablespace = pg_tablespace.oid
where pg_namespace.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
;



-- `pg_collation`: https://www.postgresql.org/docs/17/catalog-pg-collation.html


-- `pg_constraint`: https://www.postgresql.org/docs/17/catalog-pg-constraint.html


-- `pg_conversion`: https://www.postgresql.org/docs/17/catalog-pg-conversion.html


-- `pg_database`: https://www.postgresql.org/docs/17/catalog-pg-database.html


-- `pg_db_role_setting`: https://www.postgresql.org/docs/17/catalog-pg-db-role-setting.html


-- `pg_default_acl`: https://www.postgresql.org/docs/17/catalog-pg-default-acl.html


-- `pg_depend`: https://www.postgresql.org/docs/17/catalog-pg-depend.html


-- `pg_description`: https://www.postgresql.org/docs/17/catalog-pg-description.html


-- `pg_enum`: https://www.postgresql.org/docs/17/catalog-pg-enum.html


-- `pg_event_trigger`: https://www.postgresql.org/docs/17/catalog-pg-event-trigger.html


-- `pg_extension`: https://www.postgresql.org/docs/17/catalog-pg-extension.html


-- `pg_foreign_data_wrapper`: https://www.postgresql.org/docs/17/catalog-pg-foreign-data-wrapper.html


-- `pg_foreign_server`: https://www.postgresql.org/docs/17/catalog-pg-foreign-server.html


-- `pg_foreign_table`: https://www.postgresql.org/docs/17/catalog-pg-foreign-table.html


-- `pg_index`: https://www.postgresql.org/docs/17/catalog-pg-index.html


-- `pg_inherits`: https://www.postgresql.org/docs/17/catalog-pg-inherits.html


-- `pg_init_privs`: https://www.postgresql.org/docs/17/catalog-pg-init-privs.html


-- `pg_language`: https://www.postgresql.org/docs/17/catalog-pg-language.html


-- `pg_largeobject`: https://www.postgresql.org/docs/17/catalog-pg-largeobject.html


-- `pg_largeobject_metadata`: https://www.postgresql.org/docs/17/catalog-pg-largeobject-metadata.html


-- `pg_namespace`: https://www.postgresql.org/docs/17/catalog-pg-namespace.html


-- `pg_opclass`: https://www.postgresql.org/docs/17/catalog-pg-opclass.html


-- `pg_operator`: https://www.postgresql.org/docs/17/catalog-pg-operator.html


-- `pg_opfamily`: https://www.postgresql.org/docs/17/catalog-pg-opfamily.html


-- `pg_parameter_acl`: https://www.postgresql.org/docs/17/catalog-pg-parameter-acl.html


-- `pg_partitioned_table`: https://www.postgresql.org/docs/17/catalog-pg-partitioned-table.html


-- `pg_policy`: https://www.postgresql.org/docs/17/catalog-pg-policy.html


-- `pg_proc`: https://www.postgresql.org/docs/17/catalog-pg-proc.html


-- `pg_publication`: https://www.postgresql.org/docs/17/catalog-pg-publication.html


-- `pg_publication_namespace`: https://www.postgresql.org/docs/17/catalog-pg-publication-namespace.html


-- `pg_publication_rel`: https://www.postgresql.org/docs/17/catalog-pg-publication-rel.html


-- `pg_range`: https://www.postgresql.org/docs/17/catalog-pg-range.html


-- `pg_replication_origin`: https://www.postgresql.org/docs/17/catalog-pg-replication-origin.html


-- `pg_rewrite`: https://www.postgresql.org/docs/17/catalog-pg-rewrite.html


-- `pg_seclabel`: https://www.postgresql.org/docs/17/catalog-pg-seclabel.html


-- `pg_sequence`: https://www.postgresql.org/docs/17/catalog-pg-sequence.html


-- `pg_shdepend`: https://www.postgresql.org/docs/17/catalog-pg-shdepend.html


-- `pg_shdescription`: https://www.postgresql.org/docs/17/catalog-pg-shdescription.html


-- `pg_shseclabel`: https://www.postgresql.org/docs/17/catalog-pg-shseclabel.html


-- `pg_statistic`: https://www.postgresql.org/docs/17/catalog-pg-statistic.html


-- `pg_statistic_ext`: https://www.postgresql.org/docs/17/catalog-pg-statistic-ext.html


-- `pg_statistic_ext_data`: https://www.postgresql.org/docs/17/catalog-pg-statistic-ext-data.html


-- `pg_subscription`: https://www.postgresql.org/docs/17/catalog-pg-subscription.html


-- `pg_subscription_rel`: https://www.postgresql.org/docs/17/catalog-pg-subscription-rel.html


-- `pg_tablespace`: https://www.postgresql.org/docs/17/catalog-pg-tablespace.html


-- `pg_transform`: https://www.postgresql.org/docs/17/catalog-pg-transform.html


-- `pg_trigger`: https://www.postgresql.org/docs/17/catalog-pg-trigger.html


-- `pg_ts_config`: https://www.postgresql.org/docs/17/catalog-pg-ts-config.html


-- `pg_ts_config_map`: https://www.postgresql.org/docs/17/catalog-pg-ts-config-map.html


-- `pg_ts_dict`: https://www.postgresql.org/docs/17/catalog-pg-ts-dict.html


-- `pg_ts_parser`: https://www.postgresql.org/docs/17/catalog-pg-ts-parser.html


-- `pg_ts_template`: https://www.postgresql.org/docs/17/catalog-pg-ts-template.html


-- `pg_type`: https://www.postgresql.org/docs/17/catalog-pg-type.html


-- `pg_user_mapping`: https://www.postgresql.org/docs/17/catalog-pg-user-mapping.html


