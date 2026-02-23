alter database devdb set search_path = noway;
-- alter role devuser set search_path = without_db,other_without;
-- alter role devuser in database devdb set search_path = with_db,other_with;

select
	pg_roles.rolname::text as name,
	-- rolsuper, rolinherit, rolcreaterole, rolcreatedb, rolcanlogin, rolreplication, /*rolconnlimit,*/ rolvaliduntil, rolbypassrls,
	('{' || global_s.option_value || '}')::text[] as default_search_path,
	('{' || db_s.option_value || '}')::text[] as db_search_path
from
	pg_catalog.pg_roles

	left join pg_catalog.pg_db_role_setting as global_rs on pg_roles.oid = global_rs.setrole and global_rs.setdatabase = 0
	left join lateral pg_options_to_table(global_rs.setconfig) as global_s on global_s.option_name = 'search_path'

	left join pg_catalog.pg_db_role_setting as db_rs on
		pg_roles.oid = db_rs.setrole
		and db_rs.setdatabase = (select oid from pg_catalog.pg_database where datname = current_database())
	left join lateral pg_options_to_table(db_rs.setconfig) as db_s on  db_s.option_name = 'search_path'

where pg_roles.rolname not in ('pg_database_owner', 'pg_read_all_data', 'pg_write_all_data', 'pg_monitor', 'pg_read_all_settings', 'pg_read_all_stats', 'pg_stat_scan_tables', 'pg_read_server_files', 'pg_write_server_files', 'pg_execute_server_program', 'pg_signal_backend', 'pg_checkpoint', 'pg_maintain', 'pg_use_reserved_connections', 'pg_create_subscription')
;

alter database devdb set search_path = noway;
alter role devuser set search_path = "$user";
alter role devuser in database devdb set search_path = with_db,other_with;

select
	pg_roles.rolname::text as name,
	-- rolsuper, rolinherit, rolcreaterole, rolcreatedb, rolcanlogin, rolreplication, /*rolconnlimit,*/ rolvaliduntil, rolbypassrls,
	('{' || global_s.option_value || '}')::text[] as default_search_path,
	('{' || db_s.option_value || '}')::text[] as db_search_path
from
	pg_catalog.pg_roles

	left join pg_catalog.pg_db_role_setting as global_rs on pg_roles.oid = global_rs.setrole and global_rs.setdatabase = 0
	left join lateral pg_options_to_table(global_rs.setconfig) as global_s on global_s.option_name = 'search_path'

	left join pg_catalog.pg_db_role_setting as db_rs on
		pg_roles.oid = db_rs.setrole
		and db_rs.setdatabase = (select oid from pg_catalog.pg_database where datname = current_database())
	left join lateral pg_options_to_table(db_rs.setconfig) as db_s on  db_s.option_name = 'search_path'

where pg_roles.rolname not in ('pg_database_owner', 'pg_read_all_data', 'pg_write_all_data', 'pg_monitor', 'pg_read_all_settings', 'pg_read_all_stats', 'pg_stat_scan_tables', 'pg_read_server_files', 'pg_write_server_files', 'pg_execute_server_program', 'pg_signal_backend', 'pg_checkpoint', 'pg_maintain', 'pg_use_reserved_connections', 'pg_create_subscription')
;
