alter database devdb set search_path = noway;
-- alter role devuser set search_path = without_db;
-- alter role devuser in database devdb set search_path = with_db;

select string_to_array(substring(setting from 13), ',') as search_path
from
	pg_catalog.pg_db_role_setting cross join lateral unnest(pg_db_role_setting.setconfig) as s(setting)
	join pg_catalog.pg_database on pg_db_role_setting.setdatabase = pg_database.oid
where
	starts_with(setting, 'search_path=')
	and pg_database.datname = current_database()
	and pg_db_role_setting.setrole = 0
;

select
	pg_roles.rolname::text as name,
	rolsuper, rolinherit, rolcreaterole, rolcreatedb, rolcanlogin, rolreplication, /*rolconnlimit,*/ rolvaliduntil, rolbypassrls,
	coalesce(array_agg(string_to_array(substring(s.setting from 13), ',')) filter (where s.setting is not null), '{}') as search_path,
	coalesce(array_agg(pg_database.datname) filter (where s.setting is not null), '{}') as database_names

from
	pg_catalog.pg_roles
	left join pg_catalog.pg_db_role_setting on pg_roles.oid = pg_db_role_setting.setrole
	left join lateral unnest(pg_db_role_setting.setconfig) as s(setting) on true
	left join pg_catalog.pg_database on pg_db_role_setting.setdatabase = pg_database.oid
where
	pg_roles.rolname not in ('pg_database_owner', 'pg_read_all_data', 'pg_write_all_data', 'pg_monitor', 'pg_read_all_settings', 'pg_read_all_stats', 'pg_stat_scan_tables', 'pg_read_server_files', 'pg_write_server_files', 'pg_execute_server_program', 'pg_signal_backend', 'pg_checkpoint', 'pg_maintain', 'pg_use_reserved_connections', 'pg_create_subscription')
	and (pg_database.datname is null or pg_database.datname = current_database())
	and (setting is null or starts_with(setting, 'search_path='))
group by rolname, rolsuper, rolinherit, rolcreaterole, rolcreatedb, rolcanlogin, rolreplication, /*rolconnlimit,*/ rolvaliduntil, rolbypassrls
;
