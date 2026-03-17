-- create table a(
-- 	aaa text default ''
-- );

-- create function public.other_add(a integer) returns integer
-- 	language sql stable return a;

-- create type yay as enum('a', 'b');

set search_path = '';
-- show search_path;
-- select current_schemas(true);

-- show search_path;
-- select current_schemas(true);

select *
from pg_catalog.pg_database
where datname = current_database();

-- select
-- 	oid::regtype,
-- 	-- typname,
-- 	-- typnamespace::regnamespace,
-- 	-- typtype,
-- 	typinput::regproc as typinput,
-- 	case when typelem = 0 then null else typelem::regtype end as typelem,
-- 	case when typarray = 0 then null else typarray::regtype end as typarray,
-- 	case when typbasetype = 0 then null else typbasetype::regtype end as typbasetype
-- from pg_type
-- where typnamespace::regnamespace not in ('pg_catalog', 'information_schema', 'pg_toast');


-- -- proallargtypes is oid[] on pg_proc
-- select
-- 	oid::regproc,
-- 	oid::regprocedure,
-- 	proname, *
-- 	-- proallargtypes::regtype[]  as arg_type_names
-- from pg_proc
-- where
-- 	-- proallargtypes is not null
-- 	-- pronamespace::regnamespace not in ('pg_catalog', 'information_schema', 'pg_toast')
-- 	oid::regproc = 'record_in'::regproc
-- ;




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
