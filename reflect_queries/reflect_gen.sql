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


--! reflect_pg_roles : (rolconnlimit?, rolvaliduntil?, rolconfig?)
select
	rolname::text as rolname, -- name  Role name
	rolsuper as rolsuper, -- bool  Role has superuser privileges
	rolinherit as rolinherit, -- bool  Role automatically inherits privileges of roles it is a member of
	rolcreaterole as rolcreaterole, -- bool  Role can create more roles
	rolcreatedb as rolcreatedb, -- bool  Role can create databases
	rolcanlogin as rolcanlogin, -- bool  Role can log in. That is, this role can be given as the initial session authorization identifier
	rolreplication as rolreplication, -- bool  Role is a replication role. A replication role can initiate replication connections and create and drop replication slots.
	case when rolconnlimit < 0 then null else rolconnlimit end as rolconnlimit, -- int4  For roles that can log in, this sets maximum number of concurrent connections this role can make. -1 means no limit.
	-- rolpassword text  Not the password (always reads as ********)
	rolvaliduntil as rolvaliduntil, -- timestamptz  Password expiry time (only used for password authentication); null if no expiration
	rolbypassrls as rolbypassrls, -- bool  Role bypasses every row-level security policy, see Section 5.9 for more information.
	rolconfig as rolconfig -- text[]  Role-specific defaults for run-time configuration variables
	-- oid oid (references pg_authid.oid) ID of role
from
	pg_roles
;


--! reflect_pg_auth_members : ()
select
	-- oid oid  Row identifier
	pg_get_userbyid(roleid)::text as roleid, -- oid (references pg_authid.oid) ID of a role that has a member
	pg_get_userbyid(member)::text as member, -- oid (references pg_authid.oid) ID of a role that is a member of roleid
	pg_get_userbyid(grantor)::text as grantor, -- oid (references pg_authid.oid) ID of the role that granted this membership
	admin_option as admin_option, -- bool  True if member can grant membership in roleid to others
	inherit_option as inherit_option, -- bool  True if the member automatically inherits the privileges of the granted role
	set_option as set_option -- bool  True if the member can SET ROLE to the granted role
from
	pg_auth_members
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


--! reflect_pg_default_acl : (defaclnamespace?, defaclacl?)
select
	-- oid oid  Row identifier
	pg_get_userbyid(defaclrole)::text as defaclrole, -- oid (references pg_authid.oid) The OID of the role associated with this entry
	case when defaclnamespace = 0 then null else defaclnamespace::regnamespace::text end as defaclnamespace, -- oid (references pg_namespace.oid) The OID of the namespace associated with this entry, or zero if none
	defaclobjtype as defaclobjtype, -- char  Type of object this entry is for: r = relation (table, view), S = sequence, f = function, T = type, n = schema
	defaclacl::text[] as defaclacl -- aclitem[]  Access privileges that this type of object should have on creation
from
	pg_default_acl
;


--! reflect_pg_language : (lanplcallfoid?, laninline?, lanvalidator?, lanacl?)
select
	-- oid oid  Row identifier
	lanname::text as lanname, -- name  Name of the language
	pg_get_userbyid(lanowner)::text as lanowner, -- oid (references pg_authid.oid) Owner of the language
	lanispl as lanispl, -- bool  This is false for internal languages (such as SQL) and true for user-defined languages. Currently, pg_dump still uses this to determine which languages need to be dumped, but this might be replaced by a different mechanism in the future.
	lanpltrusted as lanpltrusted, -- bool  True if this is a trusted language, which means that it is believed not to grant access to anything outside the normal SQL execution environment. Only superusers can create functions in untrusted languages.
	case when lanplcallfoid = 0 then null else lanplcallfoid::regprocedure::text end as lanplcallfoid, -- oid (references pg_proc.oid) For noninternal languages this references the language handler, which is a special function that is responsible for executing all functions that are written in the particular language. Zero for internal languages.
	case when laninline = 0 then null else laninline::regprocedure::text end as laninline, -- oid (references pg_proc.oid) This references a function that is responsible for executing “inline” anonymous code blocks (DO blocks). Zero if inline blocks are not supported.
	case when lanvalidator = 0 then null else lanvalidator::regprocedure::text end as lanvalidator, -- oid (references pg_proc.oid) This references a language validator function that is responsible for checking the syntax and validity of new functions when they are created. Zero if no validator is provided.
	lanacl::text[] as lanacl -- aclitem[]  Access privileges; see Section 5.8 for details
from
	pg_language
;


--! reflect_pg_namespace : (nspacl?)
select
	-- oid oid  Row identifier
	nspname::text as nspname, -- name  Name of the namespace
	pg_get_userbyid(nspowner)::text as nspowner, -- oid (references pg_authid.oid) Owner of the namespace
	nspacl::text[] as nspacl -- aclitem[]  Access privileges; see Section 5.8 for details
from
	pg_namespace
where 
	nspname != 'pg_toast'
;


--! reflect_pg_type : (typrelid?, typsubscript?, typelem?, typarray?, typreceive?, typsend?, typmodin?, typmodout?, typanalyze?, typbasetype?, typtypmod?, typcollation?, typdefaultbin?, typdefault?, typacl?)
select
	pg_type.oid::regtype::text as oid, -- oid  Row identifier
	typname::text as typname, -- name  Data type name
	typnamespace::regnamespace::text as typnamespace, -- oid (references pg_namespace.oid) The OID of the namespace that contains this type
	pg_get_userbyid(typowner)::text as typowner, -- oid (references pg_authid.oid) Owner of the type
	typlen as typlen, -- int2  For a fixed-size type, typlen is the number of bytes in the internal representation of the type. But for a variable-length type, typlen is negative. -1 indicates a “varlena” type (one that has a length word), -2 indicates a null-terminated C string.
	typbyval as typbyval, -- bool  typbyval determines whether internal routines pass a value of this type by value or by reference. typbyval had better be false if typlen is not 1, 2, or 4 (or 8 on machines where Datum is 8 bytes). Variable-length types are always passed by reference. Note that typbyval can be false even if the length would allow pass-by-value.
	typtype as typtype, -- char  typtype is b for a base type, c for a composite type (e.g., a table's row type), d for a domain, e for an enum type, p for a pseudo-type, r for a range type, or m for a multirange type. See also typrelid and typbasetype.
	-- typcategory char  typcategory is an arbitrary classification of data types that is used by the parser to determine which implicit casts should be “preferred”. See Table 51.65.
	typispreferred as typispreferred, -- bool  True if the type is a preferred cast target within its typcategory
	typisdefined as typisdefined, -- bool  True if the type is defined, false if this is a placeholder entry for a not-yet-defined type. When typisdefined is false, nothing except the type name, namespace, and OID can be relied on.
	typdelim as typdelim, -- char  Character that separates two values of this type when parsing array input. Note that the delimiter is associated with the array element data type, not the array data type.
	case when typrelid = 0 then null else typrelid::regclass::text end as typrelid, -- oid (references pg_class.oid) If this is a composite type (see typtype), then this column points to the pg_class entry that defines the corresponding table. (For a free-standing composite type, the pg_class entry doesn't really represent a table, but it is needed anyway for the type's pg_attribute entries to link to.) Zero for non-composite types.
	case when typsubscript = 0 then null else typsubscript::regproc::text end as typsubscript, -- regproc (references pg_proc.oid) Subscripting handler function's OID, or zero if this type doesn't support subscripting. Types that are “true” array types have typsubscript = array_subscript_handler, but other types may have other handler functions to implement specialized subscripting behavior.
	case when typelem = 0 then null else typelem::regtype::text end as typelem, -- oid (references pg_type.oid) If typelem is not zero then it identifies another row in pg_type, defining the type yielded by subscripting. This should be zero if typsubscript is zero. However, it can be zero when typsubscript isn't zero, if the handler doesn't need typelem to determine the subscripting result type. Note that a typelem dependency is considered to imply physical containment of the element type in this type; so DDL changes on the element type might be restricted by the presence of this type.
	case when typarray = 0 then null else typarray::regtype::text end as typarray, -- oid (references pg_type.oid) If typarray is not zero then it identifies another row in pg_type, which is the “true” array type having this type as element
	typinput::regproc::text as typinput, -- regproc (references pg_proc.oid) Input conversion function (text format)
	typoutput::regproc::text as typoutput, -- regproc (references pg_proc.oid) Output conversion function (text format)
	case when typreceive = 0 then null else typreceive::regproc::text end as typreceive, -- regproc (references pg_proc.oid) Input conversion function (binary format), or zero if none
	case when typsend = 0 then null else typsend::regproc::text end as typsend, -- regproc (references pg_proc.oid) Output conversion function (binary format), or zero if none
	case when typmodin = 0 then null else typmodin::regproc::text end as typmodin, -- regproc (references pg_proc.oid) Type modifier input function, or zero if type does not support modifiers
	case when typmodout = 0 then null else typmodout::regproc::text end as typmodout, -- regproc (references pg_proc.oid) Type modifier output function, or zero to use the standard format
	case when typanalyze = 0 then null else typanalyze::regproc::text end as typanalyze, -- regproc (references pg_proc.oid) Custom ANALYZE function, or zero to use the standard function
	typalign as typalign, -- char  typalign is the alignment required when storing a value of this type. It applies to storage on disk as well as most representations of the value inside PostgreSQL. When multiple values are stored consecutively, such as in the representation of a complete row on disk, padding is inserted before a datum of this type so that it begins on the specified boundary. The alignment reference is the beginning of the first datum in the sequence. Possible values are: c = char alignment, i.e., no alignment needed. s = short alignment (2 bytes on most machines). i = int alignment (4 bytes on most machines). d = double alignment (8 bytes on many machines, but by no means all).
	typstorage as typstorage, -- char  typstorage tells for varlena types (those with typlen = -1) if the type is prepared for toasting and what the default strategy for attributes of this type should be. Possible values are: p (plain): Values must always be stored plain (non-varlena types always use this value). e (external): Values can be stored in a secondary “TOAST” relation (if relation has one, see pg_class.reltoastrelid). m (main): Values can be compressed and stored inline. x (extended): Values can be compressed and/or moved to a secondary relation. x is the usual choice for toast-able types. Note that m values can also be moved out to secondary storage, but only as a last resort (e and x values are moved first).
	typnotnull as typnotnull, -- bool  typnotnull represents a not-null constraint on a type. Used for domains only.
	case when typbasetype = 0 then null else typbasetype::regtype::text end as typbasetype, -- oid (references pg_type.oid) If this is a domain (see typtype), then typbasetype identifies the type that this one is based on. Zero if this type is not a domain.
	case when typtypmod < 0 then null else typtypmod end as typtypmod, -- int4  Domains use typtypmod to record the typmod to be applied to their base type (-1 if base type does not use a typmod). -1 if this type is not a domain.
	typndims as typndims, -- int4  typndims is the number of array dimensions for a domain over an array (that is, typbasetype is an array type). Zero for types other than domains over array types.
	case when typcollation = 0 then null else typcollation::regcollation::text end as typcollation, -- oid (references pg_collation.oid) typcollation specifies the collation of the type. If the type does not support collations, this will be zero. A base type that supports collations will have a nonzero value here, typically DEFAULT_COLLATION_OID. A domain over a collatable type can have a collation OID different from its base type's, if one was specified for the domain.
	pg_get_expr(typdefaultbin, 0) as typdefaultbin, -- pg_node_tree  If typdefaultbin is not null, it is the nodeToString() representation of a default expression for the type. This is only used for domains.
	typdefault as typdefault, -- text  typdefault is null if the type has no associated default value. If typdefaultbin is not null, typdefault must contain a human-readable version of the default expression represented by typdefaultbin. If typdefaultbin is null and typdefault is not, then typdefault is the external representation of the type's default value, which can be fed to the type's input converter to produce a constant.
	typacl::text[] as typacl -- aclitem[]  Access privileges; see Section 5.8 for details
from
	pg_type
;

