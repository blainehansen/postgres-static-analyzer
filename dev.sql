create or replace function add(a integer = 0, b integer = 0) returns integer
	language sql
	immutable
	strict
	return a + b;


create or replace function dup(int) returns table(f1 int, f2 text)
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
	fn.oid::regprocedure as function_signature,
	fn.proname::text as function_name,
	array_agg(argmode order by arg_position) as arg_modes,
	array_agg(case when argname = '' then null else argname::text end order by arg_position) as arg_names,
	array_agg(typ.typname::text order by arg_position) as arg_types,
	array_agg(typ_sch.nspname::text order by arg_position) as arg_type_schemas,
	array_agg(
		case
			when arg_position > fn.pronargs - fn.pronargdefaults then
				-- TODO I'm afraid of this. Maybe the right thing isn't to actually populate the default expr, but to just give a bool indicating it has one at all? in that case I'd do the same for columns right?
				-- for *diffing* the exact value actually does matter. it doesn't for checking but that's not what we're talking about
				(string_to_array(pg_get_expr(fn.proargdefaults, 0), ', '))[arg_position - (fn.pronargs - fn.pronargdefaults)]
			else null
		end
		order by arg_position
	) as arg_defaults
from
	pg_catalog.pg_proc as fn
	join pg_catalog.pg_namespace as sch on fn.pronamespace = sch.oid
	cross join lateral unnest(
		-- for proallargtypes: An array of the data types of the function arguments. This includes all arguments (including OUT and INOUT arguments); however, if all the arguments are IN arguments, this field will be null. Note that subscripting is 1-based, whereas for historical reasons proargtypes is subscripted from 0.
		-- for proargtypes: An array of the data types of the function arguments. This includes only input arguments (including INOUT and VARIADIC arguments), and thus represents the call signature of the function.
		coalesce(fn.proallargtypes, fn.proargtypes::oid[]),

		-- for proargmodes: An array of the modes of the function arguments, encoded as i for IN arguments, o for OUT arguments, b for INOUT arguments, v for VARIADIC arguments, t for TABLE arguments. If all the arguments are IN arguments, this field will be null. Note that subscripts correspond to positions of proallargtypes not proargtypes.
		coalesce(fn.proargmodes, array_fill('i'::"char", array[fn.pronargs])),

		-- for proargnames: An array of the names of the function arguments. Arguments without a name are set to empty strings in the array. If none of the arguments have a name, this field will be null. Note that subscripts correspond to positions of proallargtypes not proargtypes.
		coalesce(fn.proargnames, array_fill(null::text, array[array_length(coalesce(fn.proallargtypes, fn.proargtypes::oid[]), 1)]))

		-- for proargdefaults: Expression trees (in nodeToString() representation) for default values. This is a list with pronargdefaults elements, corresponding to the last N input arguments (i.e., the last N proargtypes positions). If none of the arguments have defaults, this field will be null.
		-- coalesce(fn.proargnames, array_fill(null::text, array[fn.pronargs]))
	) with ordinality as args(argtype, argmode, argname, arg_position)
	join pg_catalog.pg_type as typ on typ.oid = argtype
	join pg_catalog.pg_namespace as typ_sch on typ_sch.oid = typ.typnamespace
where
	sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
group by fn.oid, fn.proname, fn.pronargs, fn.pronargdefaults, fn.proargdefaults;
