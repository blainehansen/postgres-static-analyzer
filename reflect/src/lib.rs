pub(crate) use postgres_static_analyzer_reflect_queries as queries_crate;
pub(crate) use queries_crate::tokio_postgres as postgres;
pub use queries_crate::tokio_postgres::Client as PgClient;

#[cfg(test)]
mod reflect_test;

mod reflect_gen;

pub use postgres_static_analyzer_ddl_catalog_structs::*;
use futures::TryStreamExt;

pub async fn reflect_pg_state(
	client: &PgClient
) -> Result<PgState, postgres::Error> {
	client.batch_execute(include_str!("../reflect_fns.sql")).await?;

	let (
		pg_aggregate,
		pg_am,
		pg_amop,
		pg_amproc,
		pg_attrdef,
		pg_attribute,
		pg_roles,
		pg_auth_members,
		pg_cast,
		pg_class,
		pg_collation,
		pg_constraint,
		pg_conversion,
		pg_database,
		pg_db_role_setting,
		pg_default_acl,
		// pg_depend,
		// pg_description,
		pg_enum,
		pg_event_trigger,
		pg_extension,
		pg_foreign_data_wrapper,
		pg_foreign_server,
		pg_foreign_table,
		pg_index,
		pg_inherits,
		// pg_init_privs,
		pg_language,
		pg_namespace,
		pg_opclass,
		pg_operator,
		pg_opfamily,
		pg_parameter_acl,
		pg_partitioned_table,
		pg_policy,
		pg_proc,
		pg_publication,
		pg_publication_namespace,
		pg_publication_rel,
		pg_range,
		// pg_replication_origin,
		// pg_rewrite,
		pg_rules,
		pg_views,
		pg_matviews,
		// pg_seclabel,
		pg_sequence,
		// pg_shdepend,
		// pg_shdescription,
		// pg_shseclabel,
		pg_statistic_ext,
		pg_subscription,
		pg_transform,
		pg_trigger,
		pg_ts_config,
		pg_ts_config_map,
		pg_ts_dict,
		pg_ts_parser,
		pg_ts_template,
		pg_type,
		pg_user_mappings,
	) = tokio::try_join!(
		reflect_pg_aggregate(client),
		reflect_pg_am(client),
		reflect_pg_amop(client),
		reflect_pg_amproc(client),
		reflect_pg_attrdef(client),
		reflect_pg_attribute(client),
		reflect_pg_roles(client),
		reflect_pg_auth_members(client),
		reflect_pg_cast(client),
		reflect_pg_class(client),
		reflect_pg_collation(client),
		reflect_pg_constraint(client),
		reflect_pg_conversion(client),
		reflect_pg_database(client),
		reflect_pg_db_role_setting(client),
		reflect_pg_default_acl(client),
		// reflect_pg_depend(client),
		// reflect_pg_description(client),
		reflect_pg_enum(client),
		reflect_pg_event_trigger(client),
		reflect_pg_extension(client),
		reflect_pg_foreign_data_wrapper(client),
		reflect_pg_foreign_server(client),
		reflect_pg_foreign_table(client),
		reflect_pg_index(client),
		reflect_pg_inherits(client),
		// reflect_pg_init_privs(client),
		reflect_pg_language(client),
		reflect_pg_namespace(client),
		reflect_pg_opclass(client),
		reflect_pg_operator(client),
		reflect_pg_opfamily(client),
		reflect_pg_parameter_acl(client),
		reflect_pg_partitioned_table(client),
		reflect_pg_policy(client),
		reflect_pg_proc(client),
		reflect_pg_publication(client),
		reflect_pg_publication_namespace(client),
		reflect_pg_publication_rel(client),
		reflect_pg_range(client),
		// reflect_pg_replication_origin(client),
		// reflect_pg_rewrite(client),
		reflect_pg_rules(client),
		reflect_pg_views(client),
		reflect_pg_matviews(client),
		// reflect_pg_seclabel(client),
		reflect_pg_sequence(client),
		// reflect_pg_shdepend(client),
		// reflect_pg_shdescription(client),
		// reflect_pg_shseclabel(client),
		reflect_pg_statistic_ext(client),
		reflect_pg_subscription(client),
		reflect_pg_transform(client),
		reflect_pg_trigger(client),
		reflect_pg_ts_config(client),
		reflect_pg_ts_config_map(client),
		reflect_pg_ts_dict(client),
		reflect_pg_ts_parser(client),
		reflect_pg_ts_template(client),
		reflect_pg_type(client),
		reflect_pg_user_mappings(client),
	)?;

	Ok(PgState {
		pg_aggregate,
		pg_am,
		pg_amop,
		pg_amproc,
		pg_attrdef,
		pg_attribute,
		pg_roles,
		pg_auth_members,
		pg_cast,
		pg_class,
		pg_collation,
		pg_constraint,
		pg_conversion,
		pg_database,
		pg_db_role_setting,
		pg_default_acl,
		// pg_depend,
		// pg_description,
		pg_enum,
		pg_event_trigger,
		pg_extension,
		pg_foreign_data_wrapper,
		pg_foreign_server,
		pg_foreign_table,
		pg_index,
		pg_inherits,
		// pg_init_privs,
		pg_language,
		pg_namespace,
		pg_opclass,
		pg_operator,
		pg_opfamily,
		pg_parameter_acl,
		pg_partitioned_table,
		pg_policy,
		pg_proc,
		pg_publication,
		pg_publication_namespace,
		pg_publication_rel,
		pg_range,
		// pg_replication_origin,
		// pg_rewrite,
		pg_rules,
		pg_views,
		pg_matviews,
		// pg_seclabel,
		pg_sequence,
		// pg_shdepend,
		// pg_shdescription,
		// pg_shseclabel,
		pg_statistic_ext,
		pg_subscription,
		pg_transform,
		pg_trigger,
		pg_ts_config,
		pg_ts_config_map,
		pg_ts_dict,
		pg_ts_parser,
		pg_ts_template,
		pg_type,
		pg_user_mappings,
	})
}


// `pg_aggregate`: https://www.postgresql.org/docs/17/catalog-pg-aggregate.html
pub use reflect_gen::reflect_pg_aggregate;

// `pg_am`: https://www.postgresql.org/docs/17/catalog-pg-am.html
pub use reflect_gen::reflect_pg_am;

// `pg_amop`: https://www.postgresql.org/docs/17/catalog-pg-amop.html
pub use reflect_gen::reflect_pg_amop;

// `pg_amproc`: https://www.postgresql.org/docs/17/catalog-pg-amproc.html
pub use reflect_gen::reflect_pg_amproc;

// `pg_attrdef`: https://www.postgresql.org/docs/17/catalog-pg-attrdef.html
pub use reflect_gen::reflect_pg_attrdef;

// `pg_attribute`: https://www.postgresql.org/docs/17/catalog-pg-attribute.html
pub use reflect_gen::reflect_pg_attribute;

// `pg_authid`: https://www.postgresql.org/docs/17/catalog-pg-authid.html
// `pg_roles`: https://www.postgresql.org/docs/17/view-pg-roles.html
pub use reflect_gen::reflect_pg_roles;

// `pg_auth_members`: https://www.postgresql.org/docs/17/catalog-pg-auth-members.html
pub use reflect_gen::reflect_pg_auth_members;

// `pg_cast`: https://www.postgresql.org/docs/17/catalog-pg-cast.html
pub use reflect_gen::reflect_pg_cast;

// `pg_class`: https://www.postgresql.org/docs/17/catalog-pg-class.html
pub use reflect_gen::reflect_pg_class;

// `pg_collation`: https://www.postgresql.org/docs/17/catalog-pg-collation.html
pub use reflect_gen::reflect_pg_collation;

// `pg_constraint`: https://www.postgresql.org/docs/17/catalog-pg-constraint.html
pub use reflect_gen::reflect_pg_constraint;

// `pg_conversion`: https://www.postgresql.org/docs/17/catalog-pg-conversion.html
pub use reflect_gen::reflect_pg_conversion;

// `pg_database`: https://www.postgresql.org/docs/17/catalog-pg-database.html
pub use reflect_gen::reflect_pg_database;

// `pg_db_role_setting`: https://www.postgresql.org/docs/17/catalog-pg-db-role-setting.html
pub async fn reflect_pg_db_role_setting(
	client: &PgClient
) -> Result<Vec<PgDbRoleSetting>, postgres::Error> {
	let pg_db_role_setting_coll = queries_crate::queries::manual::reflect_pg_db_role_setting().bind(client)
		.map(|pg_db_role_setting| {
			PgDbRoleSetting {
				setdatabase: if pg_db_role_setting.setdatabase { Some(()) } else { None },
				setrole: pg_db_role_setting.setrole.map(Into::into),
				setconfig: pg_db_role_setting.setconfig.map(|items| items.map(Into::into).collect()),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_db_role_setting_coll)
}

// `pg_default_acl`: https://www.postgresql.org/docs/17/catalog-pg-default-acl.html
pub use reflect_gen::reflect_pg_default_acl;

// `pg_depend`: https://www.postgresql.org/docs/17/catalog-pg-depend.html

// `pg_description`: https://www.postgresql.org/docs/17/catalog-pg-description.html

// `pg_enum`: https://www.postgresql.org/docs/17/catalog-pg-enum.html
pub async fn reflect_pg_enum(
	client: &PgClient
) -> Result<Set<PgEnum>, postgres::Error> {
	let pg_enum_coll = queries_crate::queries::manual::reflect_pg_enum().bind(client)
		.map(|pg_enum| {
			PgEnum {
				enumtypid: Qual::parse(pg_enum.enumtypid),
				enumlabels: pg_enum.enumlabels.map(Into::into).collect(),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_enum_coll)
}

// `pg_event_trigger`: https://www.postgresql.org/docs/17/catalog-pg-event-trigger.html
pub use reflect_gen::reflect_pg_event_trigger;

// `pg_extension`: https://www.postgresql.org/docs/17/catalog-pg-extension.html
pub use reflect_gen::reflect_pg_extension;

// `pg_foreign_data_wrapper`: https://www.postgresql.org/docs/17/catalog-pg-foreign-data-wrapper.html
pub use reflect_gen::reflect_pg_foreign_data_wrapper;

// `pg_foreign_server`: https://www.postgresql.org/docs/17/catalog-pg-foreign-server.html
pub use reflect_gen::reflect_pg_foreign_server;

// `pg_foreign_table`: https://www.postgresql.org/docs/17/catalog-pg-foreign-table.html
pub use reflect_gen::reflect_pg_foreign_table;

// `pg_index`: https://www.postgresql.org/docs/17/catalog-pg-index.html
pub use reflect_gen::reflect_pg_index;

// `pg_inherits`: https://www.postgresql.org/docs/17/catalog-pg-inherits.html
pub use reflect_gen::reflect_pg_inherits;

// `pg_init_privs`: https://www.postgresql.org/docs/17/catalog-pg-init-privs.html

// `pg_language`: https://www.postgresql.org/docs/17/catalog-pg-language.html
pub use reflect_gen::reflect_pg_language;

// `pg_namespace`: https://www.postgresql.org/docs/17/catalog-pg-namespace.html
pub use reflect_gen::reflect_pg_namespace;

// `pg_opclass`: https://www.postgresql.org/docs/17/catalog-pg-opclass.html
pub use reflect_gen::reflect_pg_opclass;

// `pg_operator`: https://www.postgresql.org/docs/17/catalog-pg-operator.html
pub use reflect_gen::reflect_pg_operator;

// `pg_opfamily`: https://www.postgresql.org/docs/17/catalog-pg-opfamily.html
pub use reflect_gen::reflect_pg_opfamily;

// `pg_parameter_acl`: https://www.postgresql.org/docs/17/catalog-pg-parameter-acl.html
pub use reflect_gen::reflect_pg_parameter_acl;

// `pg_partitioned_table`: https://www.postgresql.org/docs/17/catalog-pg-partitioned-table.html
pub use reflect_gen::reflect_pg_partitioned_table;

// `pg_policy`: https://www.postgresql.org/docs/17/catalog-pg-policy.html
pub use reflect_gen::reflect_pg_policy;

// `pg_proc`: https://www.postgresql.org/docs/17/catalog-pg-proc.html
pub async fn reflect_pg_proc(
	client: &PgClient
) -> Result<Set<PgProc>, postgres::Error> {
	let pg_proc_coll = queries_crate::queries::manual::reflect_pg_proc().bind(client)
		.map(|pg_proc| {
			PgProc {
				oid: Qual::parse(pg_proc.oid),
				proname: pg_proc.proname.into(),
				pronamespace: pg_proc.pronamespace.into(),
				proowner: pg_proc.proowner.into(),
				prolang: pg_proc.prolang.into(),
				procost: pg_proc.procost.map(|n| ordered_float::NotNan::new(n).unwrap()),
				prorows: pg_proc.prorows.map(|n| ordered_float::NotNan::new(n).unwrap()),
				provariadic: Qual::maybe_parse(pg_proc.provariadic),
				prosupport: Qual::maybe_parse(pg_proc.prosupport),
				prokind: PgProcProkind::pg_from_char(pg_proc.prokind),
				prosecdef: pg_proc.prosecdef,
				proleakproof: pg_proc.proleakproof,
				proisstrict: pg_proc.proisstrict,
				proretset: pg_proc.proretset,
				provolatile: PgProcProvolatile::pg_from_char(pg_proc.provolatile),
				proparallel: PgProcProparallel::pg_from_char(pg_proc.proparallel),
				pronargs: pg_proc.pronargs.unsigned_abs(),
				pronargdefaults: pg_proc.pronargdefaults.unsigned_abs(),
				prorettype: Qual::parse(pg_proc.prorettype),
		    proargtypes: pg_proc.proargtypes.map(Qual::parse).collect(),
				proallargtypes: pg_proc.proallargtypes.map(|items| items.map(Qual::parse).collect()),
		    proargmodes: pg_proc.proargmodes.map(|items| items.map(PgProcProargmodes::pg_from_char).collect()),
				proargnames: pg_proc.proargnames.map(|items| items.map(Into::into).collect()),
		    proargdefaults: pg_proc.proargdefaults.map(|items| items.map(|item| item.map(Into::into)).collect()),
				protrftypes: pg_proc.protrftypes.map(|items| items.map(Qual::parse).collect()),
				prosrc: pg_proc.prosrc.map(Into::into),
				probin: pg_proc.probin.map(Into::into),
		    prosqlbody: pg_proc.prosqlbody.map(Into::into),
				proconfig: pg_proc.proconfig.map(|items| items.map(Into::into).collect()),
				proacl: pg_proc.proacl.map(|proacl| proacl.map(|acl| aclitem::aclitem(acl, &aclitem::FunctionGrantParser)).collect()),
				description: pg_proc.description.map(Into::into),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_proc_coll)
}

// `pg_publication`: https://www.postgresql.org/docs/17/catalog-pg-publication.html
pub use reflect_gen::reflect_pg_publication;

// `pg_publication_namespace`: https://www.postgresql.org/docs/17/catalog-pg-publication-namespace.html
pub use reflect_gen::reflect_pg_publication_namespace;

// `pg_publication_rel`: https://www.postgresql.org/docs/17/catalog-pg-publication-rel.html
pub use reflect_gen::reflect_pg_publication_rel;

// `pg_range`: https://www.postgresql.org/docs/17/catalog-pg-range.html
pub use reflect_gen::reflect_pg_range;

// `pg_replication_origin`: https://www.postgresql.org/docs/17/catalog-pg-replication-origin.html

// `pg_rewrite`: https://www.postgresql.org/docs/17/catalog-pg-rewrite.html

// `pg_rules`: https://www.postgresql.org/docs/17/view-pg-rules.html
pub use reflect_gen::reflect_pg_rules;
// `pg_views`: https://www.postgresql.org/docs/17/view-pg-views.html
pub use reflect_gen::reflect_pg_views;
// `pg_matviews`: https://www.postgresql.org/docs/17/view-pg-matviews.html
pub use reflect_gen::reflect_pg_matviews;

// `pg_seclabel`: https://www.postgresql.org/docs/17/catalog-pg-seclabel.html

// `pg_sequence`: https://www.postgresql.org/docs/17/catalog-pg-sequence.html
pub use reflect_gen::reflect_pg_sequence;

// `pg_shdepend`: https://www.postgresql.org/docs/17/catalog-pg-shdepend.html

// `pg_shdescription`: https://www.postgresql.org/docs/17/catalog-pg-shdescription.html

// `pg_shseclabel`: https://www.postgresql.org/docs/17/catalog-pg-shseclabel.html

// `pg_statistic_ext`: https://www.postgresql.org/docs/17/catalog-pg-statistic-ext.html
pub use reflect_gen::reflect_pg_statistic_ext;

// `pg_subscription`: https://www.postgresql.org/docs/17/catalog-pg-subscription.html
pub use reflect_gen::reflect_pg_subscription;

// `pg_transform`: https://www.postgresql.org/docs/17/catalog-pg-transform.html
pub use reflect_gen::reflect_pg_transform;

// `pg_trigger`: https://www.postgresql.org/docs/17/catalog-pg-trigger.html
pub use reflect_gen::reflect_pg_trigger;

// `pg_ts_config`: https://www.postgresql.org/docs/17/catalog-pg-ts-config.html
pub use reflect_gen::reflect_pg_ts_config;

// `pg_ts_config_map`: https://www.postgresql.org/docs/17/catalog-pg-ts-config-map.html
pub use reflect_gen::reflect_pg_ts_config_map;

// `pg_ts_dict`: https://www.postgresql.org/docs/17/catalog-pg-ts-dict.html
pub use reflect_gen::reflect_pg_ts_dict;

// `pg_ts_parser`: https://www.postgresql.org/docs/17/catalog-pg-ts-parser.html
pub use reflect_gen::reflect_pg_ts_parser;

// `pg_ts_template`: https://www.postgresql.org/docs/17/catalog-pg-ts-template.html
pub use reflect_gen::reflect_pg_ts_template;

// `pg_type`: https://www.postgresql.org/docs/17/catalog-pg-type.html
pub use reflect_gen::reflect_pg_type;

// `pg_user_mappings`: https://www.postgresql.org/docs/17/catalog-pg-user-mapping.html
pub use reflect_gen::reflect_pg_user_mappings;
