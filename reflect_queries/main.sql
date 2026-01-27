--! reflect_db_role_setting
select setconfig, setdatabase::int, setrole::int
from pg_db_role_setting
-- where setdatabase = (select oid from pg_database where datname = 'your_database_name')
	-- and setrole = 0

;
