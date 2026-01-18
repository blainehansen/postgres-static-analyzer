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


type Set<T> = hashbrown::HashSet<T>;




#[derive(Debug)]
struct SystemState {
	databases: Set<DbState>,
	roles: Set<Role>,
	settings: ConnectionSettings,
}

// https://www.postgresql.org/docs/current/sql-createrole.html
#[derive(Debug)]
struct Role {
	name: String,
	default_settings: ConnectionSettings,
}
impl_hash_and_equivalent!(Role);


// https://www.postgresql.org/docs/current/sql-createdatabase.html
#[derive(Debug)]
struct DbState {
	name: String,
	default_settings: ConnectionSettings,
	schemas: Set<SchemaState>,
	// grants https://www.postgresql.org/docs/current/sql-grant.html
}
impl_hash_and_equivalent!(DbState);


// https://www.postgresql.org/docs/current/sql-createschema.html
#[derive(Debug)]
struct SchemaState {
	name: String,
	typs: Set<Typ>,
	tables: Set<TableState>,
}
impl_hash_and_equivalent!(SchemaState);


// https://www.postgresql.org/docs/current/sql-createtable.html
#[derive(Debug)]
struct TableState {
	name: String,
	columns: Set<Column>,
	// foreign keys
	// primary key (singular)
	// unique
	default_settings: ConnectionSettings,
}
impl_hash_and_equivalent!(TableState);

#[derive(Debug)]
struct Column {
	name: String,
	typ: String,
	not_null: bool,
}
impl_hash_and_equivalent!(Column);


// https://www.postgresql.org/docs/current/sql-createtype.html
#[derive(Debug)]
struct Typ {
	name: String,
}
impl_hash_and_equivalent!(Typ);


// https://www.postgresql.org/docs/current/runtime-config-client.html
// row_security?
#[derive(Debug)]
struct ConnectionSettings {
	search_path: Vec<String>,
}
// At the database level -- only takes affect for new sessions: ALTER DATABASE mydb SET search_path = public, utility;
// At the server user level -- only takes affect for new sessions: ALTER ROLE postgres SET search_path = public,utility;
// At the database user level - only takes affect for new sessions: ALTER ROLE postgres IN DATABASE mydb SET search_path = public, utility;
// At the session level - only lasts for the life of the session: set search_path=public,utility;
// At the function level - only lasts for life of execution of function within function: ALTER FUNCTION some_func() SET search_path=public,utility;
