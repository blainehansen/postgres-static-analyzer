use crate::ConnectionSettings;

use super::*;
use std::collections::HashMap;

// async fn pg_con(config: &PgConfig) -> Result<PgClient, postgres::Error> {
// 	let (client, conn) = config.connect(postgres::NoTls).await?;
// 	tokio::spawn(async move { if let Err(e) = conn.await { log::error!("DB connection error: {}", e); } });
// 	Ok(client)
// }

fn s(val: &str) -> String {
	val.to_string()
}

fn r(ref_name: &str) -> Ref {
	Ref { schema_name: s("pg_catalog"), name: s(ref_name) }
}
fn re(schema_name: &str, name: &str) -> Ref {
	Ref { schema_name: s(schema_name), name: s(name) }
}

fn connection_settings(search_path: Vec<&str>) -> ConnectionSettings {
	ConnectionSettings { search_path: search_path.into_iter().map(str::to_string).collect() }
}


#[tokio::test]
async fn test_reflect_default_settings() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let db_default_settings = reflect::reflect_db_default_setting(&client).await?;
		assert_eq!(db_default_settings, None);

		client.batch_execute(r#"
			alter database tempdb set search_path = hmmdb;
			alter role tempuser set search_path = hmmuser;
			alter role tempuser in database tempdb set search_path = hmmuserdb;
		"#).await?;
		let db_default_settings = reflect::reflect_db_default_setting(&client).await?;
		assert_eq!(db_default_settings, Some(connection_settings(vec!["hmmdb"])));

		client.batch_execute(r#"
			alter database tempdb set search_path = hmmdb,alsohmmdb;
			alter role tempuser set search_path = hmmuser,alsohmmuser;
			alter role tempuser in database tempdb set search_path = hmmuserdb,alsohmmuserdb;
		"#).await?;
		let db_default_settings = reflect::reflect_db_default_setting(&client).await?;
		assert_eq!(db_default_settings, Some(connection_settings(vec!["hmmdb", "alsohmmdb"])));

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}

#[tokio::test]
async fn test_reflect_db_grants() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let db_grants = reflect::reflect_db_grants(&client).await?;
		assert!(db_grants.is_empty());

		client.batch_execute(r#"
			create role guy;

			revoke create on database tempdb from PUBLIC;
			grant connect on database tempdb to PUBLIC;
			grant all privileges on database tempdb to guy;
		"#).await?;
		let mut db_grants = reflect::reflect_db_grants(&client).await?;
		db_grants.values_mut().for_each(|v| v.sort());
		assert_eq!(db_grants, HashMap::from([
			(s("public"), vec![
				DbGrant { privilege_type: DbPrivilege::CONNECT, is_grantable: false, grantor: s("tempuser") },
				DbGrant { privilege_type: DbPrivilege::TEMPORARY, is_grantable: false, grantor: s("tempuser") },
			]),

			(s("tempuser"), vec![
				DbGrant { privilege_type: DbPrivilege::CREATE, is_grantable: false, grantor: s("tempuser"), },
				DbGrant { privilege_type: DbPrivilege::CONNECT, is_grantable: false, grantor: s("tempuser"), },
				DbGrant { privilege_type: DbPrivilege::TEMPORARY, is_grantable: false, grantor: s("tempuser"), },
			]),

			(s("guy"), vec![
				DbGrant { privilege_type: DbPrivilege::CREATE, is_grantable: false, grantor: s("tempuser") },
				DbGrant { privilege_type: DbPrivilege::CONNECT, is_grantable: false, grantor: s("tempuser") },
				DbGrant { privilege_type: DbPrivilege::TEMPORARY, is_grantable: false, grantor: s("tempuser") },
			]),
		]));

		client.batch_execute(r#"
			grant all privileges on database tempdb to PUBLIC;
			grant connect on database tempdb to PUBLIC;
			revoke all privileges on database tempdb from guy;
		"#).await?;
		let mut db_grants = reflect::reflect_db_grants(&client).await?;
		db_grants.values_mut().for_each(|v| v.sort());
		assert_eq!(db_grants, HashMap::from([
			(s("public"), vec![
				DbGrant { privilege_type: DbPrivilege::CREATE, is_grantable: false, grantor: s("tempuser") },
				DbGrant { privilege_type: DbPrivilege::CONNECT, is_grantable: false, grantor: s("tempuser") },
				DbGrant { privilege_type: DbPrivilege::TEMPORARY, is_grantable: false, grantor: s("tempuser") },
			]),

			(s("tempuser"), vec![
				DbGrant { privilege_type: DbPrivilege::CREATE, is_grantable: false, grantor: s("tempuser"), },
				DbGrant { privilege_type: DbPrivilege::CONNECT, is_grantable: false, grantor: s("tempuser"), },
				DbGrant { privilege_type: DbPrivilege::TEMPORARY, is_grantable: false, grantor: s("tempuser"), },
			]),
		]));

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}

#[tokio::test]
async fn test_reflect_roles() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let roles = reflect::reflect_roles(&client).await?;
		assert_eq!(roles, Set::from([
			Role {
				name: s("tempuser"),
				is_super: true, does_inherit: true, can_create_role: true, can_create_db: true, can_login: true, is_replication: true, does_bypass_rls: true,
				valid_until: None,
				default_search_path: None,
				db_search_path: None,
			},
		]));

		client.batch_execute(r#"
			alter database tempdb set search_path = hmm;
			alter role tempuser set search_path = hmmdb;
			alter role tempuser in database tempdb set search_path = hmmdbuser;
		"#).await?;
		let roles = reflect::reflect_roles(&client).await?;
		assert_eq!(roles, Set::from([
			Role {
				name: s("tempuser"),
				is_super: true, does_inherit: true, can_create_role: true, can_create_db: true, can_login: true, is_replication: true, does_bypass_rls: true,
				valid_until: None,
				default_search_path: Some(connection_settings(vec!["hmmdb"])),
				db_search_path: Some(connection_settings(vec!["hmmdbuser"])),
			},
		]));

		client.batch_execute(r#"
			alter database tempdb set search_path = hmmdb,alsohmmdb;
			alter role tempuser set search_path = hmmuser,alsohmmuser;
			alter role tempuser in database tempdb set search_path = hmmuserdb,alsohmmuserdb;
		"#).await?;
		let roles = reflect::reflect_roles(&client).await?;
		assert_eq!(roles, Set::from([
			Role {
				name: s("tempuser"),
				is_super: true, does_inherit: true, can_create_role: true, can_create_db: true, can_login: true, is_replication: true, does_bypass_rls: true,
				valid_until: None,
				default_search_path: Some(connection_settings(vec!["hmmuser", "alsohmmuser"])),
				db_search_path: Some(connection_settings(vec!["hmmuserdb", "alsohmmuserdb"])),
			},
		]));

			// | CONNECTION LIMIT connlimit
		client.batch_execute(r#"
			create role yorole NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOLOGIN NOREPLICATION NOBYPASSRLS
				PASSWORD 'yoyoyo'
				VALID UNTIL '2030-01-01 10:23:54+00';
		"#).await?;
		let roles = reflect::reflect_roles(&client).await?;
		use chrono::TimeZone;
		assert_eq!(roles, Set::from([
			Role {
				name: s("tempuser"),
				is_super: true, does_inherit: true, can_create_role: true, can_create_db: true, can_login: true, is_replication: true, does_bypass_rls: true,
				valid_until: None,
				default_search_path: Some(connection_settings(vec!["hmmuser", "alsohmmuser"])),
				db_search_path: Some(connection_settings(vec!["hmmuserdb", "alsohmmuserdb"])),
			},
			Role {
				name: s("yorole"),
				is_super: false, does_inherit: false, can_create_role: false, can_create_db: false, can_login: false, is_replication: false, does_bypass_rls: false,
				valid_until: Some(chrono::Utc.with_ymd_and_hms(2030, 1, 1, 10, 23, 54).unwrap().fixed_offset()),
				default_search_path: None,
				db_search_path: None,
			},
		]));

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}


fn empty_schema(schema_name: &str, owner: &str) -> SchemaState {
	SchemaState {
		name: schema_name.to_string(), owner: s(owner),
		tables: Set::new(), typs: Set::new(), functions: Set::new(),
	}
}

#[tokio::test]
async fn test_reflect_user_schemas() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let schemas = reflect::reflect_user_schemas(&client).await?;
		assert_eq!(schemas, Set::from([
			empty_schema("public", "pg_database_owner"),
		]));

		client.batch_execute(r#"
			create schema aaa;
			create schema bbb;
			create schema "big things poppin";
		"#).await?;
		let schemas = reflect::reflect_user_schemas(&client).await?;
		assert_eq!(schemas, Set::from([
			empty_schema("public", "pg_database_owner"),
			empty_schema("aaa", "tempuser"),
			empty_schema("bbb", "tempuser"),
			empty_schema("big things poppin", "tempuser"),
		]));

		client.batch_execute(r#"
			drop schema public;
			drop schema "big things poppin";
		"#).await?;
		let schemas = reflect::reflect_user_schemas(&client).await?;
		assert_eq!(schemas, Set::from([
			empty_schema("aaa", "tempuser"),
			empty_schema("bbb", "tempuser"),
		]));

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}

fn empty_table(table_name: &str) -> TableState {
	// TODO unique and primary keys
	TableState {
		name: table_name.to_string(), owner: s("tempuser"), columns: Set::new(),
		primary_key: None, unique_constraints: HashMap::new(),
	}
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
	TableState {
		name: table_name.to_string(), owner: s("tempuser"), columns,
		primary_key: None, unique_constraints: HashMap::new(),

	}
}


fn col(name: &str, typ: &str, not_null: bool, default_expr: Option<&str>) -> Column {
	Column {
		name: name.to_string(), typ: Ref { schema_name: "pg_catalog".to_string(), name: typ.to_string() },
		not_null, default_expr: default_expr.map(str::to_string),
	}
}

#[tokio::test]
async fn test_reflect_user_table_columns() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let columns = reflect::reflect_user_table_columns(&client).await?;
		assert!(columns.is_empty());

		client.batch_execute(r#"
			create table aaa (
				id int not null,
				hey bool default ('hey there' is null),
				yo text not null,
				hmm uuid
			);
			create table bbb (
				id bigint
			);
		"#).await?;

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

#[tokio::test]
async fn test_reflect_user_table_constraints() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let unique_constraints = reflect::reflect_user_table_unique_constraints(&client).await?;
		assert!(unique_constraints.is_empty());

		client.batch_execute(r#"
			create table aaa (
				a int primary key,
				b int, c int unique, d int not null,
				unique (a),
				unique (c, d)
			);
			create table bbb (
				a int, b int, c int, d int,
				primary key (a, b),
				unique (a, b),
				unique (b, c, d)
			);
		"#).await?;

		let tables = reflect::reflect_user_tables(&client).await?;
		assert_eq!(tables, HashMap::from([
			("public".to_string(), Set::from([
				TableState {
					name: s("aaa"), owner: s("tempuser"), columns: Set::from([
						col("a", "int4", true, None),
						col("b", "int4", false, None),
						col("c", "int4", false, None),
						col("d", "int4", true, None),
					]),
					primary_key: Some((s("aaa_pkey"), Set::from([s("a")]))),
					unique_constraints: HashMap::from([
						(s("aaa_c_d_key"), Set::from([s("c"), s("d")])),
						(s("aaa_c_key"), Set::from([s("c")])),
					]),
				},

				TableState {
					name: s("bbb"), owner: s("tempuser"), columns: Set::from([
						col("a", "int4", true, None),
						col("b", "int4", true, None),
						col("c", "int4", false, None),
						col("d", "int4", false, None),
					]),
					primary_key: Some((s("bbb_pkey"), Set::from([s("a"), s("b")]))),
					unique_constraints: HashMap::from([
						(s("bbb_b_c_d_key"), Set::from([s("b"), s("c"), s("d")])),
					]),
				},
			])),
		]));

		client.batch_execute(r#"
			drop table aaa;
			alter table bbb rename column a to heyhey;
		"#).await?;

		let tables = reflect::reflect_user_tables(&client).await?;
		assert_eq!(tables, HashMap::from([
			("public".to_string(), Set::from([
				TableState {
					name: s("bbb"), owner: s("tempuser"), columns: Set::from([
						col("heyhey", "int4", true, None),
						col("b", "int4", true, None),
						col("c", "int4", false, None),
						col("d", "int4", false, None),
					]),
					primary_key: Some((s("bbb_pkey"), Set::from([s("heyhey"), s("b")]))),
					unique_constraints: HashMap::from([
						(s("bbb_b_c_d_key"), Set::from([s("b"), s("c"), s("d")])),
					]),
				},
			])),
		]));

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}


#[tokio::test]
async fn test_reflect_foreign_keys() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let foreign_keys = reflect::reflect_foreign_keys(&client).await?;
		assert!(foreign_keys.is_empty());

		client.batch_execute(r#"
			create table aaa (
				aaa_a int, aaa_b int not null,
				unique (aaa_a, aaa_b),
				aaa_c int unique
			);
			create table bbb (
				bbb_a int not null, bbb_b int not null,
				foreign key (bbb_a, bbb_b) references aaa (aaa_a, aaa_b),
				bbb_c int references aaa(aaa_c)
			);
		"#).await?;
		let mut foreign_keys = reflect::reflect_foreign_keys(&client).await?;
		assert_eq!(foreign_keys.sort(), vec![
			ForeignKey {
				constraint_name: s("bbb_bbb_a_bbb_b_fkey"),
				referring_schema: s("public"), referring_table: s("bbb"), referring_columns: vec![s("bbb_a"), s("bbb_b")],
				referred_schema: s("public"), referred_table: s("aaa"), referred_columns: vec![s("aaa_a"), s("aaa_b")],
			},
			ForeignKey {
				constraint_name: s("bbb_bbb_c_fkey"),
				referring_schema: s("public"), referring_table: s("bbb"), referring_columns: vec![s("bbb_c")],
				referred_schema: s("public"), referred_table: s("aaa"), referred_columns: vec![s("aaa_c")],
			},
		].sort());

		client.batch_execute(r#"
			alter table aaa drop column aaa_a cascade;
			alter table aaa rename column aaa_b to aaa_heyhey;
			alter table aaa drop column aaa_c cascade;
		"#).await?;
		let mut foreign_keys = reflect::reflect_foreign_keys(&client).await?;
		assert_eq!(foreign_keys.sort(), vec![
			ForeignKey {
				constraint_name: s("bbb_bbb_a_bbb_b_fkey"),
				referring_schema: s("public"), referring_table: s("bbb"), referring_columns: vec![s("bbb_b")],
				referred_schema: s("public"), referred_table: s("aaa"), referred_columns: vec![s("aaa_heyhey")],
			},
		].sort());

		client.batch_execute(r#"
			alter table aaa drop column aaa_heyhey cascade;
		"#).await?;
		let foreign_keys = reflect::reflect_foreign_keys(&client).await?;
		assert!(foreign_keys.is_empty());

		Ok::<_, postgres::Error>(())
	}).await??;



	Ok(())
}


#[tokio::test]
async fn test_reflect_composite_types() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let typs = reflect::reflect_composite_types(&client).await?;
		assert!(typs.is_empty());

		client.batch_execute(r#"
			create type aaa as (aaa_a int, aaa_b bool, aaa_c text[], aaa_d text);
			create table bbb (bbb_a int not null);
		"#).await?;
		let typs = reflect::reflect_composite_types(&client).await?;
		assert_eq!(typs, vec![
			(s("public"), Typ { name: s("aaa"), owner: s("tempuser"), body: TypBody::Composite { fields: Set::from([
				CompositeField { name: s("aaa_a"), field_num: 1, typ: Ref { schema_name: s("pg_catalog"), name: s("int4") } },
				CompositeField { name: s("aaa_b"), field_num: 2, typ: Ref { schema_name: s("pg_catalog"), name: s("bool") } },
				CompositeField { name: s("aaa_c"), field_num: 3, typ: Ref { schema_name: s("pg_catalog"), name: s("_text") } },
				CompositeField { name: s("aaa_d"), field_num: 4, typ: Ref { schema_name: s("pg_catalog"), name: s("text") } },
			]) } }),
			(s("public"), Typ { name: s("bbb"), owner: s("tempuser"), body: TypBody::Composite { fields: Set::from([
				CompositeField { name: s("bbb_a"), field_num: 1, typ: Ref { schema_name: s("pg_catalog"), name: s("int4") } },
			]) } }),
		]);

		client.batch_execute(r#"
			drop table bbb cascade;
			alter type aaa rename attribute aaa_a to yo_a;
			alter type aaa add attribute yo_whoa date;
			alter type aaa drop attribute aaa_b;
			alter type aaa alter attribute aaa_c type timestamp;
		"#).await?;
		let typs = reflect::reflect_composite_types(&client).await?;
		assert_eq!(typs, vec![
			(s("public"), Typ { name: s("aaa"), owner: s("tempuser"),  body: TypBody::Composite { fields: Set::from([
				CompositeField { name: s("yo_a"), field_num: 1, typ: Ref { schema_name: s("pg_catalog"), name: s("int4") } },
				CompositeField { name: s("aaa_c"), field_num: 3, typ: Ref { schema_name: s("pg_catalog"), name: s("timestamp") } },
				CompositeField { name: s("aaa_d"), field_num: 4, typ: Ref { schema_name: s("pg_catalog"), name: s("text") } },
				CompositeField { name: s("yo_whoa"), field_num: 5, typ: Ref { schema_name: s("pg_catalog"), name: s("date") } },
			]) } }),
		]);

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}


#[tokio::test]
async fn test_reflect_enum_types() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let typs = reflect::reflect_enum_types(&client).await?;
		assert!(typs.is_empty());

		client.batch_execute(r#"
			create type aaa as enum ('a', 'b', 'c');
			create type bbb as enum ();
		"#).await?;
		let typs = reflect::reflect_enum_types(&client).await?;
		assert_eq!(typs, vec![
			(s("public"), Typ { name: s("aaa"), owner: s("tempuser"), body: TypBody::Enum { values: vec![s("a"), s("b"), s("c")] } }),
			(s("public"), Typ { name: s("bbb"), owner: s("tempuser"), body: TypBody::Enum { values: vec![] } }),
		]);

		client.batch_execute(r#"
			drop type bbb;
			alter type aaa add value 'yo' before 'b';
		"#).await?;
		let typs = reflect::reflect_enum_types(&client).await?;
		assert_eq!(typs, vec![
			(s("public"), Typ { name: s("aaa"), owner: s("tempuser"), body: TypBody::Enum { values: vec![s("a"), s("yo"), s("b"), s("c")] } }),
		]);

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}



#[tokio::test]
async fn test_reflect_user_schemas_full() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let schemas = reflect::reflect_user_schemas(&client).await?;
		assert_eq!(schemas, Set::from([
			empty_schema("public", "pg_database_owner"),
		]));

		client.batch_execute(r#"
			create type my_enum as enum ('my1', 'my2');

			create table aaa (
				a int primary key,
				b int, c int unique, d int not null default 1,
				unique (a, b)
			);
			create table bbb (
				bbb_a int not null, bbb_b int not null,
				foreign key (bbb_a, bbb_b) references aaa (a, b),
				bbb_c int references aaa(c),
				b_json jsonb, b_date date, b_my my_enum
			);
		"#).await?;
		let schemas = reflect::reflect_user_schemas(&client).await?;
		assert_eq!(schemas, Set::from([SchemaState { name: "public".to_string(), owner: s("pg_database_owner"), functions: Set::new(),
			tables: Set::from([
				TableState {
					name: s("aaa"), owner: s("tempuser"), columns: Set::from([
						col("a", "int4", true, None),
						col("b", "int4", false, None),
						col("c", "int4", false, None),
						Column { name: s("d"), typ: Ref { schema_name: s("pg_catalog"), name: s("int4") }, not_null: true, default_expr: Some(s("1")) },
					]),
					primary_key: Some((s("aaa_pkey"), Set::from([s("a")]))),
					unique_constraints: HashMap::from([
						(s("aaa_a_b_key"), Set::from([s("a"), s("b")])),
						(s("aaa_c_key"), Set::from([s("c")])),
					]),
				},

				TableState {
					name: s("bbb"), owner: s("tempuser"), columns: Set::from([
						col("bbb_a", "int4", true, None),
						col("bbb_b", "int4", true, None),
						col("bbb_c", "int4", false, None),
						col("b_json", "jsonb", false, None),
						col("b_date", "date", false, None),
						Column { name: s("b_my"), typ: Ref { schema_name: s("public"), name: s("my_enum") }, not_null: false, default_expr: None },
					]),
					primary_key: None,
					unique_constraints: HashMap::new(),
				},
			]),
			typs: Set::from([
				Typ { name: s("my_enum"), owner: s("tempuser"), body: TypBody::Enum { values: vec![s("my1"), s("my2")] } },
				Typ { name: s("aaa"), owner: s("tempuser"), body: TypBody::Composite { fields: Set::from([
					CompositeField { name: s("a"), field_num: 1, typ: Ref { schema_name: s("pg_catalog"), name: s("int4") } },
					CompositeField { name: s("b"), field_num: 2, typ: Ref { schema_name: s("pg_catalog"), name: s("int4") } },
					CompositeField { name: s("c"), field_num: 3, typ: Ref { schema_name: s("pg_catalog"), name: s("int4") } },
					CompositeField { name: s("d"), field_num: 4, typ: Ref { schema_name: s("pg_catalog"), name: s("int4") } },
				]) } },
				Typ { name: s("bbb"), owner: s("tempuser"),  body: TypBody::Composite { fields: Set::from([
					CompositeField { name: s("bbb_a"), field_num: 1, typ: Ref { schema_name: s("pg_catalog"), name: s("int4") } },
					CompositeField { name: s("bbb_b"), field_num: 2, typ: Ref { schema_name: s("pg_catalog"), name: s("int4") } },
					CompositeField { name: s("bbb_c"), field_num: 3, typ: Ref { schema_name: s("pg_catalog"), name: s("int4") } },
					CompositeField { name: s("b_json"), field_num: 4, typ: Ref { schema_name: s("pg_catalog"), name: s("jsonb") } },
					CompositeField { name: s("b_date"), field_num: 5, typ: Ref { schema_name: s("pg_catalog"), name: s("date") } },
					CompositeField { name: s("b_my"), field_num: 6, typ: Ref { schema_name: s("public"), name: s("my_enum") } },
				]) } },
			]) },

		]));

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}


#[tokio::test]
async fn test_reflect_functions() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let functions = reflect::reflect_functions(&client).await?;
		assert!(functions.is_empty());

		client.batch_execute(r#"
			create or replace function add(a integer, b integer = 0) returns integer
				language sql
				immutable
				strict
				return a + b;

			create or replace function dup(int) returns table(f1 int, f2 text)
				as $$ select $1, cast($1 as text) || ' is text' $$
				language sql;

			create or replace function rec(int, out f1 int, inout f2 text = 'yeah')
				as $$ select $1, cast($1 as text) || ' is text' $$
				language sql;

		"#).await?;
		let functions = reflect::reflect_functions(&client).await?;
		assert_eq!(functions, HashMap::from([
			(s("public"), Set::from([
				Function {
					name: s("add"), owner: s("tempuser"),
					args: vec![
						FunctionArg { name: Some(s("a")), typ: r("int4"),  mode: ArgMode::In, default: None },
						FunctionArg { name: Some(s("b")), typ: r("int4"),  mode: ArgMode::In, default: Some(s("0")) },
					],
					return_typ: r("int4"),
					kind: FunctionKind::Function,
					volatility: FunctionVolatility::Immutable,
					body: s("RETURN (a + b)"),
					has_sql_body: true,
					is_strict: true,
					returns_set: false,
					is_security_definer: false,
					is_leakproof: false,
					language: s("sql"),
				},
				Function {
					name: s("dup"), owner: s("tempuser"),
					args: vec![
						FunctionArg { name: None, typ: r("int4"), mode: ArgMode::In, default: None },
						FunctionArg { name: Some(s("f1")), typ: r("int4"), mode: ArgMode::Table, default: None },
						FunctionArg { name: Some(s("f2")), typ: r("text"), mode: ArgMode::Table, default: None },
					],
					return_typ: r("record"),
					kind: FunctionKind::Function,
					volatility: FunctionVolatility::Volatile,
					body: s(" select $1, cast($1 as text) || ' is text' "),
					has_sql_body: false,
					is_strict: false,
					returns_set: true,
					is_security_definer: false,
					is_leakproof: false,
					language: s("sql"),
				},
				Function {
					name: s("rec"), owner: s("tempuser"),
					args: vec![
						FunctionArg { name: None, typ: r("int4"), mode: ArgMode::In, default: None },
						FunctionArg { name: Some(s("f1")), typ: r("int4"), mode: ArgMode::Out, default: None },
						FunctionArg { name: Some(s("f2")), typ: r("text"), mode: ArgMode::InOut, default: Some(s("'yeah'::text")) },
					],
					return_typ: r("record"),
					kind: FunctionKind::Function,
					volatility: FunctionVolatility::Volatile,
					body: s(" select $1, cast($1 as text) || ' is text' "),
					has_sql_body: false,
					is_strict: false,
					returns_set: false,
					is_security_definer: false,
					is_leakproof: false,
					language: s("sql"),
				},
			]))
		]));

		client.batch_execute(r#"
			create or replace function add(a integer, b integer = 0) returns integer
				language sql
				stable
				return a + b + 1;

			drop function dup;
			drop function rec;
		"#).await?;
		let functions = reflect::reflect_functions(&client).await?;
		assert_eq!(functions, HashMap::from([
			(s("public"), Set::from([
				Function {
					name: s("add"), owner: s("tempuser"),
					args: vec![
						FunctionArg { name: Some(s("a")), typ: r("int4"),  mode: ArgMode::In, default: None },
						FunctionArg { name: Some(s("b")), typ: r("int4"),  mode: ArgMode::In, default: Some(s("0")) },
					],
					return_typ: r("int4"),
					kind: FunctionKind::Function,
					volatility: FunctionVolatility::Stable,
					body: s("RETURN ((a + b) + 1)"),
					has_sql_body: true,
					is_strict: false,
					returns_set: false,
					is_security_definer: false,
					is_leakproof: false,
					language: s("sql"),
				},
			]))
		]));

		Ok::<_, postgres::Error>(())
	}).await??;

	Ok(())
}
