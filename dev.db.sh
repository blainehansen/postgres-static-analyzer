PGPASSWORD='devpassword' psql -U devuser -h localhost devdb -f ./dev.sql

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

