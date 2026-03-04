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







crates:

- state, for the dbstate etc structs
- reflect, the one with actual tokio etc that can pull from a real database
- arbitrary statement, strategies that can create valid or invalid statements given an existing state
- diff, which can diff states and produce statements
- static analysis, which can produce states from raw strings and states etc
