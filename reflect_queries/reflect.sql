-- `pg_aggregate`: https://www.postgresql.org/docs/17/catalog-pg-aggregate.html

-- `pg_am`: https://www.postgresql.org/docs/17/catalog-pg-am.html

-- `pg_amop`: https://www.postgresql.org/docs/17/catalog-pg-amop.html

-- `pg_amproc`: https://www.postgresql.org/docs/17/catalog-pg-amproc.html

-- `pg_attrdef`: https://www.postgresql.org/docs/17/catalog-pg-attrdef.html

-- `pg_attribute`: https://www.postgresql.org/docs/17/catalog-pg-attribute.html

-- `pg_authid`: https://www.postgresql.org/docs/17/catalog-pg-authid.html
-- `pg_roles`: https://www.postgresql.org/docs/17/view-pg-roles.html

-- `pg_auth_members`: https://www.postgresql.org/docs/17/catalog-pg-auth-members.html

-- `pg_cast`: https://www.postgresql.org/docs/17/catalog-pg-cast.html

-- `pg_class`: https://www.postgresql.org/docs/17/catalog-pg-class.html

-- `pg_collation`: https://www.postgresql.org/docs/17/catalog-pg-collation.html

-- `pg_constraint`: https://www.postgresql.org/docs/17/catalog-pg-constraint.html

-- `pg_conversion`: https://www.postgresql.org/docs/17/catalog-pg-conversion.html

-- `pg_database`: https://www.postgresql.org/docs/17/catalog-pg-database.html

-- `pg_db_role_setting`: https://www.postgresql.org/docs/17/catalog-pg-db-role-setting.html
--! reflect_pg_db_role_setting : (setrole?, setconfig?)
select
	setdatabase != 0 as setdatabase, -- oid (references pg_database.oid) The OID of the database the setting is applicable to, or zero if not database-specific,
	case when setrole = 0 then null else pg_get_userbyid(setrole)::text end as setrole, -- oid (references pg_authid.oid) The OID of the role the setting is applicable to, or zero if not role-specific
	setconfig -- text[]  Defaults for run-time configuration variables
from
	pg_db_role_setting
where (setdatabase = 0 or setdatabase = (select oid from pg_database where datname = current_database()))
;

-- `pg_default_acl`: https://www.postgresql.org/docs/17/catalog-pg-default-acl.html

-- `pg_depend`: https://www.postgresql.org/docs/17/catalog-pg-depend.html

-- `pg_description`: https://www.postgresql.org/docs/17/catalog-pg-description.html

-- `pg_enum`: https://www.postgresql.org/docs/17/catalog-pg-enum.html
--! reflect_pg_enum : ()
select
	-- oid oid  Row identifier
	-- enumtypid oid (references pg_type.oid) The OID of the pg_type entry owning this enum value,
	-- enumsortorder float4  The sort position of this enum value within its enum type,
	-- enumlabel name  The textual label for this enum value
	enumtypid::regtype::text as enumtypid,
	array_agg(enumlabel::text order by enumsortorder) as enumlabels
from
	pg_enum
group by enumtypid
;

-- `pg_event_trigger`: https://www.postgresql.org/docs/17/catalog-pg-event-trigger.html

-- `pg_extension`: https://www.postgresql.org/docs/17/catalog-pg-extension.html

-- `pg_foreign_data_wrapper`: https://www.postgresql.org/docs/17/catalog-pg-foreign-data-wrapper.html

-- `pg_foreign_server`: https://www.postgresql.org/docs/17/catalog-pg-foreign-server.html

-- `pg_foreign_table`: https://www.postgresql.org/docs/17/catalog-pg-foreign-table.html

-- `pg_index`: https://www.postgresql.org/docs/17/catalog-pg-index.html

-- `pg_inherits`: https://www.postgresql.org/docs/17/catalog-pg-inherits.html

-- `pg_init_privs`: https://www.postgresql.org/docs/17/catalog-pg-init-privs.html

-- `pg_language`: https://www.postgresql.org/docs/17/catalog-pg-language.html

-- `pg_namespace`: https://www.postgresql.org/docs/17/catalog-pg-namespace.html

-- `pg_opclass`: https://www.postgresql.org/docs/17/catalog-pg-opclass.html

-- `pg_operator`: https://www.postgresql.org/docs/17/catalog-pg-operator.html

-- `pg_opfamily`: https://www.postgresql.org/docs/17/catalog-pg-opfamily.html

-- `pg_parameter_acl`: https://www.postgresql.org/docs/17/catalog-pg-parameter-acl.html

-- `pg_partitioned_table`: https://www.postgresql.org/docs/17/catalog-pg-partitioned-table.html

-- `pg_policy`: https://www.postgresql.org/docs/17/catalog-pg-policy.html

-- `pg_proc`: https://www.postgresql.org/docs/17/catalog-pg-proc.html
--! reflect_pg_proc : (procost?, prorows?, provariadic?, prosupport?, proallargtypes?, proargmodes?, proargnames?, proargdefaults?[?], protrftypes?, prosrc?, probin?, prosqlbody?, proconfig?, proacl?, description?)
select
	pg_proc.oid::regprocedure::text as oid, -- oid  Row identifier
	pg_proc.proname::text as proname, -- name  Name of the function
	pg_proc.pronamespace::regnamespace::text as pronamespace, -- oid (references pg_namespace.oid) The OID of the namespace that contains this function
	pg_get_userbyid(proowner)::text as proowner, -- oid (references pg_authid.oid) Owner of the function
	prolang_pg_language.lanname::text as prolang, -- oid (references pg_language.oid) Implementation language or call interface of this function
	pg_proc.procost as procost, -- float4  Estimated execution cost (in units of cpu_operator_cost); if proretset, this is cost per row returned
	pg_proc.prorows as prorows, -- float4  Estimated number of result rows (zero if not proretset)
	case when pg_proc.provariadic = 0 then null else pg_proc.provariadic::regtype::text end as provariadic, -- oid (references pg_type.oid) Data type of the variadic array parameter's elements, or zero if the function does not have a variadic parameter
	case when prosupport = 0 then null else prosupport::regproc::text end as prosupport, -- regproc (references pg_proc.oid) Planner support function for this function (see Section 36.11), or zero if none
	pg_proc.prokind as prokind, -- char  f for a normal function, p for a procedure, a for an aggregate function, or w for a window function
	pg_proc.prosecdef as prosecdef, -- bool  Function is a security definer (i.e., a “setuid” function)
	pg_proc.proleakproof as proleakproof, -- bool  The function has no side effects. No information about the arguments is conveyed except via the return value. Any function that might throw an error depending on the values of its arguments is not leak-proof.
	pg_proc.proisstrict as proisstrict, -- bool  Function returns null if any call argument is null. In that case the function won't actually be called at all. Functions that are not “strict” must be prepared to handle null inputs.
	pg_proc.proretset as proretset, -- bool  Function returns a set (i.e., multiple values of the specified data type)
	pg_proc.provolatile as provolatile, -- char  provolatile tells whether the function's result depends only on its input arguments, or is affected by outside factors. It is i for “immutable” functions, which always deliver the same result for the same inputs. It is s for “stable” functions, whose results (for fixed inputs) do not change within a scan. It is v for “volatile” functions, whose results might change at any time. (Use v also for functions with side-effects, so that calls to them cannot get optimized away.)
	pg_proc.proparallel as proparallel, -- char  proparallel tells whether the function can be safely run in parallel mode. It is s for functions which are safe to run in parallel mode without restriction. It is r for functions which can be run in parallel mode, but their execution is restricted to the parallel group leader; parallel worker processes cannot invoke these functions. It is u for functions which are unsafe in parallel mode; the presence of such a function forces a serial execution plan.
	pg_proc.pronargs as pronargs, -- int2  Number of input arguments
	pg_proc.pronargdefaults as pronargdefaults, -- int2  Number of arguments that have defaults
	pg_proc.prorettype::regtype::text as prorettype, -- oid (references pg_type.oid) Data type of the return value
	pg_proc.proargtypes::regtype[]::text[] as proargtypes,-- oidvector (references pg_type.oid) An array of the data types of the function arguments. This includes only input arguments (including INOUT and VARIADIC arguments), and thus represents the call signature of the function.
	pg_proc.proallargtypes::regtype[]::text[] as proallargtypes, -- oid[] (references pg_type.oid) An array of the data types of the function arguments. This includes all arguments (including OUT and INOUT arguments); however, if all the arguments are IN arguments, this field will be null. Note that subscripting is 1-based, whereas for historical reasons proargtypes is subscripted from 0.
	pg_proc.proargmodes as proargmodes,-- char[]  An array of the modes of the function arguments, encoded as i for IN arguments, o for OUT arguments, b for INOUT arguments, v for VARIADIC arguments, t for TABLE arguments. If all the arguments are IN arguments, this field will be null. Note that subscripts correspond to positions of proallargtypes not proargtypes.
	pg_proc.proargnames as proargnames, -- text[]  An array of the names of the function arguments. Arguments without a name are set to empty strings in the array. If none of the arguments have a name, this field will be null. Note that subscripts correspond to positions of proallargtypes not proargtypes.
	case when pg_proc.proargdefaults is null then null else pg_temp.format_fn_defaults(pg_proc) end as proargdefaults, -- pg_node_tree  Expression trees (in nodeToString() representation) for default values. This is a list with pronargdefaults elements, corresponding to the last N input arguments (i.e., the last N proargtypes positions). If none of the arguments have defaults, this field will be null.
	pg_proc.protrftypes::regtype[]::text[] as protrftypes, -- oid[] (references pg_type.oid) An array of the argument/result data type(s) for which to apply transforms (from the function's TRANSFORM clause). Null if none.
	case when pg_proc.prosrc = '' then null else pg_proc.prosrc end as prosrc, -- text  This tells the function handler how to invoke the function. It might be the actual source code of the function for interpreted languages, a link symbol, a file name, or just about anything else, depending on the implementation language/call convention.
	case when pg_proc.probin = '' then null else pg_proc.probin end as probin, -- text  Additional information about how to invoke the function. Again, the interpretation is language-specific.
	pg_get_function_sqlbody(pg_proc.oid) as prosqlbody, -- pg_node_tree  Pre-parsed SQL function body. This is used for SQL-language functions when the body is given in SQL-standard notation rather than as a string literal. It's null in other cases.
	pg_proc.proconfig as proconfig, -- text[]  Function's local settings for run-time configuration variables
	proacl::text[] as proacl, -- aclitem[]  Access privileges; see Section 5.8 for details
	pg_description.description as description -- text   The comment from pg_description
from
	pg_proc
	join pg_language as prolang_pg_language on pg_proc.prolang = prolang_pg_language.oid
	left join pg_description on pg_description.objoid = pg_proc.oid and pg_description.objsubid = 0
where not starts_with(pg_proc.pronamespace::regnamespace::text, 'pg_temp')
;

-- `pg_publication`: https://www.postgresql.org/docs/17/catalog-pg-publication.html

-- `pg_publication_namespace`: https://www.postgresql.org/docs/17/catalog-pg-publication-namespace.html

-- `pg_range`: https://www.postgresql.org/docs/17/catalog-pg-range.html

-- `pg_replication_origin`: https://www.postgresql.org/docs/17/catalog-pg-replication-origin.html

-- `pg_rewrite`: https://www.postgresql.org/docs/17/catalog-pg-rewrite.html

-- `pg_seclabel`: https://www.postgresql.org/docs/17/catalog-pg-seclabel.html

-- `pg_sequence`: https://www.postgresql.org/docs/17/catalog-pg-sequence.html

-- `pg_shdepend`: https://www.postgresql.org/docs/17/catalog-pg-shdepend.html

-- `pg_shdescription`: https://www.postgresql.org/docs/17/catalog-pg-shdescription.html

-- `pg_shseclabel`: https://www.postgresql.org/docs/17/catalog-pg-shseclabel.html

-- `pg_statistic_ext`: https://www.postgresql.org/docs/17/catalog-pg-statistic-ext.html

-- `pg_subscription`: https://www.postgresql.org/docs/17/catalog-pg-subscription.html

-- `pg_transform`: https://www.postgresql.org/docs/17/catalog-pg-transform.html

-- `pg_trigger`: https://www.postgresql.org/docs/17/catalog-pg-trigger.html

-- `pg_ts_config`: https://www.postgresql.org/docs/17/catalog-pg-ts-config.html

-- `pg_ts_config_map`: https://www.postgresql.org/docs/17/catalog-pg-ts-config-map.html

-- `pg_ts_dict`: https://www.postgresql.org/docs/17/catalog-pg-ts-dict.html

-- `pg_ts_parser`: https://www.postgresql.org/docs/17/catalog-pg-ts-parser.html

-- `pg_ts_template`: https://www.postgresql.org/docs/17/catalog-pg-ts-template.html

-- `pg_type`: https://www.postgresql.org/docs/17/catalog-pg-type.html

-- `pg_user_mappings`: https://www.postgresql.org/docs/17/view-pg-user-mappings.html
