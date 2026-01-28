use crate::ConnectionSettings;

use super::*;
use hashbrown::HashMap;

// async fn pg_con(config: &PgConfig) -> Result<PgClient, postgres::Error> {
// 	let (client, conn) = config.connect(postgres::NoTls).await?;
// 	tokio::spawn(async move { if let Err(e) = conn.await { log::error!("DB connection error: {}", e); } });
// 	Ok(client)
// }

fn s(search_path: Vec<&str>) -> ConnectionSettings {
	ConnectionSettings {
		search_path: search_path.into_iter().map(str::to_string).collect(),
	}
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
		assert_eq!(current_database_settings, Some(s(vec!["hmm"])));
		assert_eq!(user_settings, HashMap::from([
			("tempuser".to_string(), s(vec!["hmm"]))
		]));

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}

