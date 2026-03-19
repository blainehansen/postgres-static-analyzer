--! reflect_pg_aggregate : (aggfinalfn?, aggcombinefn?, aggserialfn?, aggdeserialfn?, aggmtransfn?, aggminvtransfn?, aggmfinalfn?, aggsortop?, aggmtranstype?, agginitval?, aggminitval?)
select
	aggfnoid::regproc::text as aggfnoid, -- regproc (references pg_proc.oid) pg_proc OID of the aggregate function
	aggkind as aggkind, -- char  Aggregate kind: n for “normal” aggregates, o for “ordered-set” aggregates, or h for “hypothetical-set” aggregates
	aggnumdirectargs as aggnumdirectargs, -- int2  Number of direct (non-aggregated) arguments of an ordered-set or hypothetical-set aggregate, counting a variadic array as one argument. If equal to pronargs, the aggregate must be variadic and the variadic array describes the aggregated arguments as well as the final direct arguments. Always zero for normal aggregates.
	aggtransfn::regproc::text as aggtransfn, -- regproc (references pg_proc.oid) Transition function
	case when aggfinalfn = 0 then null else aggfinalfn::regproc::text end as aggfinalfn, -- regproc (references pg_proc.oid) Final function (zero if none)
	case when aggcombinefn = 0 then null else aggcombinefn::regproc::text end as aggcombinefn, -- regproc (references pg_proc.oid) Combine function (zero if none)
	case when aggserialfn = 0 then null else aggserialfn::regproc::text end as aggserialfn, -- regproc (references pg_proc.oid) Serialization function (zero if none)
	case when aggdeserialfn = 0 then null else aggdeserialfn::regproc::text end as aggdeserialfn, -- regproc (references pg_proc.oid) Deserialization function (zero if none)
	case when aggmtransfn = 0 then null else aggmtransfn::regproc::text end as aggmtransfn, -- regproc (references pg_proc.oid) Forward transition function for moving-aggregate mode (zero if none)
	case when aggminvtransfn = 0 then null else aggminvtransfn::regproc::text end as aggminvtransfn, -- regproc (references pg_proc.oid) Inverse transition function for moving-aggregate mode (zero if none)
	case when aggmfinalfn = 0 then null else aggmfinalfn::regproc::text end as aggmfinalfn, -- regproc (references pg_proc.oid) Final function for moving-aggregate mode (zero if none)
	aggfinalextra as aggfinalextra, -- bool  True to pass extra dummy arguments to aggfinalfn
	aggmfinalextra as aggmfinalextra, -- bool  True to pass extra dummy arguments to aggmfinalfn
	aggfinalmodify as aggfinalmodify, -- char  Whether aggfinalfn modifies the transition state value: r if it is read-only, s if the aggtransfn cannot be applied after the aggfinalfn, or w if it writes on the value
	aggmfinalmodify as aggmfinalmodify, -- char  Like aggfinalmodify, but for the aggmfinalfn
	case when aggsortop = 0 then null else aggsortop::regoperator::text end as aggsortop, -- oid (references pg_operator.oid) Associated sort operator (zero if none)
	aggtranstype::regtype::text as aggtranstype, -- oid (references pg_type.oid) Data type of the aggregate function's internal transition (state) data
	-- aggtransspace int4  Approximate average size (in bytes) of the transition state data, or zero to use a default estimate
	case when aggmtranstype = 0 then null else aggmtranstype::regtype::text end as aggmtranstype, -- oid (references pg_type.oid) Data type of the aggregate function's internal transition (state) data for moving-aggregate mode (zero if none)
	-- aggmtransspace int4  Approximate average size (in bytes) of the transition state data for moving-aggregate mode, or zero to use a default estimate
	agginitval as agginitval, -- text  The initial value of the transition state. This is a text field containing the initial value in its external string representation. If this field is null, the transition state value starts out null.
	aggminitval as aggminitval -- text  The initial value of the transition state for moving-aggregate mode. This is a text field containing the initial value in its external string representation. If this field is null, the transition state value starts out null.
from
	pg_aggregate
;


--! reflect_pg_am : ()
select
	-- oid oid  Row identifier
	amname::text as amname, -- name  Name of the access method
	amhandler::regproc::text as amhandler, -- regproc (references pg_proc.oid) OID of a handler function that is responsible for supplying information about the access method
	amtype as amtype -- char  t = table (including materialized views), i = index.
from
	pg_am
;


--! reflect_pg_amop : (amopsortfamily?)
select
	-- oid oid  Row identifier
	quote_ident(amopfamily_pg_namespace.nspname) || '.' || quote_ident(amopfamily_pg_opfamily.opfname) as amopfamily, -- oid (references pg_opfamily.oid) The operator family this entry is for
	amoplefttype::regtype::text as amoplefttype, -- oid (references pg_type.oid) Left-hand input data type of operator
	amoprighttype::regtype::text as amoprighttype, -- oid (references pg_type.oid) Right-hand input data type of operator
	amopstrategy as amopstrategy, -- int2  Operator strategy number
	amoppurpose as amoppurpose, -- char  Operator purpose, either s for search or o for ordering
	amopopr::regoperator::text as amopopr, -- oid (references pg_operator.oid) OID of the operator
	amopmethod_pg_am.amname::text as amopmethod, -- oid (references pg_am.oid) Index access method operator family is for
	quote_ident(amopsortfamily_pg_namespace.nspname) || '.' || quote_ident(amopsortfamily_pg_opfamily.opfname) as amopsortfamily -- oid (references pg_opfamily.oid) The B-tree operator family this entry sorts according to, if an ordering operator; zero if a search operator
from
	pg_amop
	join pg_opfamily as amopfamily_pg_opfamily on pg_amop.amopfamily = amopfamily_pg_opfamily.oid
	join pg_namespace as amopfamily_pg_namespace on amopfamily_pg_opfamily.opfnamespace = amopfamily_pg_namespace.oid
	join pg_am as amopmethod_pg_am on pg_amop.amopmethod = amopmethod_pg_am.oid
	left join pg_opfamily as amopsortfamily_pg_opfamily on pg_amop.amopsortfamily = amopsortfamily_pg_opfamily.oid
	left join pg_namespace as amopsortfamily_pg_namespace on amopsortfamily_pg_opfamily.opfnamespace = amopsortfamily_pg_namespace.oid
;


--! reflect_pg_amproc : ()
select
	-- oid oid  Row identifier
	quote_ident(amprocfamily_pg_namespace.nspname) || '.' || quote_ident(amprocfamily_pg_opfamily.opfname) as amprocfamily, -- oid (references pg_opfamily.oid) The operator family this entry is for
	amproclefttype::regtype::text as amproclefttype, -- oid (references pg_type.oid) Left-hand input data type of associated operator
	amprocrighttype::regtype::text as amprocrighttype, -- oid (references pg_type.oid) Right-hand input data type of associated operator
	amprocnum as amprocnum, -- int2  Support function number
	amproc::regproc::text as amproc -- regproc (references pg_proc.oid) OID of the function
from
	pg_amproc
	join pg_opfamily as amprocfamily_pg_opfamily on pg_amproc.amprocfamily = amprocfamily_pg_opfamily.oid
	join pg_namespace as amprocfamily_pg_namespace on amprocfamily_pg_opfamily.opfnamespace = amprocfamily_pg_namespace.oid
;


--! reflect_pg_class : (reltype?, reloftype?, relam?, relacl?, reloptions?, relpartbound?)
select
	pg_class.oid::regclass::text as oid, -- oid  Row identifier
	relname::text as relname, -- name  Name of the table, index, view, etc.
	relnamespace::regnamespace::text as relnamespace, -- oid (references pg_namespace.oid) The OID of the namespace that contains this relation
	case when reltype = 0 then null else reltype::regtype::text end as reltype, -- oid (references pg_type.oid) The OID of the data type that corresponds to this table's row type, if any; zero for indexes, sequences, and toast tables, which have no pg_type entry
	case when reloftype = 0 then null else reloftype::regtype::text end as reloftype, -- oid (references pg_type.oid) For typed tables, the OID of the underlying composite type; zero for all other relations
	pg_get_userbyid(relowner)::text as relowner, -- oid (references pg_authid.oid) Owner of the relation
	relam_pg_am.amname::text as relam, -- oid (references pg_am.oid) The access method used to access this table or index. Not meaningful if the relation is a sequence or has no on-disk file, except for partitioned tables, where, if set, it takes precedence over default_table_access_method when determining the access method to use for partitions created when one is not specified in the creation command.
	-- relfilenode oid  Name of the on-disk file of this relation; zero means this is a “mapped” relation whose disk file name is determined by low-level state
	-- reltablespace oid (references pg_tablespace.oid) The tablespace in which this relation is stored. If zero, the database's default tablespace is implied. Not meaningful if the relation has no on-disk file, except for partitioned tables, where this is the tablespace in which partitions will be created when one is not specified in the creation command.
	-- relpages int4  Size of the on-disk representation of this table in pages (of size BLCKSZ). This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX.
	-- reltuples float4  Number of live rows in the table. This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX. If the table has never yet been vacuumed or analyzed, reltuples contains -1 indicating that the row count is unknown.
	-- relallvisible int4  Number of pages that are marked all-visible in the table's visibility map. This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX.
	-- reltoastrelid oid (references pg_class.oid) OID of the TOAST table associated with this table, zero if none. The TOAST table stores large attributes “out of line” in a secondary table.
	-- relhasindex bool  True if this is a table and it has (or recently had) any indexes
	relisshared as relisshared, -- bool  True if this table is shared across all databases in the cluster. Only certain system catalogs (such as pg_database) are shared.
	relpersistence as relpersistence, -- char  p = permanent table/sequence, u = unlogged table/sequence, t = temporary table/sequence
	relkind as relkind, -- char  r = ordinary table, i = index, S = sequence, t = TOAST table, v = view, m = materialized view, c = composite type, f = foreign table, p = partitioned table, I = partitioned index
	relnatts as relnatts, -- int2  Number of user columns in the relation (system columns not counted). There must be this many corresponding entries in pg_attribute. See also pg_attribute.attnum.
	relchecks as relchecks, -- int2  Number of CHECK constraints on the table; see pg_constraint catalog
	-- relhasrules bool  True if table has (or once had) rules; see pg_rewrite catalog
	-- relhastriggers bool  True if table has (or once had) triggers; see pg_trigger catalog
	-- relhassubclass bool  True if table or index has (or once had) any inheritance children or partitions
	relrowsecurity as relrowsecurity, -- bool  True if table has row-level security enabled; see pg_policy catalog
	relforcerowsecurity as relforcerowsecurity, -- bool  True if row-level security (when enabled) will also apply to table owner; see pg_policy catalog
	-- relispopulated bool  True if relation is populated (this is true for all relations other than some materialized views)
	relreplident as relreplident, -- char  Columns used to form “replica identity” for rows: d = default (primary key, if any), n = nothing, f = all columns, i = index with indisreplident set (same as nothing if the index used has been dropped)
	relispartition as relispartition, -- bool  True if table or index is a partition
	-- relrewrite oid (references pg_class.oid) For new relations being written during a DDL operation that requires a table rewrite, this contains the OID of the original relation; otherwise zero. That state is only visible internally; this field should never contain anything other than zero for a user-visible relation.
	-- relfrozenxid xid  All transaction IDs before this one have been replaced with a permanent (“frozen”) transaction ID in this table. This is used to track whether the table needs to be vacuumed in order to prevent transaction ID wraparound or to allow pg_xact to be shrunk. Zero (InvalidTransactionId) if the relation is not a table.
	-- relminmxid xid  All multixact IDs before this one have been replaced by a transaction ID in this table. This is used to track whether the table needs to be vacuumed in order to prevent multixact ID wraparound or to allow pg_multixact to be shrunk. Zero (InvalidMultiXactId) if the relation is not a table.
	relacl::text[] as relacl, -- aclitem[]  Access privileges; see Section 5.8 for details
	reloptions as reloptions, -- text[]  Access-method-specific options, as “keyword=value” strings
	pg_get_expr(relpartbound, pg_class.oid) as relpartbound -- pg_node_tree  If table is a partition (see relispartition), internal representation of the partition bound
from
	pg_class
	left join pg_am as relam_pg_am on pg_class.relam = relam_pg_am.oid
where 
	relnamespace::regnamespace != 'pg_toast'::regnamespace
;

