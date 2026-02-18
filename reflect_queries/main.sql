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


-- https://www.postgresql.org/docs/17/catalog-pg-proc.html
--! reflect_functions : (arg_names[?], arg_defaults[?])
select
	sch.nspname::text, fn.proname::text as function_name,
	fn.prokind, fn.prosecdef, fn.proleakproof, fn.proisstrict, fn.proretset, fn.provolatile,
	fn.prosqlbody is not null as has_sql_body,
	case when fn.prosqlbody is not null then pg_get_function_sqlbody(fn.oid) else fn.prosrc end as body,
	return_typ.typname::text as return_typ_name,
	return_typ_sch.nspname::text as return_typ_schema,
	lang.lanname::text as lang_name,
	array_agg(args.argmode order by args.arg_position) as arg_modes,
	array_agg(case when args.argname = '' then null else args.argname::text end order by args.arg_position) as arg_names,
	array_agg(typ.typname::text order by args.arg_position) as arg_types,
	array_agg(typ_sch.nspname::text order by args.arg_position) as arg_type_schemas,
	array_agg(
		-- not necessarily a stable public function we can use in every version
		pg_get_function_arg_default(fn.oid, args.arg_position::int)
		order by args.arg_position
	) as arg_defaults
from
	pg_catalog.pg_proc as fn
	join pg_catalog.pg_namespace as sch on fn.pronamespace = sch.oid

	join pg_catalog.pg_type as return_typ on fn.prorettype = return_typ.oid
	join pg_catalog.pg_namespace as return_typ_sch on return_typ.typnamespace = return_typ_sch.oid

	join pg_catalog.pg_language as lang on fn.prolang = lang.oid

	cross join lateral unnest(
		-- for proallargtypes: An array of the data types of the function arguments. This includes all arguments (including OUT and INOUT arguments). however, if all the arguments are IN arguments, this field will be null. Note that subscripting is 1-based, whereas for historical reasons proargtypes is subscripted from 0.
		-- for proargtypes: An array of the data types of the function arguments. This includes only input arguments (including INOUT and VARIADIC arguments), and thus represents the call signature of the function.
		coalesce(fn.proallargtypes, fn.proargtypes::oid[]),

		-- for proargmodes: An array of the modes of the function arguments, encoded as i for IN arguments, o for OUT arguments, b for INOUT arguments, v for VARIADIC arguments, t for TABLE arguments. If all the arguments are IN arguments, this field will be null. Note that subscripts correspond to positions of proallargtypes not proargtypes.
		coalesce(fn.proargmodes, array_fill('i'::"char", array[fn.pronargs])),

		-- for proargnames: An array of the names of the function arguments. Arguments without a name are set to empty strings in the array. If none of the arguments have a name, this field will be null. Note that subscripts correspond to positions of proallargtypes not proargtypes.
		coalesce(fn.proargnames, array_fill(null::text, array[array_length(coalesce(fn.proallargtypes, fn.proargtypes::oid[]), 1)]))
	) with ordinality as args(argtype, argmode, argname, arg_position)
	join pg_catalog.pg_type as typ on typ.oid = args.argtype
	join pg_catalog.pg_namespace as typ_sch on typ_sch.oid = typ.typnamespace
where
	sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
group by sch.oid, fn.oid, return_typ.oid, return_typ_sch.oid, lang.oid
;
