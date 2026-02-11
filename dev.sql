drop type if exists aaa cascade;
drop table if exists bbb cascade;

create type aaa as (aaa_a int, aaa_b bool, aaa_c text[], aaa_d text);
create table bbb (bbb_a int not null);



select
	sch.nspname::text, typ.typname::text,
	array_agg(col.attname::text order by col.attnum) as field_names,
	array_agg(col_sch.nspname::text order by col.attnum) as field_typ_schemas,
	array_agg(col_typ.typname::text order by col.attnum) as field_typs
from
	pg_catalog.pg_type as typ
	join pg_catalog.pg_namespace as sch on sch.oid = typ.typnamespace
	join pg_catalog.pg_class as tab on tab.oid = typ.typrelid
	join pg_catalog.pg_attribute as col on col.attrelid = tab.oid
	join pg_catalog.pg_type as col_typ on col_typ.oid = col.atttypid
	join pg_catalog.pg_namespace as col_sch on col_sch.oid = col_typ.typnamespace
where
	typ.typtype = 'c'
	and sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
	and col.attnum > 0
	and not col.attisdropped
group by sch.oid, typ.oid, tab.oid
;

drop table bbb cascade;
alter type aaa rename attribute aaa_a to yo_a;
alter type aaa add attribute yo_whoa date;
alter type aaa drop attribute aaa_b;
alter type aaa alter attribute aaa_c type timestamp;


select
	sch.nspname::text, typ.typname::text,
	array_agg(col.attname::text order by col.attnum) as field_names,
	array_agg(col_sch.nspname::text order by col.attnum) as field_typ_schemas,
	array_agg(col_typ.typname::text order by col.attnum) as field_typs
from
	pg_catalog.pg_type as typ
	join pg_catalog.pg_namespace as sch on sch.oid = typ.typnamespace
	join pg_catalog.pg_class as tab on tab.oid = typ.typrelid
	join pg_catalog.pg_attribute as col on col.attrelid = tab.oid
	join pg_catalog.pg_type as col_typ on col_typ.oid = col.atttypid
	join pg_catalog.pg_namespace as col_sch on col_sch.oid = col_typ.typnamespace
where
	typ.typtype = 'c'
	and sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
	and col.attnum > 0
	and not col.attisdropped
group by sch.oid, typ.oid, tab.oid
;
