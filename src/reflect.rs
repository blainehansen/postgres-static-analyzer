use crate::*;
use crate::aclitem::*;
use futures::TryStreamExt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PgState {
	// pub pg_aggregate,
	// pub pg_am,
	// pub pg_amop,
	// pub pg_amproc,
	// pub pg_attrdef,
	// pub pg_attribute,
	// pub pg_roles,
	// pub pg_auth_members,
	// pub pg_cast,
	// pub pg_class,
	// pub pg_collation,
	// pub pg_constraint,
	// pub pg_conversion,
	// pub pg_database,
	// pub pg_db_role_setting,
	// pub pg_default_acl,
	// pub pg_depend,
	// pub pg_description,
	// pub pg_enum,
	// pub pg_event_trigger,
	// pub pg_extension,
	// pub pg_foreign_data_wrapper,
	// pub pg_foreign_server,
	// pub pg_foreign_table,
	// pub pg_index,
	// pub pg_inherits,
	// pub pg_init_privs,
	// pub pg_language,
	// pub pg_namespace,
	// pub pg_opclass,
	// pub pg_operator,
	// pub pg_opfamily,
	// pub pg_parameter_acl,
	// pub pg_partitioned_table,
	// pub pg_policy,
	// pub pg_proc,
	// pub pg_publication,
	// pub pg_publication_namespace,
	// pub pg_publication_rel,
	// pub pg_range,
	// pub pg_replication_origin,
	// pub pg_rewrite,
	// pub pg_seclabel,
	// pub pg_sequence,
	// pub pg_shdepend,
	// pub pg_shdescription,
	// pub pg_shseclabel,
	// pub pg_statistic_ext,
	// pub pg_subscription,
	// pub pg_transform,
	// pub pg_trigger,
	// pub pg_ts_config,
	// pub pg_ts_config_map,
	// pub pg_ts_dict,
	// pub pg_ts_parser,
	// pub pg_ts_template,
	// pub pg_type,
	// pub pg_user_mapping,
}

pub async fn reflect_pg_state(
	client: &PgClient
) -> Result<PgState, postgres::Error> {
	client.batch_execute("set search_path = '';").await?;

	let (
		// pg_aggregate,
		// pg_am,
		// pg_amop,
		// pg_amproc,
		// pg_attrdef,
		// pg_attribute,
		// pg_roles,
		// pg_auth_members,
		// pg_cast,
		// pg_class,
		// pg_collation,
		// pg_constraint,
		// pg_conversion,
		// pg_database,
		// pg_db_role_setting,
		// pg_default_acl,
		// pg_depend,
		// pg_description,
		// pg_enum,
		// pg_event_trigger,
		// pg_extension,
		// pg_foreign_data_wrapper,
		// pg_foreign_server,
		// pg_foreign_table,
		// pg_index,
		// pg_inherits,
		// pg_init_privs,
		// pg_language,
		// pg_namespace,
		// pg_opclass,
		// pg_operator,
		// pg_opfamily,
		// pg_parameter_acl,
		// pg_partitioned_table,
		// pg_policy,
		// pg_proc,
		// pg_publication,
		// pg_publication_namespace,
		// pg_publication_rel,
		// pg_range,
		// pg_replication_origin,
		// pg_rewrite,
		// pg_seclabel,
		// pg_sequence,
		// pg_shdepend,
		// pg_shdescription,
		// pg_shseclabel,
		// pg_statistic_ext,
		// pg_subscription,
		// pg_transform,
		// pg_trigger,
		// pg_ts_config,
		// pg_ts_config_map,
		// pg_ts_dict,
		// pg_ts_parser,
		// pg_ts_template,
		// pg_type,
		// pg_user_mapping,
	) = tokio::try_join!(
		// reflect_pg_aggregate(client),
		// reflect_pg_am(client),
		// reflect_pg_amop(client),
		// reflect_pg_amproc(client),
		// reflect_pg_attrdef(client),
		// reflect_pg_attribute(client),
		// reflect_pg_roles(client),
		// reflect_pg_auth_members(client),
		// reflect_pg_cast(client),
		// reflect_pg_class(client),
		// reflect_pg_collation(client),
		// reflect_pg_constraint(client),
		// reflect_pg_conversion(client),
		// reflect_pg_database(client),
		// reflect_pg_db_role_setting(client),
		// reflect_pg_default_acl(client),
		// reflect_pg_depend(client),
		// reflect_pg_description(client),
		// reflect_pg_enum(client),
		// reflect_pg_event_trigger(client),
		// reflect_pg_extension(client),
		// reflect_pg_foreign_data_wrapper(client),
		// reflect_pg_foreign_server(client),
		// reflect_pg_foreign_table(client),
		// reflect_pg_index(client),
		// reflect_pg_inherits(client),
		// reflect_pg_init_privs(client),
		// reflect_pg_language(client),
		// reflect_pg_namespace(client),
		// reflect_pg_opclass(client),
		// reflect_pg_operator(client),
		// reflect_pg_opfamily(client),
		// reflect_pg_parameter_acl(client),
		// reflect_pg_partitioned_table(client),
		// reflect_pg_policy(client),
		// reflect_pg_proc(client),
		// reflect_pg_publication(client),
		// reflect_pg_publication_namespace(client),
		// reflect_pg_publication_rel(client),
		// reflect_pg_range(client),
		// reflect_pg_replication_origin(client),
		// reflect_pg_rewrite(client),
		// reflect_pg_seclabel(client),
		// reflect_pg_sequence(client),
		// reflect_pg_shdepend(client),
		// reflect_pg_shdescription(client),
		// reflect_pg_shseclabel(client),
		// reflect_pg_statistic_ext(client),
		// reflect_pg_subscription(client),
		// reflect_pg_transform(client),
		// reflect_pg_trigger(client),
		// reflect_pg_ts_config(client),
		// reflect_pg_ts_config_map(client),
		// reflect_pg_ts_dict(client),
		// reflect_pg_ts_parser(client),
		// reflect_pg_ts_template(client),
		// reflect_pg_type(client),
		// reflect_pg_user_mapping(client),
	)?;

	Ok(PgState {
		// pg_aggregate,
		// pg_am,
		// pg_amop,
		// pg_amproc,
		// pg_attrdef,
		// pg_attribute,
		// pg_roles,
		// pg_auth_members,
		// pg_cast,
		// pg_class,
		// pg_collation,
		// pg_constraint,
		// pg_conversion,
		// pg_database,
		// pg_db_role_setting,
		// pg_default_acl,
		// pg_depend,
		// pg_description,
		// pg_enum,
		// pg_event_trigger,
		// pg_extension,
		// pg_foreign_data_wrapper,
		// pg_foreign_server,
		// pg_foreign_table,
		// pg_index,
		// pg_inherits,
		// pg_init_privs,
		// pg_language,
		// pg_namespace,
		// pg_opclass,
		// pg_operator,
		// pg_opfamily,
		// pg_parameter_acl,
		// pg_partitioned_table,
		// pg_policy,
		// pg_proc,
		// pg_publication,
		// pg_publication_namespace,
		// pg_publication_rel,
		// pg_range,
		// pg_replication_origin,
		// pg_rewrite,
		// pg_seclabel,
		// pg_sequence,
		// pg_shdepend,
		// pg_shdescription,
		// pg_shseclabel,
		// pg_statistic_ext,
		// pg_subscription,
		// pg_transform,
		// pg_trigger,
		// pg_ts_config,
		// pg_ts_config_map,
		// pg_ts_dict,
		// pg_ts_parser,
		// pg_ts_template,
		// pg_type,
		// pg_user_mapping,
	})
}


#[macro_export]
macro_rules! impl_name_hash_and_equivalent {
	($type:ty, $field:ident) => {
		impl std::hash::Hash for $type {
			fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
				self.$field.hash(state);
			}
		}

		impl hashbrown::Equivalent<$type> for str {
			fn equivalent(&self, key: &$type) -> bool {
				key.$field == *self
			}
		}

		impl hashbrown::Equivalent<$type> for Str {
			fn equivalent(&self, key: &$type) -> bool {
				key.$field == *self
			}
		}
	};
}

#[macro_export]
macro_rules! impl_qual_hash_and_equivalent {
	($type:ty) => {
		impl std::hash::Hash for $type {
			fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
				self.oid.schema_name.hash(state);
				self.oid.name.hash(state);
			}
		}

		impl hashbrown::Equivalent<$type> for (&str, &str) {
			fn equivalent(&self, key: &$type) -> bool {
				key.oid.schema_name == *self.0 && key.oid.name == *self.1
			}
		}

		impl hashbrown::Equivalent<$type> for (&Str, &Str) {
			fn equivalent(&self, key: &$type) -> bool {
				key.oid.schema_name == *self.0 && key.oid.name == *self.1
			}
		}
	};
}

// #[macro_export]
// macro_rules! impl_pg_from_str {
// 	($type:ident, $($variant:ident),+ $(,)?) => {
// 		impl $type {
// 			fn pg_from_str(s: &str) -> $type {
// 				match s {
// 					$(stringify!($variant) => $type::$variant,)+
// 					_ => panic!("Postgres returned unexpected {} variant: {}", stringify!($type), s),
// 				}
// 			}
// 		}
// 	};
// }

#[macro_export]
macro_rules! pg_char_enum {
	($name:ident { $($char:literal => $variant:ident),* $(,)? }) => {
		#[derive(Debug, PartialEq, Eq, Clone)]
		pub enum $name {
			$($variant),*
		}

		impl $name {
			fn pg_from_char(c: i8) -> $name {
				match c as u8 as char {
					$($char => $name::$variant,)*
					_ => panic!(
						"Postgres returned an unknown {} variant: {}",
						stringify!($name),
						c as u8 as char
					),
				}
			}
		}
	};
}

// `pg_aggregate`: https://www.postgresql.org/docs/17/catalog-pg-aggregate.html
// use reflect_gen::{PgAggregate, reflect_pg_aggregate};

// `pg_am`: https://www.postgresql.org/docs/17/catalog-pg-am.html
// use reflect_gen::{PgAm, reflect_pg_am};

// `pg_amop`: https://www.postgresql.org/docs/17/catalog-pg-amop.html
// use reflect_gen::{PgAmop, reflect_pg_amop};

// `pg_amproc`: https://www.postgresql.org/docs/17/catalog-pg-amproc.html
// use reflect_gen::{PgAmproc, reflect_pg_amproc};

// `pg_attrdef`: https://www.postgresql.org/docs/17/catalog-pg-attrdef.html
// use reflect_gen::{PgAttrdef, reflect_pg_attrdef};

// `pg_attribute`: https://www.postgresql.org/docs/17/catalog-pg-attribute.html
// use reflect_gen::{PgAttribute, reflect_pg_attribute};

// `pg_authid`: https://www.postgresql.org/docs/17/catalog-pg-authid.html
// `pg_roles`: https://www.postgresql.org/docs/17/view-pg-roles.html

// `pg_auth_members`: https://www.postgresql.org/docs/17/catalog-pg-auth-members.html
// use reflect_gen::{PgAuthMembers, reflect_pg_auth_members};

// `pg_cast`: https://www.postgresql.org/docs/17/catalog-pg-cast.html
// use reflect_gen::{PgCast, reflect_pg_cast};

// `pg_class`: https://www.postgresql.org/docs/17/catalog-pg-class.html
// use reflect_gen::{PgClass, reflect_pg_cast};

// `pg_collation`: https://www.postgresql.org/docs/17/catalog-pg-collation.html
// use reflect_gen::{PgCollation, reflect_pg_collation};

// `pg_constraint`: https://www.postgresql.org/docs/17/catalog-pg-constraint.html
// use reflect_gen::{PgConstraint, reflect_pg_constraint};

// `pg_conversion`: https://www.postgresql.org/docs/17/catalog-pg-conversion.html
// use reflect_gen::{PgConversion, reflect_pg_conversion};

// `pg_database`: https://www.postgresql.org/docs/17/catalog-pg-database.html
// use reflect_gen::{PgDatabase, reflect_pg_database};

// `pg_db_role_setting`: https://www.postgresql.org/docs/17/catalog-pg-db-role-setting.html
// use reflect_gen::{PgDbRoleSetting, reflect_pg_db_role_setting};

// `pg_default_acl`: https://www.postgresql.org/docs/17/catalog-pg-default-acl.html
// use reflect_gen::{PgDefaultAcl, reflect_pg_default_acl};

// `pg_depend`: https://www.postgresql.org/docs/17/catalog-pg-depend.html
// use reflect_gen::{PgDepend, reflect_pg_depend};

// `pg_description`: https://www.postgresql.org/docs/17/catalog-pg-description.html
// use reflect_gen::{PgDescription, reflect_pg_description};

// `pg_char_enum`: https://www.postgresql.org/docs/17/catalog-pg-enum.html
// use reflect_gen::{PgCharEnum, reflect_pg_char_enum};

// `pg_event_trigger`: https://www.postgresql.org/docs/17/catalog-pg-event-trigger.html
// use reflect_gen::{PgEventTrigger, reflect_pg_event_trigger};

// `pg_extension`: https://www.postgresql.org/docs/17/catalog-pg-extension.html
// use reflect_gen::{PgExtension, reflect_pg_extension};

// `pg_foreign_data_wrapper`: https://www.postgresql.org/docs/17/catalog-pg-foreign-data-wrapper.html
// use reflect_gen::{PgForeignDataWrapper, reflect_pg_foreign_data_wrapper};

// `pg_foreign_server`: https://www.postgresql.org/docs/17/catalog-pg-foreign-server.html
// use reflect_gen::{PgForeignServer, reflect_pg_foreign_server};

// `pg_foreign_table`: https://www.postgresql.org/docs/17/catalog-pg-foreign-table.html
// use reflect_gen::{PgForeignTable, reflect_pg_foreign_table};

// `pg_index`: https://www.postgresql.org/docs/17/catalog-pg-index.html
// use reflect_gen::{PgIndex, reflect_pg_index};

// `pg_inherits`: https://www.postgresql.org/docs/17/catalog-pg-inherits.html
// use reflect_gen::{PgInherits, reflect_pg_inherits};

// `pg_init_privs`: https://www.postgresql.org/docs/17/catalog-pg-init-privs.html
// use reflect_gen::{PgInitPrivs, reflect_pg_init_privs};

// `pg_language`: https://www.postgresql.org/docs/17/catalog-pg-language.html
// use reflect_gen::{PgLanguage, reflect_pg_language};

// `pg_namespace`: https://www.postgresql.org/docs/17/catalog-pg-namespace.html
// use reflect_gen::{PgNamespace, reflect_pg_namespace};

// `pg_opclass`: https://www.postgresql.org/docs/17/catalog-pg-opclass.html
// use reflect_gen::{PgOpclass, reflect_pg_opclass};

// `pg_operator`: https://www.postgresql.org/docs/17/catalog-pg-operator.html
// use reflect_gen::{PgOperator, reflect_pg_operator};

// `pg_opfamily`: https://www.postgresql.org/docs/17/catalog-pg-opfamily.html
// use reflect_gen::{PgOpfamily, reflect_pg_opfamily};

// `pg_parameter_acl`: https://www.postgresql.org/docs/17/catalog-pg-parameter-acl.html
// use reflect_gen::{PgParameterAcl, reflect_pg_parameter_acl};

// `pg_partitioned_table`: https://www.postgresql.org/docs/17/catalog-pg-partitioned-table.html
// use reflect_gen::{PgPartitionedTable, reflect_pg_partitioned_table};

// `pg_policy`: https://www.postgresql.org/docs/17/catalog-pg-policy.html
// use reflect_gen::{PgPolicy, reflect_pg_policy};

// `pg_proc`: https://www.postgresql.org/docs/17/catalog-pg-proc.html
// use reflect_gen::{PgProc, reflect_pg_proc};

// `pg_publication`: https://www.postgresql.org/docs/17/catalog-pg-publication.html
// use reflect_gen::{PgPublication, reflect_pg_publication};

// `pg_publication_namespace`: https://www.postgresql.org/docs/17/catalog-pg-publication-namespace.html
// use reflect_gen::{PgPublicationNamespace, reflect_pg_publication_namespace};

// `pg_publication_rel`: https://www.postgresql.org/docs/17/catalog-pg-publication-rel.html
// use reflect_gen::{PgPublicationRel, reflect_pg_publication_rel};

// `pg_range`: https://www.postgresql.org/docs/17/catalog-pg-range.html
// use reflect_gen::{PgRange, reflect_pg_range};

// `pg_replication_origin`: https://www.postgresql.org/docs/17/catalog-pg-replication-origin.html
// use reflect_gen::{PgReplicationOrigin, reflect_pg_replication_origin};

// `pg_rewrite`: https://www.postgresql.org/docs/17/catalog-pg-rewrite.html
// use reflect_gen::{PgRewrite, reflect_pg_rewrite};

// `pg_seclabel`: https://www.postgresql.org/docs/17/catalog-pg-seclabel.html
// use reflect_gen::{PgSeclabel, reflect_pg_seclabel};

// `pg_sequence`: https://www.postgresql.org/docs/17/catalog-pg-sequence.html
// use reflect_gen::{PgSequence, reflect_pg_sequence};

// `pg_shdepend`: https://www.postgresql.org/docs/17/catalog-pg-shdepend.html
// use reflect_gen::{PgShdepend, reflect_pg_shdepend};

// `pg_shdescription`: https://www.postgresql.org/docs/17/catalog-pg-shdescription.html
// use reflect_gen::{PgShdescription, reflect_pg_shdescription};

// `pg_shseclabel`: https://www.postgresql.org/docs/17/catalog-pg-shseclabel.html
// use reflect_gen::{PgShseclabel, reflect_pg_shseclabel};

// `pg_statistic_ext`: https://www.postgresql.org/docs/17/catalog-pg-statistic-ext.html
// use reflect_gen::{PgStatisticExt, reflect_pg_statistic_ext};

// `pg_subscription`: https://www.postgresql.org/docs/17/catalog-pg-subscription.html
// use reflect_gen::{PgSubscription, reflect_pg_subscription};

// `pg_transform`: https://www.postgresql.org/docs/17/catalog-pg-transform.html
// use reflect_gen::{PgTransform, reflect_pg_transform};

// `pg_trigger`: https://www.postgresql.org/docs/17/catalog-pg-trigger.html
// use reflect_gen::{PgTrigger, reflect_pg_trigger};

// `pg_ts_config`: https://www.postgresql.org/docs/17/catalog-pg-ts-config.html
// use reflect_gen::{PgTsConfig, reflect_pg_ts_config};

// `pg_ts_config_map`: https://www.postgresql.org/docs/17/catalog-pg-ts-config-map.html
// use reflect_gen::{PgTsConfigMap, reflect_pg_ts_config_map};

// `pg_ts_dict`: https://www.postgresql.org/docs/17/catalog-pg-ts-dict.html
// use reflect_gen::{PgTsDict, reflect_pg_ts_dict};

// `pg_ts_parser`: https://www.postgresql.org/docs/17/catalog-pg-ts-parser.html
// use reflect_gen::{PgTsParser, reflect_pg_ts_parser};

// `pg_ts_template`: https://www.postgresql.org/docs/17/catalog-pg-ts-template.html
// use reflect_gen::{PgTsTemplate, reflect_pg_ts_template};

// `pg_type`: https://www.postgresql.org/docs/17/catalog-pg-type.html
// use reflect_gen::{PgType, reflect_pg_type};

// `pg_user_mapping`: https://www.postgresql.org/docs/17/catalog-pg-user-mapping.html
// use reflect_gen::{PgUserMapping, reflect_pg_user_mapping};
