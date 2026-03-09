use crate::{ClassKind, ClassPersistence, PgClass, PgClient, PgState, Ref, Set, aclitem::{TableGrantParser, aclitem}, postgres};
use futures::TryStreamExt;

fn make_ref((schema_name, name): (&str, &str)) -> Ref {
	Ref { schema_name: schema_name.into(), name: name.into() }
}

pub async fn reflect_pg_state(
	client: &PgClient
) -> Result<PgState, postgres::Error> {
	let (pg_class,) = tokio::try_join!(
		reflect_pg_class(client),
	)?;

	Ok(PgState { pg_class })
}

pub async fn reflect_pg_class(
	client: &PgClient
) -> Result<Set<PgClass>, postgres::Error> {
	let pg_class_set = reflect_crate::queries::main::reflect_pg_class().bind(client)
		.map(|pg_class| {
			PgClass {
				relname: pg_class.relname.into(),
				relnamespace: pg_class.relnamespace.into(),
				reltype: pg_class.reltype_schema_name.zip(pg_class.reltype_name).map(make_ref),
				reloftype: pg_class.reloftype_schema_name.zip(pg_class.reloftype_name).map(make_ref),
				relowner: pg_class.relowner.into(),
				reltablespace: pg_class.reltablespace.map(Into::into),
				relisshared: pg_class.relisshared,
				relpersistence: ClassPersistence::pg_from_char(pg_class.relpersistence),
				relkind: ClassKind::pg_from_char(pg_class.relkind),
				relrowsecurity: pg_class.relrowsecurity,
				relforcerowsecurity: pg_class.relforcerowsecurity,
				relispartition: pg_class.relispartition,
				relacl: pg_class.relacl.map(|acl| aclitem(acl, &TableGrantParser)).collect(),
				reloptions: pg_class.reloptions.map(Into::into).collect(),
			}
		})
		.iter()
		.await?
		.try_collect()
		.await?;

	Ok(pg_class_set)
}
