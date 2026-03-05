pub use reflect::reflect_db_state;

// use sqlparser::ast::Statement as SqlStatement;
use pg_query::{Node, NodeEnum};

fn nodes_to_enum(nodes: Vec<Node>) -> Vec<NodeEnum> {
	nodes.into_iter().filter_map(|n| n.node).collect()
}

pub(crate) use reflect_crate::tokio_postgres::{self as postgres, /*Config as PgConfig,*/ Client as PgClient};

pub type Set<T> = hashbrown::HashSet<T>;
pub type Map<T> = hashbrown::HashMap<String, T>;
pub(crate) use hashbrown::HashMap;

mod reflect;
#[cfg(test)]
mod reflect_test;


/// This error is given when a sql block contains statements that either *can't* be supported or *won't* be supported.
#[derive(thiserror::Error, Debug)]
pub enum NotSupportedError {
	#[error(transparent)]
	SqlParserError(#[from] pg_query::Error),

	#[error("Create or alter database commands can't be analyzed by postgres-static-analyzer, which requires all statements happen within the context of a single database.")]
	ManipulateDatabase,
}
pub type SupportResult<T> = Result<T, NotSupportedError>;


#[derive(thiserror::Error, Debug)]
pub enum ApplyError {
	#[error("tried to create a conflicting schema {0}")]
	CreateConflictingSchema(String)
}

pub struct ApplyOutcome {
	pub db_state: DbState,
	pub flags: ApplyFlags,
	pub errors: Vec<ApplyError>,
}

pub struct ApplyFlags {
	pub destroys_objects: bool,
	pub mutates_objects: bool,
	pub destroys_data: bool,
	pub mutates_data: bool,
}


pub(crate) fn apply_command(
	_db_settings: ConnectionSettings,
	mut db_state: DbState,
	sql_statement: NodeEnum,
) -> SupportResult<ApplyOutcome> {
	use pg_query::protobuf as n;

	let mut errors = vec![];
	let mut flags = ApplyFlags { destroys_objects: false, mutates_objects: false, destroys_data: false, mutates_data: false };

	match sql_statement {
		NodeEnum::CreatedbStmt(_)  | NodeEnum::AlterDatabaseStmt(_) | NodeEnum::AlterDatabaseSetStmt(_) | NodeEnum::AlterDatabaseRefreshCollStmt(_)
			=> { return Err(NotSupportedError::ManipulateDatabase) },

		NodeEnum::CreateSchemaStmt(n::CreateSchemaStmt { schemaname, authrole: _, schema_elts, if_not_exists }) => {
			let exists = db_state.schemas.contains(&schemaname.as_str());

			match (exists, if_not_exists) {
				(false, _) => {
					let mut schema = SchemaState { name: schemaname, owner: "TODO".to_string(), tables: Set::new(), typs: Set::new(), functions: Set::new(), grants: HashMap::new() };
					add_nodes_to_schema(&mut flags, &mut errors, &mut schema, nodes_to_enum(schema_elts))?;
					db_state.schemas.insert(schema);
				}
				(true, true) => { /* do nothing, ignore */ }
				(true, false) => {
					errors.push(ApplyError::CreateConflictingSchema(schemaname));
				}
			};
		},

		_ => unimplemented!(),
	};

	Ok(ApplyOutcome { db_state, flags, errors })
}

fn add_nodes_to_schema(
	flags: &mut ApplyFlags, errors: &mut Vec<ApplyError>, schema: &mut SchemaState,
	nodes: Vec<NodeEnum>,
) -> SupportResult<ApplyOutcome> {
	unimplemented!()
}


pub type SqlBlock = String;

/// Walks through the blocks, assuming `db_settings` already applies from a create/alter database command that was issued to the database before the blocks.
pub fn try_seq_db_settings(
	db_settings: ConnectionSettings,
	sql_blocks: Vec<SqlBlock>,
	stop_on_error: bool,
) -> ApplyOutcome {

	// destroys_objects
	// mutates_objects
	// destroys_data
	// mutates_data
	// errors

	// sqlparser::parser::Parser::parse_sql(&sqlparser::dialect::PostgreSqlDialect, sql);

	unimplemented!()
}

pub fn try_seq(sql_blocks: Vec<SqlBlock>, stop_on_error: bool) -> ApplyOutcome {
	let db_settings = ConnectionSettings { search_path: vec!["\"$user\"".to_string(), "public".to_string()] };

	try_seq_db_settings(db_settings, sql_blocks, stop_on_error)
}


// https://www.postgresql.org/docs/current/sql-commands.html
// https://www.postgresql.org/docs/current/config-setting.html#CONFIG-SETTING-SQL



//  ($type:ty, $field:ident) => {
macro_rules! impl_hash_and_equivalent {
	($type:ty) => {
		impl std::hash::Hash for $type {
			fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
				self.name.hash(state);
			}
		}

		impl hashbrown::Equivalent<$type> for &str {
			fn equivalent(&self, key: &$type) -> bool {
				key.name == *self
			}
		}
	};
}
macro_rules! impl_pg_from_str {
	($type:ident, $($variant:ident),+ $(,)?) => {
		impl $type {
			fn pg_from_str(s: &str) -> $type {
				match s {
					$(stringify!($variant) => $type::$variant,)+
					_ => panic!("Postgres returned unexpected {} variant: {}", stringify!($type), s),
				}
			}
		}
	};
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DbState {
	pub roles: Set<Role>,
	pub role_memberships: Vec<RoleMembership>,
	pub default_settings: ConnectionSettings,
	pub schemas: Set<SchemaState>,
	pub foreign_keys: Vec<ForeignKey>,
	pub grants: HashMap<String, Vec<DbGrant>>,
	// pub languages: Set<Language>,

	// TODO we assume that the "local" settings in connection params or whatever don't matter to us right?
	// we assume we're checking for any possible future connection?
	// which means if they're going to use different settings they have to pass them in the seq functions
	// pub settings: ConnectionSettings,
}

// https://www.postgresql.org/docs/17/catalog-pg-auth-members.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RoleMembership {
	pub parent_role: String,
	pub child_role: String,
	pub grantor: String,
	pub can_regrant_option: bool,
	pub does_auto_inherit: bool,
	pub can_set_to: bool,

}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ForeignKey {
	constraint_name: String,
	referring_schema: String,
	referring_table: String,
	referring_columns: Vec<String>,
	referred_schema: String,
	referred_table: String,
	referred_columns: Vec<String>,
}


// a Ref with a nullable schema name is inherently an *ast* concept
// if we're actually putting together a real State, it's implied that we must have been able to figure out the fully qualified name as we're doing the checking. for example if we're doing a seq over a create table command, then any unqualified type names in the ast we absolutely must be able to use the connection settings/search path to look in our list of types and figure out which one it really is
// this means at the State level the schema_name should absolutely never be None
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ref {
	pub schema_name: String,
	pub name: String,
}

// https://www.postgresql.org/docs/current/sql-createrole.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Role {
	pub name: String,
	pub is_super: bool,
	pub does_inherit: bool,
	pub can_create_role: bool,
	pub can_create_db: bool,
	pub can_login: bool,
	pub is_replication: bool,
	pub does_bypass_rls: bool,
	// rolconnlimit int4
	pub valid_until: Option<chrono::DateTime<chrono::FixedOffset>>,

	pub default_search_path: Option<ConnectionSettings>,
	pub db_search_path: Option<ConnectionSettings>,
}
impl_hash_and_equivalent!(Role);


// https://www.postgresql.org/docs/current/sql-createschema.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SchemaState {
	pub name: String,
	pub owner: String,
	pub tables: Set<TableState>,
	pub typs: Set<Typ>,
	pub functions: Set<Function>,
	pub grants: HashMap<String, Vec<SchemaGrant>>,
}
impl_hash_and_equivalent!(SchemaState);


// https://www.postgresql.org/docs/current/sql-createtype.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Typ {
	pub name: String,
	pub owner: String,
	pub body: TypBody,
	pub grants: HashMap<String, Vec<TypeGrant>>,
}
impl_hash_and_equivalent!(Typ);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypBody {
	Enum { values: Vec<String> },
	Composite { fields: Set<CompositeField> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompositeField {
	pub name: String,
	pub field_num: u16,
	pub typ: Ref,
}
impl_hash_and_equivalent!(CompositeField);


// https://www.postgresql.org/docs/current/sql-createtable.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TableState {
	pub name: String,
	pub owner: String,
	pub columns: Set<Column>,
	pub primary_key: Option<(String, Set<String>)>,
	pub unique_constraints: HashMap<String, Set<String>>,
	// foreign keys
	pub grants: HashMap<String, Vec<TableGrant>>,
}
impl_hash_and_equivalent!(TableState);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Column {
	pub name: String,
	pub typ: Ref,
	pub not_null: bool,
	pub default_expr: Option<String>,
	// pub attgenerated
	pub grants: HashMap<String, Vec<TableColumnGrant>>,
}
impl_hash_and_equivalent!(Column);



#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Function {
	pub name: String,
	pub owner: String,
	pub args: Vec<FunctionArg>,
	// TODO I think I'll want to actually parse the args and pull the out ones apart and put them in the return type
	// so return type would be an enum of either a ref to some actual type or a description of the record type implied by the out args
	pub return_typ: Ref,
	pub kind: FunctionKind,
	pub volatility: FunctionVolatility,
	pub body: String,
	pub has_sql_body: bool,
	pub is_strict: bool,
	pub returns_set: bool,
	pub is_security_definer: bool,
	pub is_leakproof: bool,
	pub language: String,
	pub grants: HashMap<String, Vec<FunctionGrant>>,
}
impl_hash_and_equivalent!(Function);

// f for a normal function, p for a procedure, a for an aggregate function, or w for a window function
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FunctionKind { Function, Procedure, Aggregate, Window }
impl FunctionKind {
	fn pg_from_char(c: i8) -> FunctionKind {
		match c as u8 as char {
			'f' => FunctionKind::Function, 'p' => FunctionKind::Procedure, 'a' => FunctionKind::Aggregate, 'w' => FunctionKind::Window,
			_ => panic!("Postgres returned an unknown function variant: {}", c as u8 as char),
		}
	}
}
// provolatile tells whether the function's result depends only on its input arguments, or is affected by outside factors. It is i for “immutable” functions, which always deliver the same result for the same inputs. It is s for “stable” functions, whose results (for fixed inputs) do not change within a scan. It is v for “volatile” functions, whose results might change at any time. (Use v also for functions with side-effects, so that calls to them cannot get optimized away.)
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FunctionVolatility { Immutable, Stable, Volatile }
impl FunctionVolatility {
	fn pg_from_char(c: i8) -> FunctionVolatility {
		match c as u8 as char {
			'i' => FunctionVolatility::Immutable, 's' => FunctionVolatility::Stable, 'v' => FunctionVolatility::Volatile,
			_ => panic!("Postgres returned an unknown function volatility: {}", c as u8 as char),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionArg {
	pub name: Option<String>,
	pub mode: ArgMode,
	pub typ: Ref,
	pub default: Option<String>,
}

// encoded as i for IN arguments, o for OUT arguments, b for INOUT arguments, v for VARIADIC arguments, t for TABLE arguments
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ArgMode { In, Out, InOut, Variadic, Table }
impl ArgMode {
	fn pg_from_char(c: i8) -> ArgMode {
		match c as u8 as char {
			'i' => ArgMode::In, 'o' => ArgMode::Out, 'b' => ArgMode::InOut, 'v' => ArgMode::Variadic, 't' => ArgMode::Table,
			_ => panic!("Postgres returned an unknown arg mode: {}", c as u8 as char),
		}
	}
}

// https://www.postgresql.org/docs/current/runtime-config-client.html
// row_security?
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConnectionSettings {
	pub search_path: Vec<String>,
}
// SHOW search_path ;
// "$user",public

pub fn make_default_settings() -> ConnectionSettings {
	ConnectionSettings {
		search_path: vec!["\"$user\"".to_string(), "public".to_string()]
	}
}


// At the database level -- only takes affect for new sessions: ALTER DATABASE mydb SET search_path = public, utility;
// At the server user level -- only takes affect for new sessions: ALTER ROLE postgres SET search_path = public,utility;
// At the database user level - only takes affect for new sessions: ALTER ROLE postgres IN DATABASE mydb SET search_path = public, utility;
// At the session level - only lasts for the life of the session: set search_path=public,utility;
// At the function level - only lasts for life of execution of function within function: ALTER FUNCTION some_func() SET search_path=public,utility;

// https://www.janbasktraining.com/community/sql-server/what-is-the-postgres-set-search-path-for-a-given-database-and-user
// SELECT boot_val FROM pg_settings WHERE name='search_path';
// SELECT reset_val FROM pg_settings WHERE name='search_path';

// https://www.postgresql.org/docs/current/catalog-pg-db-role-setting.html
// "or zero if not role/database-specific"
// SELECT setconfig
// FROM pg_db_role_setting
// WHERE setdatabase = (SELECT oid FROM pg_database WHERE datname = 'your_database_name')
// 	AND setrole = 0;

// SELECT setconfig
// FROM pg_db_role_setting
// WHERE setrole = (SELECT oid FROM pg_roles WHERE rolname = 'username')
//   AND setdatabase = 0;

// SELECT setconfig
// FROM pg_db_role_setting
// WHERE setdatabase = (SELECT oid FROM pg_database WHERE datname = 'your_database_name')
//   AND setrole = (SELECT oid FROM pg_roles WHERE rolname = 'username');

// SELECT proname, pronamespace, proconfig
// FROM pg_proc
// WHERE proconfig IS NOT NULL
//   AND array_to_string(proconfig, ',') LIKE '%search_path%';


// SELECT current_schemas(true);  -- includes implicit schemas like pg_catalog
// SHOW search_path;              -- shows the raw search_path setting



// the big question is where to store all these grants. where will we want them when we need them?
// what questions will we be asking when we need to look at grants?
// the main one will be "for this user who is implied to be logged in for this query, are they allowed to do it? so for the database in general, and then the database objects that this query acts upon, are each of those actions allowed". so that will always
// another possibly useful question is "can this database object be acted on in this family of ways (mutation, deletion) by anyone in or out of this set of roles", which is usefully distinct from "can this database object be acted on in this particular way by this particular role" which is the question we're asking above
// my options for where to put these grant objects:
// - in one big list. this is actually *not* even how postgres does it! this is simple to query though, since I just do the big union all. and when answering the most common question of "can this role do this thing", I'd have to look separately in this list. however it makes "can this wildcard thing be done by this wildcard role on this wildcard object" much more comparatively easy
// - on the database objects they apply to (HashMap<RoleName, TableGrant>). this is more type work, but it means that when I'm answering the question of "can this person do this", it's very simple since I have to look up the specific object anyway to see if what they're doing is well-structured, so the grant information comes along naturally. I only have to "multiplex" to make sure I check for all roles they have access to
// however this structure complicates the querying for these objects greatly. either you have to modify the existing queries that get the information about the object (which could be much more difficult because the acl items are stored as a list and need to be unnested), or a separate query that we then correlate back to the objects (which sounds annoying and inefficient)

// what about the default grants? they go in a sequence from schema-specific to database-wide to postgres-default
// when we're answering "can this person do this thing to this object" we perhaps *start* by looking at the object itself, if we find nothing cascade up to the schema level, and then the database level, then the postgres-default (or not? only if the schema/database defaults don't exist?)

// https://www.cybertec-postgresql.com/en/postgresql-alter-default-privileges-permissions-explained/
// Default privileges are the privileges on an object right after you created it. On all object types, the default privileges allow everything to the object owner. On most objects, nobody else has any privileges by default. But on some objects, PUBLIC (everybody) has certain privileges:


#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub struct Grant<P> {
	// pub grantee: String,
	pub grantor: String,
	pub privilege_type: P,
	pub is_grantable: bool,
}

// DATABASE	CTc	Tc	\l
pub type DbGrant = Grant<DbPrivilege>;
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum DbPrivilege { CREATE, CONNECT, TEMPORARY }
impl_pg_from_str!(DbPrivilege, CREATE, CONNECT, TEMPORARY);

// // DOMAIN  U  U  \dD+
// pub type DomainGrant = Grant<DomainUsage>;
// #[allow(non_camel_case_types)]
// #[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
// pub struct DomainUsage;

// FUNCTION or PROCEDURE	X	X	\df+
pub type FunctionGrant = Grant<FunctionExecute>;
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub struct FunctionExecute;

// // FOREIGN DATA WRAPPER	U	none	\dew+
// pub type ForeignDataWrapperGrant = Grant<ForeignDataWrapperUsage>;
// #[allow(non_camel_case_types)]
// #[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
// pub struct ForeignDataWrapperUsage;

// // FOREIGN SERVER	U	none	\des+
// pub type ForeignServerGrant = Grant<ForeignServerUsage>;
// #[allow(non_camel_case_types)]
// #[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
// pub struct ForeignServerUsage;

// // LANGUAGE	U	U	\dL+
// pub type LanguageGrant = Grant<LanguageUsage>;
// #[allow(non_camel_case_types)]
// #[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
// pub struct LanguageUsage;

// // LARGE OBJECT	rw	none	\dl+
// pub type LargeObjectGrant = Grant<LargeObjectPrivilege>;
// #[allow(non_camel_case_types)]
// #[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
// pub enum LargeObjectPrivilege { SELECT, UPDATE }
// impl_pg_from_str!(LargeObjectPrivilege, SELECT, UPDATE);

// // PARAMETER	sA	none	\dconfig+
// pub type ParameterGrant = Grant<ParameterPrivilege>;
// #[allow(non_camel_case_types)]
// #[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
// pub enum ParameterPrivilege { SET, ALTER_SYSTEM }
// impl_pg_from_str!(ParameterPrivilege, SET, ALTER_SYSTEM);

// SCHEMA	UC	none	\dn+
pub type SchemaGrant = Grant<SchemaPrivilege>;
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum SchemaPrivilege { USAGE, CREATE }
impl_pg_from_str!(SchemaPrivilege, USAGE, CREATE);

// // SEQUENCE	rwU	none	\dp
// pub type SequenceGrant = Grant<SequencePrivilege>;
// #[allow(non_camel_case_types)]
// #[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
// pub enum SequencePrivilege { SELECT, UPDATE, USAGE }
// impl_pg_from_str!(SequencePrivilege, SELECT, UPDATE, USAGE);

// TABLE (and table-like objects)	arwdDxtm	none	\dp
pub type TableGrant = Grant<TablePrivilege>;
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum TablePrivilege { INSERT, SELECT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER, MAINTAIN }
impl_pg_from_str!(TablePrivilege, INSERT, SELECT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER, MAINTAIN);

// Table column	arwx	none	\dp
pub type TableColumnGrant = Grant<TableColumnPrivilege>;
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum TableColumnPrivilege { INSERT, SELECT, UPDATE, REFERENCES }
impl_pg_from_str!(TableColumnPrivilege, INSERT, SELECT, UPDATE, REFERENCES);

// // TABLESPACE	C	none	\db+
// pub type TablespaceGrant = Grant<TablespaceCreate>;
// #[allow(non_camel_case_types)]
// #[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
// pub struct TablespaceCreate;

// TYPE	U	U	\dT+
pub type TypeGrant = Grant<TypeUsage>;
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub struct TypeUsage;



// SELECT	r (“read”)	LARGE OBJECT, SEQUENCE, TABLE (and table-like objects), table column
// INSERT	a (“append”)	TABLE, table column
// UPDATE	w (“write”)	LARGE OBJECT, SEQUENCE, TABLE, table column
// DELETE	d	TABLE
// TRUNCATE	D	TABLE
// REFERENCES	x	TABLE, table column
// TRIGGER	t	TABLE
// CREATE	C	DATABASE, SCHEMA, TABLESPACE
// CONNECT	c	DATABASE
// TEMPORARY	T	DATABASE
// EXECUTE	X	FUNCTION, PROCEDURE
// USAGE	U	DOMAIN, FOREIGN DATA WRAPPER, FOREIGN SERVER, LANGUAGE, SCHEMA, SEQUENCE, TYPE
// SET	s	PARAMETER
// ALTER SYSTEM	A	PARAMETER
// MAINTAIN	m	TABLE


#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Hash2Key(String, String);

impl std::hash::Hash for Hash2Key {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.0.hash(state);
		self.1.hash(state);
	}
}

impl hashbrown::Equivalent<Hash2Key> for (&str, &str) {
	fn equivalent(&self, key: &Hash2Key) -> bool {
		self.0 == key.0 && self.1 == key.1
	}
}
