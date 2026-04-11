-- ====================================================================
-- 1. CLUSTER-LEVEL OBJECTS
-- Populates: pg_database, pg_authid (pg_roles), pg_auth_members,
--            pg_db_role_setting, pg_parameter_acl, pg_shdepend
-- ====================================================================

CREATE ROLE catalog_admin LOGIN PASSWORD 'super_secret';
CREATE ROLE catalog_user  NOLOGIN;
GRANT catalog_user TO catalog_admin;               -- pg_auth_members

ALTER DATABASE tempdb SET work_mem = '12MB'; -- pg_db_role_setting
ALTER ROLE catalog_admin SET work_mem = '16MB'; -- pg_db_role_setting
ALTER ROLE catalog_admin IN DATABASE tempdb SET work_mem = '14MB'; -- pg_db_role_setting

GRANT SET ON PARAMETER work_mem TO catalog_admin;  -- pg_parameter_acl

-- ====================================================================
-- 2. SCHEMAS AND EXTENSIONS
-- Populates: pg_namespace, pg_extension, pg_init_privs, pg_language
-- ====================================================================

CREATE EXTENSION IF NOT EXISTS pageinspect;          -- pg_extension, pg_init_privs
REVOKE ALL ON FUNCTION get_raw_page(text, bigint) FROM PUBLIC; -- uses default search_path

CREATE EXTENSION IF NOT EXISTS postgres_fdw;         -- needed in section 10

CREATE SCHEMA my_schema AUTHORIZATION catalog_admin; -- pg_namespace

SET search_path TO my_schema, public;

-- ====================================================================
-- 3. TYPES, ENUMS, RANGES, COLLATIONS
-- Populates: pg_type, pg_enum, pg_range, pg_collation
-- ====================================================================

CREATE TYPE status_enum AS ENUM ('active', 'pending', 'archived'); -- pg_enum

CREATE TYPE point_composite AS (x numeric, y numeric);             -- pg_type (composite)
CREATE DOMAIN point_composite_safe AS point_composite
	CONSTRAINT con_point_composite_safe CHECK ((VALUE).x IS NOT NULL AND (VALUE).y IS NOT NULL);

CREATE TYPE float_range AS RANGE (                                 -- pg_range
	subtype      = float8,
	subtype_diff = float8mi
);

CREATE COLLATION custom_collation (                                -- pg_collation
	PROVIDER = icu,
	LOCALE   = 'en-US-u-ks-level2'
);

-- ====================================================================
-- 4. SEQUENCES AND TABLES
-- Populates: pg_class, pg_attribute, pg_attrdef, pg_constraint,
--            pg_sequence, pg_inherits, pg_partitioned_table
-- ====================================================================

CREATE SEQUENCE table_id_seq START WITH 1 INCREMENT BY 1;          -- pg_sequence
GRANT USAGE ON SEQUENCE table_id_seq TO catalog_user;

CREATE TABLE parent_table (
	id            integer      DEFAULT nextval('table_id_seq') PRIMARY KEY, -- pg_attrdef, pg_constraint (PK)
	name          text         COLLATE custom_collation NOT NULL,
	status        status_enum  DEFAULT 'pending',                           -- pg_attrdef
	metrics       point_composite,
	active_period float_range,
	score         numeric      CHECK (score >= 0),                          -- pg_constraint (CHECK)
	score_text text GENERATED ALWAYS AS (score::text) STORED,
	CONSTRAINT unique_name UNIQUE (name)                                    -- pg_constraint (UNIQUE)
);

CREATE TABLE child_table (
	extra_data jsonb
) INHERITS (parent_table);                                         -- pg_inherits

CREATE TABLE partitioned_log (
	log_id   integer,
	log_date date,
	message  text
) PARTITION BY RANGE (log_date);                                   -- pg_partitioned_table

CREATE TABLE partitioned_log_2026
	PARTITION OF partitioned_log
	FOR VALUES FROM ('2026-01-01') TO ('2027-01-01');

CREATE TABLE audit_log (
	audit_id  integer PRIMARY KEY,
	table_ref integer REFERENCES parent_table(id),                 -- pg_constraint (FK)
	action    text
);

-- ====================================================================
-- 5. INDEXES AND EXTENDED STATISTICS
-- Populates: pg_index, pg_statistic_ext
-- ====================================================================

CREATE INDEX parent_name_btree_idx ON parent_table USING btree (name);       -- pg_index

CREATE INDEX parent_name_tsvec_idx ON parent_table
	USING gin (to_tsvector('english'::regconfig, name));

CREATE INDEX parent_status_hash_idx ON parent_table USING hash (status);

CREATE STATISTICS parent_table_stats (dependencies, ndistinct)
	ON status, active_period FROM parent_table;                    -- pg_statistic_ext

-- ====================================================================
-- 6. HELPER FUNCTIONS FOR CUSTOM TYPE (point_composite)
-- These must exist before operators and operator classes reference them.
-- ====================================================================

CREATE FUNCTION point_composite_lt(point_composite, point_composite) RETURNS boolean
LANGUAGE sql IMMUTABLE AS $$
	SELECT $1.x < $2.x OR ($1.x = $2.x AND $1.y < $2.y);
$$;

CREATE FUNCTION point_composite_gt(point_composite, point_composite) RETURNS boolean
LANGUAGE sql IMMUTABLE AS $$
	SELECT $1.x > $2.x OR ($1.x = $2.x AND $1.y > $2.y);
$$;

CREATE FUNCTION point_composite_eq(point_composite, point_composite) RETURNS boolean
LANGUAGE sql IMMUTABLE AS $$
	SELECT $1.x = $2.x AND $1.y = $2.y;
$$;

-- Support function 1 for btree: comparison returning -1 / 0 / 1
CREATE FUNCTION point_composite_cmp(point_composite, point_composite) RETURNS integer
LANGUAGE sql IMMUTABLE AS $$
	SELECT CASE
		WHEN $1.x < $2.x THEN -1
		WHEN $1.x > $2.x THEN  1
		WHEN $1.y < $2.y THEN -1
		WHEN $1.y > $2.y THEN  1
		ELSE 0
	END;
$$;

CREATE FUNCTION point_composite_add(point_composite, point_composite) RETURNS point_composite
LANGUAGE sql IMMUTABLE AS $$
	SELECT ($1.x + $2.x, $1.y + $2.y)::point_composite;
$$;

-- ====================================================================
-- 7. OPERATORS
-- Populates: pg_operator
-- ====================================================================

CREATE OPERATOR < (
	LEFTARG    = point_composite,
	RIGHTARG   = point_composite,
	FUNCTION   = point_composite_lt,
	COMMUTATOR = >
);

CREATE OPERATOR > (
	LEFTARG    = point_composite,
	RIGHTARG   = point_composite,
	FUNCTION   = point_composite_gt,
	COMMUTATOR = <
);

CREATE OPERATOR = (
	LEFTARG    = point_composite,
	RIGHTARG   = point_composite,
	FUNCTION   = point_composite_eq,
	COMMUTATOR = =
);

-- Custom additive operator (+?)
CREATE OPERATOR +? (
	LEFTARG  = point_composite,
	RIGHTARG = point_composite,
	FUNCTION = point_composite_add
);

-- ====================================================================
-- 8. ACCESS METHODS, OPERATOR FAMILIES, OPERATOR CLASSES
-- Populates: pg_am, pg_opfamily, pg_opclass, pg_amop, pg_amproc
-- ====================================================================

CREATE ACCESS METHOD custom_index_am TYPE INDEX HANDLER bthandler;  -- pg_am

CREATE OPERATOR FAMILY custom_op_family USING btree;                -- pg_opfamily

CREATE OPERATOR CLASS custom_op_class
	DEFAULT FOR TYPE point_composite USING btree FAMILY custom_op_family AS
	OPERATOR 1 <  (point_composite, point_composite),
	OPERATOR 3 =  (point_composite, point_composite),
	FUNCTION 1    point_composite_cmp(point_composite, point_composite);  -- pg_opclass, pg_amop, pg_amproc

-- ====================================================================
-- 9. FUNCTIONS, PROCEDURES, AGGREGATES, CASTS
-- Populates: pg_proc, pg_aggregate, pg_cast
-- ====================================================================

CREATE FUNCTION add_tax(price numeric) RETURNS numeric
LANGUAGE plpgsql AS $$
BEGIN
	RETURN price * 1.20;
END;
$$;

-- A set-returning function
CREATE FUNCTION generate_ids(n integer) RETURNS SETOF integer
LANGUAGE sql STABLE AS $$
	SELECT generate_series(1, n);
$$;

-- Window function wrapper
CREATE FUNCTION running_total(numeric) RETURNS numeric
LANGUAGE sql IMMUTABLE AS $$
	SELECT $1;
$$;

create function add(a integer, b integer = 0) returns integer
	language sql
	immutable
	strict
	return a + b;

create function yo(int = 1) returns table(f1 int, f2 text)
	as $$ select $1, cast($1 as text) || ' is text' $$
	language sql;

create function rec(int, out f1 int, inout f2 text = 'yeah')
	as $$ select $1, cast($1 as text) || ' is text' $$
	language sql;

CREATE AGGREGATE catalog_sum(numeric) (
	SFUNC    = numeric_add,
	STYPE    = numeric,
	INITCOND = '0'
);

-- Ordered-set aggregate (hypothetical rank)
CREATE AGGREGATE first_value_agg(anycompatible) (
	SFUNC    = array_append,
	STYPE    = anycompatiblearray,
	INITCOND = '{}'
);

CREATE PROCEDURE archive_old_records(cutoff_date date)
LANGUAGE plpgsql AS $$
BEGIN
	DELETE FROM parent_table WHERE status = 'archived';
	COMMIT;
END;
$$;

CREATE FUNCTION text_to_point_composite(text) RETURNS point_composite
LANGUAGE sql IMMUTABLE AS $$
	SELECT (0.0, 0.0)::point_composite;
$$;

CREATE CAST (text AS point_composite)
	WITH FUNCTION text_to_point_composite(text) AS ASSIGNMENT;      -- pg_cast

-- ====================================================================
-- 10. FOREIGN DATA WRAPPERS
-- Populates: pg_foreign_data_wrapper, pg_foreign_server,
--            pg_foreign_table, pg_user_mappings
-- ====================================================================

CREATE SERVER ext_server FOREIGN DATA WRAPPER postgres_fdw          -- pg_foreign_server
	OPTIONS (host 'localhost', dbname 'other_db');

CREATE USER MAPPING FOR catalog_admin SERVER ext_server             -- pg_user_mappings
	OPTIONS (user 'remote_usr', password 'remote_pass');

CREATE FOREIGN TABLE ext_table (                                    -- pg_foreign_table
	id   integer,
	data text
) SERVER ext_server OPTIONS (table_name 'remote_table');

-- ====================================================================
-- 11. TEXT SEARCH
-- Populates: pg_ts_config, pg_ts_config_map, pg_ts_dict
--            (pg_ts_parser and pg_ts_template are system-populated;
--             we reference them via the built-in 'default' parser
--             and 'simple' template below)
-- ====================================================================

CREATE TEXT SEARCH DICTIONARY custom_dict (                         -- pg_ts_dict
	TEMPLATE  = simple,                                             -- references pg_ts_template
	STOPWORDS = english
);

CREATE TEXT SEARCH CONFIGURATION custom_ts_conf (                   -- pg_ts_config
	PARSER = default                                                -- references pg_ts_parser
);

ALTER TEXT SEARCH CONFIGURATION custom_ts_conf
	ADD MAPPING FOR asciiword WITH custom_dict;                     -- pg_ts_config_map

-- ====================================================================
-- 12. TRIGGERS, EVENT TRIGGERS, VIEWS, RULES, POLICIES
-- Populates: pg_trigger, pg_event_trigger, pg_rewrite, pg_policy
-- ====================================================================

CREATE FUNCTION audit_trigger_func() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
	RETURN NEW;
END;
$$;

CREATE TRIGGER parent_audit_trig                                    -- pg_trigger
	BEFORE UPDATE ON parent_table
	FOR EACH ROW EXECUTE FUNCTION audit_trigger_func();

-- Statement-level trigger
CREATE TRIGGER parent_statement_trig
	AFTER INSERT ON parent_table
	FOR EACH STATEMENT EXECUTE FUNCTION audit_trigger_func();

CREATE FUNCTION ddl_log_func() RETURNS event_trigger LANGUAGE plpgsql AS $$
BEGIN
	RAISE NOTICE 'DDL event: %', tg_tag;
END;
$$;

CREATE EVENT TRIGGER log_ddl_events                                 -- pg_event_trigger
	ON ddl_command_start EXECUTE FUNCTION ddl_log_func();

CREATE VIEW parent_view AS                                          -- pg_rewrite (view rule)
	SELECT id, name, status FROM parent_table;

create materialized view thing AS                                          -- pg_rewrite (view rule)
	SELECT id, name, status FROM parent_table;

CREATE RULE parent_view_insert AS ON INSERT TO parent_view          -- pg_rewrite (explicit rule)
	DO INSTEAD
	INSERT INTO parent_table (id, name) VALUES (NEW.id, NEW.name);

ALTER TABLE parent_table ENABLE ROW LEVEL SECURITY;

CREATE POLICY active_only_policy ON parent_table                    -- pg_policy
	FOR SELECT TO catalog_user USING (status = 'active');

CREATE POLICY admin_all_policy ON parent_table                      -- pg_policy (second policy)
	FOR ALL TO catalog_admin USING (true) WITH CHECK (true);

-- ====================================================================
-- 13. PUBLICATIONS, SUBSCRIPTIONS, REPLICATION ORIGIN
-- Populates: pg_publication, pg_publication_rel,
--            pg_publication_namespace, pg_subscription,
--            pg_replication_origin
-- ====================================================================

CREATE PUBLICATION test_pub FOR TABLE parent_table where (score > 5);                 -- pg_publication, pg_publication_rel

CREATE PUBLICATION test_schema_pub                                  -- pg_publication_namespace
	FOR TABLES IN SCHEMA my_schema;

CREATE SUBSCRIPTION test_sub                                        -- pg_subscription
	CONNECTION 'host=localhost dbname=other_db'
	PUBLICATION test_pub
	WITH (connect = false);

SELECT pg_replication_origin_create('custom_origin');               -- pg_replication_origin

-- ====================================================================
-- 14. DEFAULT PRIVILEGES, CONVERSIONS, DESCRIPTIONS, DEPENDENCIES, COMMENTS
-- Populates: pg_default_acl, pg_conversion, pg_description, pg_depend
-- ====================================================================

ALTER DEFAULT PRIVILEGES IN SCHEMA my_schema                   -- pg_default_acl
	GRANT SELECT ON TABLES TO catalog_user;

CREATE CONVERSION custom_conv FOR 'UTF8' TO 'LATIN1'               -- pg_conversion
	FROM utf8_to_iso8859_1;

COMMENT ON DATABASE tempdb IS 'COMMENT ON DATABASE tempdb';
COMMENT ON SCHEMA my_schema IS 'COMMENT ON SCHEMA my_schema';
COMMENT ON TABLE parent_table IS 'COMMENT ON TABLE parent_table';
COMMENT ON COLUMN parent_table.status IS 'COMMENT ON COLUMN parent_table.status';
COMMENT ON COLUMN parent_table.active_period IS 'COMMENT ON COLUMN parent_table.active_period';
COMMENT ON FUNCTION add_tax(numeric) IS 'COMMENT ON FUNCTION add_tax(numeric)';
COMMENT ON SEQUENCE table_id_seq IS 'COMMENT ON SEQUENCE table_id_seq';
COMMENT ON VIEW parent_view IS 'COMMENT ON VIEW parent_view';
COMMENT ON TYPE status_enum IS 'COMMENT ON TYPE status_enum';
COMMENT ON TYPE float_range IS 'COMMENT ON TYPE float_range';
COMMENT ON INDEX parent_name_btree_idx IS 'COMMENT ON INDEX parent_name_btree_idx';
COMMENT ON TEXT SEARCH CONFIGURATION custom_ts_conf IS 'COMMENT ON TEXT SEARCH CONFIGURATION custom_ts_conf';
COMMENT ON OPERATOR = (point_composite, point_composite) IS 'COMMENT ON OPERATOR = (point_composite, point_composite)';
COMMENT ON ACCESS METHOD custom_index_am IS 'COMMENT ON ACCESS METHOD custom_index_am';
COMMENT ON AGGREGATE first_value_agg(anycompatible) IS 'COMMENT ON AGGREGATE first_value_agg(anycompatible)';
COMMENT ON CAST (text AS point_composite) IS 'COMMENT ON CAST (text AS point_composite)';
COMMENT ON COLLATION custom_collation IS 'COMMENT ON COLLATION custom_collation';
COMMENT ON COLUMN parent_table.metrics IS 'COMMENT ON COLUMN parent_table.metrics';
COMMENT ON CONSTRAINT unique_name ON parent_table IS 'COMMENT ON CONSTRAINT unique_name ON parent_table';
COMMENT ON CONSTRAINT con_point_composite_safe ON DOMAIN point_composite_safe IS 'COMMENT ON CONSTRAINT con_point_composite_safe ON DOMAIN point_composite_safe';
COMMENT ON CONVERSION custom_conv IS 'COMMENT ON CONVERSION custom_conv';
COMMENT ON DOMAIN point_composite_safe IS 'COMMENT ON DOMAIN point_composite_safe';
COMMENT ON EXTENSION pageinspect IS 'COMMENT ON EXTENSION pageinspect';
COMMENT ON EXTENSION postgres_fdw IS 'COMMENT ON EXTENSION postgres_fdw';
COMMENT ON EVENT TRIGGER log_ddl_events IS 'COMMENT ON EVENT TRIGGER log_ddl_events';
COMMENT ON FOREIGN DATA WRAPPER postgres_fdw IS 'COMMENT ON FOREIGN DATA WRAPPER postgres_fdw';
COMMENT ON FOREIGN TABLE ext_table IS 'COMMENT ON FOREIGN TABLE ext_table';
COMMENT ON FUNCTION generate_ids(integer) IS 'COMMENT ON FUNCTION generate_ids(integer)';
COMMENT ON INDEX parent_status_hash_idx IS 'COMMENT ON INDEX parent_status_hash_idx';
COMMENT ON MATERIALIZED VIEW thing IS 'COMMENT ON MATERIALIZED VIEW thing';
COMMENT ON OPERATOR < (point_composite, point_composite) IS 'COMMENT ON OPERATOR < (point_composite, point_composite)';
COMMENT ON OPERATOR CLASS custom_op_class USING btree IS 'COMMENT ON OPERATOR CLASS custom_op_class USING btree';
COMMENT ON OPERATOR FAMILY custom_op_family USING btree IS 'COMMENT ON OPERATOR FAMILY custom_op_family USING btree';
COMMENT ON POLICY active_only_policy ON parent_table IS 'COMMENT ON POLICY active_only_policy ON parent_table';
COMMENT ON LANGUAGE plpgsql IS 'COMMENT ON LANGUAGE plpgsql';
COMMENT ON PROCEDURE archive_old_records(date) IS 'COMMENT ON PROCEDURE archive_old_records(date)';
COMMENT ON PUBLICATION test_pub IS 'COMMENT ON PUBLICATION test_pub';
COMMENT ON ROLE catalog_admin IS 'COMMENT ON ROLE catalog_admin';
COMMENT ON ROUTINE add(integer, integer) IS 'COMMENT ON ROUTINE add(integer, integer)';
COMMENT ON RULE parent_view_insert ON parent_view IS 'COMMENT ON RULE parent_view_insert ON parent_view';
COMMENT ON SERVER ext_server IS 'COMMENT ON SERVER ext_server';
COMMENT ON STATISTICS parent_table_stats IS 'COMMENT ON STATISTICS parent_table_stats';
COMMENT ON SUBSCRIPTION test_sub IS 'COMMENT ON SUBSCRIPTION test_sub';
COMMENT ON TABLE audit_log IS 'COMMENT ON TABLE audit_log';
COMMENT ON TEXT SEARCH DICTIONARY custom_dict IS 'COMMENT ON TEXT SEARCH DICTIONARY custom_dict';
COMMENT ON TRIGGER parent_audit_trig ON parent_table IS 'COMMENT ON TRIGGER parent_audit_trig ON parent_table';
COMMENT ON TYPE point_composite IS 'COMMENT ON TYPE point_composite';

-- pg_depend is populated implicitly throughout this script whenever any object
-- depends on another (e.g. indexes on tables, columns on types, functions on
-- schemas, triggers on tables, etc.). No explicit INSERT is needed or possible.

-- ====================================================================
-- NOTES ON CATALOG TABLES REQUIRING SPECIAL INFRASTRUCTURE:
--
-- pg_seclabel / pg_shseclabel:
--   Requires a loaded MAC security provider (e.g. SELinux/sepgsql or
--   the dummy_seclabel extension in contrib).  Syntax when available:
--     SECURITY LABEL FOR dummy ON TABLE parent_table IS 'unclassified';
--     SECURITY LABEL FOR dummy ON ROLE  catalog_user IS 'unclassified';
--
-- pg_transform:
--   Requires C-language FROM SQL / TO SQL functions paired with a PL.
--   Syntax when the C functions are available:
--     CREATE TRANSFORM FOR point_composite LANGUAGE plpython3u (
--         FROM SQL WITH FUNCTION <c_from_sql_func>(internal),
--         TO   SQL WITH FUNCTION <c_to_sql_func>(internal, integer)
--     );
--
-- pg_ts_parser / pg_ts_template:
--   These are read-only system catalogs populated by built-in C parsers
--   and templates.  CREATE TEXT SEARCH PARSER / TEMPLATE also require
--   C handler functions; the entries we reference ('default' parser,
--   'simple' template) already satisfy these catalogs non-trivially.
-- ====================================================================


-- TODO put these all in!
-- create schema _NiGhTmArE_$_sChEmA;

-- create role "oh no""... agh!";

-- create schema " bam. ";

-- grant usage on schema " bam. " to "oh no""... agh!";

-- create function " bam. "." terrible .🙃{}"(a integer) returns integer
-- 	language sql immutable strict return a + 1;
