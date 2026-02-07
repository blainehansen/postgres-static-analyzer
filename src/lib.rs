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
	db_settings: ConnectionSettings,
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
					let mut schema = SchemaState { name: schemaname, tables: Set::new() };
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


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ref {
	pub schema_name: Option<String>,
	pub name: String,
}

// https://www.postgresql.org/docs/current/sql-createrole.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Role {
	pub name: String,
	pub default_settings: ConnectionSettings,
}
impl_hash_and_equivalent!(Role);


// https://www.postgresql.org/docs/current/sql-createschema.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SchemaState {
	pub name: String,
	// pub typs: Set<Typ>,
	pub tables: Set<TableState>,
}
impl_hash_and_equivalent!(SchemaState);


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


// https://www.postgresql.org/docs/current/sql-createtype.html
// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct Typ {
// 	pub name: String,
// }
// impl_hash_and_equivalent!(Typ);


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
