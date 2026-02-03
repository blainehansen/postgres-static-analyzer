drop table if exists aaa;

create table aaa (
	id int primary key,
	hey bool default ('hey there' is null),
	yo text not null,
	hmm uuid,
	stuff char[]
);

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
