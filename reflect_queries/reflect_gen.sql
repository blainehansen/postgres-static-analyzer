--! reflect_pg_aggregate : (aggfinalfn?, aggcombinefn?, aggserialfn?, aggdeserialfn?, aggmtransfn?, aggminvtransfn?, aggmfinalfn?, aggsortop?, aggmtranstype?, agginitval?, aggminitval?)
select
	aggfnoid::regproc::text as aggfnoid, -- regproc (references pg_proc.oid) pg_proc OID of the aggregate function
	pg_aggregate.aggkind as aggkind, -- char  Aggregate kind: n for “normal” aggregates, o for “ordered-set” aggregates, or h for “hypothetical-set” aggregates
	pg_aggregate.aggnumdirectargs as aggnumdirectargs, -- int2  Number of direct (non-aggregated) arguments of an ordered-set or hypothetical-set aggregate, counting a variadic array as one argument. If equal to pronargs, the aggregate must be variadic and the variadic array describes the aggregated arguments as well as the final direct arguments. Always zero for normal aggregates.
	aggtransfn::regproc::text as aggtransfn, -- regproc (references pg_proc.oid) Transition function
	case when aggfinalfn = 0 then null else aggfinalfn::regproc::text end as aggfinalfn, -- regproc (references pg_proc.oid) Final function (zero if none)
	case when aggcombinefn = 0 then null else aggcombinefn::regproc::text end as aggcombinefn, -- regproc (references pg_proc.oid) Combine function (zero if none)
	case when aggserialfn = 0 then null else aggserialfn::regproc::text end as aggserialfn, -- regproc (references pg_proc.oid) Serialization function (zero if none)
	case when aggdeserialfn = 0 then null else aggdeserialfn::regproc::text end as aggdeserialfn, -- regproc (references pg_proc.oid) Deserialization function (zero if none)
	case when aggmtransfn = 0 then null else aggmtransfn::regproc::text end as aggmtransfn, -- regproc (references pg_proc.oid) Forward transition function for moving-aggregate mode (zero if none)
	case when aggminvtransfn = 0 then null else aggminvtransfn::regproc::text end as aggminvtransfn, -- regproc (references pg_proc.oid) Inverse transition function for moving-aggregate mode (zero if none)
	case when aggmfinalfn = 0 then null else aggmfinalfn::regproc::text end as aggmfinalfn, -- regproc (references pg_proc.oid) Final function for moving-aggregate mode (zero if none)
	pg_aggregate.aggfinalextra as aggfinalextra, -- bool  True to pass extra dummy arguments to aggfinalfn
	pg_aggregate.aggmfinalextra as aggmfinalextra, -- bool  True to pass extra dummy arguments to aggmfinalfn
	pg_aggregate.aggfinalmodify as aggfinalmodify, -- char  Whether aggfinalfn modifies the transition state value: r if it is read-only, s if the aggtransfn cannot be applied after the aggfinalfn, or w if it writes on the value
	pg_aggregate.aggmfinalmodify as aggmfinalmodify, -- char  Like aggfinalmodify, but for the aggmfinalfn
	case when pg_aggregate.aggsortop = 0 then null else pg_aggregate.aggsortop::regoperator::text end as aggsortop, -- oid (references pg_operator.oid) Associated sort operator (zero if none)
	pg_aggregate.aggtranstype::regtype::text as aggtranstype, -- oid (references pg_type.oid) Data type of the aggregate function's internal transition (state) data
	-- aggtransspace int4  Approximate average size (in bytes) of the transition state data, or zero to use a default estimate
	case when pg_aggregate.aggmtranstype = 0 then null else pg_aggregate.aggmtranstype::regtype::text end as aggmtranstype, -- oid (references pg_type.oid) Data type of the aggregate function's internal transition (state) data for moving-aggregate mode (zero if none)
	-- aggmtransspace int4  Approximate average size (in bytes) of the transition state data for moving-aggregate mode, or zero to use a default estimate
	pg_aggregate.agginitval as agginitval, -- text  The initial value of the transition state. This is a text field containing the initial value in its external string representation. If this field is null, the transition state value starts out null.
	pg_aggregate.aggminitval as aggminitval -- text  The initial value of the transition state for moving-aggregate mode. This is a text field containing the initial value in its external string representation. If this field is null, the transition state value starts out null.
from
	pg_aggregate
;


--! reflect_pg_am : ()
select
	-- oid oid  Row identifier
	pg_am.amname::text as amname, -- name  Name of the access method
	amhandler::regproc::text as amhandler, -- regproc (references pg_proc.oid) OID of a handler function that is responsible for supplying information about the access method
	pg_am.amtype as amtype -- char  t = table (including materialized views), i = index.
from
	pg_am
;


--! reflect_pg_amop : (amopsortfamily?)
select
	-- oid oid  Row identifier
	quote_ident(amopfamily_pg_namespace.nspname) || '.' || quote_ident(amopfamily_pg_opfamily.opfname) as amopfamily, -- oid (references pg_opfamily.oid) The operator family this entry is for
	pg_amop.amoplefttype::regtype::text as amoplefttype, -- oid (references pg_type.oid) Left-hand input data type of operator
	pg_amop.amoprighttype::regtype::text as amoprighttype, -- oid (references pg_type.oid) Right-hand input data type of operator
	pg_amop.amopstrategy as amopstrategy, -- int2  Operator strategy number
	pg_amop.amoppurpose as amoppurpose, -- char  Operator purpose, either s for search or o for ordering
	pg_amop.amopopr::regoperator::text as amopopr, -- oid (references pg_operator.oid) OID of the operator
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
	pg_amproc.amproclefttype::regtype::text as amproclefttype, -- oid (references pg_type.oid) Left-hand input data type of associated operator
	pg_amproc.amprocrighttype::regtype::text as amprocrighttype, -- oid (references pg_type.oid) Right-hand input data type of associated operator
	pg_amproc.amprocnum as amprocnum, -- int2  Support function number
	amproc::regproc::text as amproc -- regproc (references pg_proc.oid) OID of the function
from
	pg_amproc
	join pg_opfamily as amprocfamily_pg_opfamily on pg_amproc.amprocfamily = amprocfamily_pg_opfamily.oid
	join pg_namespace as amprocfamily_pg_namespace on amprocfamily_pg_opfamily.opfnamespace = amprocfamily_pg_namespace.oid
;


--! reflect_pg_attrdef : ()
select
	-- oid oid  Row identifier
	pg_attrdef.adrelid::regclass::text as adrelid, -- oid (references pg_class.oid) The table this column belongs to
	pg_attrdef.adnum as adnum, -- int2 (references pg_attribute.attnum) The number of the column
	pg_get_expr(pg_attrdef.adbin, pg_attrdef.adrelid) as adbin -- pg_node_tree  The column default value, in nodeToString() representation. Use pg_get_expr(adbin, adrelid) to convert it to an SQL expression.
from
	pg_attrdef
where 
	adnum > 0
;


--! reflect_pg_attribute : (atttypid?, atttypmod?, attcompression?, attidentity?, attgenerated?, attcollation?, attstattarget?, attacl?, attoptions?, attfdwoptions?)
select
	pg_attribute.attrelid::regclass::text as attrelid, -- oid (references pg_class.oid) The table this column belongs to
	pg_attribute.attname::text as attname, -- name  The column name
	case when pg_attribute.atttypid = 0 then null else pg_attribute.atttypid::regtype::text end as atttypid, -- oid (references pg_type.oid) The data type of this column (zero for a dropped column)
	-- attlen int2  A copy of pg_type.typlen of this column's type
	pg_attribute.attnum as attnum, -- int2  The number of the column. Ordinary columns are numbered from 1 up. System columns, such as ctid, have (arbitrary) negative numbers.
	-- attcacheoff int4  Always -1 in storage, but when loaded into a row descriptor in memory this might be updated to cache the offset of the attribute within the row
	case when atttypmod < 0 then null else atttypmod end as atttypmod, -- int4  atttypmod records type-specific data supplied at table creation time (for example, the maximum length of a varchar column). It is passed to type-specific input functions and length coercion functions. The value will generally be -1 for types that do not need atttypmod.
	pg_attribute.attndims as attndims, -- int2  Number of dimensions, if the column is an array type; otherwise 0. (Presently, the number of dimensions of an array is not enforced, so any nonzero value effectively means “it's an array”.)
	-- attbyval bool  A copy of pg_type.typbyval of this column's type
	-- attalign char  A copy of pg_type.typalign of this column's type
	-- attstorage char  Normally a copy of pg_type.typstorage of this column's type. For TOAST-able data types, this can be altered after column creation to control storage policy.
	case when pg_attribute.attcompression = '' then null else pg_attribute.attcompression end as attcompression, -- char  The current compression method of the column. Typically this is '\0' to specify use of the current default setting (see default_toast_compression). Otherwise, 'p' selects pglz compression, while 'l' selects LZ4 compression. However, this field is ignored whenever attstorage does not allow compression.
	pg_attribute.attnotnull as attnotnull, -- bool  This represents a not-null constraint.
	pg_attribute.atthasdef as atthasdef, -- bool  This column has a default expression or generation expression, in which case there will be a corresponding entry in the pg_attrdef catalog that actually defines the expression. (Check attgenerated to determine whether this is a default or a generation expression.)
	-- atthasmissing bool  This column has a value which is used where the column is entirely missing from the row, as happens when a column is added with a non-volatile DEFAULT value after the row is created. The actual value used is stored in the attmissingval column.
	case when pg_attribute.attidentity = '' then null else pg_attribute.attidentity end as attidentity, -- char  If a zero byte (''), then not an identity column. Otherwise, a = generated always, d = generated by default.
	case when pg_attribute.attgenerated = '' then null else pg_attribute.attgenerated end as attgenerated, -- char  If a zero byte (''), then not a generated column. Otherwise, s = stored. (Other values might be added in the future.)
	pg_attribute.attisdropped as attisdropped, -- bool  This column has been dropped and is no longer valid. A dropped column is still physically present in the table, but is ignored by the parser and so cannot be accessed via SQL.
	pg_attribute.attislocal as attislocal, -- bool  This column is defined locally in the relation. Note that a column can be locally defined and inherited simultaneously.
	pg_attribute.attinhcount as attinhcount, -- int2  The number of direct ancestors this column has. A column with a nonzero number of ancestors cannot be dropped nor renamed.
	case when pg_attribute.attcollation = 0 then null else pg_attribute.attcollation::regcollation::text end as attcollation, -- oid (references pg_collation.oid) The defined collation of the column, or zero if the column is not of a collatable data type
	pg_attribute.attstattarget as attstattarget, -- int2  attstattarget controls the level of detail of statistics accumulated for this column by ANALYZE. A zero value indicates that no statistics should be collected. A null value says to use the system default statistics target. The exact meaning of positive values is data type-dependent. For scalar data types, attstattarget is both the target number of “most common values” to collect, and the target number of histogram bins to create.
	attacl::text[] as attacl, -- aclitem[]  Column-level access privileges, if any have been granted specifically on this column
	pg_attribute.attoptions as attoptions, -- text[]  Attribute-level options, as “keyword=value” strings
	pg_attribute.attfdwoptions as attfdwoptions -- text[]  Attribute-level foreign data wrapper options, as “keyword=value” strings
	-- attmissingval anyarray  This column has a one element array containing the value used when the column is entirely missing from the row, as happens when the column is added with a non-volatile DEFAULT value after the row is created. The value is only used when atthasmissing is true. If there is no value the column is null.
from
	pg_attribute
where 
	not starts_with(pg_attribute.attrelid::regclass::text, 'pg_toast')
	and attnum > 0
	and not pg_attribute.attisdropped
;


--! reflect_pg_roles : (rolconnlimit?, rolvaliduntil?, rolconfig?)
select
	pg_roles.rolname::text as rolname, -- name  Role name
	pg_roles.rolsuper as rolsuper, -- bool  Role has superuser privileges
	pg_roles.rolinherit as rolinherit, -- bool  Role automatically inherits privileges of roles it is a member of
	pg_roles.rolcreaterole as rolcreaterole, -- bool  Role can create more roles
	pg_roles.rolcreatedb as rolcreatedb, -- bool  Role can create databases
	pg_roles.rolcanlogin as rolcanlogin, -- bool  Role can log in. That is, this role can be given as the initial session authorization identifier
	pg_roles.rolreplication as rolreplication, -- bool  Role is a replication role. A replication role can initiate replication connections and create and drop replication slots.
	case when rolconnlimit < 0 then null else rolconnlimit end as rolconnlimit, -- int4  For roles that can log in, this sets maximum number of concurrent connections this role can make. -1 means no limit.
	-- rolpassword text  Not the password (always reads as ********)
	pg_roles.rolvaliduntil as rolvaliduntil, -- timestamptz  Password expiry time (only used for password authentication); null if no expiration
	pg_roles.rolbypassrls as rolbypassrls, -- bool  Role bypasses every row-level security policy, see Section 5.9 for more information.
	pg_roles.rolconfig as rolconfig -- text[]  Role-specific defaults for run-time configuration variables
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
	pg_auth_members.admin_option as admin_option, -- bool  True if member can grant membership in roleid to others
	pg_auth_members.inherit_option as inherit_option, -- bool  True if the member automatically inherits the privileges of the granted role
	pg_auth_members.set_option as set_option -- bool  True if the member can SET ROLE to the granted role
from
	pg_auth_members
;


--! reflect_pg_cast : (castfunc?)
select
	-- oid oid  Row identifier
	pg_cast.castsource::regtype::text as castsource, -- oid (references pg_type.oid) OID of the source data type
	pg_cast.casttarget::regtype::text as casttarget, -- oid (references pg_type.oid) OID of the target data type
	case when pg_cast.castfunc = 0 then null else pg_cast.castfunc::regprocedure::text end as castfunc, -- oid (references pg_proc.oid) The OID of the function to use to perform this cast. Zero is stored if the cast method doesn't require a function.
	pg_cast.castcontext as castcontext, -- char  Indicates what contexts the cast can be invoked in. e means only as an explicit cast (using CAST or :: syntax). a means implicitly in assignment to a target column, as well as explicitly. i means implicitly in expressions, as well as the other cases.
	pg_cast.castmethod as castmethod -- char  Indicates how the cast is performed. f means that the function specified in the castfunc field is used. i means that the input/output functions are used. b means that the types are binary-coercible, thus no conversion is required.
from
	pg_cast
;


--! reflect_pg_class : (reltype?, reloftype?, relam?, relacl?, reloptions?, relpartbound?)
select
	pg_class.oid::regclass::text as oid, -- oid  Row identifier
	pg_class.relname::text as relname, -- name  Name of the table, index, view, etc.
	pg_class.relnamespace::regnamespace::text as relnamespace, -- oid (references pg_namespace.oid) The OID of the namespace that contains this relation
	case when pg_class.reltype = 0 then null else pg_class.reltype::regtype::text end as reltype, -- oid (references pg_type.oid) The OID of the data type that corresponds to this table's row type, if any; zero for indexes, sequences, and toast tables, which have no pg_type entry
	case when pg_class.reloftype = 0 then null else pg_class.reloftype::regtype::text end as reloftype, -- oid (references pg_type.oid) For typed tables, the OID of the underlying composite type; zero for all other relations
	pg_get_userbyid(relowner)::text as relowner, -- oid (references pg_authid.oid) Owner of the relation
	relam_pg_am.amname::text as relam, -- oid (references pg_am.oid) The access method used to access this table or index. Not meaningful if the relation is a sequence or has no on-disk file, except for partitioned tables, where, if set, it takes precedence over default_table_access_method when determining the access method to use for partitions created when one is not specified in the creation command.
	-- relfilenode oid  Name of the on-disk file of this relation; zero means this is a “mapped” relation whose disk file name is determined by low-level state
	-- reltablespace oid (references pg_tablespace.oid) The tablespace in which this relation is stored. If zero, the database's default tablespace is implied. Not meaningful if the relation has no on-disk file, except for partitioned tables, where this is the tablespace in which partitions will be created when one is not specified in the creation command.
	-- relpages int4  Size of the on-disk representation of this table in pages (of size BLCKSZ). This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX.
	-- reltuples float4  Number of live rows in the table. This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX. If the table has never yet been vacuumed or analyzed, reltuples contains -1 indicating that the row count is unknown.
	-- relallvisible int4  Number of pages that are marked all-visible in the table's visibility map. This is only an estimate used by the planner. It is updated by VACUUM, ANALYZE, and a few DDL commands such as CREATE INDEX.
	-- reltoastrelid oid (references pg_class.oid) OID of the TOAST table associated with this table, zero if none. The TOAST table stores large attributes “out of line” in a secondary table.
	-- relhasindex bool  True if this is a table and it has (or recently had) any indexes
	pg_class.relisshared as relisshared, -- bool  True if this table is shared across all databases in the cluster. Only certain system catalogs (such as pg_database) are shared.
	pg_class.relpersistence as relpersistence, -- char  p = permanent table/sequence, u = unlogged table/sequence, t = temporary table/sequence
	pg_class.relkind as relkind, -- char  r = ordinary table, i = index, S = sequence, t = TOAST table, v = view, m = materialized view, c = composite type, f = foreign table, p = partitioned table, I = partitioned index
	pg_class.relnatts as relnatts, -- int2  Number of user columns in the relation (system columns not counted). There must be this many corresponding entries in pg_attribute. See also pg_attribute.attnum.
	pg_class.relchecks as relchecks, -- int2  Number of CHECK constraints on the table; see pg_constraint catalog
	-- relhasrules bool  True if table has (or once had) rules; see pg_rewrite catalog
	-- relhastriggers bool  True if table has (or once had) triggers; see pg_trigger catalog
	-- relhassubclass bool  True if table or index has (or once had) any inheritance children or partitions
	pg_class.relrowsecurity as relrowsecurity, -- bool  True if table has row-level security enabled; see pg_policy catalog
	pg_class.relforcerowsecurity as relforcerowsecurity, -- bool  True if row-level security (when enabled) will also apply to table owner; see pg_policy catalog
	-- relispopulated bool  True if relation is populated (this is true for all relations other than some materialized views)
	pg_class.relreplident as relreplident, -- char  Columns used to form “replica identity” for rows: d = default (primary key, if any), n = nothing, f = all columns, i = index with indisreplident set (same as nothing if the index used has been dropped)
	pg_class.relispartition as relispartition, -- bool  True if table or index is a partition
	-- relrewrite oid (references pg_class.oid) For new relations being written during a DDL operation that requires a table rewrite, this contains the OID of the original relation; otherwise zero. That state is only visible internally; this field should never contain anything other than zero for a user-visible relation.
	-- relfrozenxid xid  All transaction IDs before this one have been replaced with a permanent (“frozen”) transaction ID in this table. This is used to track whether the table needs to be vacuumed in order to prevent transaction ID wraparound or to allow pg_xact to be shrunk. Zero (InvalidTransactionId) if the relation is not a table.
	-- relminmxid xid  All multixact IDs before this one have been replaced by a transaction ID in this table. This is used to track whether the table needs to be vacuumed in order to prevent multixact ID wraparound or to allow pg_multixact to be shrunk. Zero (InvalidMultiXactId) if the relation is not a table.
	relacl::text[] as relacl, -- aclitem[]  Access privileges; see Section 5.8 for details
	pg_class.reloptions as reloptions, -- text[]  Access-method-specific options, as “keyword=value” strings
	pg_get_expr(relpartbound, pg_class.oid) as relpartbound -- pg_node_tree  If table is a partition (see relispartition), internal representation of the partition bound
from
	pg_class
	left join pg_am as relam_pg_am on pg_class.relam = relam_pg_am.oid
where 
	relnamespace::regnamespace != 'pg_toast'::regnamespace
;


--! reflect_pg_collation : (collencoding?, collcollate?, collctype?, colllocale?, collicurules?, collversion?)
select
	pg_collation.oid::regcollation::text as oid, -- oid  Row identifier
	pg_collation.collname::text as collname, -- name  Collation name (unique per namespace and encoding)
	pg_collation.collnamespace::regnamespace::text as collnamespace, -- oid (references pg_namespace.oid) The OID of the namespace that contains this collation
	pg_get_userbyid(collowner)::text as collowner, -- oid (references pg_authid.oid) Owner of the collation
	pg_collation.collprovider as collprovider, -- char  Provider of the collation: d = database default, b = builtin, c = libc, i = icu
	pg_collation.collisdeterministic as collisdeterministic, -- bool  Is the collation deterministic?
	case when collencoding < 0 then null else pg_encoding_to_char(collencoding)::text end as collencoding, -- int4  Encoding in which the collation is applicable, or -1 if it works for any encoding
	pg_collation.collcollate as collcollate, -- text  LC_COLLATE for this collation object. If the provider is not libc, collcollate is NULL and colllocale is used instead.
	pg_collation.collctype as collctype, -- text  LC_CTYPE for this collation object. If the provider is not libc, collctype is NULL and colllocale is used instead.
	pg_collation.colllocale as colllocale, -- text  Collation provider locale name for this collation object. If the provider is libc, colllocale is NULL; collcollate and collctype are used instead.
	pg_collation.collicurules as collicurules, -- text  ICU collation rules for this collation object
	pg_collation.collversion as collversion -- text  Provider-specific version of the collation. This is recorded when the collation is created and then checked when it is used, to detect changes in the collation definition that could lead to data corruption.
from
	pg_collation
;


--! reflect_pg_constraint : (conrelid?, contypid?, conindid?, conparentid?, confrelid?, confupdtype?, confdeltype?, confmatchtype?, conkey?, confkey?, conpfeqop?, conppeqop?, conffeqop?, confdelsetcols?, conexclop?, conbin?)
select
	-- oid oid  Row identifier
	pg_constraint.conname::text as conname, -- name  Constraint name (not necessarily unique!)
	pg_constraint.connamespace::regnamespace::text as connamespace, -- oid (references pg_namespace.oid) The OID of the namespace that contains this constraint
	pg_constraint.contype as contype, -- char  c = check constraint, f = foreign key constraint, n = not-null constraint (domains only), p = primary key constraint, u = unique constraint, t = constraint trigger, x = exclusion constraint
	pg_constraint.condeferrable as condeferrable, -- bool  Is the constraint deferrable?
	pg_constraint.condeferred as condeferred, -- bool  Is the constraint deferred by default?
	pg_constraint.convalidated as convalidated, -- bool  Has the constraint been validated? Currently, can be false only for foreign keys and CHECK constraints
	case when pg_constraint.conrelid = 0 then null else pg_constraint.conrelid::regclass::text end as conrelid, -- oid (references pg_class.oid) The table this constraint is on; zero if not a table constraint
	case when pg_constraint.contypid = 0 then null else pg_constraint.contypid::regtype::text end as contypid, -- oid (references pg_type.oid) The domain this constraint is on; zero if not a domain constraint
	case when pg_constraint.conindid = 0 then null else pg_constraint.conindid::regclass::text end as conindid, -- oid (references pg_class.oid) The index supporting this constraint, if it's a unique, primary key, foreign key, or exclusion constraint; else zero
	quote_ident(conparentid_pg_namespace.nspname) || '.' || quote_ident(conparentid_pg_constraint.conname) as conparentid, -- oid (references pg_constraint.oid) The corresponding constraint of the parent partitioned table, if this is a constraint on a partition; else zero
	case when pg_constraint.confrelid = 0 then null else pg_constraint.confrelid::regclass::text end as confrelid, -- oid (references pg_class.oid) If a foreign key, the referenced table; else zero
	case when pg_constraint.confupdtype = ' ' then null else pg_constraint.confupdtype end as confupdtype, -- char  Foreign key update action code: a = no action, r = restrict, c = cascade, n = set null, d = set default
	case when pg_constraint.confdeltype = ' ' then null else pg_constraint.confdeltype end as confdeltype, -- char  Foreign key deletion action code: a = no action, r = restrict, c = cascade, n = set null, d = set default
	case when pg_constraint.confmatchtype = ' ' then null else pg_constraint.confmatchtype end as confmatchtype, -- char  Foreign key match type: f = full, p = partial, s = simple
	pg_constraint.conislocal as conislocal, -- bool  This constraint is defined locally for the relation. Note that a constraint can be locally defined and inherited simultaneously.
	pg_constraint.coninhcount as coninhcount, -- int2  The number of direct inheritance ancestors this constraint has. A constraint with a nonzero number of ancestors cannot be dropped nor renamed.
	pg_constraint.connoinherit as connoinherit, -- bool  This constraint is defined locally for the relation. It is a non-inheritable constraint.
	pg_constraint.conkey as conkey, -- int2[] (references pg_attribute.attnum) If a table constraint (including foreign keys, but not constraint triggers), list of the constrained columns
	pg_constraint.confkey as confkey, -- int2[] (references pg_attribute.attnum) If a foreign key, list of the referenced columns
	pg_constraint.conpfeqop::regoperator[]::text[] as conpfeqop, -- oid[] (references pg_operator.oid) If a foreign key, list of the equality operators for PK = FK comparisons
	pg_constraint.conppeqop::regoperator[]::text[] as conppeqop, -- oid[] (references pg_operator.oid) If a foreign key, list of the equality operators for PK = PK comparisons
	pg_constraint.conffeqop::regoperator[]::text[] as conffeqop, -- oid[] (references pg_operator.oid) If a foreign key, list of the equality operators for FK = FK comparisons
	pg_constraint.confdelsetcols as confdelsetcols, -- int2[] (references pg_attribute.attnum) If a foreign key with a SET NULL or SET DEFAULT delete action, the columns that will be updated. If null, all of the referencing columns will be updated.
	pg_constraint.conexclop::regoperator[]::text[] as conexclop, -- oid[] (references pg_operator.oid) If an exclusion constraint, list of the per-column exclusion operators
	pg_get_expr(pg_constraint.conbin, pg_constraint.conrelid) as conbin -- pg_node_tree  If a check constraint, an internal representation of the expression. (It's recommended to use pg_get_constraintdef() to extract the definition of a check constraint.)
from
	pg_constraint
	left join pg_constraint as conparentid_pg_constraint on pg_constraint.conparentid = conparentid_pg_constraint.oid
	left join pg_namespace as conparentid_pg_namespace on conparentid_pg_constraint.connamespace = conparentid_pg_namespace.oid
where 
	(pg_constraint.conkey is null or not (0 >= any(pg_constraint.conkey)))
	and (pg_constraint.confkey is null or not (0 >= any(pg_constraint.confkey)))
	and (pg_constraint.confdelsetcols is null or not (0 >= any(pg_constraint.confdelsetcols)))
;


--! reflect_pg_conversion : ()
select
	-- oid oid  Row identifier
	pg_conversion.conname::text as conname, -- name  Conversion name (unique within a namespace)
	pg_conversion.connamespace::regnamespace::text as connamespace, -- oid (references pg_namespace.oid) The OID of the namespace that contains this conversion
	pg_get_userbyid(conowner)::text as conowner, -- oid (references pg_authid.oid) Owner of the conversion
	pg_encoding_to_char(conforencoding)::text as conforencoding, -- int4  Source encoding ID (pg_encoding_to_char() can translate this number to the encoding name)
	pg_encoding_to_char(contoencoding)::text as contoencoding, -- int4  Destination encoding ID (pg_encoding_to_char() can translate this number to the encoding name)
	conproc::regproc::text as conproc, -- regproc (references pg_proc.oid) Conversion function
	pg_conversion.condefault as condefault -- bool  True if this is the default conversion
from
	pg_conversion
;


--! reflect_pg_default_acl : (defaclnamespace?, defaclacl?)
select
	-- oid oid  Row identifier
	pg_get_userbyid(defaclrole)::text as defaclrole, -- oid (references pg_authid.oid) The OID of the role associated with this entry
	case when pg_default_acl.defaclnamespace = 0 then null else pg_default_acl.defaclnamespace::regnamespace::text end as defaclnamespace, -- oid (references pg_namespace.oid) The OID of the namespace associated with this entry, or zero if none
	pg_default_acl.defaclobjtype as defaclobjtype, -- char  Type of object this entry is for: r = relation (table, view), S = sequence, f = function, T = type, n = schema
	defaclacl::text[] as defaclacl -- aclitem[]  Access privileges that this type of object should have on creation
from
	pg_default_acl
;


--! reflect_pg_event_trigger : (evttags?)
select
	-- oid oid  Row identifier
	pg_event_trigger.evtname::text as evtname, -- name  Trigger name (must be unique)
	pg_event_trigger.evtevent::text as evtevent, -- name  Identifies the event for which this trigger fires
	pg_get_userbyid(evtowner)::text as evtowner, -- oid (references pg_authid.oid) Owner of the event trigger
	pg_event_trigger.evtfoid::regprocedure::text as evtfoid, -- oid (references pg_proc.oid) The function to be called
	pg_event_trigger.evtenabled as evtenabled, -- char  Controls in which session_replication_role modes the event trigger fires. O = trigger fires in “origin” and “local” modes, D = trigger is disabled, R = trigger fires in “replica” mode, A = trigger fires always.
	pg_event_trigger.evttags as evttags -- text[]  Command tags for which this trigger will fire. If NULL, the firing of this trigger is not restricted on the basis of the command tag.
from
	pg_event_trigger
;


--! reflect_pg_language : (lanplcallfoid?, laninline?, lanvalidator?, lanacl?)
select
	-- oid oid  Row identifier
	pg_language.lanname::text as lanname, -- name  Name of the language
	pg_get_userbyid(lanowner)::text as lanowner, -- oid (references pg_authid.oid) Owner of the language
	pg_language.lanispl as lanispl, -- bool  This is false for internal languages (such as SQL) and true for user-defined languages. Currently, pg_dump still uses this to determine which languages need to be dumped, but this might be replaced by a different mechanism in the future.
	pg_language.lanpltrusted as lanpltrusted, -- bool  True if this is a trusted language, which means that it is believed not to grant access to anything outside the normal SQL execution environment. Only superusers can create functions in untrusted languages.
	case when pg_language.lanplcallfoid = 0 then null else pg_language.lanplcallfoid::regprocedure::text end as lanplcallfoid, -- oid (references pg_proc.oid) For noninternal languages this references the language handler, which is a special function that is responsible for executing all functions that are written in the particular language. Zero for internal languages.
	case when pg_language.laninline = 0 then null else pg_language.laninline::regprocedure::text end as laninline, -- oid (references pg_proc.oid) This references a function that is responsible for executing “inline” anonymous code blocks (DO blocks). Zero if inline blocks are not supported.
	case when pg_language.lanvalidator = 0 then null else pg_language.lanvalidator::regprocedure::text end as lanvalidator, -- oid (references pg_proc.oid) This references a language validator function that is responsible for checking the syntax and validity of new functions when they are created. Zero if no validator is provided.
	lanacl::text[] as lanacl -- aclitem[]  Access privileges; see Section 5.8 for details
from
	pg_language
;


--! reflect_pg_namespace : (nspacl?)
select
	-- oid oid  Row identifier
	pg_namespace.nspname::text as nspname, -- name  Name of the namespace
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
	pg_type.typname::text as typname, -- name  Data type name
	pg_type.typnamespace::regnamespace::text as typnamespace, -- oid (references pg_namespace.oid) The OID of the namespace that contains this type
	pg_get_userbyid(typowner)::text as typowner, -- oid (references pg_authid.oid) Owner of the type
	pg_type.typlen as typlen, -- int2  For a fixed-size type, typlen is the number of bytes in the internal representation of the type. But for a variable-length type, typlen is negative. -1 indicates a “varlena” type (one that has a length word), -2 indicates a null-terminated C string.
	pg_type.typbyval as typbyval, -- bool  typbyval determines whether internal routines pass a value of this type by value or by reference. typbyval had better be false if typlen is not 1, 2, or 4 (or 8 on machines where Datum is 8 bytes). Variable-length types are always passed by reference. Note that typbyval can be false even if the length would allow pass-by-value.
	pg_type.typtype as typtype, -- char  typtype is b for a base type, c for a composite type (e.g., a table's row type), d for a domain, e for an enum type, p for a pseudo-type, r for a range type, or m for a multirange type. See also typrelid and typbasetype.
	-- typcategory char  typcategory is an arbitrary classification of data types that is used by the parser to determine which implicit casts should be “preferred”. See Table 51.65.
	pg_type.typispreferred as typispreferred, -- bool  True if the type is a preferred cast target within its typcategory
	pg_type.typisdefined as typisdefined, -- bool  True if the type is defined, false if this is a placeholder entry for a not-yet-defined type. When typisdefined is false, nothing except the type name, namespace, and OID can be relied on.
	pg_type.typdelim as typdelim, -- char  Character that separates two values of this type when parsing array input. Note that the delimiter is associated with the array element data type, not the array data type.
	case when pg_type.typrelid = 0 then null else pg_type.typrelid::regclass::text end as typrelid, -- oid (references pg_class.oid) If this is a composite type (see typtype), then this column points to the pg_class entry that defines the corresponding table. (For a free-standing composite type, the pg_class entry doesn't really represent a table, but it is needed anyway for the type's pg_attribute entries to link to.) Zero for non-composite types.
	case when typsubscript = 0 then null else typsubscript::regproc::text end as typsubscript, -- regproc (references pg_proc.oid) Subscripting handler function's OID, or zero if this type doesn't support subscripting. Types that are “true” array types have typsubscript = array_subscript_handler, but other types may have other handler functions to implement specialized subscripting behavior.
	case when pg_type.typelem = 0 then null else pg_type.typelem::regtype::text end as typelem, -- oid (references pg_type.oid) If typelem is not zero then it identifies another row in pg_type, defining the type yielded by subscripting. This should be zero if typsubscript is zero. However, it can be zero when typsubscript isn't zero, if the handler doesn't need typelem to determine the subscripting result type. Note that a typelem dependency is considered to imply physical containment of the element type in this type; so DDL changes on the element type might be restricted by the presence of this type.
	case when pg_type.typarray = 0 then null else pg_type.typarray::regtype::text end as typarray, -- oid (references pg_type.oid) If typarray is not zero then it identifies another row in pg_type, which is the “true” array type having this type as element
	typinput::regproc::text as typinput, -- regproc (references pg_proc.oid) Input conversion function (text format)
	typoutput::regproc::text as typoutput, -- regproc (references pg_proc.oid) Output conversion function (text format)
	case when typreceive = 0 then null else typreceive::regproc::text end as typreceive, -- regproc (references pg_proc.oid) Input conversion function (binary format), or zero if none
	case when typsend = 0 then null else typsend::regproc::text end as typsend, -- regproc (references pg_proc.oid) Output conversion function (binary format), or zero if none
	case when typmodin = 0 then null else typmodin::regproc::text end as typmodin, -- regproc (references pg_proc.oid) Type modifier input function, or zero if type does not support modifiers
	case when typmodout = 0 then null else typmodout::regproc::text end as typmodout, -- regproc (references pg_proc.oid) Type modifier output function, or zero to use the standard format
	case when typanalyze = 0 then null else typanalyze::regproc::text end as typanalyze, -- regproc (references pg_proc.oid) Custom ANALYZE function, or zero to use the standard function
	pg_type.typalign as typalign, -- char  typalign is the alignment required when storing a value of this type. It applies to storage on disk as well as most representations of the value inside PostgreSQL. When multiple values are stored consecutively, such as in the representation of a complete row on disk, padding is inserted before a datum of this type so that it begins on the specified boundary. The alignment reference is the beginning of the first datum in the sequence. Possible values are: c = char alignment, i.e., no alignment needed. s = short alignment (2 bytes on most machines). i = int alignment (4 bytes on most machines). d = double alignment (8 bytes on many machines, but by no means all).
	pg_type.typstorage as typstorage, -- char  typstorage tells for varlena types (those with typlen = -1) if the type is prepared for toasting and what the default strategy for attributes of this type should be. Possible values are: p (plain): Values must always be stored plain (non-varlena types always use this value). e (external): Values can be stored in a secondary “TOAST” relation (if relation has one, see pg_class.reltoastrelid). m (main): Values can be compressed and stored inline. x (extended): Values can be compressed and/or moved to a secondary relation. x is the usual choice for toast-able types. Note that m values can also be moved out to secondary storage, but only as a last resort (e and x values are moved first).
	pg_type.typnotnull as typnotnull, -- bool  typnotnull represents a not-null constraint on a type. Used for domains only.
	case when pg_type.typbasetype = 0 then null else pg_type.typbasetype::regtype::text end as typbasetype, -- oid (references pg_type.oid) If this is a domain (see typtype), then typbasetype identifies the type that this one is based on. Zero if this type is not a domain.
	case when typtypmod < 0 then null else typtypmod end as typtypmod, -- int4  Domains use typtypmod to record the typmod to be applied to their base type (-1 if base type does not use a typmod). -1 if this type is not a domain.
	pg_type.typndims as typndims, -- int4  typndims is the number of array dimensions for a domain over an array (that is, typbasetype is an array type). Zero for types other than domains over array types.
	case when pg_type.typcollation = 0 then null else pg_type.typcollation::regcollation::text end as typcollation, -- oid (references pg_collation.oid) typcollation specifies the collation of the type. If the type does not support collations, this will be zero. A base type that supports collations will have a nonzero value here, typically DEFAULT_COLLATION_OID. A domain over a collatable type can have a collation OID different from its base type's, if one was specified for the domain.
	pg_get_expr(typdefaultbin, 0) as typdefaultbin, -- pg_node_tree  If typdefaultbin is not null, it is the nodeToString() representation of a default expression for the type. This is only used for domains.
	pg_type.typdefault as typdefault, -- text  typdefault is null if the type has no associated default value. If typdefaultbin is not null, typdefault must contain a human-readable version of the default expression represented by typdefaultbin. If typdefaultbin is null and typdefault is not, then typdefault is the external representation of the type's default value, which can be fed to the type's input converter to produce a constant.
	typacl::text[] as typacl -- aclitem[]  Access privileges; see Section 5.8 for details
from
	pg_type
;

