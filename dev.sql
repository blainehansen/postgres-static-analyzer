set search_path = '';
-- show search_path;
-- select current_schemas(true);
-- select current_schemas(false);

select
	pg_rules.schemaname::text as schemaname, -- name (references pg_namespace.nspname) Name of schema containing table
	quote_ident(pg_rules.schemaname) || '.' || quote_ident(pg_rules.tablename) as tablename, -- name (references pg_class.relname) Name of table the rule is for
	pg_rules.rulename::text as rulename, -- name (references pg_rewrite.rulename) Name of rule
	-- pg_rules.definition as definition -- text  Rule definition (a reconstructed creation command)
	pg_description.description
from
	pg_rules
	join pg_rewrite on pg_rewrite.ev_type != '1' and pg_rules.rulename = pg_rewrite.rulename and (quote_ident(pg_rules.schemaname) || '.' || quote_ident(pg_rules.tablename))::regclass::oid = pg_rewrite.ev_class
	left join pg_description on pg_description.objoid = pg_rewrite.oid and pg_description.objsubid = 0
;


-- select oid, rulename, ev_class, ev_type, ev_enabled, is_instead, description
-- from
-- 	pg_rewrite
-- 	left join pg_description on pg_description.objoid = pg_rewrite.oid and pg_description.objsubid = 0

-- where ev_type != '1'
-- -- where rulename = 'parent_view_insert'
-- ;


-- select *
-- from pg_rules
-- ;
