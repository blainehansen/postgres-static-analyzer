use crate::{
	Column, CompositeField, ConnectionSettings, DbState, ForeignKey, Map, PgClient, Ref, Role, SchemaState, Set, TableState, Typ, TypBody, make_default_settings, postgres
};
use std::collections::HashMap;

pub async fn reflect_db_state(
	client: &PgClient
) -> Result<DbState, postgres::Error> {
	let (current_database_settings, user_settings) = reflect_default_settings(client).await?;
	let current_database_settings = current_database_settings.unwrap_or_else(make_default_settings);
	let schemas = reflect_user_schemas(client).await?;
	let foreign_keys = reflect_foreign_keys(client).await?;

	Ok(DbState {
		// TODO this is definitely not the correct way to do this over time, which will become clear once Role has a bit more information in it
		roles: user_settings.into_iter().map(|(name, default_settings)| Role { name, default_settings }).collect(),
		default_settings: current_database_settings,
		schemas,
		foreign_keys,
	})
}


pub(crate) async fn reflect_default_settings(
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

	let (schemas, mut tables_map, all_typs) = tokio::try_join!(
		schema_query.bind(client).all(),
		reflect_user_tables(client),
		reflect_types(client)
	)?;

	use itertools::Itertools;
	let mut typs_map = all_typs.into_iter().into_grouping_map().collect::<Set<_>>();

	let schemas = schemas.into_iter().map(|schema_name| {
		let tables = tables_map.remove(&schema_name).unwrap_or_default();
		let typs = typs_map.remove(&schema_name).unwrap_or_default();
		SchemaState { name: schema_name, tables, typs }
	}).collect();

	Ok(schemas)
}


// https://www.postgresql.org/docs/current/catalog-pg-class.html
pub(crate) async fn reflect_user_tables(
	client: &PgClient
) -> Result<HashMap<String, Set<TableState>>, postgres::Error> {


	let tables_query = reflect_crate::queries::main::reflect_user_tables();
	let (tables, all_columns, all_unique_constraints) = tokio::try_join!(
		tables_query.bind(client)
			.map(|t| {
				(t.nspname.to_string(), TableState {
					name: t.relname.to_string(),
					columns: Set::new(),
					primary_key: t.primary_key_columns.map(|c| {
						(t.conname.unwrap_or_default().to_string(), c.map(str::to_string).collect())
					}),
					unique_constraints: HashMap::new(),
				})
			})
			// TODO probably possible to use the iter() method to get a stream and then deal with that
			.all(),
		reflect_user_table_columns(client),
		reflect_user_table_unique_constraints(client),
	)?;

	use itertools::Itertools;
	let mut tables = tables.into_iter().into_grouping_map().collect::<Set<_>>();

	for ((schema_name, table_name), columns) in all_columns {
		if let Some(tables_in_schema) = tables.get_mut(&schema_name) {
			if let Some(mut table) = tables_in_schema.take(&table_name.as_str()) {
				table.columns.extend(columns);
				tables_in_schema.insert(table);
			}
		}
	}

	for ((schema_name, table_name), unique_constraints) in all_unique_constraints {
		if let Some(tables_in_schema) = tables.get_mut(&schema_name) {
			if let Some(mut table) = tables_in_schema.take(&table_name.as_str()) {
				table.unique_constraints.extend(unique_constraints);
				tables_in_schema.insert(table);
			}
		}

	}

	Ok(tables)
}


// https://www.postgresql.org/docs/current/catalog-pg-attribute.html
pub(crate) async fn reflect_user_table_columns(
	client: &PgClient
) -> Result<HashMap<(String, String), Vec<Column>>, postgres::Error> {
	use itertools::Itertools;

	let columns = reflect_crate::queries::main::reflect_user_table_columns().bind(client)
		.map(|a| {
			// TODO attgenerated
			let column = Column {
				name: a.attname.to_string(),
				typ: Ref { schema_name: a.typ_nspname.to_string(), name: a.typname.to_string() },
				not_null: a.attnotnull,
				default_expr: a.attdef.map(str::to_string),
			};

			((a.nspname.to_string(), a.relname.to_string()), column)
		})
		.all()
		.await?
		.into_iter().into_grouping_map().collect();

	Ok(columns)
}

pub(crate) async fn reflect_user_table_unique_constraints(
	client: &PgClient
) -> Result<HashMap<(String, String), Vec<(String, Set<String>)>>, postgres::Error> {
	use itertools::Itertools;

	let unique_constraints = reflect_crate::queries::main::reflect_user_table_unique_constraints().bind(client)
		.map(|uc| {
			(
				(uc.nspname.to_string(), uc.relname.to_string()),
				(uc.conname.to_string(), uc.unique_columns.map(str::to_string).collect()),
			)
		})
		.all()
		.await?
		.into_iter()
		.into_grouping_map()
		.collect::<Vec<_>>();

	Ok(unique_constraints)
}


pub(crate) async fn reflect_foreign_keys(
	client: &PgClient
) -> Result<Vec<ForeignKey>, postgres::Error> {
	let foreign_keys = reflect_crate::queries::main::reflect_foreign_keys().bind(client)
		.map(|fk| {
			ForeignKey {
				constraint_name: fk.conname.to_string(),
				referring_schema: fk.referring_schema.to_string(),
				referring_table: fk.referring_table.to_string(),
				referring_columns: fk.referring_columns.map(str::to_string).collect(),
				referred_schema: fk.referred_schema.to_string(),
				referred_table: fk.referred_table.to_string(),
				referred_columns: fk.referred_columns.map(str::to_string).collect(),
			}
		})
		.all().await?;
	Ok(foreign_keys)
}



pub(crate) async fn reflect_types(
	client: &PgClient
) -> Result<Vec<(String, Typ)>, postgres::Error> {
	let (enum_types, composite_types) = tokio::try_join!(
		reflect_enum_types(client),
		reflect_composite_types(client),
	)?;

	let all_typs = [
		enum_types,
		composite_types,
	].concat();

	Ok(all_typs)
}

pub(crate) async fn reflect_composite_types(
	client: &PgClient
) -> Result<Vec<(String, Typ)>, postgres::Error> {
	let composite_types = reflect_crate::queries::main::reflect_composite_types().bind(client)
		.map(|t| {
			let fields = itertools::izip!(t.field_nums, t.field_names, t.field_typ_schemas, t.field_typs)
				.map(|(field_num, field_name, field_typ_schema, field_typ)| {
					debug_assert!(field_num > 0);

					CompositeField {
						name: field_name.to_string(),
						field_num: field_num.unsigned_abs(),
						typ: Ref {
							schema_name: field_typ_schema.to_string(),
							name: field_typ.to_string()
						}
					}
				})
				.collect::<Set<_>>();

			(
				t.nspname.to_string(),
				Typ {
					name: t.typname.to_string(),
					body: TypBody::Composite { fields },
				},
			)
		})
		.all().await?;

	Ok(composite_types)
}

pub(crate) async fn reflect_enum_types(
	client: &PgClient
) -> Result<Vec<(String, Typ)>, postgres::Error> {
	let enum_types = reflect_crate::queries::main::reflect_enum_types().bind(client)
		.map(|t| {
			(
				t.nspname.to_string(),
				Typ {
					name: t.typname.to_string(),
					body: TypBody::Enum { values: t.enum_values.map(str::to_string).collect() },
				},
			)
		})
		.all().await?;

	Ok(enum_types)
}
