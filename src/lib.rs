pub use reflect::reflect_db_state;

// use sqlparser::ast::Statement as SqlStatement;
use pg_query::{Node, NodeEnum};

fn nodes_to_enum(nodes: Vec<Node>) -> Vec<NodeEnum> {
	nodes.into_iter().filter_map(|n| n.node).collect()
}

pub(crate) use reflect_crate::tokio_postgres::{self as postgres, /*Config as PgConfig,*/ Client as PgClient};

pub type Set<T> = hashbrown::HashSet<T>;
pub type Map<T> = hashbrown::HashMap<String, T>;

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
					let mut schema = SchemaState { name: schemaname, tables: Set::new(), typs: Set::new(), functions: Set::new() };
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DbState {
	pub roles: Set<Role>,
	pub default_settings: ConnectionSettings,
	pub schemas: Set<SchemaState>,
	pub foreign_keys: Vec<ForeignKey>,
	// pub languages: Set<Language>,

	// TODO we assume that the "local" settings in connection params or whatever don't matter to us right?
	// we assume we're checking for any possible future connection?
	// which means if they're going to use different settings they have to pass them in the seq functions
	// pub settings: ConnectionSettings,
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
	pub valid_until: chrono::DateTime<chrono::FixedOffset>,

	pub default_settings: ConnectionSettings,
}
impl_hash_and_equivalent!(Role);


// https://www.postgresql.org/docs/current/sql-createschema.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SchemaState {
	pub name: String,
	pub tables: Set<TableState>,
	pub typs: Set<Typ>,
	pub functions: Set<Function>,
}
impl_hash_and_equivalent!(SchemaState);


// https://www.postgresql.org/docs/current/sql-createtype.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Typ {
	pub name: String,
	pub body: TypBody,

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
	pub columns: Set<Column>,
	pub primary_key: Option<(String, Set<String>)>,
	pub unique_constraints: std::collections::HashMap<String, Set<String>>,
	// foreign keys
}
impl_hash_and_equivalent!(TableState);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Column {
	pub name: String,
	pub typ: Ref,
	pub not_null: bool,
	pub default_expr: Option<String>,
	// pub attgenerated
}
impl_hash_and_equivalent!(Column);



#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Function {
	pub name: String,
	pub args: Vec<FunctionArg>,
	// TODO I think I'll want to actually parse the args and pull the out ones apart and put them in the return type
	// so return type would be an enum of either a ref to some actual type or a description of the record type implied by the out args
	pub return_type: Ref,
	pub kind: FunctionKind,
	pub volatility: FunctionVolatility,
	pub body: String,
	pub has_sql_body: bool,
	pub is_strict: bool,
	pub returns_set: bool,
	pub is_security_definer: bool,
	pub is_leakproof: bool,
	pub language: String,
}
impl_hash_and_equivalent!(Function);

// f for a normal function, p for a procedure, a for an aggregate function, or w for a window function
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FunctionKind { Function, Procedure, Aggregate, Window }

// provolatile tells whether the function's result depends only on its input arguments, or is affected by outside factors. It is i for “immutable” functions, which always deliver the same result for the same inputs. It is s for “stable” functions, whose results (for fixed inputs) do not change within a scan. It is v for “volatile” functions, whose results might change at any time. (Use v also for functions with side-effects, so that calls to them cannot get optimized away.)
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FunctionVolatility { Immutable, Stable, Volatile }

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionArg {
	pub name: Option<String>,
	pub mode: ArgMode,
	pub typ: Ref,
	pub default: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ArgMode { In, Out, InOut, Variadic, Table }


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






// // https://www.postgresql.org/docs/current/sql-createdatabase.html
// #[derive(Debug)]
// pub struct DbState {
// 	name: String,
// 	default_settings: ConnectionSettings,
// 	schemas: Set<SchemaState>,
// 	// grants https://www.postgresql.org/docs/current/sql-grant.html
// }
// impl_hash_and_equivalent!(DbState);
