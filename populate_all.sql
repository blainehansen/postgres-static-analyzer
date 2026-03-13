["pg_seclabel", "https://www.postgresql.org/docs/17/catalog-pg-seclabel.html"],
["pg_shdepend", "https://www.postgresql.org/docs/17/catalog-pg-shdepend.html"],
["pg_shseclabel", "https://www.postgresql.org/docs/17/catalog-pg-shseclabel.html"],



-- ====================================================================
-- 1. CLUSTER-LEVEL OBJECTS & SECURITY
-- ====================================================================

-- Populates: pg_tablespace
CREATE TABLESPACE my_tablespace LOCATION '/tmp/pg_data';

-- Populates: pg_roles, pg_authid (pg_roles is a view on pg_authid)
CREATE ROLE my_role NOLOGIN;
CREATE ROLE my_admin_role NOLOGIN;

-- Populates: pg_database
CREATE DATABASE my_database;

-- Populates: pg_auth_members
GRANT my_role TO my_admin_role;

-- Populates: pg_db_role_setting
ALTER ROLE my_role IN DATABASE my_database SET work_mem = '4MB';

-- Populates: pg_shdescription (Shared object descriptions)
COMMENT ON ROLE my_role IS 'A sample role for catalog population';

-- Populates: pg_shseclabel (Requires a security provider like sepgsql, using dummy syntax here)
-- SECURITY LABEL ON ROLE my_role IS 'classified';

-- Populates: pg_parameter_acl (Introduced in PG15)
GRANT SET ON PARAMETER work_mem TO my_role;

-- Populates: pg_shdepend
-- (Automatically populated by the database associating my_role with my_database ownership/grants)


-- ====================================================================
-- 2. SCHEMAS, LANGUAGES & EXTENSIONS
-- ====================================================================

-- Populates: pg_namespace
CREATE SCHEMA my_schema;

-- Populates: pg_default_acl
ALTER DEFAULT PRIVILEGES IN SCHEMA my_schema GRANT SELECT ON TABLES TO my_role;

-- Populates: pg_language
-- (plpgsql is usually installed, but this ensures a language entry)
CREATE LANGUAGE plperl;

-- Populates: pg_extension
CREATE EXTENSION IF NOT EXISTS hstore SCHEMA my_schema;

-- Populates: pg_transform
CREATE TRANSFORM FOR my_schema.hstore LANGUAGE plperl (
	FROM SQL WITH FUNCTION hstore_to_plperl(internal),
	TO SQL WITH FUNCTION plperl_to_hstore(internal)
);

-- Populates: pg_depend
-- (Automatically populated continuously throughout this script as objects depend on my_schema, etc.)


-- ====================================================================
-- 3. DATA TYPES, ENUMS & RANGES
-- ====================================================================

-- Populates: pg_enum, pg_type
CREATE TYPE my_schema.my_colors AS ENUM ('red', 'green', 'blue');

-- Populates: pg_range, pg_type
CREATE TYPE my_schema.my_int_range AS RANGE (subtype = integer);

-- Populates: pg_collation
CREATE COLLATION my_schema.my_collation (locale = 'en_US.utf8');

-- Populates: pg_conversion
CREATE CONVERSION my_schema.my_conv FOR 'UTF8' TO 'LATIN1' FROM iso8859_1_to_utf8;

-- Populates: pg_cast
CREATE CAST (integer AS text) WITH INOUT AS ASSIGNMENT;


-- ====================================================================
-- 4. TABLES, CONSTRAINTS & INHERITANCE
-- ====================================================================

-- Populates: pg_sequence, pg_class
CREATE SEQUENCE my_schema.my_seq;

-- Populates: pg_class, pg_attribute, pg_attrdef (via DEFAULT), pg_constraint (via PRIMARY KEY)
CREATE TABLE my_schema.my_parent (
	id integer PRIMARY KEY DEFAULT nextval('my_schema.my_seq'),
	val text COLLATE my_schema.my_collation,
	status my_schema.my_colors
);

-- Populates: pg_inherits
CREATE TABLE my_schema.my_child (
	extra_data text
) INHERITS (my_schema.my_parent);

-- Populates: pg_partitioned_table
CREATE TABLE my_schema.my_part_table (
	id int,
	data text
) PARTITION BY RANGE (id);

CREATE TABLE my_schema.my_part_1 PARTITION OF my_schema.my_part_table FOR VALUES FROM (1) TO (100);

-- Populates: pg_index
CREATE INDEX my_idx ON my_schema.my_parent (val);

-- Populates: pg_statistic_ext
CREATE STATISTICS my_schema.my_stats ON id, val FROM my_schema.my_parent;


-- ====================================================================
-- 5. FUNCTIONS, AGGREGATES & TRIGGERS
-- ====================================================================

-- Populates: pg_proc
CREATE FUNCTION my_schema.my_func() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
	RETURN NEW;
END;
$$;

-- Populates: pg_aggregate
CREATE AGGREGATE my_schema.my_agg(integer) (
	sfunc = int4pl,
	stype = integer,
	initcond = '0'
);

-- Populates: pg_trigger
CREATE TRIGGER my_before_insert_trig
	BEFORE INSERT ON my_schema.my_parent
	FOR EACH ROW EXECUTE FUNCTION my_schema.my_func();

-- Populates: pg_event_trigger
CREATE EVENT TRIGGER my_evt_trigger ON ddl_command_start
	EXECUTE FUNCTION my_schema.my_func();

-- Populates: pg_rewrite (Creating a View or a Rule generates rewrite logic)
CREATE RULE my_rule AS ON INSERT TO my_schema.my_parent
	WHERE NEW.id > 1000 DO INSTEAD NOTHING;

-- Populates: pg_policy (Row Level Security)
ALTER TABLE my_schema.my_parent ENABLE ROW LEVEL SECURITY;
CREATE POLICY my_select_policy ON my_schema.my_parent
	FOR SELECT TO my_role USING (id > 0);

-- Populates: pg_seclabel
-- (Requires loadable module like 'auth_delay', showing syntax here)
-- SECURITY LABEL ON TABLE my_schema.my_parent IS 'unclassified';


-- ====================================================================
-- 6. FOREIGN DATA WRAPPERS
-- ====================================================================

-- Populates: pg_foreign_data_wrapper
CREATE FOREIGN DATA WRAPPER my_fdw;

-- Populates: pg_foreign_server
CREATE SERVER my_server FOREIGN DATA WRAPPER my_fdw;

-- Populates: pg_user_mapping
CREATE USER MAPPING FOR my_role SERVER my_server;

-- Populates: pg_foreign_table
CREATE FOREIGN TABLE my_schema.my_ft (
	id integer,
	data text
) SERVER my_server;


-- ====================================================================
-- 7. OPERATORS & ACCESS METHODS
-- ====================================================================

-- Populates: pg_operator
CREATE OPERATOR my_schema.@@ (
	leftarg = integer,
	rightarg = integer,
	function = int4eq
);

-- Populates: pg_am
CREATE ACCESS METHOD my_am TYPE INDEX HANDLER bthandler;

-- Populates: pg_opfamily
CREATE OPERATOR FAMILY my_schema.my_opfamily USING btree;

-- Populates: pg_opclass, pg_amop (Operator binding), pg_amproc (Function binding)
CREATE OPERATOR CLASS my_schema.my_opclass
	DEFAULT FOR TYPE integer USING btree FAMILY my_schema.my_opfamily AS
	OPERATOR 1 =,
	FUNCTION 1 btint4cmp(integer, integer);


-- ====================================================================
-- 8. TEXT SEARCH CONFIGURATION
-- ====================================================================

-- Populates: pg_ts_parser
-- (Reusing existing parser functions to satisfy syntax without writing C code)
CREATE TEXT SEARCH PARSER my_schema.my_parser (
	START = prsd_start, GETTOKEN = prsd_nexttoken, END = prsd_end, LEXTYPES = prsd_lextype
);

-- Populates: pg_ts_template
CREATE TEXT SEARCH TEMPLATE my_schema.my_template (
	LEXIZE = dsimple_lexize
);

-- Populates: pg_ts_dict
CREATE TEXT SEARCH DICTIONARY my_schema.my_dict (
	TEMPLATE = simple
);

-- Populates: pg_ts_config
CREATE TEXT SEARCH CONFIGURATION my_schema.my_ts_conf (
	COPY = english
);

-- Populates: pg_ts_config_map
ALTER TEXT SEARCH CONFIGURATION my_schema.my_ts_conf
	ADD MAPPING FOR asciiword WITH my_schema.my_dict;


-- ====================================================================
-- 9. LOGICAL REPLICATION & PUB/SUB
-- ====================================================================

-- Populates: pg_publication
-- Populates: pg_publication_namespace (Since it targets a schema)
CREATE PUBLICATION my_schema_pub FOR TABLES IN SCHEMA my_schema;

-- Populates: pg_publication_rel (Since it targets a specific table)
CREATE PUBLICATION my_table_pub FOR TABLE my_schema.my_parent;

-- Populates: pg_subscription
-- (Connect = false prevents it from immediately trying to hit the dummy URL)
CREATE SUBSCRIPTION my_sub
	CONNECTION 'dbname=dummy_db host=localhost'
	PUBLICATION my_table_pub
	WITH (connect = false);

-- Populates: pg_replication_origin
-- (This requires executing a system administration function rather than standard DDL)
SELECT pg_replication_origin_create('my_origin_node');
