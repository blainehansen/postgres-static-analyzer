use crate::{Column, ConnectionSettings, Map, PgClient, Ref, SchemaState, Set, TableState, postgres};
use std::collections::HashMap;

pub(crate) async fn reflect_all_settings(
	client: &PgClient
) -> Result<(Option<ConnectionSettings>, Map<ConnectionSettings>), postgres::Error> {
	let all_settings = reflect_crate::queries::main::reflect_db_role_setting().bind(client)
		.map(|s| {
			(s.search_path.map(str::to_string).collect::<Vec<_>>(), s.rolname.map(str::to_string))
		}).all().await?;

	let mut current_database_settings = None;
	let mut user_settings: Map<ConnectionSettings> = Map::new();
	for (search_path, rolname) in all_settings {
		let settings = ConnectionSettings { search_path };

		if let Some(rolname) = rolname {
			user_settings.insert(rolname, settings);
		}
		else {
			current_database_settings = Some(settings);
		}
	}

	Ok((current_database_settings, user_settings))
}


// https://www.postgresql.org/docs/current/catalog-pg-namespace.html
pub(crate) async fn reflect_user_schemas(
	client: &PgClient
) -> Result<Set<SchemaState>, postgres::Error> {
	let schema_query = reflect_crate::queries::main::reflect_user_schemas();

	let (schemas, mut tables_map) = tokio::try_join!(
		schema_query.bind(client).all(),
		reflect_user_tables(client),
	)?;

	let schemas = schemas.into_iter().map(|schema_name| {
		let tables = tables_map.remove(&schema_name).unwrap_or_default();
		SchemaState { name: schema_name, tables }
	}).collect();

	Ok(schemas)
}


// https://www.postgresql.org/docs/current/catalog-pg-class.html
pub(crate) async fn reflect_user_tables(
	client: &PgClient
) -> Result<HashMap<String, Set<TableState>>, postgres::Error> {
	use itertools::Itertools;

	let tables = reflect_crate::queries::main::reflect_user_tables().bind(client)
		.map(|t| {
			(t.nspname.to_string(), TableState { name: t.relname.to_string(), columns: Set::new() })
		})
		// TODO probably possible to use the iter() method to get a stream and then deal with that
		.all()
		.await?
		.into_iter().into_grouping_map().collect::<Set<_>>();

		// TODO put the columns on the tables
		// reflect_user_table_columns

	Ok(tables)
}


// https://www.postgresql.org/docs/current/catalog-pg-attribute.html
pub(crate) async fn reflect_user_table_columns(
	client: &PgClient
) -> Result<HashMap<(String, String), Set<Column>>, postgres::Error> {
	use itertools::Itertools;

	let columns = reflect_crate::queries::main::reflect_user_table_columns().bind(client)
		.map(|a| {
			// TODO attgenerated
			let column = Column {
				name: a.attname.to_string(),
				typ: Ref { schema_name: Some(a.typ_nspname.to_string()), name: a.typname.to_string() },
				not_null: a.attnotnull,
				default_expr: a.attdef.map(str::to_string),
			};

			((a.nspname.to_string(), a.relname.to_string()), column)
		})
		.all()
		.await?
		.into_iter().into_grouping_map().collect::<Set<_>>();

	Ok(columns)
}
