use super::*;

#[tokio::test]
async fn test_reflect_pg_state() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let populate_all_sql = tokio::fs::read_to_string("./populate_all.sql").await?;
		client.batch_execute(&populate_all_sql).await?;

		let pg_state = reflect_pg_state(&client).await?;

		insta::assert_ron_snapshot!(
			pg_state,
			{
				".**" => insta::sorted_redaction(),
			}
		);

		Ok::<_, anyhow::Error>(())
	}).await??;

	Ok(())
}



// fn s(val: &str) -> Str {
// 	val.into()
// }
// fn q(ref_name: &str) -> Qual {
// 	Qual { schema_name: s("pg_catalog"), name: s(ref_name) }
// }
// fn qu(schema_name: &str, name: &str) -> Qual {
// 	Qual { schema_name: s(schema_name), name: s(name) }
// }

// #[tokio::test]
// async fn test_reflect_pg_roles() -> anyhow::Result<()> {
// 	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
// 		let pg_roles = reflect::reflect_pg_roles(&client).await?;
// 		assert_eq!(pg_roles, Set::from([
// 			PgRoles {
// 				rolname: s("tempuser"),
// 				rolsuper: true,
// 				rolinherit: true,
// 				rolcreaterole: true,
// 				rolcreatedb: true,
// 				rolcanlogin: true,
// 				rolreplication: true,
// 				rolconnlimit: None,
// 				rolvaliduntil: None,
// 				rolbypassrls: true,
// 				rolconfig: None,
// 			},
// 		]));

// 		client.batch_execute(r#"
// 			create role yorole NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOLOGIN NOREPLICATION NOBYPASSRLS
// 				PASSWORD 'yoyoyo'
// 				VALID UNTIL '2030-01-01 10:23:54+00';
// 		"#).await?;
// 		use chrono::TimeZone;
// 		let pg_roles = reflect::reflect_pg_roles(&client).await?;
// 		assert_eq!(pg_roles, Set::from([
// 			PgRoles {
// 				rolname: s("tempuser"),
// 				rolsuper: true,
// 				rolinherit: true,
// 				rolcreaterole: true,
// 				rolcreatedb: true,
// 				rolcanlogin: true,
// 				rolreplication: true,
// 				rolconnlimit: None,
// 				rolvaliduntil: None,
// 				rolbypassrls: true,
// 				rolconfig: None,
// 			},
// 			PgRoles {
// 				rolname: s("yorole"),
// 				rolsuper: false,
// 				rolinherit: false,
// 				rolcreaterole: false,
// 				rolcreatedb: false,
// 				rolcanlogin: false,
// 				rolreplication: false,
// 				rolconnlimit: None,
// 				rolvaliduntil: Some(chrono::Utc.with_ymd_and_hms(2030, 1, 1, 10, 23, 54).unwrap().fixed_offset()),
// 				rolbypassrls: false,
// 				rolconfig: None,
// 			},
// 		]));

// 		Ok::<_, postgres::Error>(())
// 	}).await??;

// 	Ok(())
// }


// fn bland_table(table_name: &str) -> PgClass {
// 	PgClass {
//     relname: s(table_name),
//     relnamespace: s("public"),
//     reltype: Some(qu("public", table_name)),
//     reloftype: None,
//     relowner: s("tempuser"),
//     reltablespace: None,
//     relisshared: false,
//     relpersistence: ClassPersistence::Permanant,
//     relkind: ClassKind::Table,
//     relrowsecurity: false,
//     relforcerowsecurity: false,
//     relispartition: false,
//     relacl: None,
//     reloptions: None,
//     relpartbound: None,
// }
// }

// #[tokio::test]
// async fn test_reflect_pg_class() -> anyhow::Result<()> {
// 	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
// 		let pg_class = reflect::reflect_pg_class(&client).await?;
// 		assert!(pg_class.is_empty());

// 		client.batch_execute(r#"
// 			create table aaa ();
// 			create table bbb ();
// 			create table "big things poppin" ();
// 		"#).await?;
// 		let pg_class = reflect::reflect_pg_class(&client).await?;
// 		assert_eq!(pg_class, Set::from([
// 			bland_table("aaa"),
// 			bland_table("bbb"),
// 			bland_table("big things poppin"),
// 		]));

// 		Ok::<_, postgres::Error>(())
// 	}).await??;

// 	Ok(())
// }
