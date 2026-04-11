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
$$ language plpgsql stable;

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
$$ language plpgsql stable;

create function pg_temp.format_pg_collation_oidvector(oids oidvector) returns text[] as $$
	begin
		return array (
			select case when o = 0 then null else o::regcollation::text end
			from unnest(oids) with ordinality as p(o, ordinality)
			order by ordinality
		);
	end;
$$ language plpgsql stable;

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
$$ language plpgsql stable;



-- DATABASE	CTc	Tc	\l
create type pg_temp.db_aclprivilege as enum('CREATE', 'TEMPORARY', 'CONNECT');

-- -- DOMAIN  U  U  \dD+
-- create type pg_temp.domain_aclprivilege as enum('USAGE');

-- FUNCTION or PROCEDURE	X	X	\df+
create type pg_temp.function_aclprivilege as enum('EXECUTE');

-- FOREIGN DATA WRAPPER	U	none	\dew+
create type pg_temp.foreigndatawrapper_aclprivilege as enum('USAGE');

-- FOREIGN SERVER	U	none	\des+
create type pg_temp.foreignserver_aclprivilege as enum('USAGE');

-- LANGUAGE	U	U	\dL+
create type pg_temp.language_aclprivilege as enum('USAGE');

-- PARAMETER	sA	none	\dconfig+
create type pg_temp.parameter_aclprivilege as enum('SET', 'ALTER SYSTEM');

-- SCHEMA	UC	none	\dn+
create type pg_temp.schema_aclprivilege as enum('USAGE', 'CREATE');

-- -- SEQUENCE	rwU	none	\dp
-- create type pg_temp.sequence_aclprivilege as enum('SELECT', 'UPDATE', 'USAGE');

-- TABLE (and table-like objects)	arwdDxtm	none	\dp
create type pg_temp.table_aclprivilege as enum('INSERT', 'SELECT', 'UPDATE', 'DELETE', 'TRUNCATE', 'REFERENCES', 'TRIGGER', 'MAINTAIN', 'USAGE');

-- Table column	arwx	none	\dp
create type pg_temp.tablecolumn_aclprivilege as enum('INSERT', 'SELECT', 'UPDATE', 'REFERENCES');

-- TYPE	U	U	\dT+
create type pg_temp.type_aclprivilege as enum('USAGE');

create type pg_temp.acldefault_aclprivilege as enum(
	'INSERT', 'SELECT', 'UPDATE', 'DELETE', 'TRUNCATE', 'REFERENCES', 'TRIGGER', 'MAINTAIN',
	'USAGE',
	'EXECUTE',
);


create type pg_temp.db_aclgrant as (
	privilege pg_temp.db_aclprivilege,
	with_grant_option bool
);
create type pg_temp.db_aclitem as (
	grantee text,
	grantor text,
	grants pg_temp.db_aclgrant[]
);
create function pg_temp.format_db_aclitems(acls aclitem[]) returns pg_temp.db_aclitem[] as $$
	begin
		return array (
			select row(
				case when grantee = 0 then 'public' else pg_get_userbyid(grantee) end,
				pg_get_userbyid(grantor),
				array_agg(row(privilege_type::pg_temp.db_aclprivilege, is_grantable)::pg_temp.db_aclgrant)
			)::pg_temp.db_aclitem
			from aclexplode(acls)
			group by grantee, grantor
		);
	end;
$$ language plpgsql immutable;


create type pg_temp.function_aclgrant as (
	privilege pg_temp.function_aclprivilege,
	with_grant_option bool
);
create type pg_temp.function_aclitem as (
	grantee text,
	grantor text,
	grants pg_temp.function_aclgrant[]
);
create function pg_temp.format_function_aclitems(acls aclitem[]) returns pg_temp.function_aclitem[] as $$
	begin
		return array (
			select row(
				case when grantee = 0 then 'public' else pg_get_userbyid(grantee) end,
				pg_get_userbyid(grantor),
				array_agg(row(privilege_type::pg_temp.function_aclprivilege, is_grantable)::pg_temp.function_aclgrant)
			)::pg_temp.function_aclitem
			from aclexplode(acls)
			group by grantee, grantor
		);
	end;
$$ language plpgsql immutable;


create type pg_temp.foreigndatawrapper_aclgrant as (
	privilege pg_temp.foreigndatawrapper_aclprivilege,
	with_grant_option bool
);
create type pg_temp.foreigndatawrapper_aclitem as (
	grantee text,
	grantor text,
	grants pg_temp.foreigndatawrapper_aclgrant[]
);
create function pg_temp.format_foreigndatawrapper_aclitems(acls aclitem[]) returns pg_temp.foreigndatawrapper_aclitem[] as $$
	begin
		return array (
			select row(
				case when grantee = 0 then 'public' else pg_get_userbyid(grantee) end,
				pg_get_userbyid(grantor),
				array_agg(row(privilege_type::pg_temp.foreigndatawrapper_aclprivilege, is_grantable)::pg_temp.foreigndatawrapper_aclgrant)
			)::pg_temp.foreigndatawrapper_aclitem
			from aclexplode(acls)
			group by grantee, grantor
		);
	end;
$$ language plpgsql immutable;


create type pg_temp.foreignserver_aclgrant as (
	privilege pg_temp.foreignserver_aclprivilege,
	with_grant_option bool
);
create type pg_temp.foreignserver_aclitem as (
	grantee text,
	grantor text,
	grants pg_temp.foreignserver_aclgrant[]
);
create function pg_temp.format_foreignserver_aclitems(acls aclitem[]) returns pg_temp.foreignserver_aclitem[] as $$
	begin
		return array (
			select row(
				case when grantee = 0 then 'public' else pg_get_userbyid(grantee) end,
				pg_get_userbyid(grantor),
				array_agg(row(privilege_type::pg_temp.foreignserver_aclprivilege, is_grantable)::pg_temp.foreignserver_aclgrant)
			)::pg_temp.foreignserver_aclitem
			from aclexplode(acls)
			group by grantee, grantor
		);
	end;
$$ language plpgsql immutable;


create type pg_temp.language_aclgrant as (
	privilege pg_temp.language_aclprivilege,
	with_grant_option bool
);
create type pg_temp.language_aclitem as (
	grantee text,
	grantor text,
	grants pg_temp.language_aclgrant[]
);
create function pg_temp.format_language_aclitems(acls aclitem[]) returns pg_temp.language_aclitem[] as $$
	begin
		return array (
			select row(
				case when grantee = 0 then 'public' else pg_get_userbyid(grantee) end,
				pg_get_userbyid(grantor),
				array_agg(row(privilege_type::pg_temp.language_aclprivilege, is_grantable)::pg_temp.language_aclgrant)
			)::pg_temp.language_aclitem
			from aclexplode(acls)
			group by grantee, grantor
		);
	end;
$$ language plpgsql immutable;


create type pg_temp.parameter_aclgrant as (
	privilege pg_temp.parameter_aclprivilege,
	with_grant_option bool
);
create type pg_temp.parameter_aclitem as (
	grantee text,
	grantor text,
	grants pg_temp.parameter_aclgrant[]
);
create function pg_temp.format_parameter_aclitems(acls aclitem[]) returns pg_temp.parameter_aclitem[] as $$
	begin
		return array (
			select row(
				case when grantee = 0 then 'public' else pg_get_userbyid(grantee) end,
				pg_get_userbyid(grantor),
				array_agg(row(privilege_type::pg_temp.parameter_aclprivilege, is_grantable)::pg_temp.parameter_aclgrant)
			)::pg_temp.parameter_aclitem
			from aclexplode(acls)
			group by grantee, grantor
		);
	end;
$$ language plpgsql immutable;


create type pg_temp.schema_aclgrant as (
	privilege pg_temp.schema_aclprivilege,
	with_grant_option bool
);
create type pg_temp.schema_aclitem as (
	grantee text,
	grantor text,
	grants pg_temp.schema_aclgrant[]
);
create function pg_temp.format_schema_aclitems(acls aclitem[]) returns pg_temp.schema_aclitem[] as $$
	begin
		return array (
			select row(
				case when grantee = 0 then 'public' else pg_get_userbyid(grantee) end,
				pg_get_userbyid(grantor),
				array_agg(row(privilege_type::pg_temp.schema_aclprivilege, is_grantable)::pg_temp.schema_aclgrant)
			)::pg_temp.schema_aclitem
			from aclexplode(acls)
			group by grantee, grantor
		);
	end;
$$ language plpgsql immutable;


create type pg_temp.table_aclgrant as (
	privilege pg_temp.table_aclprivilege,
	with_grant_option bool
);
create type pg_temp.table_aclitem as (
	grantee text,
	grantor text,
	grants pg_temp.table_aclgrant[]
);
create function pg_temp.format_table_aclitems(acls aclitem[]) returns pg_temp.table_aclitem[] as $$
	begin
		return array (
			select row(
				case when grantee = 0 then 'public' else pg_get_userbyid(grantee) end,
				pg_get_userbyid(grantor),
				array_agg(row(privilege_type::pg_temp.table_aclprivilege, is_grantable)::pg_temp.table_aclgrant)
			)::pg_temp.table_aclitem
			from aclexplode(acls)
			group by grantee, grantor
		);
	end;
$$ language plpgsql immutable;


create type pg_temp.tablecolumn_aclgrant as (
	privilege pg_temp.tablecolumn_aclprivilege,
	with_grant_option bool
);
create type pg_temp.tablecolumn_aclitem as (
	grantee text,
	grantor text,
	grants pg_temp.tablecolumn_aclgrant[]
);
create function pg_temp.format_tablecolumn_aclitems(acls aclitem[]) returns pg_temp.tablecolumn_aclitem[] as $$
	begin
		return array (
			select row(
				case when grantee = 0 then 'public' else pg_get_userbyid(grantee) end,
				pg_get_userbyid(grantor),
				array_agg(row(privilege_type::pg_temp.tablecolumn_aclprivilege, is_grantable)::pg_temp.tablecolumn_aclgrant)
			)::pg_temp.tablecolumn_aclitem
			from aclexplode(acls)
			group by grantee, grantor
		);
	end;
$$ language plpgsql immutable;


create type pg_temp.type_aclgrant as (
	privilege pg_temp.type_aclprivilege,
	with_grant_option bool
);
create type pg_temp.type_aclitem as (
	grantee text,
	grantor text,
	grants pg_temp.type_aclgrant[]
);
create function pg_temp.format_type_aclitems(acls aclitem[]) returns pg_temp.type_aclitem[] as $$
	begin
		return array (
			select row(
				case when grantee = 0 then 'public' else pg_get_userbyid(grantee) end,
				pg_get_userbyid(grantor),
				array_agg(row(privilege_type::pg_temp.type_aclprivilege, is_grantable)::pg_temp.type_aclgrant)
			)::pg_temp.type_aclitem
			from aclexplode(acls)
			group by grantee, grantor
		);
	end;
$$ language plpgsql immutable;


create type pg_temp.acldefault_aclgrant as (
	privilege pg_temp.acldefault_aclprivilege,
	with_grant_option bool
);
create type pg_temp.acldefault_aclitem as (
	grantee text,
	grantor text,
	grants pg_temp.acldefault_aclgrant[]
);
create function pg_temp.format_acldefault_aclitems(acls aclitem[]) returns pg_temp.acldefault_aclitem[] as $$
	begin
		return array (
			select row(
				case when grantee = 0 then 'public' else pg_get_userbyid(grantee) end,
				pg_get_userbyid(grantor),
				array_agg(row(privilege_type::pg_temp.acldefault_aclprivilege, is_grantable)::pg_temp.acldefault_aclgrant)
			)::pg_temp.acldefault_aclitem
			from aclexplode(acls)
			group by grantee, grantor
		);
	end;
$$ language plpgsql immutable;
