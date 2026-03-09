--! reflect_pg_roles : (rolconnlimit?, rolvaliduntil?, rolconfig?)
select
	pg_get_userbyid(oid)::text as rolname, -- name  Role name
	rolsuper, -- bool  Role has superuser privileges
	rolinherit, -- bool  Role automatically inherits privileges of roles it is a member of
	rolcreaterole, -- bool  Role can create more roles
	rolcreatedb, -- bool  Role can create databases
	rolcanlogin, -- bool  Role can log in. That is, this role can be given as the initial session authorization identifier
	rolreplication, -- bool  Role is a replication role. A replication role can initiate replication connections and create and drop replication slots.
	case when rolconnlimit < 0 then null else rolconnlimit end, -- int4  For roles that can log in, this sets maximum number of concurrent connections this role can make. -1 means no limit.
	rolvaliduntil, -- timestamptz  Password expiry time (only used for password authentication); null if no expiration
	rolbypassrls, -- bool  Role bypasses every row-level security policy, see Section 5.9 for more information.
	rolconfig -- text[]  Role-specific defaults for run-time configuration variables
from pg_roles
where pg_get_userbyid(pg_roles.oid) not in ('pg_database_owner', 'pg_read_all_data', 'pg_write_all_data', 'pg_monitor', 'pg_read_all_settings', 'pg_read_all_stats', 'pg_stat_scan_tables', 'pg_read_server_files', 'pg_write_server_files', 'pg_execute_server_program', 'pg_signal_backend', 'pg_checkpoint', 'pg_maintain', 'pg_use_reserved_connections', 'pg_create_subscription')



-- we have a couple layers here:
-- - target_role: Change default privileges for objects created by the target_role, or the current role if unspecified.
-- - schema_name: The name of an existing schema. If specified, the default privileges are altered for objects later created in that schema. If IN SCHEMA is omitted, the global default privileges are altered. IN SCHEMA is not allowed when setting privileges for schemas, since schemas can't be nested.
-- - role_name: The name of an existing role to grant or revoke privileges for. This parameter, and all the other parameters in abbreviated_grant_or_revoke, act as described under GRANT or REVOKE, except that one is setting permissions for a whole class of objects rather than specific named objects.





-- --! reflect_default_acls : (applicable_schema?)
-- select
--  pg_get_userbyid(defaclrole)::text as applicable_object_owner, -- defaclrole specifies the object owner, the person who's owned objects are included in this default privilege
--  sch.nspname::text as applicable_schema,
--  defaclobjtype,
--  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
--  pg_get_userbyid(grantor)::text as grantor,
--  privilege_type,
--  is_grantable

-- from
--  pg_catalog.pg_default_acl cross join lateral aclexplode(defaclacl)
--  left join pg_catalog.pg_namespace as sch on pg_default_acl.defaclnamespace = sch.oid
-- -- group by defaclrole, sch.nspname, defaclobjtype, grantee, grantor
-- ;

-- alter default privileges for devuser revoke all privileges on functions from public;
-- alter default privileges for a revoke all privileges on functions from public;



-- acldefault *computes* the effective privileges. if they don't differ from the "code default"


-- create function add(a integer) returns integer
-- 	language sql stable return a;

-- select
-- 	fn.proname::text, sch.nspname::text,
-- 	array_agg(case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end order by a.ordinality) as grantees,
-- 	array_agg(pg_get_userbyid(grantee)::text order by a.ordinality) as grantees,
-- 	-- array_agg(privilege_type order by a.ordinality) as privilege_types,
-- 	array_agg(is_grantable order by a.ordinality) as is_grantables,
-- 	array_agg(pg_get_userbyid(grantor)::text order by a.ordinality) as grantors
-- from
-- 	pg_catalog.pg_proc as fn
-- 	join pg_catalog.pg_namespace as sch on fn.pronamespace = sch.oid
-- 	cross join lateral aclexplode(fn.proacl) with ordinality as a
-- where sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
-- group by fn.oid, sch.oid
-- ;

-- alter default privileges revoke all privileges on functions from public;

-- create function other_add(a integer) returns integer
-- 	language sql stable return a;


-- select
-- 	fn.proname::text, sch.nspname::text,
-- 	array_agg(case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end order by a.ordinality) as grantees,
-- 	array_agg(pg_get_userbyid(grantee)::text order by a.ordinality) as grantees,
-- 	-- array_agg(privilege_type order by a.ordinality) as privilege_types,
-- 	array_agg(is_grantable order by a.ordinality) as is_grantables,
-- 	array_agg(pg_get_userbyid(grantor)::text order by a.ordinality) as grantors
-- from
-- 	pg_catalog.pg_proc as fn
-- 	join pg_catalog.pg_namespace as sch on fn.pronamespace = sch.oid
-- 	cross join lateral aclexplode(fn.proacl) with ordinality as a
-- where sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
-- group by fn.oid, sch.oid
-- ;
