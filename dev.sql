set search_path = '';
-- show search_path;
-- select current_schemas(true);
-- select current_schemas(false);

select pg_class.relname, classoid::regclass, objsubid, privtype, initprivs
from pg_init_privs join pg_class on objoid = pg_class.oid
where classoid::regclass::text = 'pg_class' and objsubid = 0;
