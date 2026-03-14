tasks:

- get the `populate_all.sql` script fully working
- make the decisions ts file more dumb for now, just get the first table working fully using the full_test script
- then work through the tables in order, doing them manually


pg_database.oid should be filtered to `current_database` and not returned
columns with type `regproc` aren't precise enough

columns that have "reg" aliases can have a "qual" or something field that contains a Qual (renamed from Ref), and this Qual identifies them and acts as their hash identity
Qual is equivalent to (str, str)

we should have a single *snapshot* test that truly reflects the whole database, `pg_catalog` and `information_schema` included
the only filtering we'll do is to the current database, and to objects that are actually "live" such as non-deleted columns


columns referencing these things still need joins to get the name
pg_am
pg_tablespace
pg_constraint
pg_opclass
pg_language

actually things referencing pg_tablespace should just be skipped, since I'm skipping it, and that holds for all the other tables I'm skipping entirely
so if I find a column that's referencing something, and that something isn't in the list of tables I'm going to reflect, I skip the column entirely



crates:

- state, for the dbstate etc structs
- reflect, the one with actual tokio etc that can pull from a real database
- arbitrary statement, strategies that can create valid or invalid statements given an existing state
- diff, which can diff states and produce statements
- static analysis, which can produce states from raw strings and states etc






default privileges for objects as witnessed by acldefault

to sum up:
the owner is given all privileges.
column privileges aren't specified in situations where table privileges are in place that make them unnecessary?
by default, public has TEMPORARY, CONNECT on all databases
by default, public has EXECUTE on all functions
by default, public has USAGE on all languages
by default, public has USAGE on all types/domains

```
COLUMN
 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------

 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------


TABLE and table-like objects
 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 devuser | INSERT         | f            | devuser
 devuser | SELECT         | f            | devuser
 devuser | UPDATE         | f            | devuser
 devuser | DELETE         | f            | devuser
 devuser | TRUNCATE       | f            | devuser
 devuser | REFERENCES     | f            | devuser
 devuser | TRIGGER        | f            | devuser
 devuser | MAINTAIN       | f            | devuser

 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 a       | INSERT         | f            | a
 a       | SELECT         | f            | a
 a       | UPDATE         | f            | a
 a       | DELETE         | f            | a
 a       | TRUNCATE       | f            | a
 a       | REFERENCES     | f            | a
 a       | TRIGGER        | f            | a
 a       | MAINTAIN       | f            | a


SEQUENCE
 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 devuser | SELECT         | f            | devuser
 devuser | UPDATE         | f            | devuser
 devuser | USAGE          | f            | devuser

 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 a       | SELECT         | f            | a
 a       | UPDATE         | f            | a
 a       | USAGE          | f            | a


DATABASE
 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 public  | TEMPORARY      | f            | devuser
 public  | CONNECT        | f            | devuser
 devuser | CREATE         | f            | devuser
 devuser | TEMPORARY      | f            | devuser
 devuser | CONNECT        | f            | devuser

 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 public  | TEMPORARY      | f            | a
 public  | CONNECT        | f            | a
 a       | CREATE         | f            | a
 a       | TEMPORARY      | f            | a
 a       | CONNECT        | f            | a


FUNCTION or PROCEDURE
 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 public  | EXECUTE        | f            | devuser
 devuser | EXECUTE        | f            | devuser

 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 public  | EXECUTE        | f            | a
 a       | EXECUTE        | f            | a


LANGUAGE
 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 public  | USAGE          | f            | devuser
 devuser | USAGE          | f            | devuser

 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 public  | USAGE          | f            | a
 a       | USAGE          | f            | a


LARGE OBJECT
 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 devuser | SELECT         | f            | devuser
 devuser | UPDATE         | f            | devuser

 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 a       | SELECT         | f            | a
 a       | UPDATE         | f            | a


SCHEMA
 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 devuser | USAGE          | f            | devuser
 devuser | CREATE         | f            | devuser

 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 a       | USAGE          | f            | a
 a       | CREATE         | f            | a


PARAMETER
 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 devuser | SET            | f            | devuser
 devuser | ALTER SYSTEM   | f            | devuser

 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 a       | SET            | f            | a
 a       | ALTER SYSTEM   | f            | a


TABLESPACE
 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 devuser | CREATE         | f            | devuser

 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 a       | CREATE         | f            | a


FOREIGN DATA WRAPPER
 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 devuser | USAGE          | f            | devuser

 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 a       | USAGE          | f            | a


FOREIGN SERVER
 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 devuser | USAGE          | f            | devuser

 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 a       | USAGE          | f            | a


TYPE or DOMAIN
 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 public  | USAGE          | f            | devuser
 devuser | USAGE          | f            | devuser

 grantee | privilege_type | is_grantable | grantor
---------+----------------+--------------+---------
 public  | USAGE          | f            | a
 a       | USAGE          | f            | a
```



```
echo ''
echo 'COLUMN'
PGPASSWORD='devpassword' psql -U devuser -h localhost devdb <<EOF
select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('c', 'devuser'::regrole::oid));

select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('c', 'a'::regrole::oid));
EOF

echo ''
echo 'TABLE and table-like objects'
PGPASSWORD='devpassword' psql -U devuser -h localhost devdb <<EOF
select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('r', 'devuser'::regrole::oid));

select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('r', 'a'::regrole::oid));
EOF

echo ''
echo 'SEQUENCE'
PGPASSWORD='devpassword' psql -U devuser -h localhost devdb <<EOF
select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('s', 'devuser'::regrole::oid));

select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('s', 'a'::regrole::oid));
EOF

echo ''
echo 'DATABASE'
PGPASSWORD='devpassword' psql -U devuser -h localhost devdb <<EOF
select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('d', 'devuser'::regrole::oid));

select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('d', 'a'::regrole::oid));
EOF

echo ''
echo 'FUNCTION or PROCEDURE'
PGPASSWORD='devpassword' psql -U devuser -h localhost devdb <<EOF
select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('f', 'devuser'::regrole::oid));

select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('f', 'a'::regrole::oid));
EOF

echo ''
echo 'LANGUAGE'
PGPASSWORD='devpassword' psql -U devuser -h localhost devdb <<EOF
select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('l', 'devuser'::regrole::oid));

select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('l', 'a'::regrole::oid));
EOF

echo ''
echo 'LARGE OBJECT'
PGPASSWORD='devpassword' psql -U devuser -h localhost devdb <<EOF
select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('L', 'devuser'::regrole::oid));

select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('L', 'a'::regrole::oid));
EOF

echo ''
echo 'SCHEMA'
PGPASSWORD='devpassword' psql -U devuser -h localhost devdb <<EOF
select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('n', 'devuser'::regrole::oid));

select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('n', 'a'::regrole::oid));
EOF

echo ''
echo 'PARAMETER'
PGPASSWORD='devpassword' psql -U devuser -h localhost devdb <<EOF
select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('p', 'devuser'::regrole::oid));

select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('p', 'a'::regrole::oid));
EOF

echo ''
echo 'TABLESPACE'
PGPASSWORD='devpassword' psql -U devuser -h localhost devdb <<EOF
select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('t', 'devuser'::regrole::oid));

select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('t', 'a'::regrole::oid));
EOF

echo ''
echo 'FOREIGN DATA WRAPPER'
PGPASSWORD='devpassword' psql -U devuser -h localhost devdb <<EOF
select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('F', 'devuser'::regrole::oid));

select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('F', 'a'::regrole::oid));
EOF

echo ''
echo 'FOREIGN SERVER'
PGPASSWORD='devpassword' psql -U devuser -h localhost devdb <<EOF
select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('S', 'devuser'::regrole::oid));

select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('S', 'a'::regrole::oid));
EOF

echo ''
echo 'TYPE or DOMAIN'
PGPASSWORD='devpassword' psql -U devuser -h localhost devdb <<EOF
select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('T', 'devuser'::regrole::oid));

select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('T', 'a'::regrole::oid));
EOF
```



`pg_aggregate`: https://www.postgresql.org/docs/17/catalog-pg-aggregate.html
`pg_am`: https://www.postgresql.org/docs/17/catalog-pg-am.html
`pg_amop`: https://www.postgresql.org/docs/17/catalog-pg-amop.html
`pg_amproc`: https://www.postgresql.org/docs/17/catalog-pg-amproc.html
`pg_attrdef`: https://www.postgresql.org/docs/17/catalog-pg-attrdef.html
`pg_attribute`: https://www.postgresql.org/docs/17/catalog-pg-attribute.html
`pg_authid`: https://www.postgresql.org/docs/17/catalog-pg-authid.html
`pg_auth_members`: https://www.postgresql.org/docs/17/catalog-pg-auth-members.html
`pg_cast`: https://www.postgresql.org/docs/17/catalog-pg-cast.html
`pg_class`: https://www.postgresql.org/docs/17/catalog-pg-class.html
`pg_collation`: https://www.postgresql.org/docs/17/catalog-pg-collation.html
`pg_constraint`: https://www.postgresql.org/docs/17/catalog-pg-constraint.html
`pg_conversion`: https://www.postgresql.org/docs/17/catalog-pg-conversion.html
`pg_database`: https://www.postgresql.org/docs/17/catalog-pg-database.html
`pg_db_role_setting`: https://www.postgresql.org/docs/17/catalog-pg-db-role-setting.html
`pg_default_acl`: https://www.postgresql.org/docs/17/catalog-pg-default-acl.html
`pg_depend`: https://www.postgresql.org/docs/17/catalog-pg-depend.html
`pg_description`: https://www.postgresql.org/docs/17/catalog-pg-description.html
`pg_enum`: https://www.postgresql.org/docs/17/catalog-pg-enum.html
`pg_event_trigger`: https://www.postgresql.org/docs/17/catalog-pg-event-trigger.html
`pg_extension`: https://www.postgresql.org/docs/17/catalog-pg-extension.html
`pg_foreign_data_wrapper`: https://www.postgresql.org/docs/17/catalog-pg-foreign-data-wrapper.html
`pg_foreign_server`: https://www.postgresql.org/docs/17/catalog-pg-foreign-server.html
`pg_foreign_table`: https://www.postgresql.org/docs/17/catalog-pg-foreign-table.html
`pg_index`: https://www.postgresql.org/docs/17/catalog-pg-index.html
`pg_inherits`: https://www.postgresql.org/docs/17/catalog-pg-inherits.html
`pg_init_privs`: https://www.postgresql.org/docs/17/catalog-pg-init-privs.html
`pg_language`: https://www.postgresql.org/docs/17/catalog-pg-language.html
`pg_largeobject`: https://www.postgresql.org/docs/17/catalog-pg-largeobject.html
`pg_largeobject_metadata`: https://www.postgresql.org/docs/17/catalog-pg-largeobject-metadata.html
`pg_namespace`: https://www.postgresql.org/docs/17/catalog-pg-namespace.html
`pg_opclass`: https://www.postgresql.org/docs/17/catalog-pg-opclass.html
`pg_operator`: https://www.postgresql.org/docs/17/catalog-pg-operator.html
`pg_opfamily`: https://www.postgresql.org/docs/17/catalog-pg-opfamily.html
`pg_parameter_acl`: https://www.postgresql.org/docs/17/catalog-pg-parameter-acl.html
`pg_partitioned_table`: https://www.postgresql.org/docs/17/catalog-pg-partitioned-table.html
`pg_policy`: https://www.postgresql.org/docs/17/catalog-pg-policy.html
`pg_proc`: https://www.postgresql.org/docs/17/catalog-pg-proc.html
`pg_publication`: https://www.postgresql.org/docs/17/catalog-pg-publication.html
`pg_publication_namespace`: https://www.postgresql.org/docs/17/catalog-pg-publication-namespace.html
`pg_publication_rel`: https://www.postgresql.org/docs/17/catalog-pg-publication-rel.html
`pg_range`: https://www.postgresql.org/docs/17/catalog-pg-range.html
`pg_replication_origin`: https://www.postgresql.org/docs/17/catalog-pg-replication-origin.html
`pg_rewrite`: https://www.postgresql.org/docs/17/catalog-pg-rewrite.html
`pg_seclabel`: https://www.postgresql.org/docs/17/catalog-pg-seclabel.html
`pg_sequence`: https://www.postgresql.org/docs/17/catalog-pg-sequence.html
`pg_shdepend`: https://www.postgresql.org/docs/17/catalog-pg-shdepend.html
`pg_shdescription`: https://www.postgresql.org/docs/17/catalog-pg-shdescription.html
`pg_shseclabel`: https://www.postgresql.org/docs/17/catalog-pg-shseclabel.html
`pg_statistic`: https://www.postgresql.org/docs/17/catalog-pg-statistic.html
`pg_statistic_ext`: https://www.postgresql.org/docs/17/catalog-pg-statistic-ext.html
`pg_statistic_ext_data`: https://www.postgresql.org/docs/17/catalog-pg-statistic-ext-data.html
`pg_subscription`: https://www.postgresql.org/docs/17/catalog-pg-subscription.html
`pg_subscription_rel`: https://www.postgresql.org/docs/17/catalog-pg-subscription-rel.html
`pg_tablespace`: https://www.postgresql.org/docs/17/catalog-pg-tablespace.html
`pg_transform`: https://www.postgresql.org/docs/17/catalog-pg-transform.html
`pg_trigger`: https://www.postgresql.org/docs/17/catalog-pg-trigger.html
`pg_ts_config`: https://www.postgresql.org/docs/17/catalog-pg-ts-config.html
`pg_ts_config_map`: https://www.postgresql.org/docs/17/catalog-pg-ts-config-map.html
`pg_ts_dict`: https://www.postgresql.org/docs/17/catalog-pg-ts-dict.html
`pg_ts_parser`: https://www.postgresql.org/docs/17/catalog-pg-ts-parser.html
`pg_ts_template`: https://www.postgresql.org/docs/17/catalog-pg-ts-template.html
`pg_type`: https://www.postgresql.org/docs/17/catalog-pg-type.html
`pg_user_mapping`: https://www.postgresql.org/docs/17/catalog-pg-user-mapping.html
