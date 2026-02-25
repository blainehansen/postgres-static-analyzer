use crate::{
	FunctionArg, ArgMode, Column, CompositeField, ConnectionSettings, DbState, ForeignKey, Function, FunctionKind, FunctionVolatility, PgClient, Ref, Role, SchemaState, Set, TableState, Typ, TypBody, make_default_settings, postgres
};
use std::collections::HashMap;

pub async fn reflect_db_state(
	client: &PgClient
) -> Result<DbState, postgres::Error> {
	let (
		roles,
		default_settings,
		schemas,
		foreign_keys,
	) = tokio::try_join!(
		reflect_roles(client),
		reflect_db_default_setting(client),
		reflect_user_schemas(client),
		reflect_foreign_keys(client),
	)?;
	let default_settings = default_settings.unwrap_or_else(make_default_settings);

	Ok(DbState { roles, default_settings, schemas, foreign_keys })
}


pub(crate) async fn reflect_roles(
	client: &PgClient
) -> Result<Set<Role>, postgres::Error> {
	let roles = reflect_crate::queries::main::reflect_roles().bind(client)
		.map(|r| {
			Role {
				name: r.name.to_string(),
				can_create_db: r.rolcreatedb,
				can_create_role: r.rolcreaterole,
				can_login: r.rolcanlogin,
				is_super: r.rolsuper,
				does_inherit: r.rolinherit,
				is_replication: r.rolreplication,
				does_bypass_rls: r.rolbypassrls,
				valid_until: r.rolvaliduntil,
				default_search_path: r.default_search_path.map(|i| ConnectionSettings { search_path: i.map(str::to_string).collect() }),
				db_search_path: r.db_search_path.map(|i| ConnectionSettings { search_path: i.map(str::to_string).collect() }),
			}
		})
		.all().await?.into_iter().collect();

	Ok(roles)
}


pub(crate) async fn reflect_db_default_setting(
	client: &PgClient
) -> Result<Option<ConnectionSettings>, postgres::Error> {
	let db_default_setting = reflect_crate::queries::main::reflect_db_default_setting().bind(client).opt().await?
		.map(|s| ConnectionSettings { search_path: s.to_owned() });
	Ok(db_default_setting)
}


// https://www.postgresql.org/docs/current/catalog-pg-namespace.html
pub(crate) async fn reflect_user_schemas(
	client: &PgClient
) -> Result<Set<SchemaState>, postgres::Error> {
	let schema_query = reflect_crate::queries::main::reflect_user_schemas();

	let (schemas, mut tables_map, all_typs, mut functions_map) = tokio::try_join!(
		schema_query.bind(client).all(),
		reflect_user_tables(client),
		reflect_types(client),
		reflect_functions(client),
	)?;

	use itertools::Itertools;
	let mut typs_map = all_typs.into_iter().into_grouping_map().collect::<Set<_>>();

	let schemas = schemas.into_iter().map(|s| {
		let tables = tables_map.remove(&s.nspname).unwrap_or_default();
		let typs = typs_map.remove(&s.nspname).unwrap_or_default();
		let functions = functions_map.remove(&s.nspname).unwrap_or_default();
		SchemaState { name: s.nspname, tables, typs, functions, owner: s.owner }
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
					name: t.relname.to_string(), owner: t.owner.to_string(),
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
					name: t.typname.to_string(), owner: t.owner.to_string(),
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
					name: t.typname.to_string(), owner: t.owner.to_string(),
					body: TypBody::Enum { values: t.enum_values.map(str::to_string).collect() },
				},
			)
		})
		.all().await?;

	Ok(enum_types)
}


pub(crate) async fn reflect_functions(
	client: &PgClient
) -> Result<HashMap<String, Set<Function>>, postgres::Error> {
	use itertools::Itertools;

	let functions = reflect_crate::queries::main::reflect_functions().bind(client)
		.map(|f| {
			let args = itertools::izip!(f.arg_modes, f.arg_names, f.arg_types, f.arg_type_schemas, f.arg_defaults)
				.map(|(mode, name, typ, typ_schema, default)| {
					let typ = Ref { schema_name: typ_schema.to_string(), name: typ.to_string() };
					// encoded as i for IN arguments, o for OUT arguments, b for INOUT arguments, v for VARIADIC arguments, t for TABLE arguments
					let mode = match mode as u8 as char {
						'i' => ArgMode::In, 'o' => ArgMode::Out, 'b' => ArgMode::InOut, 'v' => ArgMode::Variadic, 't' => ArgMode::Table,
						_ => ArgMode::In,
					};
					FunctionArg { name: name.map(str::to_string), mode, typ, default: default.map(str::to_string) }
				})
				.collect::<Vec<_>>();

			(
				f.nspname.to_string(),
				Function {
					name: f.function_name.to_string(), owner: f.owner.to_string(),
					return_typ: Ref { schema_name: f.return_typ_schema.to_string(), name: f.return_typ_name.to_string() },
					args,
					// f for a normal function, p for a procedure, a for an aggregate function, or w for a window function
					kind: match f.prokind as u8 as char {
						'f' => FunctionKind::Function, 'p' => FunctionKind::Procedure, 'a' => FunctionKind::Aggregate, 'w' => FunctionKind::Window,
						_ => FunctionKind::Function,
					},
					// It is i for "immutable" functions, which always deliver the same result for the same inputs. It is s for "stable" functions, whose results (for fixed inputs) do not change within a scan. It is v for "volatile" functions, whose results might change at any time.
					volatility: match f.provolatile as u8 as char {
						'i' => FunctionVolatility::Immutable, 's' => FunctionVolatility::Stable, 'v' => FunctionVolatility::Volatile,
						_ => FunctionVolatility::Volatile,
					},
					body: f.body.to_string(),
					has_sql_body: f.has_sql_body,
					is_strict: f.proisstrict,
					returns_set: f.proretset,
					is_security_definer: f.prosecdef,
					is_leakproof: f.proleakproof,
					language: f.lang_name.to_string(),
				}
			)
		})
		.all().await?
		.into_iter()
		.into_grouping_map()
		.collect();

	Ok(functions)

}

