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


--! reflect_pg_am : ()
select
	-- oid oid  Row identifier
	amname::text as amname, -- name  Name of the access method
	amhandler::regproc::text as amhandler, -- regproc (references pg_proc.oid) OID of a handler function that is responsible for supplying information about the access method
	amtype -- char  t = table (including materialized views), i = index.
from
	pg_am
;


--! reflect_pg_amop : (amopsortfamily?)
select
	-- oid oid  Row identifier
	quote_ident(amopfamily_pg_namespace.nspname) || '.' || quote_ident(amopfamily_pg_opfamily.opfname) as amopfamily, -- oid (references pg_opfamily.oid) The operator family this entry is for
	amoplefttype::regtype::text as amoplefttype, -- oid (references pg_type.oid) Left-hand input data type of operator
	amoprighttype::regtype::text as amoprighttype, -- oid (references pg_type.oid) Right-hand input data type of operator
	amopstrategy, -- int2  Operator strategy number
	amoppurpose, -- char  Operator purpose, either s for search or o for ordering
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
	amprocnum, -- int2  Support function number
	amproc::regproc::text as amproc -- regproc (references pg_proc.oid) OID of the function
from
	pg_amproc
	join pg_opfamily as amprocfamily_pg_opfamily on pg_amproc.amprocfamily = amprocfamily_pg_opfamily.oid
	join pg_namespace as amprocfamily_pg_namespace on amprocfamily_pg_opfamily.opfnamespace = amprocfamily_pg_namespace.oid
;

