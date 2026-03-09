use crate::{ClassKind, ClassPersistence, PgClass, PgClient, PgRoles, PgState, Ref, Set, aclitem::{TableGrantParser, aclitem}, postgres};
use futures::TryStreamExt;

fn make_ref(schema_name: &str, name: &str) -> Ref {
	Ref { schema_name: schema_name.into(), name: name.into() }
}

fn maybe_ref(schema_name: Option<&str>, name: Option<&str>) -> Option<Ref> {
	match (schema_name, name) {
		(Some(schema_name), Some(name)) => Some(make_ref(schema_name, name)),
		_ => None,
	}
}

pub async fn reflect_pg_state(
	client: &PgClient
) -> Result<PgState, postgres::Error> {
	let (pg_class,) = tokio::try_join!(
		reflect_pg_class(client),
	)?;

	Ok(PgState { pg_class })
}

pub async fn reflect_pg_roles(
	client: &PgClient
) -> Result<Set<PgRoles>, postgres::Error> {
	let pg_roles_set = reflect_crate::queries::main::reflect_pg_roles().bind(client)
		.map(|pg_role| {
			PgRoles {
				rolname: pg_role.rolname.into(),
				rolsuper: pg_role.rolsuper,
				rolinherit: pg_role.rolinherit,
				rolcreaterole: pg_role.rolcreaterole,
				rolcreatedb: pg_role.rolcreatedb,
				rolcanlogin: pg_role.rolcanlogin,
				rolreplication: pg_role.rolreplication,
				rolbypassrls: pg_role.rolbypassrls,
				rolconnlimit: pg_role.rolconnlimit.map(i32::unsigned_abs),
				rolvaliduntil: pg_role.rolvaliduntil.into(),
				rolconfig: pg_role.rolconfig.map(|config| config.map(Into::into).collect()),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_roles_set)
}

pub async fn reflect_pg_class(
	client: &PgClient
) -> Result<Set<PgClass>, postgres::Error> {
	let pg_class_set = reflect_crate::queries::main::reflect_pg_class().bind(client)
		.map(|pg_class| {
			PgClass {
				relname: pg_class.relname.into(),
				relnamespace: pg_class.relnamespace.into(),
				reltype: maybe_ref(pg_class.reltype_schema_name, pg_class.reltype_name),
				reloftype: maybe_ref(pg_class.reloftype_schema_name, pg_class.reloftype_name),
				relowner: pg_class.relowner.into(),
				reltablespace: pg_class.reltablespace.map(Into::into),
				relisshared: pg_class.relisshared,
				relpersistence: ClassPersistence::pg_from_char(pg_class.relpersistence),
				relkind: ClassKind::pg_from_char(pg_class.relkind),
				relrowsecurity: pg_class.relrowsecurity,
				relforcerowsecurity: pg_class.relforcerowsecurity,
				relispartition: pg_class.relispartition,
				relacl: pg_class.relacl.map(|relacl| relacl.map(|acl| aclitem(acl, &TableGrantParser)).collect()),
				reloptions: pg_class.reloptions.map(|reloptions| reloptions.map(Into::into).collect()),
				relpartbound: pg_class.relpartbound.map(Into::into),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_class_set)
}
