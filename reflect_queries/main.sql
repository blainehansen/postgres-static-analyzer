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


--! reflect_user_tables : (conname?, primary_key_columns?)
select
	sch.nspname::text,
	tab.relname::text,
	con.conname::text,
	array_agg(col.attname::text order by col.attnum) filter (where con.contype = 'p') as primary_key_columns
from
	pg_catalog.pg_class as tab
	join pg_catalog.pg_namespace as sch on tab.relnamespace = sch.oid
	left join pg_catalog.pg_constraint as con on con.conrelid = tab.oid and con.contype = 'p'
	left join pg_catalog.pg_attribute as col on col.attrelid = tab.oid and col.attnum = ANY(con.conkey)
where
	tab.relkind = 'r'
	and sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
group by sch.oid, tab.oid, con.oid
;


--! reflect_user_table_columns : (attdef?)
select
	sch.nspname::text, tab.relname::text,
	col.attname::text,
	col.attnotnull,
	case when col.atthasdef then pg_get_expr(col_def.adbin, col_def.adrelid) else null end as attdef,
	col.attgenerated,
	typ_sch.nspname::text as typ_nspname, typ.typname::text
from
	pg_catalog.pg_attribute as col
	join pg_catalog.pg_class as tab on col.attrelid = tab.oid
	join pg_catalog.pg_namespace as sch on tab.relnamespace = sch.oid
	join pg_catalog.pg_type as typ on col.atttypid = typ.oid
	join pg_catalog.pg_namespace as typ_sch on typ.typnamespace = typ_sch.oid
	left join pg_catalog.pg_attrdef as col_def on tab.oid = col_def.adrelid and col.attnum = col_def.adnum
where
	tab.relkind = 'r'
	and sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
	and not col.attisdropped
	and col.attnum > 0
;


--! reflect_user_table_unique_constraints
select
	sch.nspname::text, tab.relname::text, con.conname::text,
	array_agg(col.attname::text order by col.attnum) as unique_columns
from
	pg_catalog.pg_constraint as con
	join pg_catalog.pg_class as tab on tab.oid = con.conrelid
	join pg_catalog.pg_namespace as sch on tab.relnamespace = sch.oid
	join pg_catalog.pg_attribute as col on col.attrelid = con.conrelid and col.attnum = any(con.conkey)
where
	tab.relkind = 'r'
	and con.contype = 'u'
	and sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
	and not col.attisdropped
	and col.attnum > 0
group by sch.nspname, tab.relname, con.conname
;


-- https://www.postgresql.org/docs/17/catalog-pg-constraint.html
--! reflect_foreign_keys
select
	con.conname::text,
	referring_sch.nspname::text as referring_schema,
	referring_tab.relname::text as referring_table,
	array_agg(referring_col.attname::text order by referring_tup.ordinality) as referring_columns,
	referred_sch.nspname::text as referred_schema,
	referred_tab.relname::text as referred_table,
	array_agg(referred_col.attname::text order by referred_tup.ordinality) as referred_columns
from
	pg_constraint as con
	join pg_class as referring_tab on con.conrelid = referring_tab.oid
	join pg_namespace as referring_sch on referring_tab.relnamespace = referring_sch.oid
	join pg_class as referred_tab on con.confrelid = referred_tab.oid
	join pg_namespace as referred_sch on referred_tab.relnamespace = referred_sch.oid
	cross join lateral unnest(con.conkey) with ordinality as referring_tup(attnum, ordinality)
	cross join lateral unnest(con.confkey) with ordinality as referred_tup(attnum, ordinality)
	join pg_attribute as referring_col on referring_col.attrelid = referring_tab.oid and referring_col.attnum = referring_tup.attnum
	join pg_attribute as referred_col on referred_col.attrelid = referred_tab.oid and referred_col.attnum = referred_tup.attnum
where
	con.contype = 'f' and referring_tup.ordinality = referred_tup.ordinality
	and referring_sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
	and referred_sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
group by con.oid, referring_sch.oid, referring_tab.oid, referred_sch.oid, referred_tab.oid
;


--! reflect_composite_types
select
	sch.nspname::text, typ.typname::text,
	array_agg(col.attnum order by col.attnum) as field_nums,
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


-- https://www.postgresql.org/docs/current/catalog-pg-type.html
-- https://www.postgresql.org/docs/current/catalog-pg-enum.html
--! reflect_enum_types
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
group by sch.oid, typ.oid
;
