use crate::{postgres, PgClient, Set, Map, ConnectionSettings, SchemaState};

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


pub(crate) async fn reflect_user_schemas(
	client: &PgClient
) -> Result<Set<SchemaState>, postgres::Error> {
	let schema_names = reflect_crate::queries::main::reflect_user_schemas().bind(client).all().await?;

	Ok(schema_names.into_iter().map(|schema_name| {
		SchemaState { name: schema_name, tables: Set::new() }
	}).collect())
}
