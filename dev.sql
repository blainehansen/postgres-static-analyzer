create or replace function add(a integer = 0, b integer = 0) returns integer
	language sql
	immutable
	strict
	return a + b;


create or replace function dup(int) returns table(f1 int, f2 text)
	as $$ select $1, cast($1 as text) || ' is text' $$
	language sql;

create or replace function dup_agh(int, out f1 int, inout f2 text = 'yeah')
	as $$ select $1, cast($1 as text) || ' is text' $$
	language sql;

create or replace function rec(int, out f1 int, inout f2 text = 'yeah')
	as $$ select $1, cast($1 as text) || ' is text' $$
	language sql;


select
	sch.nspname,

	-- oid,
	proname,
	-- pronamespace,
	-- proowner,
	-- prolang,
	-- procost,
	-- prorows,
	-- provariadic,
	-- prosupport,
	-- prokind,
	-- prosecdef,
	-- proleakproof,
	-- proisstrict,
	proretset,
	-- provolatile,
	-- proparallel,
	pronargs,
	pronargdefaults,
	prorettype,
	proargtypes,
	-- proargdefaults
	pg_get_expr(fn.proargdefaults, 0)
	-- protrftypes,
	-- prosrc,
	-- probin
	-- prosqlbody
	-- proconfig
	-- proacl
	-- oid,
	-- nspname,
	-- nspowner,
	-- nspacl,
	-- nspname
from
	pg_catalog.pg_proc as fn
	join pg_catalog.pg_namespace as sch on fn.pronamespace = sch.oid
where
	sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
;


-- https://www.postgresql.org/docs/17/catalog-pg-proc.html
select
	fn.proname::text as function_name, sch.nspname::text as schema_name,
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
group by fn.oid, return_typ.oid, return_typ_sch.oid, lang.oid
;
