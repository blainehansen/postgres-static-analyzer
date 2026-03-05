use crate::{
	ArgMode, Column, CompositeField, ConnectionSettings, DbGrant, DbPrivilege, DbState, ForeignKey, Function, FunctionArg, FunctionExecute, FunctionGrant, FunctionKind, FunctionVolatility, Hash2Key, Hash3Key, PgClient, Ref, Role, RoleMembership, SchemaGrant, SchemaPrivilege, SchemaState, Set, TableColumnGrant, TableColumnPrivilege, TableGrant, TablePrivilege, TableState, Typ, TypBody, TypeGrant, TypeUsage, make_default_settings, postgres, HashMap, GroupMapHb, Str,
};

pub async fn reflect_db_state(
	client: &PgClient
) -> Result<DbState, postgres::Error> {
	let (
		roles,
		role_memberships,
		default_settings,
		grants,
		schemas,
		foreign_keys,
	) = tokio::try_join!(
		reflect_roles(client),
		reflect_role_memberships(client),
		reflect_db_default_setting(client),
		reflect_db_grants(client),
		reflect_user_schemas(client),
		reflect_foreign_keys(client),
	)?;
	let default_settings = default_settings.unwrap_or_else(make_default_settings);

	Ok(DbState { roles, role_memberships, default_settings, schemas, foreign_keys, grants })
}

pub async fn reflect_role_memberships(
	client: &PgClient
) -> Result<Vec<RoleMembership>, postgres::Error> {
	let role_memberships = reflect_crate::queries::main::reflect_role_memberships().bind(client)
		.map(|rm| RoleMembership {
			parent_role: rm.parent_role.into(),
			child_role: rm.child_role.into(),
			grantor: rm.grantor.into(),
			can_regrant_option: rm.admin_option,
			does_auto_inherit: rm.inherit_option,
			can_set_to: rm.set_option,
		})
		.all().await?;

	Ok(role_memberships)
}


pub(crate) async fn reflect_roles(
	client: &PgClient
) -> Result<Set<Role>, postgres::Error> {
	let roles = reflect_crate::queries::main::reflect_roles().bind(client)
		.map(|r| {
			Role {
				name: r.name.into(),
				can_create_db: r.rolcreatedb,
				can_create_role: r.rolcreaterole,
				can_login: r.rolcanlogin,
				is_super: r.rolsuper,
				does_inherit: r.rolinherit,
				is_replication: r.rolreplication,
				does_bypass_rls: r.rolbypassrls,
				valid_until: r.rolvaliduntil,
				default_search_path: r.default_search_path.map(|i| ConnectionSettings { search_path: i.into_iter().map(Str::from).collect() }),
				db_search_path: r.db_search_path.map(|i| ConnectionSettings { search_path: i.into_iter().map(Str::from).collect() }),
			}
		})
		.all().await?.into_iter().collect();

	Ok(roles)
}


pub(crate) async fn reflect_db_default_setting(
	client: &PgClient
) -> Result<Option<ConnectionSettings>, postgres::Error> {
	let db_default_setting = reflect_crate::queries::main::reflect_db_default_setting().bind(client).opt().await?
		.map(|s| ConnectionSettings { search_path: s.into_iter().map(Str::from).collect() });
	Ok(db_default_setting)
}

pub(crate) async fn reflect_db_grants(
	client: &PgClient
) -> Result<HashMap<Str, Vec<DbGrant>>, postgres::Error> {
	let grants = reflect_crate::queries::main::reflect_db_grants().bind(client)
		.map(|s| {
			let user_grants = itertools::izip!(s.privilege_types, s.is_grantables, s.grantors)
				.map(|(privilege_type, is_grantable, grantor)| {
					let privilege_type = DbPrivilege::pg_from_str(privilege_type);
					DbGrant { privilege_type, is_grantable, grantor: grantor.into() }
				})
				.collect();

			(s.grantee.into(), user_grants)
		})
		.all().await?
		.into_iter().collect();
	Ok(grants)
}


pub(crate) async fn reflect_schema_grants(
	client: &PgClient
) -> Result<HashMap<Str, HashMap<Str, Vec<SchemaGrant>>>, postgres::Error> {

	let grants_map = reflect_crate::queries::main::reflect_schema_grants().bind(client)
		.map(|g| {
			let user_grants = itertools::izip!(g.grantees, g.privilege_types, g.is_grantables, g.grantors)
				.map(|(grantee, privilege_type, is_grantable, grantor)| {
					let privilege_type = SchemaPrivilege::pg_from_str(privilege_type);
					(grantee.into(), SchemaGrant { privilege_type, is_grantable, grantor: grantor.into() })
				})
				.into_group_map_hashbrown();

			(g.nspname.into(), user_grants)
		})
		.all().await?
		.into_iter().collect::<HashMap<_, _>>();

	Ok(grants_map)
}


// https://www.postgresql.org/docs/current/catalog-pg-namespace.html
pub(crate) async fn reflect_user_schemas(
	client: &PgClient
) -> Result<Set<SchemaState>, postgres::Error> {
	let schema_query = reflect_crate::queries::main::reflect_user_schemas();

	let (schemas, mut tables_map, all_typs, mut functions_map, mut grants_map) = tokio::try_join!(
		schema_query.bind(client).all(),
		reflect_user_tables(client),
		reflect_types(client),
		reflect_functions(client),
		reflect_schema_grants(client),
	)?;

	let mut typs_map = all_typs.into_iter()
		.into_group_map_hashbrown();

	let schemas = schemas.into_iter().map(|s| {
		let tables = tables_map.remove(s.nspname.as_str()).unwrap_or_default();
		let typs = typs_map.remove(s.nspname.as_str()).unwrap_or_default();
		let functions = functions_map.remove(s.nspname.as_str()).unwrap_or_default();
		let grants = grants_map.remove(s.nspname.as_str()).unwrap_or_default();
		SchemaState { name: s.nspname.into(), tables, typs, functions, owner: s.owner.into(), grants }
	}).collect();

	Ok(schemas)
}


// https://www.postgresql.org/docs/current/catalog-pg-class.html
pub(crate) async fn reflect_user_tables(
	client: &PgClient
) -> Result<HashMap<Str, Set<TableState>>, postgres::Error> {
	let tables_query = reflect_crate::queries::main::reflect_user_tables();
	let (tables, all_columns, all_unique_constraints, grants_map, column_grants_map) = tokio::try_join!(
		tables_query.bind(client)
			.map(|t| {
				(t.nspname.into(), TableState {
					name: t.relname.into(), owner: t.owner.into(),
					columns: Set::new(),
					primary_key: t.primary_key_columns.map(|c| {
						(t.conname.unwrap_or_default().into(), c.into_iter().map(Str::from).collect())
					}),
					unique_constraints: HashMap::new(),
					grants: HashMap::new(),
				})
			})
			// TODO probably possible to use the iter() method to get a stream and then deal with that
			.all(),
		reflect_user_table_columns(client),
		reflect_user_table_unique_constraints(client),
		reflect_table_grants(client),
		reflect_column_grants(client),
	)?;

	let mut tables = tables.into_iter()
		.into_group_map_hashbrown::<_, _, Set<_>>();

	for (Hash2Key(schema_name, table_name), columns) in all_columns {
		if let Some(tables_in_schema) = tables.get_mut(schema_name.as_str()) {
			if let Some(mut table) = tables_in_schema.take(table_name.as_str()) {
				table.columns.extend(columns);
				tables_in_schema.insert(table);
			}
		}
	}

	for (Hash2Key(schema_name, table_name), unique_constraints) in all_unique_constraints {
		if let Some(tables_in_schema) = tables.get_mut(schema_name.as_str()) {
			if let Some(mut table) = tables_in_schema.take(table_name.as_str()) {
				table.unique_constraints.extend(unique_constraints);
				tables_in_schema.insert(table);
			}
		}
	}

	// TODO since we just iterate over grants_map, and since the underlying query in reflect_table_grants is already unique, maybe we don't need a hashmap at all?
	for (Hash2Key(schema_name, table_name), grants) in grants_map {
		if let Some(tables_in_schema) = tables.get_mut(schema_name.as_str()) {
			if let Some(mut table) = tables_in_schema.take(table_name.as_str()) {
				table.grants = grants;
				tables_in_schema.insert(table);
			}
		}
	}

	for (Hash3Key(schema_name, table_name, column_name), grants) in column_grants_map {
		if let Some(tables_in_schema) = tables.get_mut(schema_name.as_str()) {
			if let Some(mut table) = tables_in_schema.take(table_name.as_str()) {
				if let Some(mut column) = table.columns.take(column_name.as_str()) {
					column.grants = grants;
					table.columns.insert(column);
				}
				tables_in_schema.insert(table);
			}
		}
	}

	Ok(tables)
}


pub(crate) async fn reflect_table_grants(
	client: &PgClient
) -> Result<HashMap<Hash2Key, HashMap<Str, Vec<TableGrant>>>, postgres::Error> {

	let grants_map = reflect_crate::queries::main::reflect_table_grants().bind(client)
		.map(|g| {
			let user_grants = itertools::izip!(g.grantees, g.privilege_types, g.is_grantables, g.grantors)
				.map(|(grantee, privilege_type, is_grantable, grantor)| {
					let privilege_type = TablePrivilege::pg_from_str(privilege_type);
					(Str::from(grantee), TableGrant { privilege_type, is_grantable, grantor: grantor.into() })
				})
				.into_group_map_hashbrown::<_, _, Vec<_>>();

			(Hash2Key(g.nspname.into(), g.relname.into()), user_grants)
		})
		.all().await?
		.into_iter().collect::<HashMap<_, _>>();

	Ok(grants_map)
}


pub(crate) async fn reflect_column_grants(
	client: &PgClient
) -> Result<HashMap<Hash3Key, HashMap<Str, Vec<TableColumnGrant>>>, postgres::Error> {

	let grants_map = reflect_crate::queries::main::reflect_column_grants().bind(client)
		.map(|g| {
			let user_grants = itertools::izip!(g.grantees, g.privilege_types, g.is_grantables, g.grantors)
				.map(|(grantee, privilege_type, is_grantable, grantor)| {
					let privilege_type = TableColumnPrivilege::pg_from_str(privilege_type);
					(Str::from(grantee), TableColumnGrant { privilege_type, is_grantable, grantor: grantor.into() })
				})
				.into_group_map_hashbrown::<_, _, Vec<_>>();

			(Hash3Key(g.nspname.into(), g.relname.into(), g.attname.into()), user_grants)
		})
		.all().await?
		.into_iter().collect::<HashMap<_, _>>();

	Ok(grants_map)
}


// https://www.postgresql.org/docs/current/catalog-pg-attribute.html
pub(crate) async fn reflect_user_table_columns(
	client: &PgClient
) -> Result<HashMap<Hash2Key, Vec<Column>>, postgres::Error> {
	let columns = reflect_crate::queries::main::reflect_user_table_columns().bind(client)
		.map(|a| {
			// TODO attgenerated
			let column = Column {
				name: a.attname.into(),
				typ: Ref { schema_name: a.typ_nspname.into(), name: a.typname.into() },
				not_null: a.attnotnull,
				default_expr: a.attdef.map(Str::from),
				grants: HashMap::new(),
			};

			(Hash2Key(a.nspname.into(), a.relname.into()), column)
		})
		.all()
		.await?
		.into_iter()
		.into_group_map_hashbrown::<_, _, Vec<_>>();

	Ok(columns)
}

pub(crate) async fn reflect_user_table_unique_constraints(
	client: &PgClient
) -> Result<HashMap<Hash2Key, Vec<(Str, Set<Str>)>>, postgres::Error> {

	let unique_constraints = reflect_crate::queries::main::reflect_user_table_unique_constraints().bind(client)
		.map(|uc| {
			(
				Hash2Key(uc.nspname.into(), uc.relname.into()),
				(uc.conname.into(), uc.unique_columns.into_iter().map(Str::from).collect()),
			)
		})
		.all()
		.await?
		.into_iter()
		.into_group_map_hashbrown::<_, _, Vec<_>>();

	Ok(unique_constraints)
}


pub(crate) async fn reflect_foreign_keys(
	client: &PgClient
) -> Result<Vec<ForeignKey>, postgres::Error> {
	let foreign_keys = reflect_crate::queries::main::reflect_foreign_keys().bind(client)
		.map(|fk| {
			ForeignKey {
				constraint_name: fk.conname.into(),
				referring_schema: fk.referring_schema.into(),
				referring_table: fk.referring_table.into(),
				referring_columns: fk.referring_columns.into_iter().map(Str::from).collect(),
				referred_schema: fk.referred_schema.into(),
				referred_table: fk.referred_table.into(),
				referred_columns: fk.referred_columns.into_iter().map(Str::from).collect(),
			}
		})
		.all().await?;
	Ok(foreign_keys)
}



pub(crate) async fn reflect_types(
	client: &PgClient
) -> Result<Vec<(Str, Typ)>, postgres::Error> {
	let (enum_types, composite_types, mut grants_map) = tokio::try_join!(
		reflect_enum_types(client),
		reflect_composite_types(client),
		reflect_type_grants(client),
	)?;

	let mut all_typs = [enum_types, composite_types].concat();

	for (schema_name, typ) in all_typs.iter_mut() {
		if let Some(grants) = grants_map.remove(&(schema_name.clone(), typ.name.clone())) {
			typ.grants = grants;
		}
	}

	Ok(all_typs)
}

pub(crate) async fn reflect_composite_types(
	client: &PgClient
) -> Result<Vec<(Str, Typ)>, postgres::Error> {
	let composite_types = reflect_crate::queries::main::reflect_composite_types().bind(client)
		.map(|t| {
			let fields = itertools::izip!(t.field_nums, t.field_names, t.field_typ_schemas, t.field_typs)
				.map(|(field_num, field_name, field_typ_schema, field_typ)| {
					debug_assert!(field_num > 0);

					CompositeField {
						name: field_name.into(),
						field_num: field_num.unsigned_abs(),
						typ: Ref {
							schema_name: field_typ_schema.into(),
							name: field_typ.into(),
						},
					}
				})
				.collect::<Set<_>>();

			(
				t.nspname.into(),
				Typ {
					name: t.typname.into(), owner: t.owner.into(),
					body: TypBody::Composite { fields },
					grants: HashMap::new(),
				},
			)
		})
		.all().await?;

	Ok(composite_types)
}

pub(crate) async fn reflect_enum_types(
	client: &PgClient
) -> Result<Vec<(Str, Typ)>, postgres::Error> {
	let enum_types = reflect_crate::queries::main::reflect_enum_types().bind(client)
		.map(|t| {
			(
				t.nspname.into(),
				Typ {
					name: t.typname.into(), owner: t.owner.into(),
					body: TypBody::Enum { values: t.enum_values.into_iter().map(Str::from).collect() },
					grants: HashMap::new(),
				},
			)
		})
		.all().await?;

	Ok(enum_types)
}

pub(crate) async fn reflect_type_grants(
	client: &PgClient
) -> Result<HashMap<Hash2Key, HashMap<Str, Vec<TypeGrant>>>, postgres::Error> {
	let grants_map = reflect_crate::queries::main::reflect_type_grants().bind(client)
		.map(|g| {
			let user_grants = itertools::izip!(g.grantees, g.is_grantables, g.grantors)
				.map(|(grantee, is_grantable, grantor)| {
					(
						grantee.into(),
						TypeGrant { privilege_type: TypeUsage, is_grantable, grantor: grantor.into() },
					)
				})
				.into_group_map_hashbrown::<_, _, Vec<_>>();

			(Hash2Key(g.nspname.into(), g.typname.into()), user_grants)
		})
		.all().await?
		.into_iter()
		.collect::<HashMap<_, _>>();

	Ok(grants_map)
}

pub(crate) async fn reflect_functions(
	client: &PgClient
) -> Result<HashMap<Str, Set<Function>>, postgres::Error> {

	let grants_map = reflect_crate::queries::main::reflect_function_grants().bind(client)
		.map(|g| {
			let user_grants = itertools::izip!(g.grantees, g.is_grantables, g.grantors)
				.map(|(grantee, is_grantable, grantor)| {
					(Str::from(grantee), FunctionGrant { privilege_type: FunctionExecute, is_grantable, grantor: grantor.into() })
				})
				.into_group_map_hashbrown::<_, _, Vec<_>>();

			((Str::from(g.nspname), Str::from(g.proname)), user_grants)
		})
		.all().await?
		.into_iter().collect::<HashMap<_, _>>();

	let functions = reflect_crate::queries::main::reflect_functions().bind(client)
		.map(|f| {
			let args = itertools::izip!(f.arg_modes, f.arg_names, f.arg_types, f.arg_type_schemas, f.arg_defaults)
				.map(|(mode, name, typ, typ_schema, default)| {
					let typ = Ref { schema_name: typ_schema.into(), name: typ.into() };
					let mode = ArgMode::pg_from_char(mode);
					FunctionArg { name: name.map(Str::from), mode, typ, default: default.map(Str::from) }
				})
				.collect::<Vec<_>>();

			(
				f.nspname.into(),
				Function {
					name: f.function_name.into(), owner: f.owner.into(),
					return_typ: Ref { schema_name: f.return_typ_schema.into(), name: f.return_typ_name.into() },
					args,
					kind: FunctionKind::pg_from_char(f.prokind),
					volatility: FunctionVolatility::pg_from_char(f.provolatile),
					body: f.body.into(),
					has_sql_body: f.has_sql_body,
					is_strict: f.proisstrict,
					returns_set: f.proretset,
					is_security_definer: f.prosecdef,
					is_leakproof: f.proleakproof,
					language: f.lang_name.into(),
					grants: HashMap::new(),
				},
			)
		})
		.all().await?;
	let mut functions = functions.into_iter()
		.into_group_map_hashbrown::<_, _, Set<_>>();

	for ((schema_name, function_name), grants) in grants_map {
		if let Some(functions_in_schema) = functions.get_mut(schema_name.as_str()) {
			if let Some(mut function) = functions_in_schema.take(function_name.as_str()) {
				function.grants = grants;
				functions_in_schema.insert(function);
			}
		}
	}

	Ok(functions)
}
