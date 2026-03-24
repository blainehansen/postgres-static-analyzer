set search_path = '';

create function pg_temp.format_role_oid_array(role_oids oid[]) returns text[] as $$
	begin
		return array (
			-- select case when ${name} = 0 then null else pg_get_userbyid(${name})::text end
			-- pg_catalog.format_type(p.arg_type_oid, null)
			select case when role_oid = 0 then null else pg_get_userbyid(role_oid)::text end
			from unnest(role_oids)
				with ordinality as p(role_oid, ordinality)
			order by ordinality
		);
	end;
$$ language plpgsql immutable;

create function pg_temp.format_fn_defaults(fn pg_proc) returns text[] as $$
	declare
		numallargs integer;
	begin
		numallargs := coalesce(array_length(fn.proallargtypes, 1), fn.pronargs);
		return array (
			select pg_get_function_arg_default(fn.oid, default_number)
			from generate_series(1, numallargs) as p(default_number)
			order by default_number
		);
	end;
$$ language plpgsql immutable;
