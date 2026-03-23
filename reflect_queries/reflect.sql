-- `pg_aggregate`: https://www.postgresql.org/docs/17/catalog-pg-aggregate.html

-- `pg_am`: https://www.postgresql.org/docs/17/catalog-pg-am.html

-- `pg_amop`: https://www.postgresql.org/docs/17/catalog-pg-amop.html

-- `pg_amproc`: https://www.postgresql.org/docs/17/catalog-pg-amproc.html

-- `pg_attrdef`: https://www.postgresql.org/docs/17/catalog-pg-attrdef.html

-- `pg_attribute`: https://www.postgresql.org/docs/17/catalog-pg-attribute.html

-- `pg_authid`: https://www.postgresql.org/docs/17/catalog-pg-authid.html
-- `pg_roles`: https://www.postgresql.org/docs/17/view-pg-roles.html

-- `pg_auth_members`: https://www.postgresql.org/docs/17/catalog-pg-auth-members.html

-- `pg_cast`: https://www.postgresql.org/docs/17/catalog-pg-cast.html

-- `pg_class`: https://www.postgresql.org/docs/17/catalog-pg-class.html

-- `pg_collation`: https://www.postgresql.org/docs/17/catalog-pg-collation.html

-- `pg_constraint`: https://www.postgresql.org/docs/17/catalog-pg-constraint.html

-- `pg_conversion`: https://www.postgresql.org/docs/17/catalog-pg-conversion.html

-- `pg_database`: https://www.postgresql.org/docs/17/catalog-pg-database.html
--! reflect_pg_database : (datconnlimit?, datcollate?, datctype?, datlocale?, daticurules?, datcollversion?, datacl?)
select
	-- oid oid  Row identifier
	datname::text as datname, -- name  Database name
	pg_get_userbyid(datdba)::text as datdba, -- oid (references pg_authid.oid) Owner of the database, usually the user who created it
	pg_encoding_to_char(encoding)::text as encoding, -- int4  Character encoding for this database (pg_encoding_to_char() can translate this number to the encoding name)
	datlocprovider, -- char  Locale provider for this database: b = builtin, c = libc, i = icu
	datistemplate, -- bool  If true, then this database can be cloned by any user with CREATEDB privileges; if false, then only superusers or the owner of the database can clone it.
	datallowconn, -- bool  If false then no one can connect to this database. This is used to protect the template0 database from being altered.
	-- dathasloginevt bool  Indicates that there are login event triggers defined for this database. This flag is used to avoid extra lookups on the pg_event_trigger table during each backend startup. This flag is used internally by PostgreSQL and should not be manually altered or read for monitoring purposes.
	case when datconnlimit < 0 then null else datconnlimit end as datconnlimit, -- int4  Sets maximum number of concurrent connections that can be made to this database. -1 means no limit, -2 indicates the database is invalid.
	-- datfrozenxid xid  All transaction IDs before this one have been replaced with a permanent (“frozen”) transaction ID in this database. This is used to track whether the database needs to be vacuumed in order to prevent transaction ID wraparound or to allow pg_xact to be shrunk. It is the minimum of the per-table pg_class.relfrozenxid values.
	-- datminmxid xid  All multixact IDs before this one have been replaced with a transaction ID in this database. This is used to track whether the database needs to be vacuumed in order to prevent multixact ID wraparound or to allow pg_multixact to be shrunk. It is the minimum of the per-table pg_class.relminmxid values.
	-- dattablespace oid (references pg_tablespace.oid) The default tablespace for the database. Within this database, all tables for which pg_class.reltablespace is zero will be stored in this tablespace; in particular, all the non-shared system catalogs will be there.
	datcollate, -- text  LC_COLLATE for this database
	datctype, -- text  LC_CTYPE for this database
	datlocale, -- text  Collation provider locale name for this database. If the provider is libc, datlocale is NULL; datcollate and datctype are used instead.
	daticurules, -- text  ICU collation rules for this database
	datcollversion, -- text  Provider-specific version of the collation. This is recorded when the database is created and then checked when it is used, to detect changes in the collation definition that could lead to data corruption.
	datacl::text[] as datacl -- aclitem[]  Access privileges; see Section 5.8 for details
from
	pg_database
where
	datname = current_database()
;

-- `pg_db_role_setting`: https://www.postgresql.org/docs/17/catalog-pg-db-role-setting.html
--! reflect_pg_db_role_setting : (setrole?, setconfig?)
select
	setdatabase != 0 as setdatabase, -- oid (references pg_database.oid) The OID of the database the setting is applicable to, or zero if not database-specific,
	case when setrole = 0 then null else pg_get_userbyid(setrole)::text end as setrole, -- oid (references pg_authid.oid) The OID of the role the setting is applicable to, or zero if not role-specific
	setconfig -- text[]  Defaults for run-time configuration variables
from
	pg_db_role_setting
where (setdatabase = 0 or setdatabase = (select oid from pg_database where datname = current_database()))
;

-- `pg_default_acl`: https://www.postgresql.org/docs/17/catalog-pg-default-acl.html

-- `pg_depend`: https://www.postgresql.org/docs/17/catalog-pg-depend.html

-- `pg_description`: https://www.postgresql.org/docs/17/catalog-pg-description.html

-- `pg_enum`: https://www.postgresql.org/docs/17/catalog-pg-enum.html
--! reflect_pg_enum : ()
select
	-- oid oid  Row identifier
	-- enumtypid oid (references pg_type.oid) The OID of the pg_type entry owning this enum value,
	-- enumsortorder float4  The sort position of this enum value within its enum type,
	-- enumlabel name  The textual label for this enum value
	enumtypid::regtype::text as enumtypid,
	array_agg(enumlabel::text order by enumsortorder) as enumlabels
from
	pg_enum
group by enumtypid
;

-- `pg_event_trigger`: https://www.postgresql.org/docs/17/catalog-pg-event-trigger.html

-- `pg_extension`: https://www.postgresql.org/docs/17/catalog-pg-extension.html

-- `pg_foreign_data_wrapper`: https://www.postgresql.org/docs/17/catalog-pg-foreign-data-wrapper.html

-- `pg_foreign_server`: https://www.postgresql.org/docs/17/catalog-pg-foreign-server.html

-- `pg_foreign_table`: https://www.postgresql.org/docs/17/catalog-pg-foreign-table.html

-- `pg_index`: https://www.postgresql.org/docs/17/catalog-pg-index.html

-- `pg_inherits`: https://www.postgresql.org/docs/17/catalog-pg-inherits.html

-- `pg_init_privs`: https://www.postgresql.org/docs/17/catalog-pg-init-privs.html

-- `pg_language`: https://www.postgresql.org/docs/17/catalog-pg-language.html

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

-- `pg_range`: https://www.postgresql.org/docs/17/catalog-pg-range.html

-- `pg_replication_origin`: https://www.postgresql.org/docs/17/catalog-pg-replication-origin.html

-- `pg_rewrite`: https://www.postgresql.org/docs/17/catalog-pg-rewrite.html

-- `pg_seclabel`: https://www.postgresql.org/docs/17/catalog-pg-seclabel.html

-- `pg_sequence`: https://www.postgresql.org/docs/17/catalog-pg-sequence.html

-- `pg_shdepend`: https://www.postgresql.org/docs/17/catalog-pg-shdepend.html

-- `pg_shdescription`: https://www.postgresql.org/docs/17/catalog-pg-shdescription.html

-- `pg_shseclabel`: https://www.postgresql.org/docs/17/catalog-pg-shseclabel.html

-- `pg_statistic_ext`: https://www.postgresql.org/docs/17/catalog-pg-statistic-ext.html

-- `pg_subscription`: https://www.postgresql.org/docs/17/catalog-pg-subscription.html

-- `pg_transform`: https://www.postgresql.org/docs/17/catalog-pg-transform.html

-- `pg_trigger`: https://www.postgresql.org/docs/17/catalog-pg-trigger.html

-- `pg_ts_config`: https://www.postgresql.org/docs/17/catalog-pg-ts-config.html

-- `pg_ts_config_map`: https://www.postgresql.org/docs/17/catalog-pg-ts-config-map.html

-- `pg_ts_dict`: https://www.postgresql.org/docs/17/catalog-pg-ts-dict.html

-- `pg_ts_parser`: https://www.postgresql.org/docs/17/catalog-pg-ts-parser.html

-- `pg_ts_template`: https://www.postgresql.org/docs/17/catalog-pg-ts-template.html

-- `pg_type`: https://www.postgresql.org/docs/17/catalog-pg-type.html

-- `pg_user_mappings`: https://www.postgresql.org/docs/17/view-pg-user-mappings.html
