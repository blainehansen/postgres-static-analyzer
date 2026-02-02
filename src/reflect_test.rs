use crate::ConnectionSettings;

use super::*;
use hashbrown::HashMap;

// async fn pg_con(config: &PgConfig) -> Result<PgClient, postgres::Error> {
// 	let (client, conn) = config.connect(postgres::NoTls).await?;
// 	tokio::spawn(async move { if let Err(e) = conn.await { log::error!("DB connection error: {}", e); } });
// 	Ok(client)
// }

fn connection_settings(search_path: Vec<&str>) -> ConnectionSettings {
	ConnectionSettings { search_path: search_path.into_iter().map(str::to_string).collect() }
}


#[tokio::test]
async fn test_reflect_all_settings() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let (current_database_settings, user_settings) = reflect::reflect_all_settings(&client).await?;

		assert_eq!(current_database_settings, None);
		assert!(user_settings.is_empty());

		client.batch_execute(r#"
			alter database tempdb set search_path = hmm;
			alter role tempuser set search_path = hmm;
			alter role tempuser in database tempdb set search_path = hmm;
		"#).await?;
		let (current_database_settings, user_settings) = reflect::reflect_all_settings(&client).await?;
		assert_eq!(current_database_settings, Some(connection_settings(vec!["hmm"])));
		assert_eq!(user_settings, HashMap::from([
			("tempuser".to_string(), connection_settings(vec!["hmm"]))
		]));

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}


fn empty_schema(schema_name: &str) -> SchemaState {
	SchemaState { name: schema_name.to_string(), tables: Set::new() }
}

#[tokio::test]
async fn test_reflect_schemas() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let schemas = reflect::reflect_user_schemas(&client).await?;
		assert_eq!(schemas, Set::from([
			empty_schema("public"),
		]));

		client.batch_execute(r#"
			create schema aaa;
			create schema bbb;
			create schema "big things poppin";
		"#).await?;
		let schemas = reflect::reflect_user_schemas(&client).await?;
		assert_eq!(schemas, Set::from([
			empty_schema("public"),
			empty_schema("aaa"),
			empty_schema("bbb"),
			empty_schema("big things poppin"),
		]));

		client.batch_execute(r#"
			drop schema public;
			drop schema "big things poppin";
		"#).await?;
		let schemas = reflect::reflect_user_schemas(&client).await?;
		assert_eq!(schemas, Set::from([
			empty_schema("aaa"),
			empty_schema("bbb"),
		]));

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}
