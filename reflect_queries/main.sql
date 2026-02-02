--! reflect_db_role_setting : (rolname?)
select
	string_to_array(substring(setting from 13), ',') as search_path,
	pg_roles.rolname::text
from
	pg_db_role_setting cross join lateral unnest(pg_db_role_setting.setconfig) as s(setting)
	left join pg_roles on pg_db_role_setting.setrole = pg_roles.oid
	left join pg_database on pg_db_role_setting.setdatabase = pg_database.oid
where
	starts_with(setting, 'search_path=')
	and (
		pg_database.datname = current_database()
		or pg_db_role_setting.setrole != 0
	)
;


--! reflect_user_schemas
select
	pg_namespace.nspname::text
	-- oid
	-- nspowner
	-- nspacl
from pg_namespace
where pg_namespace.nspname not in ('information_schema', 'pg_catalog', 'pg_toast')
;


-- SHOW search_path ;

-- "$user",public

