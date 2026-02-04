use crate::ConnectionSettings;

use super::*;
use std::collections::HashMap;

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
		assert_eq!(user_settings, Map::from([
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
async fn test_reflect_user_schemas() -> anyhow::Result<()> {
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

fn empty_table(table_name: &str) -> TableState {
	// TODO unique and primary keys
	TableState { name: table_name.to_string(), columns: Set::new() }
}

#[tokio::test]
async fn test_reflect_user_tables_empty() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let tables = reflect::reflect_user_tables(&client).await?;
		assert!(tables.is_empty());

		client.batch_execute(r#"
			create table aaa ();
			create table bbb ();
			create table "big things poppin" ();
		"#).await?;
		let tables = reflect::reflect_user_tables(&client).await?;
		assert_eq!(tables, HashMap::from([
			("public".to_string(), Set::from([
				empty_table("aaa"),
				empty_table("bbb"),
				empty_table("big things poppin"),
			])),
		]));

		client.batch_execute(r#"
			drop table aaa;
			drop table "big things poppin";
		"#).await?;
		let tables = reflect::reflect_user_tables(&client).await?;
		assert_eq!(tables, HashMap::from([
			("public".to_string(), Set::from([
				empty_table("bbb"),
			])),
		]));

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}

fn tab(table_name: &'static str, columns: Set<Column>) -> TableState {
	// TODO unique and primary keys
	TableState { name: table_name.to_string(), columns }
}


fn col(name: &str, typ: &str, not_null: bool, default_expr: Option<&str>) -> Column {
	Column {
		name: name.to_string(), typ: Ref { schema_name: Some("pg_catalog".to_string()), name: typ.to_string() },
		not_null, default_expr: default_expr.map(str::to_string),
	}
}

#[tokio::test]
async fn reflect_user_table_columns() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let columns = reflect::reflect_user_table_columns(&client).await?;
		assert!(columns.is_empty());

		client.batch_execute(r#"
			create table aaa (
				id int primary key,
				hey bool default ('hey there' is null),
				yo text not null,
				hmm uuid
			);
			create table bbb (
				id bigint
			);
		"#).await?;
		let columns = reflect::reflect_user_table_columns(&client).await?;
		assert_eq!(columns, HashMap::from([
			(("public".to_string(), "aaa".to_string()), Set::from([
				col("id", "int4", true, None),
				col("hey", "bool", false, Some("('hey there' IS NULL)")),
				col("yo", "text", true, None),
				col("hmm", "uuid", false, None),
			])),
			(("public".to_string(), "bbb".to_string()), Set::from([
				col("id", "int8", false, None),
			])),
		]));

		let tables = reflect::reflect_user_tables(&client).await?;
		assert_eq!(tables, HashMap::from([
			("public".to_string(), Set::from([
				tab("aaa", Set::from([
					col("id", "int4", true, None),
					col("hey", "bool", false, Some("('hey there' IS NULL)")),
					col("yo", "text", true, None),
					col("hmm", "uuid", false, None),
				])),
				tab("bbb", Set::from([
					col("id", "int8", false, None),
				])),
			])),
		]));

		client.batch_execute(r#"
			drop table aaa;
			alter table bbb rename column id to heyhey;
		"#).await?;
		let columns = reflect::reflect_user_table_columns(&client).await?;
		assert_eq!(columns, HashMap::from([
			(("public".to_string(), "bbb".to_string()), Set::from([
				col("heyhey", "int8", false, None),
			])),
		]));

		let tables = reflect::reflect_user_tables(&client).await?;
		assert_eq!(tables, HashMap::from([
			("public".to_string(), Set::from([
				tab("bbb", Set::from([
					col("heyhey", "int8", false, None),
				])),
			])),
		]));

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}
