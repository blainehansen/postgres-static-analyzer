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

-- SHOW search_path ;

-- "$user",public

