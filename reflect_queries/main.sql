-- `pg_aggregate`: https://www.postgresql.org/docs/17/catalog-pg-aggregate.html


-- `pg_am`: https://www.postgresql.org/docs/17/catalog-pg-am.html


-- `pg_amop`: https://www.postgresql.org/docs/17/catalog-pg-amop.html


-- `pg_amproc`: https://www.postgresql.org/docs/17/catalog-pg-amproc.html


-- `pg_attrdef`: https://www.postgresql.org/docs/17/catalog-pg-attrdef.html


-- `pg_attribute`: https://www.postgresql.org/docs/17/catalog-pg-attribute.html


-- `pg_authid`: https://www.postgresql.org/docs/17/catalog-pg-authid.html


-- `pg_auth_members`: https://www.postgresql.org/docs/17/catalog-pg-auth-members.html


-- `pg_cast`: https://www.postgresql.org/docs/17/catalog-pg-cast.html


-- `pg_class`: https://www.postgresql.org/docs/17/catalog-pg-class.html
--! reflect_pg_class : (reltype_schema_name?, reltype_name?, reloftype_schema_name?, reloftype_name?, reltablespace?)
select
	relname::text, -- Str,
	pg_namespace.nspname::text as relnamespace, -- Str,
	reltype_typ_sch.nspname::text as reltype_schema_name, reltype_typ.typname::text as reltype_name, -- Option<Ref>,
	reloftype_typ_sch.nspname::text as reloftype_schema_name, reloftype_typ.typname::text as reloftype_name, -- Option<Ref>,
	pg_get_userbyid(relowner)::text as relowner, -- Str,
	-- relam, -- Option<Str>,
	pg_tablespace.spcname::text as reltablespace, -- Option<Str>,
	relisshared, -- bool,
	relpersistence, -- ClassPersistence,
	relkind, -- ClassKind,
	relrowsecurity, -- bool,
	relforcerowsecurity, -- bool,
	relispartition, -- bool,
	relacl::text[] as relacl, -- Vec<AclItem>,
	reloptions::text[] -- Vec<Str>,
	-- relpartbound -- Option<Str>,
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


