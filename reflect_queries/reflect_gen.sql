--! reflect_pg_aggregate : (aggfinalfn?, aggcombinefn?, aggserialfn?, aggdeserialfn?, aggmtransfn?, aggminvtransfn?, aggmfinalfn?, aggsortop?, aggmtranstype?, agginitval?, aggminitval?)
select
	aggfnoid::regproc::text as aggfnoid, -- regproc (references pg_proc.oid) pg_proc OID of the aggregate function
	aggkind, -- char  Aggregate kind: n for “normal” aggregates, o for “ordered-set” aggregates, or h for “hypothetical-set” aggregates
	aggnumdirectargs, -- int2  Number of direct (non-aggregated) arguments of an ordered-set or hypothetical-set aggregate, counting a variadic array as one argument. If equal to pronargs, the aggregate must be variadic and the variadic array describes the aggregated arguments as well as the final direct arguments. Always zero for normal aggregates.
	aggtransfn::regproc::text as aggtransfn, -- regproc (references pg_proc.oid) Transition function
	case when aggfinalfn = 0 then null else aggfinalfn::regproc::text end as aggfinalfn, -- regproc (references pg_proc.oid) Final function (zero if none)
	case when aggcombinefn = 0 then null else aggcombinefn::regproc::text end as aggcombinefn, -- regproc (references pg_proc.oid) Combine function (zero if none)
	case when aggserialfn = 0 then null else aggserialfn::regproc::text end as aggserialfn, -- regproc (references pg_proc.oid) Serialization function (zero if none)
	case when aggdeserialfn = 0 then null else aggdeserialfn::regproc::text end as aggdeserialfn, -- regproc (references pg_proc.oid) Deserialization function (zero if none)
	case when aggmtransfn = 0 then null else aggmtransfn::regproc::text end as aggmtransfn, -- regproc (references pg_proc.oid) Forward transition function for moving-aggregate mode (zero if none)
	case when aggminvtransfn = 0 then null else aggminvtransfn::regproc::text end as aggminvtransfn, -- regproc (references pg_proc.oid) Inverse transition function for moving-aggregate mode (zero if none)
	case when aggmfinalfn = 0 then null else aggmfinalfn::regproc::text end as aggmfinalfn, -- regproc (references pg_proc.oid) Final function for moving-aggregate mode (zero if none)
	aggfinalextra, -- bool  True to pass extra dummy arguments to aggfinalfn
	aggmfinalextra, -- bool  True to pass extra dummy arguments to aggmfinalfn
	aggfinalmodify, -- char  Whether aggfinalfn modifies the transition state value: r if it is read-only, s if the aggtransfn cannot be applied after the aggfinalfn, or w if it writes on the value
	aggmfinalmodify, -- char  Like aggfinalmodify, but for the aggmfinalfn
	case when aggsortop = 0 then null else aggsortop::regoperator::text end as aggsortop, -- oid (references pg_operator.oid) Associated sort operator (zero if none)
	aggtranstype::regtype::text as aggtranstype, -- oid (references pg_type.oid) Data type of the aggregate function's internal transition (state) data
	-- aggtransspace int4  Approximate average size (in bytes) of the transition state data, or zero to use a default estimate
	case when aggmtranstype = 0 then null else aggmtranstype::regtype::text end as aggmtranstype, -- oid (references pg_type.oid) Data type of the aggregate function's internal transition (state) data for moving-aggregate mode (zero if none)
	-- aggmtransspace int4  Approximate average size (in bytes) of the transition state data for moving-aggregate mode, or zero to use a default estimate
	agginitval, -- text  The initial value of the transition state. This is a text field containing the initial value in its external string representation. If this field is null, the transition state value starts out null.
	aggminitval -- text  The initial value of the transition state for moving-aggregate mode. This is a text field containing the initial value in its external string representation. If this field is null, the transition state value starts out null.
from
	pg_aggregate
;


--! reflect_pg_database : (datconnlimit?, datcollate?, datctype?, datlocale?, daticurules?, datcollversion?, datacl?)
select
	-- oid oid  Row identifier
	datname::text as datname, -- name  Database name
	pg_get_userbyid(datdba)::text as datdba, -- oid (references pg_authid.oid) Owner of the database, usually the user who created it
	pg_encoding_to_char(encoding)::text as encoding, -- int4  Character encoding for this database (pg_encoding_to_char() can translate this number to the encoding name)
	datlocprovider, -- char  Locale provider for this database: b = builtin, c = libc, i = icu
	datistemplate, -- bool  If true, then this database can be cloned by any user with CREATEDB privileges; if false, then only superusers or the owner of the database can clone it.
	datallowconn, -- bool  If false then no one can connect to this database. This is used to protect the template0 database from being altered.
	-- dathasloginevt bool  Indicates that there are login event triggers defined for this database. This flag is used to avoid extra lookups on the pg_event_trigger table during each backend startup. This flag is used internally by PostgreSQL and should not be manually altered or read for monitoring purposes.
	case when datconnlimit < 0 then null else datconnlimit end as datconnlimit, -- int4  Sets maximum number of concurrent connections that can be made to this database. -1 means no limit, -2 indicates the database is invalid.
	-- datfrozenxid xid  All transaction IDs before this one have been replaced with a permanent (“frozen”) transaction ID in this database. This is used to track whether the database needs to be vacuumed in order to prevent transaction ID wraparound or to allow pg_xact to be shrunk. It is the minimum of the per-table pg_class.relfrozenxid values.
	-- datminmxid xid  All multixact IDs before this one have been replaced with a transaction ID in this database. This is used to track whether the database needs to be vacuumed in order to prevent multixact ID wraparound or to allow pg_multixact to be shrunk. It is the minimum of the per-table pg_class.relminmxid values.
	-- dattablespace oid (references pg_tablespace.oid) The default tablespace for the database. Within this database, all tables for which pg_class.reltablespace is zero will be stored in this tablespace; in particular, all the non-shared system catalogs will be there.
	datcollate, -- text  LC_COLLATE for this database
	datctype, -- text  LC_CTYPE for this database
	datlocale, -- text  Collation provider locale name for this database. If the provider is libc, datlocale is NULL; datcollate and datctype are used instead.
	daticurules, -- text  ICU collation rules for this database
	datcollversion, -- text  Provider-specific version of the collation. This is recorded when the database is created and then checked when it is used, to detect changes in the collation definition that could lead to data corruption.
	datacl::text[] as datacl -- aclitem[]  Access privileges; see Section 5.8 for details
from
	pg_database
where 
	datname = current_database()
;

