create or replace function add(a integer, b integer = 0) returns integer
	language sql
	immutable
	strict
	return a + b;

create or replace function dup(int) returns table(f1 int, f2 text)
	as $$ select $1, cast($1 as text) || ' is text' $$
	language sql;

create or replace function rec(int, out f1 int, inout f2 text = 'yeah')
	as $$ select $1, cast($1 as text) || ' is text' $$
	language sql;

grant all privileges on function add to devuser;

select
	fn.proname::text, sch.nspname::text,
	array_agg(case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end order by a.ordinality) as grantees,
	array_agg(privilege_type order by a.ordinality) as privilege_types,
	array_agg(is_grantable order by a.ordinality) as is_grantables,
	array_agg(pg_get_userbyid(grantor)::text order by a.ordinality) as grantors
from
	pg_catalog.pg_proc as fn
	join pg_catalog.pg_namespace as sch on fn.pronamespace = sch.oid
	cross join lateral aclexplode(fn.proacl) with ordinality as a
where sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
group by fn.oid, sch.oid
;
