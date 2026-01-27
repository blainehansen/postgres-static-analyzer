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

// https://www.postgresql.org/docs/current/sql-commands.html
// https://www.postgresql.org/docs/current/config-setting.html#CONFIG-SETTING-SQL


pub type Set<T> = hashbrown::HashSet<T>;
pub type Map<T> = hashbrown::HashMap<String, T>;


// we could handle this complexity a few ways
// probably for now we could just reject any "create database" or "alter database" statements
// why are you so afraid of just reflecting this? because blaine, the whole point
// current_database()

// we're going to just reject any create database or alter database statements, and instead require
// the reflection functions just have to be given a specific database to connect to, which is obvious since you have to given them


// I think the right way to go about this is to have the seq or whatever functions assume you're in some unnamed database, and if you want different default settings, such as search_path, you have to pass them in

pub struct ApplyError(String);
pub type SqlBlock = String;

pub struct ApplyOutcome {
	pub db_state: SystemState,
	pub destroys_objects: bool,
	pub mutates_objects: bool,
	pub destroys_data: bool,
	pub mutates_data: bool,
	pub errors: Vec<ApplyError>,
}

/// Walks through the blocks, assuming `db_settings` already applies from a create/alter database command that was issued to the database before the blocks.
pub fn try_seq_db_settings(
	db_settings: ConnectionSettings,
	sql_blocks: Vec<SqlBlock>,
	stop_on_error: bool,
) -> ApplyOutcome {
	unimplemented!()
}

pub fn try_seq(sql_blocks: Vec<SqlBlock>, stop_on_error: bool) -> ApplyOutcome {
	let db_settings = ConnectionSettings { search_path: vec!["\"$user\"".to_string(), "public".to_string()] };

	try_seq_db_settings(db_settings, sql_blocks, stop_on_error)
}




#[derive(Debug)]
pub struct SystemState {
	pub roles: Set<Role>,
	pub default_settings: ConnectionSettings,
	pub schemas: Set<SchemaState>,
	pub settings: ConnectionSettings,
}

// https://www.postgresql.org/docs/current/sql-createrole.html
#[derive(Debug)]
pub struct Role {
	pub name: String,
	pub default_settings: ConnectionSettings,
}
impl_hash_and_equivalent!(Role);


// https://www.postgresql.org/docs/current/sql-createschema.html
#[derive(Debug)]
pub struct SchemaState {
	pub name: String,
	pub typs: Set<Typ>,
	pub tables: Set<TableState>,
}
impl_hash_and_equivalent!(SchemaState);


// https://www.postgresql.org/docs/current/sql-createtable.html
#[derive(Debug)]
pub struct TableState {
	pub name: String,
	pub columns: Set<Column>,
	// foreign keys
	// primary key (singular)
	// unique
	pub default_settings: ConnectionSettings,
}
impl_hash_and_equivalent!(TableState);

#[derive(Debug)]
pub struct Column {
	pub name: String,
	pub typ: String,
	pub not_null: bool,
}
impl_hash_and_equivalent!(Column);


// https://www.postgresql.org/docs/current/sql-createtype.html
#[derive(Debug)]
pub struct Typ {
	pub name: String,
}
impl_hash_and_equivalent!(Typ);


// https://www.postgresql.org/docs/current/runtime-config-client.html
// row_security?
#[derive(Debug, PartialEq)]
pub struct ConnectionSettings {
	pub search_path: Vec<String>,
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
