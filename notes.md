crates:

- state, for the dbstate etc structs
- reflect, the one with actual tokio etc that can pull from a real database
- arbitrary statement, strategies that can create valid or invalid statements given an existing state
- diff, which can diff states and produce statements
- static analysis, which can produce states from raw strings and states etc



```postgresql
-- --! reflect_default_acls : (applicable_schema?)
-- select
--  pg_get_userbyid(defaclrole) as applicable_object_owner, -- defaclrole specifies the object owner, the person who's owned objects are included in this default privilege
--  sch.nspname::text as applicable_schema,
--  defaclobjtype,
--  pg_get_userbyid(grantor) as grantor,
--  case when grantee = 0 then 'public' else pg_get_userbyid(grantee) end as grantee,
--  privilege_type,
--  is_grantable

-- from
--  pg_catalog.pg_default_acl cross join lateral aclexplode(defaclacl)
--  left join pg_catalog.pg_namespace as sch on pg_default_acl.defaclnamespace = sch.oid

-- group by defaclrole, sch.nspname, defaclobjtype, grantee



-- SELECT
--  obj_type,
--  object_schema,
--  object_name,
--  grantor_role,
--  grantee_role,
--  privilege_type,
--  is_grantable
-- FROM (

--  -- TABLEs, VIEWs, MATERIALIZED VIEWs, FOREIGN TABLEs, PARTITIONs
--  SELECT
--    c.relkind::text AS obj_type,
--    n.nspname       AS object_schema,
--    c.relname       AS object_name,
--    pg_get_userbyid(acl.grantor) AS grantor_role,
--    CASE acl.grantee WHEN 0 THEN 'PUBLIC' ELSE pg_get_userbyid(acl.grantee) END AS grantee_role,
--    acl.privilege_type,
--    acl.is_grantable
--  FROM pg_catalog.pg_class c
--  JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
--  CROSS JOIN LATERAL aclexplode(COALESCE(c.relacl, acldefault('r', c.relowner))) AS acl
--  WHERE c.relkind IN ('r','v','m','f','p')  -- table, view, mat view, foreign table, partitioned table

--  UNION ALL

--  -- TABLE COLUMNS
--  SELECT
--    'column'        AS obj_type,
--    n.nspname       AS object_schema,
--    c.relname || '.' || a.attname AS object_name,
--    pg_get_userbyid(acl.grantor) AS grantor_role,
--    CASE acl.grantee WHEN 0 THEN 'PUBLIC' ELSE pg_get_userbyid(acl.grantee) END AS grantee_role,
--    acl.privilege_type,
--    acl.is_grantable
--  FROM pg_catalog.pg_attribute a
--  JOIN pg_catalog.pg_class c ON c.oid = a.attrelid
--  JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
--  CROSS JOIN LATERAL aclexplode(a.attacl) AS acl
--  WHERE a.attacl IS NOT NULL
--    AND a.attnum > 0
--    AND NOT a.attisdropped

--  UNION ALL

--  -- SEQUENCEs
--  SELECT
--    'sequence'      AS obj_type,
--    n.nspname       AS object_schema,
--    c.relname       AS object_name,
--    pg_get_userbyid(acl.grantor) AS grantor_role,
--    CASE acl.grantee WHEN 0 THEN 'PUBLIC' ELSE pg_get_userbyid(acl.grantee) END AS grantee_role,
--    acl.privilege_type,
--    acl.is_grantable
--  FROM pg_catalog.pg_class c
--  JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
--  CROSS JOIN LATERAL aclexplode(COALESCE(c.relacl, acldefault('S', c.relowner))) AS acl
--  WHERE c.relkind = 'S'

--  UNION ALL

--  -- FUNCTIONs and PROCEDUREs
--  SELECT
--    CASE p.prokind WHEN 'f' THEN 'function' WHEN 'p' THEN 'procedure' ELSE 'function' END AS obj_type,
--    n.nspname       AS object_schema,
--    p.proname || '(' || pg_get_function_identity_arguments(p.oid) || ')' AS object_name,
--    pg_get_userbyid(acl.grantor) AS grantor_role,
--    CASE acl.grantee WHEN 0 THEN 'PUBLIC' ELSE pg_get_userbyid(acl.grantee) END AS grantee_role,
--    acl.privilege_type,
--    acl.is_grantable
--  FROM pg_catalog.pg_proc p
--  JOIN pg_catalog.pg_namespace n ON n.oid = p.pronamespace
--  CROSS JOIN LATERAL aclexplode(COALESCE(p.proacl, acldefault('f', p.proowner))) AS acl

--  UNION ALL

--  -- SCHEMAs
--  SELECT
--    'schema'        AS obj_type,
--    NULL            AS object_schema,
--    n.nspname       AS object_name,
--    pg_get_userbyid(acl.grantor) AS grantor_role,
--    CASE acl.grantee WHEN 0 THEN 'PUBLIC' ELSE pg_get_userbyid(acl.grantee) END AS grantee_role,
--    acl.privilege_type,
--    acl.is_grantable
--  FROM pg_catalog.pg_namespace n
--  CROSS JOIN LATERAL aclexplode(COALESCE(n.nspacl, acldefault('n', n.nspowner))) AS acl

--  UNION ALL

--  -- DATABASEs
--  SELECT
--    'database'      AS obj_type,
--    NULL            AS object_schema,
--    d.datname       AS object_name,
--    pg_get_userbyid(acl.grantor) AS grantor_role,
--    CASE acl.grantee WHEN 0 THEN 'PUBLIC' ELSE pg_get_userbyid(acl.grantee) END AS grantee_role,
--    acl.privilege_type,
--    acl.is_grantable
--  FROM pg_catalog.pg_database d
--  CROSS JOIN LATERAL aclexplode(COALESCE(d.datacl, acldefault('d', d.datdba))) AS acl

--  UNION ALL

--  -- TABLESPACEs
--  SELECT
--    'tablespace'    AS obj_type,
--    NULL            AS object_schema,
--    t.spcname       AS object_name,
--    pg_get_userbyid(acl.grantor) AS grantor_role,
--    CASE acl.grantee WHEN 0 THEN 'PUBLIC' ELSE pg_get_userbyid(acl.grantee) END AS grantee_role,
--    acl.privilege_type,
--    acl.is_grantable
--  FROM pg_catalog.pg_tablespace t
--  CROSS JOIN LATERAL aclexplode(COALESCE(t.spcacl, acldefault('t', t.spcowner))) AS acl

--  UNION ALL

--  -- LANGUAGEs
--  SELECT
--    'language'      AS obj_type,
--    NULL            AS object_schema,
--    l.lanname       AS object_name,
--    pg_get_userbyid(acl.grantor) AS grantor_role,
--    CASE acl.grantee WHEN 0 THEN 'PUBLIC' ELSE pg_get_userbyid(acl.grantee) END AS grantee_role,
--    acl.privilege_type,
--    acl.is_grantable
--  FROM pg_catalog.pg_language l
--  CROSS JOIN LATERAL aclexplode(COALESCE(l.lanacl, acldefault('l', l.lanowner))) AS acl

--  UNION ALL

--  -- TYPEs and DOMAINs
--  SELECT
--    CASE t.typtype WHEN 'd' THEN 'domain' ELSE 'type' END AS obj_type,
--    n.nspname       AS object_schema,
--    t.typname       AS object_name,
--    pg_get_userbyid(acl.grantor) AS grantor_role,
--    CASE acl.grantee WHEN 0 THEN 'PUBLIC' ELSE pg_get_userbyid(acl.grantee) END AS grantee_role,
--    acl.privilege_type,
--    acl.is_grantable
--  FROM pg_catalog.pg_type t
--  JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace
--  CROSS JOIN LATERAL aclexplode(COALESCE(t.typacl, acldefault('T', t.typowner))) AS acl
--  WHERE t.typtype IN ('b','c','d','e','r')  -- base, composite, domain, enum, range

--  UNION ALL

--  -- FOREIGN DATA WRAPPERs
--  SELECT
--    'foreign_data_wrapper' AS obj_type,
--    NULL            AS object_schema,
--    w.fdwname       AS object_name,
--    pg_get_userbyid(acl.grantor) AS grantor_role,
--    CASE acl.grantee WHEN 0 THEN 'PUBLIC' ELSE pg_get_userbyid(acl.grantee) END AS grantee_role,
--    acl.privilege_type,
--    acl.is_grantable
--  FROM pg_catalog.pg_foreign_data_wrapper w
--  CROSS JOIN LATERAL aclexplode(COALESCE(w.fdwacl, acldefault('F', w.fdwowner))) AS acl

--  UNION ALL

--  -- FOREIGN SERVERs
--  SELECT
--    'foreign_server' AS obj_type,
--    NULL            AS object_schema,
--    s.srvname       AS object_name,
--    pg_get_userbyid(acl.grantor) AS grantor_role,
--    CASE acl.grantee WHEN 0 THEN 'PUBLIC' ELSE pg_get_userbyid(acl.grantee) END AS grantee_role,
--    acl.privilege_type,
--    acl.is_grantable
--  FROM pg_catalog.pg_foreign_server s
--  CROSS JOIN LATERAL aclexplode(COALESCE(s.srvacl, acldefault('S', s.srvowner))) AS acl

--  UNION ALL

--  -- LARGE OBJECTs
--  SELECT
--    'large_object'  AS obj_type,
--    NULL            AS object_schema,
--    lo.oid::text    AS object_name,
--    pg_get_userbyid(acl.grantor) AS grantor_role,
--    CASE acl.grantee WHEN 0 THEN 'PUBLIC' ELSE pg_get_userbyid(acl.grantee) END AS grantee_role,
--    acl.privilege_type,
--    acl.is_grantable
--  FROM pg_catalog.pg_largeobject_metadata lo
--  CROSS JOIN LATERAL aclexplode(COALESCE(lo.lomacl, acldefault('L', lo.lomowner))) AS acl

--  UNION ALL

--  -- CONFIGURATIONs / PARAMETERs (pg_parameter_acl, PG 15+)
--  SELECT
--    'parameter'     AS obj_type,
--    NULL            AS object_schema,
--    p.parname       AS object_name,
--    pg_get_userbyid(acl.grantor) AS grantor_role,
--    CASE acl.grantee WHEN 0 THEN 'PUBLIC' ELSE pg_get_userbyid(acl.grantee) END AS grantee_role,
--    acl.privilege_type,
--    acl.is_grantable
--  FROM pg_catalog.pg_parameter_acl p
--  CROSS JOIN LATERAL aclexplode(p.paracl) AS acl

-- ) grants
-- ORDER BY obj_type, object_schema, object_name, grantee_role, privilege_type;

```
