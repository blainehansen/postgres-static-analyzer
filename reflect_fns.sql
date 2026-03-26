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

create function pg_temp.format_pg_collation_oidvector(oids oidvector) returns text[] as $$
	begin
		return array (
			select case when o = 0 then null else o::regcollation::text end
			from unnest(oids) with ordinality as p(o, ordinality)
			order by ordinality
		);
	end;
$$ language plpgsql immutable;

create function pg_temp.format_pg_opclass_oidvector(oids oidvector) returns text[] as $$
	begin
		return array (
			select quote_ident(pg_namespace.nspname) || '.' || quote_ident(pg_opclass.opcname)
			from
				unnest(oids) with ordinality as p(o, ordinality)
				left join pg_opclass on p.o = pg_opclass.oid
				left join pg_namespace on pg_opclass.opcnamespace = pg_namespace.oid
			order by ordinality
		);
	end;
$$ language plpgsql immutable;

