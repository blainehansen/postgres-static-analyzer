pub(crate) use smol_str::SmolStr as Str;

pub(crate) use reflect_crate::tokio_postgres::{self as postgres, /*Config as PgConfig,*/ Client as PgClient};

pub type Set<T> = hashbrown::HashSet<T>;
pub type Map<T> = hashbrown::HashMap<Str, T>;
pub(crate) use hashbrown::HashMap;

mod reflect;
#[cfg(test)]
mod reflect_test;

mod reflect_gen;

pub use reflect::*;

mod aclitem;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Qual {
	pub schema_name: Str,
	pub name: Str,
}
impl Qual {
	pub(crate) fn make(schema_name: &str, name: &str) -> Qual {
		Qual { schema_name: schema_name.into(), name: name.into() }
	}

	pub(crate) fn maybe(schema_name: Option<&str>, name: Option<&str>) -> Option<Qual> {
		match (schema_name, name) {
			(Some(schema_name), Some(name)) => Some(Qual::make(schema_name, name)),
			_ => None,
		}
	}

	pub(crate) fn parse(qualified: &str) -> Qual {
		// TODO this needs to be smarter to account for complex quoted identifiers that could contain .
		let (schema_name, name) = qualified.split_once(".").unwrap_or(("pg_catalog", qualified));
		Qual { schema_name: schema_name.into(), name: name.into() }
	}

	pub(crate) fn maybe_parse(qualified: Option<&str>) -> Option<Qual> {
		qualified.map(Qual::parse)
	}
}
// impl<T: AsRef<str>> hashbrown::Equivalent<Qual> for (&T, &T) {
// 	fn equivalent(&self, key: &Qual) -> bool {
// 		key.schema_name == self.0.as_ref() && key.name == self.1.as_ref()
// 	}
// }
