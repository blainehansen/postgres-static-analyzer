use super::*;
use futures::TryStreamExt;

/// Asynchronously pull the contents of [`pg_aggregate`](https://www.postgresql.org/docs/17/catalog-pg-aggregate.html)
pub async fn reflect_pg_aggregate(
	client: &PgClient
) -> Result<Vec<PgAggregate>, postgres::Error> {
	let pg_aggregate_coll = queries_crate::queries::query_gen::reflect_pg_aggregate().bind(client)
		.map(|pg_aggregate| {
			PgAggregate {
				aggfnoid: pg_aggregate.aggfnoid.into(),
				aggkind: PgAggregateAggkind::pg_from_char(pg_aggregate.aggkind),
				aggnumdirectargs: pg_aggregate.aggnumdirectargs.unsigned_abs(),
				aggtransfn: pg_aggregate.aggtransfn.into(),
				aggfinalfn: pg_aggregate.aggfinalfn.map(Into::into),
				aggcombinefn: pg_aggregate.aggcombinefn.map(Into::into),
				aggserialfn: pg_aggregate.aggserialfn.map(Into::into),
				aggdeserialfn: pg_aggregate.aggdeserialfn.map(Into::into),
				aggmtransfn: pg_aggregate.aggmtransfn.map(Into::into),
				aggminvtransfn: pg_aggregate.aggminvtransfn.map(Into::into),
				aggmfinalfn: pg_aggregate.aggmfinalfn.map(Into::into),
				aggfinalextra: pg_aggregate.aggfinalextra,
				aggmfinalextra: pg_aggregate.aggmfinalextra,
				aggfinalmodify: PgAggregateAggfinalmodify::pg_from_char(pg_aggregate.aggfinalmodify),
				aggmfinalmodify: PgAggregateAggmfinalmodify::pg_from_char(pg_aggregate.aggmfinalmodify),
				aggsortop: pg_aggregate.aggsortop.map(Into::into),
				aggtranstype: pg_aggregate.aggtranstype.into(),
				aggmtranstype: pg_aggregate.aggmtranstype.map(Into::into),
				agginitval: pg_aggregate.agginitval.map(Into::into),
				aggminitval: pg_aggregate.aggminitval.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_aggregate_coll)
}


/// Asynchronously pull the contents of [`pg_am`](https://www.postgresql.org/docs/17/catalog-pg-am.html)
pub async fn reflect_pg_am(
	client: &PgClient
) -> Result<Set<PgAm>, postgres::Error> {
	let pg_am_coll = queries_crate::queries::query_gen::reflect_pg_am().bind(client)
		.map(|pg_am| {
			PgAm {
				amname: pg_am.amname.into(),
				amhandler: pg_am.amhandler.into(),
				amtype: PgAmAmtype::pg_from_char(pg_am.amtype),
				description: pg_am.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_am_coll)
}


/// Asynchronously pull the contents of [`pg_amop`](https://www.postgresql.org/docs/17/catalog-pg-amop.html)
pub async fn reflect_pg_amop(
	client: &PgClient
) -> Result<Vec<PgAmop>, postgres::Error> {
	let pg_amop_coll = queries_crate::queries::query_gen::reflect_pg_amop().bind(client)
		.map(|pg_amop| {
			PgAmop {
				amopfamily: pg_amop.amopfamily.into(),
				amoplefttype: pg_amop.amoplefttype.into(),
				amoprighttype: pg_amop.amoprighttype.into(),
				amopstrategy: pg_amop.amopstrategy.unsigned_abs(),
				amoppurpose: PgAmopAmoppurpose::pg_from_char(pg_amop.amoppurpose),
				amopopr: pg_amop.amopopr.into(),
				amopmethod: pg_amop.amopmethod.into(),
				amopsortfamily: pg_amop.amopsortfamily.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_amop_coll)
}


/// Asynchronously pull the contents of [`pg_amproc`](https://www.postgresql.org/docs/17/catalog-pg-amproc.html)
pub async fn reflect_pg_amproc(
	client: &PgClient
) -> Result<Vec<PgAmproc>, postgres::Error> {
	let pg_amproc_coll = queries_crate::queries::query_gen::reflect_pg_amproc().bind(client)
		.map(|pg_amproc| {
			PgAmproc {
				amprocfamily: pg_amproc.amprocfamily.into(),
				amproclefttype: pg_amproc.amproclefttype.into(),
				amprocrighttype: pg_amproc.amprocrighttype.into(),
				amprocnum: pg_amproc.amprocnum.unsigned_abs(),
				amproc: pg_amproc.amproc.into(),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_amproc_coll)
}


/// Asynchronously pull the contents of [`pg_attrdef`](https://www.postgresql.org/docs/17/catalog-pg-attrdef.html)
pub async fn reflect_pg_attrdef(
	client: &PgClient
) -> Result<Vec<PgAttrdef>, postgres::Error> {
	let pg_attrdef_coll = queries_crate::queries::query_gen::reflect_pg_attrdef().bind(client)
		.map(|pg_attrdef| {
			PgAttrdef {
				adrelid: pg_attrdef.adrelid.into(),
				adnum: pg_attrdef.adnum.unsigned_abs(),
				adbin: pg_attrdef.adbin.into(),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_attrdef_coll)
}


/// Asynchronously pull the contents of [`pg_attribute`](https://www.postgresql.org/docs/17/catalog-pg-attribute.html)
pub async fn reflect_pg_attribute(
	client: &PgClient
) -> Result<Vec<PgAttribute>, postgres::Error> {
	let pg_attribute_coll = queries_crate::queries::query_gen::reflect_pg_attribute().bind(client)
		.map(|pg_attribute| {
			PgAttribute {
				attrelid: pg_attribute.attrelid.into(),
				attname: pg_attribute.attname.into(),
				atttypid: pg_attribute.atttypid.into(),
				attnum: pg_attribute.attnum.unsigned_abs(),
				atttypmod: pg_attribute.atttypmod.map(i32::unsigned_abs),
				attndims: pg_attribute.attndims.unsigned_abs(),
				attcompression: pg_attribute.attcompression.map(PgAttributeAttcompression::pg_from_char),
				attnotnull: pg_attribute.attnotnull,
				atthasdef: pg_attribute.atthasdef,
				attidentity: pg_attribute.attidentity.map(PgAttributeAttidentity::pg_from_char),
				attgenerated: pg_attribute.attgenerated.map(PgAttributeAttgenerated::pg_from_char),
				attisdropped: pg_attribute.attisdropped,
				attislocal: pg_attribute.attislocal,
				attinhcount: pg_attribute.attinhcount.unsigned_abs(),
				attcollation: pg_attribute.attcollation.map(Into::into),
				attstattarget: pg_attribute.attstattarget.map(i16::unsigned_abs),
				attacl: pg_attribute.attacl.map(|attacl| attacl.map(|acl| aclitems!(acl, TableColumnAclItem, TableColumnGrant)).collect()),
				attoptions: pg_attribute.attoptions.map(|items| items.map(Into::into).collect()),
				attfdwoptions: pg_attribute.attfdwoptions.map(|items| items.map(Into::into).collect()),
				description: pg_attribute.description.map(Into::into),
				seclabel: pg_attribute.seclabel.map(Into::into),
				seclabel_provider: pg_attribute.seclabel_provider.map(Into::into),
				initprivs: pg_attribute.initprivs.map(|initprivs| initprivs.map(|acl| aclitems!(acl, TableColumnAclItem, TableColumnGrant)).collect()),
				initprivs_type: pg_attribute.initprivs_type.map(PgAttributeInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_attribute_coll)
}


/// Asynchronously pull the contents of [`pg_roles`](https://www.postgresql.org/docs/17/view-pg-roles.html)
pub async fn reflect_pg_roles(
	client: &PgClient
) -> Result<Set<PgRoles>, postgres::Error> {
	let pg_roles_coll = queries_crate::queries::query_gen::reflect_pg_roles().bind(client)
		.map(|pg_roles| {
			PgRoles {
				rolname: pg_roles.rolname.into(),
				rolsuper: pg_roles.rolsuper,
				rolinherit: pg_roles.rolinherit,
				rolcreaterole: pg_roles.rolcreaterole,
				rolcreatedb: pg_roles.rolcreatedb,
				rolcanlogin: pg_roles.rolcanlogin,
				rolreplication: pg_roles.rolreplication,
				rolconnlimit: pg_roles.rolconnlimit.map(i32::unsigned_abs),
				rolvaliduntil: pg_roles.rolvaliduntil,
				rolbypassrls: pg_roles.rolbypassrls,
				rolconfig: pg_roles.rolconfig.map(|items| items.map(Into::into).collect()),
				description: pg_roles.description.map(Into::into),
				seclabel: pg_roles.seclabel.map(Into::into),
				seclabel_provider: pg_roles.seclabel_provider.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_roles_coll)
}


/// Asynchronously pull the contents of [`pg_auth_members`](https://www.postgresql.org/docs/17/catalog-pg-auth-members.html)
pub async fn reflect_pg_auth_members(
	client: &PgClient
) -> Result<Vec<PgAuthMembers>, postgres::Error> {
	let pg_auth_members_coll = queries_crate::queries::query_gen::reflect_pg_auth_members().bind(client)
		.map(|pg_auth_members| {
			PgAuthMembers {
				roleid: pg_auth_members.roleid.into(),
				member: pg_auth_members.member.into(),
				grantor: pg_auth_members.grantor.into(),
				admin_option: pg_auth_members.admin_option,
				inherit_option: pg_auth_members.inherit_option,
				set_option: pg_auth_members.set_option,
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_auth_members_coll)
}


/// Asynchronously pull the contents of [`pg_cast`](https://www.postgresql.org/docs/17/catalog-pg-cast.html)
pub async fn reflect_pg_cast(
	client: &PgClient
) -> Result<Vec<PgCast>, postgres::Error> {
	let pg_cast_coll = queries_crate::queries::query_gen::reflect_pg_cast().bind(client)
		.map(|pg_cast| {
			PgCast {
				castsource: pg_cast.castsource.into(),
				casttarget: pg_cast.casttarget.into(),
				castfunc: pg_cast.castfunc.map(Into::into),
				castcontext: PgCastCastcontext::pg_from_char(pg_cast.castcontext),
				castmethod: PgCastCastmethod::pg_from_char(pg_cast.castmethod),
				description: pg_cast.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_cast_coll)
}


/// Asynchronously pull the contents of [`pg_class`](https://www.postgresql.org/docs/17/catalog-pg-class.html)
pub async fn reflect_pg_class(
	client: &PgClient
) -> Result<Set<PgClass>, postgres::Error> {
	let pg_class_coll = queries_crate::queries::query_gen::reflect_pg_class().bind(client)
		.map(|pg_class| {
			PgClass {
				oid: pg_class.oid.into(),
				relname: pg_class.relname.into(),
				relnamespace: pg_class.relnamespace.into(),
				reltype: pg_class.reltype.map(Into::into),
				reloftype: pg_class.reloftype.map(Into::into),
				relowner: pg_class.relowner.into(),
				relam: pg_class.relam.map(Into::into),
				relisshared: pg_class.relisshared,
				relpersistence: PgClassRelpersistence::pg_from_char(pg_class.relpersistence),
				relkind: PgClassRelkind::pg_from_char(pg_class.relkind),
				relnatts: pg_class.relnatts.unsigned_abs(),
				relchecks: pg_class.relchecks.unsigned_abs(),
				relrowsecurity: pg_class.relrowsecurity,
				relforcerowsecurity: pg_class.relforcerowsecurity,
				relreplident: PgClassRelreplident::pg_from_char(pg_class.relreplident),
				relispartition: pg_class.relispartition,
				relacl: pg_class.relacl.map(|relacl| relacl.map(|acl| aclitems!(acl, TableAclItem, TableGrant)).collect()),
				reloptions: pg_class.reloptions.map(|items| items.map(Into::into).collect()),
				relpartbound: pg_class.relpartbound.map(Into::into),
				description: pg_class.description.map(Into::into),
				seclabel: pg_class.seclabel.map(Into::into),
				seclabel_provider: pg_class.seclabel_provider.map(Into::into),
				initprivs: pg_class.initprivs.map(|initprivs| initprivs.map(|acl| aclitems!(acl, TableAclItem, TableGrant)).collect()),
				initprivs_type: pg_class.initprivs_type.map(PgClassInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_class_coll)
}


/// Asynchronously pull the contents of [`pg_collation`](https://www.postgresql.org/docs/17/catalog-pg-collation.html)
pub async fn reflect_pg_collation(
	client: &PgClient
) -> Result<Vec<PgCollation>, postgres::Error> {
	let pg_collation_coll = queries_crate::queries::query_gen::reflect_pg_collation().bind(client)
		.map(|pg_collation| {
			PgCollation {
				oid: pg_collation.oid.into(),
				collname: pg_collation.collname.into(),
				collnamespace: pg_collation.collnamespace.into(),
				collowner: pg_collation.collowner.into(),
				collprovider: PgCollationCollprovider::pg_from_char(pg_collation.collprovider),
				collisdeterministic: pg_collation.collisdeterministic,
				collencoding: pg_collation.collencoding.map(Into::into),
				collcollate: pg_collation.collcollate.map(Into::into),
				collctype: pg_collation.collctype.map(Into::into),
				colllocale: pg_collation.colllocale.map(Into::into),
				collicurules: pg_collation.collicurules.map(Into::into),
				collversion: pg_collation.collversion.map(Into::into),
				description: pg_collation.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_collation_coll)
}


/// Asynchronously pull the contents of [`pg_constraint`](https://www.postgresql.org/docs/17/catalog-pg-constraint.html)
pub async fn reflect_pg_constraint(
	client: &PgClient
) -> Result<Vec<PgConstraint>, postgres::Error> {
	let pg_constraint_coll = queries_crate::queries::query_gen::reflect_pg_constraint().bind(client)
		.map(|pg_constraint| {
			PgConstraint {
				conname: pg_constraint.conname.into(),
				connamespace: pg_constraint.connamespace.into(),
				contype: PgConstraintContype::pg_from_char(pg_constraint.contype),
				condeferrable: pg_constraint.condeferrable,
				condeferred: pg_constraint.condeferred,
				convalidated: pg_constraint.convalidated,
				conrelid: pg_constraint.conrelid.map(Into::into),
				contypid: pg_constraint.contypid.map(Into::into),
				conindid: pg_constraint.conindid.map(Into::into),
				conparentid: pg_constraint.conparentid.map(Into::into),
				confrelid: pg_constraint.confrelid.map(Into::into),
				confupdtype: pg_constraint.confupdtype.map(PgConstraintConfupdtype::pg_from_char),
				confdeltype: pg_constraint.confdeltype.map(PgConstraintConfdeltype::pg_from_char),
				confmatchtype: pg_constraint.confmatchtype.map(PgConstraintConfmatchtype::pg_from_char),
				conislocal: pg_constraint.conislocal,
				coninhcount: pg_constraint.coninhcount.unsigned_abs(),
				connoinherit: pg_constraint.connoinherit,
				conkey: pg_constraint.conkey.map(|items| items.map(i16::unsigned_abs).collect()),
				confkey: pg_constraint.confkey.map(|items| items.map(i16::unsigned_abs).collect()),
				conpfeqop: pg_constraint.conpfeqop.map(|items| items.map(Str::new).collect()),
				conppeqop: pg_constraint.conppeqop.map(|items| items.map(Str::new).collect()),
				conffeqop: pg_constraint.conffeqop.map(|items| items.map(Str::new).collect()),
				confdelsetcols: pg_constraint.confdelsetcols.map(|items| items.map(i16::unsigned_abs).collect()),
				conexclop: pg_constraint.conexclop.map(|items| items.map(Str::new).collect()),
				conbin: pg_constraint.conbin.map(Into::into),
				description: pg_constraint.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_constraint_coll)
}


/// Asynchronously pull the contents of [`pg_conversion`](https://www.postgresql.org/docs/17/catalog-pg-conversion.html)
pub async fn reflect_pg_conversion(
	client: &PgClient
) -> Result<Vec<PgConversion>, postgres::Error> {
	let pg_conversion_coll = queries_crate::queries::query_gen::reflect_pg_conversion().bind(client)
		.map(|pg_conversion| {
			PgConversion {
				conname: pg_conversion.conname.into(),
				connamespace: pg_conversion.connamespace.into(),
				conowner: pg_conversion.conowner.into(),
				conforencoding: pg_conversion.conforencoding.into(),
				contoencoding: pg_conversion.contoencoding.into(),
				conproc: pg_conversion.conproc.into(),
				condefault: pg_conversion.condefault,
				description: pg_conversion.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_conversion_coll)
}


/// Asynchronously pull the contents of [`pg_database`](https://www.postgresql.org/docs/17/catalog-pg-database.html)
pub async fn reflect_pg_database(
	client: &PgClient
) -> Result<PgDatabase, postgres::Error> {
	let pg_database_coll = queries_crate::queries::query_gen::reflect_pg_database().bind(client)
		.map(|pg_database| {
			PgDatabase {
				datname: pg_database.datname.into(),
				datdba: pg_database.datdba.into(),
				encoding: pg_database.encoding.into(),
				datlocprovider: PgDatabaseDatlocprovider::pg_from_char(pg_database.datlocprovider),
				datistemplate: pg_database.datistemplate,
				datallowconn: pg_database.datallowconn,
				datconnlimit: pg_database.datconnlimit.map(i32::unsigned_abs),
				datcollate: pg_database.datcollate.map(Into::into),
				datctype: pg_database.datctype.map(Into::into),
				datlocale: pg_database.datlocale.map(Into::into),
				daticurules: pg_database.daticurules.map(Into::into),
				datcollversion: pg_database.datcollversion.map(Into::into),
				datacl: pg_database.datacl.map(|datacl| datacl.map(|acl| aclitems!(acl, DbAclItem, DbGrant)).collect()),
				description: pg_database.description.map(Into::into),
				seclabel: pg_database.seclabel.map(Into::into),
				seclabel_provider: pg_database.seclabel_provider.map(Into::into),
			}
		})
		.one()
		.await?;

	Ok(pg_database_coll)
}


/// Asynchronously pull the contents of [`pg_default_acl`](https://www.postgresql.org/docs/17/catalog-pg-default-acl.html)
pub async fn reflect_pg_default_acl(
	client: &PgClient
) -> Result<Vec<PgDefaultAcl>, postgres::Error> {
	let pg_default_acl_coll = queries_crate::queries::query_gen::reflect_pg_default_acl().bind(client)
		.map(|pg_default_acl| {
			PgDefaultAcl {
				defaclrole: pg_default_acl.defaclrole.into(),
				defaclnamespace: pg_default_acl.defaclnamespace.map(Into::into),
				defaclobjtype: PgDefaultAclDefaclobjtype::pg_from_char(pg_default_acl.defaclobjtype),
				defaclacl: pg_default_acl.defaclacl.map(|defaclacl| defaclacl.map(|acl| aclitems!(acl, AclDefaultAclItem, AclDefaultGrant)).collect()),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_default_acl_coll)
}


/// Asynchronously pull the contents of [`pg_event_trigger`](https://www.postgresql.org/docs/17/catalog-pg-event-trigger.html)
pub async fn reflect_pg_event_trigger(
	client: &PgClient
) -> Result<Vec<PgEventTrigger>, postgres::Error> {
	let pg_event_trigger_coll = queries_crate::queries::query_gen::reflect_pg_event_trigger().bind(client)
		.map(|pg_event_trigger| {
			PgEventTrigger {
				evtname: pg_event_trigger.evtname.into(),
				evtevent: pg_event_trigger.evtevent.into(),
				evtowner: pg_event_trigger.evtowner.into(),
				evtfoid: pg_event_trigger.evtfoid.into(),
				evtenabled: PgEventTriggerEvtenabled::pg_from_char(pg_event_trigger.evtenabled),
				evttags: pg_event_trigger.evttags.map(|items| items.map(Into::into).collect()),
				description: pg_event_trigger.description.map(Into::into),
				seclabel: pg_event_trigger.seclabel.map(Into::into),
				seclabel_provider: pg_event_trigger.seclabel_provider.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_event_trigger_coll)
}


/// Asynchronously pull the contents of [`pg_extension`](https://www.postgresql.org/docs/17/catalog-pg-extension.html)
pub async fn reflect_pg_extension(
	client: &PgClient
) -> Result<Vec<PgExtension>, postgres::Error> {
	let pg_extension_coll = queries_crate::queries::query_gen::reflect_pg_extension().bind(client)
		.map(|pg_extension| {
			PgExtension {
				extname: pg_extension.extname.into(),
				extowner: pg_extension.extowner.into(),
				extnamespace: pg_extension.extnamespace.into(),
				extrelocatable: pg_extension.extrelocatable,
				extversion: pg_extension.extversion.into(),
				extconfig: pg_extension.extconfig.map(|items| items.map(Str::new).collect()),
				extcondition: pg_extension.extcondition.map(|items| items.map(Into::into).collect()),
				description: pg_extension.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_extension_coll)
}


/// Asynchronously pull the contents of [`pg_foreign_data_wrapper`](https://www.postgresql.org/docs/17/catalog-pg-foreign-data-wrapper.html)
pub async fn reflect_pg_foreign_data_wrapper(
	client: &PgClient
) -> Result<Vec<PgForeignDataWrapper>, postgres::Error> {
	let pg_foreign_data_wrapper_coll = queries_crate::queries::query_gen::reflect_pg_foreign_data_wrapper().bind(client)
		.map(|pg_foreign_data_wrapper| {
			PgForeignDataWrapper {
				fdwname: pg_foreign_data_wrapper.fdwname.into(),
				fdwowner: pg_foreign_data_wrapper.fdwowner.into(),
				fdwhandler: pg_foreign_data_wrapper.fdwhandler.map(Into::into),
				fdwvalidator: pg_foreign_data_wrapper.fdwvalidator.map(Into::into),
				fdwacl: pg_foreign_data_wrapper.fdwacl.map(|fdwacl| fdwacl.map(|acl| aclitems!(acl, ForeignDataWrapperAclItem, ForeignDataWrapperGrant)).collect()),
				fdwoptions: pg_foreign_data_wrapper.fdwoptions.map(|items| items.map(Into::into).collect()),
				description: pg_foreign_data_wrapper.description.map(Into::into),
				initprivs: pg_foreign_data_wrapper.initprivs.map(|initprivs| initprivs.map(|acl| aclitems!(acl, ForeignDataWrapperAclItem, ForeignDataWrapperGrant)).collect()),
				initprivs_type: pg_foreign_data_wrapper.initprivs_type.map(PgForeignDataWrapperInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_foreign_data_wrapper_coll)
}


/// Asynchronously pull the contents of [`pg_foreign_server`](https://www.postgresql.org/docs/17/catalog-pg-foreign-server.html)
pub async fn reflect_pg_foreign_server(
	client: &PgClient
) -> Result<Vec<PgForeignServer>, postgres::Error> {
	let pg_foreign_server_coll = queries_crate::queries::query_gen::reflect_pg_foreign_server().bind(client)
		.map(|pg_foreign_server| {
			PgForeignServer {
				srvname: pg_foreign_server.srvname.into(),
				srvowner: pg_foreign_server.srvowner.into(),
				srvfdw: pg_foreign_server.srvfdw.into(),
				srvtype: pg_foreign_server.srvtype.map(Into::into),
				srvversion: pg_foreign_server.srvversion.map(Into::into),
				srvacl: pg_foreign_server.srvacl.map(|srvacl| srvacl.map(|acl| aclitems!(acl, ForeignServerAclItem, ForeignServerGrant)).collect()),
				srvoptions: pg_foreign_server.srvoptions.map(|items| items.map(Into::into).collect()),
				description: pg_foreign_server.description.map(Into::into),
				initprivs: pg_foreign_server.initprivs.map(|initprivs| initprivs.map(|acl| aclitems!(acl, ForeignServerAclItem, ForeignServerGrant)).collect()),
				initprivs_type: pg_foreign_server.initprivs_type.map(PgForeignServerInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_foreign_server_coll)
}


/// Asynchronously pull the contents of [`pg_foreign_table`](https://www.postgresql.org/docs/17/catalog-pg-foreign-table.html)
pub async fn reflect_pg_foreign_table(
	client: &PgClient
) -> Result<Vec<PgForeignTable>, postgres::Error> {
	let pg_foreign_table_coll = queries_crate::queries::query_gen::reflect_pg_foreign_table().bind(client)
		.map(|pg_foreign_table| {
			PgForeignTable {
				ftrelid: pg_foreign_table.ftrelid.into(),
				ftserver: pg_foreign_table.ftserver.into(),
				ftoptions: pg_foreign_table.ftoptions.map(|items| items.map(Into::into).collect()),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_foreign_table_coll)
}


/// Asynchronously pull the contents of [`pg_index`](https://www.postgresql.org/docs/17/catalog-pg-index.html)
pub async fn reflect_pg_index(
	client: &PgClient
) -> Result<Vec<PgIndex>, postgres::Error> {
	let pg_index_coll = queries_crate::queries::query_gen::reflect_pg_index().bind(client)
		.map(|pg_index| {
			PgIndex {
				indexrelid: pg_index.indexrelid.into(),
				indrelid: pg_index.indrelid.into(),
				indnatts: pg_index.indnatts.unsigned_abs(),
				indnkeyatts: pg_index.indnkeyatts.unsigned_abs(),
				indisunique: pg_index.indisunique,
				indnullsnotdistinct: pg_index.indnullsnotdistinct,
				indisprimary: pg_index.indisprimary,
				indisexclusion: pg_index.indisexclusion,
				indimmediate: pg_index.indimmediate,
				indisclustered: pg_index.indisclustered,
				indisreplident: pg_index.indisreplident,
				indkey: pg_index.indkey.map(i16::unsigned_abs).collect(),
				indcollation: pg_index.indcollation.map(maybe_str).collect(),
				indclass: pg_index.indclass.map(Str::new).collect(),
				indoption: pg_index.indoption.collect(),
				indexprs: pg_index.indexprs.map(Into::into),
				indpred: pg_index.indpred.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_index_coll)
}


/// Asynchronously pull the contents of [`pg_inherits`](https://www.postgresql.org/docs/17/catalog-pg-inherits.html)
pub async fn reflect_pg_inherits(
	client: &PgClient
) -> Result<Vec<PgInherits>, postgres::Error> {
	let pg_inherits_coll = queries_crate::queries::query_gen::reflect_pg_inherits().bind(client)
		.map(|pg_inherits| {
			PgInherits {
				inhrelid: pg_inherits.inhrelid.into(),
				inhparent: pg_inherits.inhparent.into(),
				inhseqno: pg_inherits.inhseqno.unsigned_abs(),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_inherits_coll)
}


/// Asynchronously pull the contents of [`pg_language`](https://www.postgresql.org/docs/17/catalog-pg-language.html)
pub async fn reflect_pg_language(
	client: &PgClient
) -> Result<Set<PgLanguage>, postgres::Error> {
	let pg_language_coll = queries_crate::queries::query_gen::reflect_pg_language().bind(client)
		.map(|pg_language| {
			PgLanguage {
				lanname: pg_language.lanname.into(),
				lanowner: pg_language.lanowner.into(),
				lanispl: pg_language.lanispl,
				lanpltrusted: pg_language.lanpltrusted,
				lanplcallfoid: pg_language.lanplcallfoid.map(Into::into),
				laninline: pg_language.laninline.map(Into::into),
				lanvalidator: pg_language.lanvalidator.map(Into::into),
				lanacl: pg_language.lanacl.map(|lanacl| lanacl.map(|acl| aclitems!(acl, LanguageAclItem, LanguageGrant)).collect()),
				description: pg_language.description.map(Into::into),
				seclabel: pg_language.seclabel.map(Into::into),
				seclabel_provider: pg_language.seclabel_provider.map(Into::into),
				initprivs: pg_language.initprivs.map(|initprivs| initprivs.map(|acl| aclitems!(acl, LanguageAclItem, LanguageGrant)).collect()),
				initprivs_type: pg_language.initprivs_type.map(PgLanguageInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_language_coll)
}


/// Asynchronously pull the contents of [`pg_namespace`](https://www.postgresql.org/docs/17/catalog-pg-namespace.html)
pub async fn reflect_pg_namespace(
	client: &PgClient
) -> Result<Set<PgNamespace>, postgres::Error> {
	let pg_namespace_coll = queries_crate::queries::query_gen::reflect_pg_namespace().bind(client)
		.map(|pg_namespace| {
			PgNamespace {
				nspname: pg_namespace.nspname.into(),
				nspowner: pg_namespace.nspowner.into(),
				nspacl: pg_namespace.nspacl.map(|nspacl| nspacl.map(|acl| aclitems!(acl, SchemaAclItem, SchemaGrant)).collect()),
				description: pg_namespace.description.map(Into::into),
				seclabel: pg_namespace.seclabel.map(Into::into),
				seclabel_provider: pg_namespace.seclabel_provider.map(Into::into),
				initprivs: pg_namespace.initprivs.map(|initprivs| initprivs.map(|acl| aclitems!(acl, SchemaAclItem, SchemaGrant)).collect()),
				initprivs_type: pg_namespace.initprivs_type.map(PgNamespaceInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_namespace_coll)
}


/// Asynchronously pull the contents of [`pg_opclass`](https://www.postgresql.org/docs/17/catalog-pg-opclass.html)
pub async fn reflect_pg_opclass(
	client: &PgClient
) -> Result<Vec<PgOpclass>, postgres::Error> {
	let pg_opclass_coll = queries_crate::queries::query_gen::reflect_pg_opclass().bind(client)
		.map(|pg_opclass| {
			PgOpclass {
				opcmethod: pg_opclass.opcmethod.into(),
				opcname: pg_opclass.opcname.into(),
				opcnamespace: pg_opclass.opcnamespace.into(),
				opcowner: pg_opclass.opcowner.into(),
				opcfamily: pg_opclass.opcfamily.into(),
				opcintype: pg_opclass.opcintype.into(),
				opcdefault: pg_opclass.opcdefault,
				opckeytype: pg_opclass.opckeytype.map(Into::into),
				description: pg_opclass.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_opclass_coll)
}


/// Asynchronously pull the contents of [`pg_operator`](https://www.postgresql.org/docs/17/catalog-pg-operator.html)
pub async fn reflect_pg_operator(
	client: &PgClient
) -> Result<Set<PgOperator>, postgres::Error> {
	let pg_operator_coll = queries_crate::queries::query_gen::reflect_pg_operator().bind(client)
		.map(|pg_operator| {
			PgOperator {
				oid: pg_operator.oid.into(),
				oprname: pg_operator.oprname.into(),
				oprnamespace: pg_operator.oprnamespace.into(),
				oprowner: pg_operator.oprowner.into(),
				oprkind: PgOperatorOprkind::pg_from_char(pg_operator.oprkind),
				oprcanmerge: pg_operator.oprcanmerge,
				oprcanhash: pg_operator.oprcanhash,
				oprleft: pg_operator.oprleft.map(Into::into),
				oprright: pg_operator.oprright.into(),
				oprresult: pg_operator.oprresult.map(Into::into),
				oprcom: pg_operator.oprcom.map(Into::into),
				oprnegate: pg_operator.oprnegate.map(Into::into),
				oprcode: pg_operator.oprcode.map(Into::into),
				oprrest: pg_operator.oprrest.map(Into::into),
				oprjoin: pg_operator.oprjoin.map(Into::into),
				description: pg_operator.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_operator_coll)
}


/// Asynchronously pull the contents of [`pg_opfamily`](https://www.postgresql.org/docs/17/catalog-pg-opfamily.html)
pub async fn reflect_pg_opfamily(
	client: &PgClient
) -> Result<Vec<PgOpfamily>, postgres::Error> {
	let pg_opfamily_coll = queries_crate::queries::query_gen::reflect_pg_opfamily().bind(client)
		.map(|pg_opfamily| {
			PgOpfamily {
				opfmethod: pg_opfamily.opfmethod.into(),
				opfname: pg_opfamily.opfname.into(),
				opfnamespace: pg_opfamily.opfnamespace.into(),
				opfowner: pg_opfamily.opfowner.into(),
				description: pg_opfamily.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_opfamily_coll)
}


/// Asynchronously pull the contents of [`pg_parameter_acl`](https://www.postgresql.org/docs/17/catalog-pg-parameter-acl.html)
pub async fn reflect_pg_parameter_acl(
	client: &PgClient
) -> Result<Vec<PgParameterAcl>, postgres::Error> {
	let pg_parameter_acl_coll = queries_crate::queries::query_gen::reflect_pg_parameter_acl().bind(client)
		.map(|pg_parameter_acl| {
			PgParameterAcl {
				parname: pg_parameter_acl.parname.into(),
				paracl: pg_parameter_acl.paracl.map(|paracl| paracl.map(|acl| aclitems!(acl, ParameterAclItem, ParameterGrant)).collect()),
				initprivs: pg_parameter_acl.initprivs.map(|initprivs| initprivs.map(|acl| aclitems!(acl, ParameterAclItem, ParameterGrant)).collect()),
				initprivs_type: pg_parameter_acl.initprivs_type.map(PgParameterAclInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_parameter_acl_coll)
}


/// Asynchronously pull the contents of [`pg_partitioned_table`](https://www.postgresql.org/docs/17/catalog-pg-partitioned-table.html)
pub async fn reflect_pg_partitioned_table(
	client: &PgClient
) -> Result<Vec<PgPartitionedTable>, postgres::Error> {
	let pg_partitioned_table_coll = queries_crate::queries::query_gen::reflect_pg_partitioned_table().bind(client)
		.map(|pg_partitioned_table| {
			PgPartitionedTable {
				partrelid: pg_partitioned_table.partrelid.into(),
				partstrat: PgPartitionedTablePartstrat::pg_from_char(pg_partitioned_table.partstrat),
				partnatts: pg_partitioned_table.partnatts.unsigned_abs(),
				partdefid: pg_partitioned_table.partdefid.map(Into::into),
				partattrs: pg_partitioned_table.partattrs.map(i16::unsigned_abs).collect(),
				partclass: pg_partitioned_table.partclass.map(Str::new).collect(),
				partcollation: pg_partitioned_table.partcollation.map(maybe_str).collect(),
				partexprs: pg_partitioned_table.partexprs.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_partitioned_table_coll)
}


/// Asynchronously pull the contents of [`pg_policy`](https://www.postgresql.org/docs/17/catalog-pg-policy.html)
pub async fn reflect_pg_policy(
	client: &PgClient
) -> Result<Vec<PgPolicy>, postgres::Error> {
	let pg_policy_coll = queries_crate::queries::query_gen::reflect_pg_policy().bind(client)
		.map(|pg_policy| {
			PgPolicy {
				polname: pg_policy.polname.into(),
				polrelid: pg_policy.polrelid.into(),
				polcmd: PgPolicyPolcmd::pg_from_char(pg_policy.polcmd),
				polpermissive: pg_policy.polpermissive,
				polroles: pg_policy.polroles.map(|item| item.map(Into::into)).collect(),
				polqual: pg_policy.polqual.map(Into::into),
				polwithcheck: pg_policy.polwithcheck.map(Into::into),
				description: pg_policy.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_policy_coll)
}


/// Asynchronously pull the contents of [`pg_publication`](https://www.postgresql.org/docs/17/catalog-pg-publication.html)
pub async fn reflect_pg_publication(
	client: &PgClient
) -> Result<Set<PgPublication>, postgres::Error> {
	let pg_publication_coll = queries_crate::queries::query_gen::reflect_pg_publication().bind(client)
		.map(|pg_publication| {
			PgPublication {
				pubname: pg_publication.pubname.into(),
				pubowner: pg_publication.pubowner.into(),
				puballtables: pg_publication.puballtables,
				pubinsert: pg_publication.pubinsert,
				pubupdate: pg_publication.pubupdate,
				pubdelete: pg_publication.pubdelete,
				pubtruncate: pg_publication.pubtruncate,
				pubviaroot: pg_publication.pubviaroot,
				description: pg_publication.description.map(Into::into),
				seclabel: pg_publication.seclabel.map(Into::into),
				seclabel_provider: pg_publication.seclabel_provider.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_publication_coll)
}


/// Asynchronously pull the contents of [`pg_publication_namespace`](https://www.postgresql.org/docs/17/catalog-pg-publication-namespace.html)
pub async fn reflect_pg_publication_namespace(
	client: &PgClient
) -> Result<Vec<PgPublicationNamespace>, postgres::Error> {
	let pg_publication_namespace_coll = queries_crate::queries::query_gen::reflect_pg_publication_namespace().bind(client)
		.map(|pg_publication_namespace| {
			PgPublicationNamespace {
				pnpubid: pg_publication_namespace.pnpubid.into(),
				pnnspid: pg_publication_namespace.pnnspid.into(),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_publication_namespace_coll)
}


/// Asynchronously pull the contents of [`pg_publication_rel`](https://www.postgresql.org/docs/17/catalog-pg-publication-rel.html)
pub async fn reflect_pg_publication_rel(
	client: &PgClient
) -> Result<Vec<PgPublicationRel>, postgres::Error> {
	let pg_publication_rel_coll = queries_crate::queries::query_gen::reflect_pg_publication_rel().bind(client)
		.map(|pg_publication_rel| {
			PgPublicationRel {
				prpubid: pg_publication_rel.prpubid.into(),
				prrelid: pg_publication_rel.prrelid.into(),
				prqual: pg_publication_rel.prqual.map(Into::into),
				prattrs: pg_publication_rel.prattrs.map(|items| items.map(i16::unsigned_abs).collect()),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_publication_rel_coll)
}


/// Asynchronously pull the contents of [`pg_range`](https://www.postgresql.org/docs/17/catalog-pg-range.html)
pub async fn reflect_pg_range(
	client: &PgClient
) -> Result<Vec<PgRange>, postgres::Error> {
	let pg_range_coll = queries_crate::queries::query_gen::reflect_pg_range().bind(client)
		.map(|pg_range| {
			PgRange {
				rngtypid: pg_range.rngtypid.into(),
				rngsubtype: pg_range.rngsubtype.into(),
				rngmultitypid: pg_range.rngmultitypid.into(),
				rngcollation: pg_range.rngcollation.map(Into::into),
				rngsubopc: pg_range.rngsubopc.into(),
				rngcanonical: pg_range.rngcanonical.map(Into::into),
				rngsubdiff: pg_range.rngsubdiff.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_range_coll)
}


/// Asynchronously pull the contents of [`pg_rules`](https://www.postgresql.org/docs/17/view-pg-rules.html)
pub async fn reflect_pg_rules(
	client: &PgClient
) -> Result<Vec<PgRules>, postgres::Error> {
	let pg_rules_coll = queries_crate::queries::query_gen::reflect_pg_rules().bind(client)
		.map(|pg_rules| {
			PgRules {
				schemaname: pg_rules.schemaname.into(),
				tablename: pg_rules.tablename.into(),
				rulename: pg_rules.rulename.into(),
				definition: pg_rules.definition.into(),
				description: pg_rules.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_rules_coll)
}


/// Asynchronously pull the contents of [`pg_views`](https://www.postgresql.org/docs/17/view-pg-views.html)
pub async fn reflect_pg_views(
	client: &PgClient
) -> Result<Vec<PgViews>, postgres::Error> {
	let pg_views_coll = queries_crate::queries::query_gen::reflect_pg_views().bind(client)
		.map(|pg_views| {
			PgViews {
				schemaname: pg_views.schemaname.into(),
				viewname: pg_views.viewname.into(),
				viewowner: pg_views.viewowner.into(),
				definition: pg_views.definition.into(),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_views_coll)
}


/// Asynchronously pull the contents of [`pg_matviews`](https://www.postgresql.org/docs/17/view-pg-matviews.html)
pub async fn reflect_pg_matviews(
	client: &PgClient
) -> Result<Vec<PgMatviews>, postgres::Error> {
	let pg_matviews_coll = queries_crate::queries::query_gen::reflect_pg_matviews().bind(client)
		.map(|pg_matviews| {
			PgMatviews {
				schemaname: pg_matviews.schemaname.into(),
				matviewname: pg_matviews.matviewname.into(),
				matviewowner: pg_matviews.matviewowner.into(),
				definition: pg_matviews.definition.into(),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_matviews_coll)
}


/// Asynchronously pull the contents of [`pg_sequence`](https://www.postgresql.org/docs/17/catalog-pg-sequence.html)
pub async fn reflect_pg_sequence(
	client: &PgClient
) -> Result<Vec<PgSequence>, postgres::Error> {
	let pg_sequence_coll = queries_crate::queries::query_gen::reflect_pg_sequence().bind(client)
		.map(|pg_sequence| {
			PgSequence {
				seqrelid: pg_sequence.seqrelid.into(),
				seqtypid: pg_sequence.seqtypid.into(),
				seqstart: pg_sequence.seqstart,
				seqincrement: pg_sequence.seqincrement,
				seqmax: pg_sequence.seqmax,
				seqmin: pg_sequence.seqmin,
				seqcache: pg_sequence.seqcache,
				seqcycle: pg_sequence.seqcycle,
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_sequence_coll)
}


/// Asynchronously pull the contents of [`pg_statistic_ext`](https://www.postgresql.org/docs/17/catalog-pg-statistic-ext.html)
pub async fn reflect_pg_statistic_ext(
	client: &PgClient
) -> Result<Vec<PgStatisticExt>, postgres::Error> {
	let pg_statistic_ext_coll = queries_crate::queries::query_gen::reflect_pg_statistic_ext().bind(client)
		.map(|pg_statistic_ext| {
			PgStatisticExt {
				stxrelid: pg_statistic_ext.stxrelid.into(),
				stxname: pg_statistic_ext.stxname.into(),
				stxnamespace: pg_statistic_ext.stxnamespace.into(),
				stxowner: pg_statistic_ext.stxowner.into(),
				stxkeys: pg_statistic_ext.stxkeys.map(i16::unsigned_abs).collect(),
				stxstattarget: pg_statistic_ext.stxstattarget.map(i16::unsigned_abs),
				stxkind: pg_statistic_ext.stxkind.map(PgStatisticExtStxkind::pg_from_char).collect(),
				stxexprs: pg_statistic_ext.stxexprs.map(Into::into),
				description: pg_statistic_ext.description.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_statistic_ext_coll)
}


/// Asynchronously pull the contents of [`pg_subscription`](https://www.postgresql.org/docs/17/catalog-pg-subscription.html)
pub async fn reflect_pg_subscription(
	client: &PgClient
) -> Result<Vec<PgSubscription>, postgres::Error> {
	let pg_subscription_coll = queries_crate::queries::query_gen::reflect_pg_subscription().bind(client)
		.map(|pg_subscription| {
			PgSubscription {
				subname: pg_subscription.subname.into(),
				subowner: pg_subscription.subowner.into(),
				subenabled: pg_subscription.subenabled,
				subbinary: pg_subscription.subbinary,
				substream: PgSubscriptionSubstream::pg_from_char(pg_subscription.substream),
				subtwophasestate: PgSubscriptionSubtwophasestate::pg_from_char(pg_subscription.subtwophasestate),
				subdisableonerr: pg_subscription.subdisableonerr,
				subpasswordrequired: pg_subscription.subpasswordrequired,
				subrunasowner: pg_subscription.subrunasowner,
				subfailover: pg_subscription.subfailover,
				subconninfo: pg_subscription.subconninfo.into(),
				subslotname: pg_subscription.subslotname.map(Into::into),
				subsynccommit: pg_subscription.subsynccommit.into(),
				subpublications: pg_subscription.subpublications.map(Into::into).collect(),
				suborigin: pg_subscription.suborigin.map(Into::into),
				description: pg_subscription.description.map(Into::into),
				seclabel: pg_subscription.seclabel.map(Into::into),
				seclabel_provider: pg_subscription.seclabel_provider.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_subscription_coll)
}


/// Asynchronously pull the contents of [`pg_transform`](https://www.postgresql.org/docs/17/catalog-pg-transform.html)
pub async fn reflect_pg_transform(
	client: &PgClient
) -> Result<Vec<PgTransform>, postgres::Error> {
	let pg_transform_coll = queries_crate::queries::query_gen::reflect_pg_transform().bind(client)
		.map(|pg_transform| {
			PgTransform {
				trftype: pg_transform.trftype.into(),
				trflang: pg_transform.trflang.into(),
				trffromsql: pg_transform.trffromsql.map(Into::into),
				trftosql: pg_transform.trftosql.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_transform_coll)
}


/// Asynchronously pull the contents of [`pg_trigger`](https://www.postgresql.org/docs/17/catalog-pg-trigger.html)
pub async fn reflect_pg_trigger(
	client: &PgClient
) -> Result<Vec<PgTrigger>, postgres::Error> {
	let pg_trigger_coll = queries_crate::queries::query_gen::reflect_pg_trigger().bind(client)
		.map(|pg_trigger| {
			PgTrigger {
				tgrelid: pg_trigger.tgrelid.into(),
				tgparentid: pg_trigger.tgparentid.map(Into::into),
				tgname: pg_trigger.tgname.into(),
				tgfoid: pg_trigger.tgfoid.into(),
				tgtype: pg_trigger.tgtype,
				tgenabled: PgTriggerTgenabled::pg_from_char(pg_trigger.tgenabled),
				tgisinternal: pg_trigger.tgisinternal,
				tgconstrrelid: pg_trigger.tgconstrrelid.map(Into::into),
				tgconstrindid: pg_trigger.tgconstrindid.map(Into::into),
				tgconstraint: pg_trigger.tgconstraint.map(Into::into),
				tgdeferrable: pg_trigger.tgdeferrable,
				tginitdeferred: pg_trigger.tginitdeferred,
				tgnargs: pg_trigger.tgnargs.unsigned_abs(),
				tgattr: pg_trigger.tgattr.map(i16::unsigned_abs).collect(),
				tgargs: pg_trigger.tgargs.into(),
				tgqual: pg_trigger.tgqual.map(Into::into),
				tgoldtable: pg_trigger.tgoldtable.map(Into::into),
				tgnewtable: pg_trigger.tgnewtable.map(Into::into),
				description: pg_trigger.description.map(Into::into),
				seclabel: pg_trigger.seclabel.map(Into::into),
				seclabel_provider: pg_trigger.seclabel_provider.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_trigger_coll)
}


/// Asynchronously pull the contents of [`pg_ts_config`](https://www.postgresql.org/docs/17/catalog-pg-ts-config.html)
pub async fn reflect_pg_ts_config(
	client: &PgClient
) -> Result<Set<PgTsConfig>, postgres::Error> {
	let pg_ts_config_coll = queries_crate::queries::query_gen::reflect_pg_ts_config().bind(client)
		.map(|pg_ts_config| {
			PgTsConfig {
				oid: pg_ts_config.oid.into(),
				cfgname: pg_ts_config.cfgname.into(),
				cfgnamespace: pg_ts_config.cfgnamespace.into(),
				cfgowner: pg_ts_config.cfgowner.into(),
				description: pg_ts_config.description.map(Into::into),
				seclabel: pg_ts_config.seclabel.map(Into::into),
				seclabel_provider: pg_ts_config.seclabel_provider.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_ts_config_coll)
}


/// Asynchronously pull the contents of [`pg_ts_config_map`](https://www.postgresql.org/docs/17/catalog-pg-ts-config-map.html)
pub async fn reflect_pg_ts_config_map(
	client: &PgClient
) -> Result<Vec<PgTsConfigMap>, postgres::Error> {
	let pg_ts_config_map_coll = queries_crate::queries::query_gen::reflect_pg_ts_config_map().bind(client)
		.map(|pg_ts_config_map| {
			PgTsConfigMap {
				mapcfg: pg_ts_config_map.mapcfg.into(),
				maptokentype: pg_ts_config_map.maptokentype,
				mapseqno: pg_ts_config_map.mapseqno,
				mapdict: pg_ts_config_map.mapdict.into(),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_ts_config_map_coll)
}


/// Asynchronously pull the contents of [`pg_ts_dict`](https://www.postgresql.org/docs/17/catalog-pg-ts-dict.html)
pub async fn reflect_pg_ts_dict(
	client: &PgClient
) -> Result<Set<PgTsDict>, postgres::Error> {
	let pg_ts_dict_coll = queries_crate::queries::query_gen::reflect_pg_ts_dict().bind(client)
		.map(|pg_ts_dict| {
			PgTsDict {
				oid: pg_ts_dict.oid.into(),
				dictname: pg_ts_dict.dictname.into(),
				dictnamespace: pg_ts_dict.dictnamespace.into(),
				dictowner: pg_ts_dict.dictowner.into(),
				dictinitoption: pg_ts_dict.dictinitoption.map(Into::into),
				description: pg_ts_dict.description.map(Into::into),
				seclabel: pg_ts_dict.seclabel.map(Into::into),
				seclabel_provider: pg_ts_dict.seclabel_provider.map(Into::into),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_ts_dict_coll)
}


/// Asynchronously pull the contents of [`pg_ts_parser`](https://www.postgresql.org/docs/17/catalog-pg-ts-parser.html)
pub async fn reflect_pg_ts_parser(
	client: &PgClient
) -> Result<Vec<PgTsParser>, postgres::Error> {
	let pg_ts_parser_coll = queries_crate::queries::query_gen::reflect_pg_ts_parser().bind(client)
		.map(|pg_ts_parser| {
			PgTsParser {
				prsname: pg_ts_parser.prsname.into(),
				prsnamespace: pg_ts_parser.prsnamespace.into(),
				prsstart: pg_ts_parser.prsstart.into(),
				prstoken: pg_ts_parser.prstoken.into(),
				prsend: pg_ts_parser.prsend.into(),
				prsheadline: pg_ts_parser.prsheadline.map(Into::into),
				prslextype: pg_ts_parser.prslextype.into(),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_ts_parser_coll)
}


/// Asynchronously pull the contents of [`pg_ts_template`](https://www.postgresql.org/docs/17/catalog-pg-ts-template.html)
pub async fn reflect_pg_ts_template(
	client: &PgClient
) -> Result<Vec<PgTsTemplate>, postgres::Error> {
	let pg_ts_template_coll = queries_crate::queries::query_gen::reflect_pg_ts_template().bind(client)
		.map(|pg_ts_template| {
			PgTsTemplate {
				tmplname: pg_ts_template.tmplname.into(),
				tmplnamespace: pg_ts_template.tmplnamespace.into(),
				tmplinit: pg_ts_template.tmplinit.map(Into::into),
				tmpllexize: pg_ts_template.tmpllexize.into(),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_ts_template_coll)
}


/// Asynchronously pull the contents of [`pg_type`](https://www.postgresql.org/docs/17/catalog-pg-type.html)
pub async fn reflect_pg_type(
	client: &PgClient
) -> Result<Set<PgType>, postgres::Error> {
	let pg_type_coll = queries_crate::queries::query_gen::reflect_pg_type().bind(client)
		.map(|pg_type| {
			PgType {
				oid: pg_type.oid.into(),
				typname: pg_type.typname.into(),
				typnamespace: pg_type.typnamespace.into(),
				typowner: pg_type.typowner.into(),
				typlen: pg_type.typlen,
				typbyval: pg_type.typbyval,
				typtype: PgTypeTyptype::pg_from_char(pg_type.typtype),
				typispreferred: pg_type.typispreferred,
				typisdefined: pg_type.typisdefined,
				typdelim: pg_type.typdelim,
				typrelid: pg_type.typrelid.map(Into::into),
				typsubscript: pg_type.typsubscript.map(Into::into),
				typelem: pg_type.typelem.map(Into::into),
				typarray: pg_type.typarray.map(Into::into),
				typinput: pg_type.typinput.into(),
				typoutput: pg_type.typoutput.into(),
				typreceive: pg_type.typreceive.map(Into::into),
				typsend: pg_type.typsend.map(Into::into),
				typmodin: pg_type.typmodin.map(Into::into),
				typmodout: pg_type.typmodout.map(Into::into),
				typanalyze: pg_type.typanalyze.map(Into::into),
				typalign: PgTypeTypalign::pg_from_char(pg_type.typalign),
				typstorage: PgTypeTypstorage::pg_from_char(pg_type.typstorage),
				typnotnull: pg_type.typnotnull,
				typbasetype: pg_type.typbasetype.map(Into::into),
				typtypmod: pg_type.typtypmod.map(i32::unsigned_abs),
				typndims: pg_type.typndims.unsigned_abs(),
				typcollation: pg_type.typcollation.map(Into::into),
				typdefaultbin: pg_type.typdefaultbin.map(Into::into),
				typdefault: pg_type.typdefault.map(Into::into),
				typacl: pg_type.typacl.map(|typacl| typacl.map(|acl| aclitems!(acl, TypeAclItem, TypeGrant)).collect()),
				description: pg_type.description.map(Into::into),
				seclabel: pg_type.seclabel.map(Into::into),
				seclabel_provider: pg_type.seclabel_provider.map(Into::into),
				initprivs: pg_type.initprivs.map(|initprivs| initprivs.map(|acl| aclitems!(acl, TypeAclItem, TypeGrant)).collect()),
				initprivs_type: pg_type.initprivs_type.map(PgTypeInitprivsType::pg_from_char),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_type_coll)
}


/// Asynchronously pull the contents of [`pg_user_mappings`](https://www.postgresql.org/docs/17/view-pg-user-mappings.html)
pub async fn reflect_pg_user_mappings(
	client: &PgClient
) -> Result<Vec<PgUserMappings>, postgres::Error> {
	let pg_user_mappings_coll = queries_crate::queries::query_gen::reflect_pg_user_mappings().bind(client)
		.map(|pg_user_mappings| {
			PgUserMappings {
				srvname: pg_user_mappings.srvname.into(),
				umuser: pg_user_mappings.umuser.map(Into::into),
				usename: pg_user_mappings.usename.into(),
				umoptions: pg_user_mappings.umoptions.map(|items| items.map(Into::into).collect()),
			}
		})
		.iter().await?.try_collect()
		.await?;

	Ok(pg_user_mappings_coll)
}

