set search_path = '';
-- show search_path;
-- select current_schemas(true);
-- select current_schemas(false);


select *, amproc::regprocedure
from pg_amproc
where amproc::regproc::text = 'pg_catalog.gtsquery_consistent'
;
