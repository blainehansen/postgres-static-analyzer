use super::*;

// async fn pg_con(config: &PgConfig) -> Result<PgClient, postgres::Error> {
// 	let (client, conn) = config.connect(postgres::NoTls).await?;
// 	tokio::spawn(async move { if let Err(e) = conn.await { log::error!("DB connection error: {}", e); } });
// 	Ok(client)
// }


#[tokio::test]
async fn test_thing() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {

		let a = reflect_crate::queries::main::reflect_db_role_setting().bind(&client).all().await?;
		dbg!(a);

		Ok::<_, postgres::Error>(())
	}).await??;


	assert!(true);

	Ok(())
}

