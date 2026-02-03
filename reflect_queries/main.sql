--! reflect_db_role_setting : (rolname?)
select
	string_to_array(substring(setting from 13), ',') as search_path,
	pg_roles.rolname::text
from
	pg_catalog.pg_db_role_setting cross join lateral unnest(pg_db_role_setting.setconfig) as s(setting)
	left join pg_catalog.pg_roles on pg_db_role_setting.setrole = pg_roles.oid
	left join pg_catalog.pg_database on pg_db_role_setting.setdatabase = pg_database.oid
where
	starts_with(setting, 'search_path=')
	and (
		pg_database.datname = current_database()
		or pg_db_role_setting.setrole != 0
	)
;


--! reflect_user_schemas
select
	nspname::text
	-- oid
	-- nspowner
	-- nspacl
from pg_catalog.pg_namespace
where pg_namespace.nspname not in ('information_schema', 'pg_catalog', 'pg_toast')
;


--! reflect_user_tables
select
	pg_namespace.nspname::text,
	pg_class.relname::text
from
	pg_catalog.pg_class
	join pg_catalog.pg_namespace on pg_class.relnamespace = pg_namespace.oid
where
	pg_class.relkind = 'r' -- r = ordinary table, i = index, S = sequence, t = TOAST table, v = view, m = materialized view, c = composite type, f = foreign table, p = partitioned table, I = partitioned index
	and pg_namespace.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
	-- and pg_catalog.pg_table_is_visible(pg_class.oid) -- optional: only show tables in the current search path
;


--! reflect_user_table_columns : (attdef?)
select
	pg_namespace.nspname::text, pg_class.relname::text,
	pg_attribute.attname::text,
	pg_attribute.attnotnull,
	case when pg_attribute.atthasdef then pg_get_expr(pg_attrdef.adbin, pg_attrdef.adrelid) else null end as attdef,
	pg_attribute.attgenerated,
	type_pg_namespace.nspname::text as typ_nspname, pg_type.typname::text
from
	pg_catalog.pg_attribute
	join pg_catalog.pg_class on pg_attribute.attrelid = pg_class.oid
	join pg_catalog.pg_namespace on pg_class.relnamespace = pg_namespace.oid
	join pg_catalog.pg_type on pg_attribute.atttypid = pg_type.oid
	join pg_catalog.pg_namespace as type_pg_namespace on pg_type.typnamespace = type_pg_namespace.oid
	left join pg_catalog.pg_attrdef on pg_class.oid = pg_attrdef.adrelid and pg_attribute.attnum = pg_attrdef.adnum
where
	pg_class.relkind = 'r'
	and pg_namespace.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
	and not pg_attribute.attisdropped
	and pg_attribute.attnum > 0
;



-- select
-- 	pg_namespace.nspname::text,
-- 	pg_class.relname::text,
-- 	array(
-- 		select pg_attribute.attname, pg_attribute.attnotnull, pg_attribute.atthasdef, pg_attribute.attgenerated
-- 		from pg_attribute
-- 	) as columns
-- from
-- 	pg_catalog.pg_class
-- 	join pg_catalog.pg_namespace on pg_class.relnamespace = pg_namespace.oid
-- 	join pg_catalog.pg_attribute on pg_class.oid = pg_attribute.attrelid
-- 		and not pg_attribute.attisdropped and pg_attribute.attnum > 0
-- where
-- 	pg_class.relkind = 'r' -- r = ordinary table, i = index, S = sequence, t = TOAST table, v = view, m = materialized view, c = composite type, f = foreign table, p = partitioned table, I = partitioned index
-- 	and pg_namespace.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
-- 	-- and pg_catalog.pg_table_is_visible(pg_class.oid) -- optional: only show tables in the current search path
-- group by pg_namespace.nspname, pg_class.relname
-- ;
