drop type if exists aaa cascade;
drop type if exists bbb cascade;

create type aaa as enum ('a', 'b', 'c');
create type bbb as enum ();

select
	sch.nspname::text, typ.typname::text,
	coalesce(array_agg(enu.enumlabel::text order by enu.enumsortorder) filter (where enu.enumlabel is not null), '{}') as enum_values
from
	pg_catalog.pg_type as typ
	left join pg_catalog.pg_enum as enu on typ.oid = enu.enumtypid
	join pg_catalog.pg_namespace as sch on sch.oid = typ.typnamespace
where
	typ.typtype = 'e'
	and sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
group by sch.oid, typ.oid;
