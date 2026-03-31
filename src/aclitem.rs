use crate::Str;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct AclItem<G> {
	pub grantee: Option<Str>, // None for public
	pub grantor: Str,
	pub grants: Vec<Grant<G>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Grant<G> {
	pub privilege: G,
	pub with_grant_option: bool,
}


use paste::paste;
macro_rules! pg_acl {
	($name:ident { $($char:literal => $variant:ident),* $(,)? }) => {
		paste! {
			pub type [< $name AclItem >] = AclItem<[< $name Privilege >]>;

			#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Ord, PartialOrd)]
			pub enum [< $name Privilege >] {
				$($variant),*
			}

			pub(crate) struct [< $name GrantParser >];
			impl GrantParser for [< $name GrantParser >] {
				type Grant = [< $name Privilege >];
				fn parse_grant<'a>(&self, input: &'a str) -> IResult<&'a str, [< $name Privilege >]> {
					map(one_of(concat!( $($char,)* )), |c| match c {
						$($char => [< $name Privilege >]::$variant,)*
						_ => unreachable!(),
					})
					.parse(input)
				}
			}
		}
	};
}

// DATABASE	CTc	Tc	\l
pg_acl!(Db { 'C' => Create, 'T' => Temporary, 'c' => Connect });

// // DOMAIN  U  U  \dD+
// pg_acl!(Domain { 'U' => Usage });

// FUNCTION or PROCEDURE	X	X	\df+
pg_acl!(Function { 'X' => Execute });

// FOREIGN DATA WRAPPER	U	none	\dew+
pg_acl!(ForeignDataWrapper { 'U' => Usage });

// FOREIGN SERVER	U	none	\des+
pg_acl!(ForeignServer { 'U' => Usage });

// LANGUAGE	U	U	\dL+
pg_acl!(Language { 'U' => Usage });

// PARAMETER	sA	none	\dconfig+
pg_acl!(Parameter { 's' => Set, 'A' => AlterSystem });

// SCHEMA	UC	none	\dn+
pg_acl!(Schema { 'U' => Usage, 'C' => Create });

// // SEQUENCE	rwU	none	\dp
// pg_acl!(Sequence { 'r' => Select, 'w' => Update, 'U' => Usage });

// TABLE (and table-like objects)	arwdDxtm	none	\dp
pg_acl!(Table { 'a' => Insert, 'r' => Select, 'w' => Update, 'd' => Delete, 'D' => Truncate, 'x' => References, 't' => Trigger, 'm' => Maintain, 'U' => Usage });

// Table column	arwx	none	\dp
pg_acl!(TableColumn { 'a' => Insert, 'r' => Select, 'w' => Update, 'x' => References });

// TYPE	U	U	\dT+
pg_acl!(Type { 'U' => Usage });

pg_acl!(AclDefault {
	'a' => Insert, 'r' => Select, 'w' => Update, 'd' => Delete, 'D' => Truncate, 'x' => References, 't' => Trigger, 'm' => Maintain,
	'U' => Usage,
	'X' => Execute,
});

// SELECT	r (“read”)	LARGE OBJECT, SEQUENCE, TABLE (and table-like objects), table column
// INSERT	a (“append”)	TABLE, table column
// UPDATE	w (“write”)	LARGE OBJECT, SEQUENCE, TABLE, table column
// DELETE	d	TABLE
// TRUNCATE	D	TABLE
// REFERENCES	x	TABLE, table column
// TRIGGER	t	TABLE
// CREATE	C	DATABASE, SCHEMA, TABLESPACE
// CONNECT	c	DATABASE
// TEMPORARY	T	DATABASE
// EXECUTE	X	FUNCTION, PROCEDURE
// USAGE	U	DOMAIN, FOREIGN DATA WRAPPER, FOREIGN SERVER, LANGUAGE, SCHEMA, SEQUENCE, TYPE
// SET	s	PARAMETER
// ALTER SYSTEM	A	PARAMETER
// MAINTAIN	m	TABLE

use nom::{
	IResult, Parser,
	branch::alt,
	bytes::complete::take_until,
	character::complete::{char, one_of, satisfy},
	combinator::{all_consuming, map, opt, recognize},
	multi::many0,
	sequence::pair,
};


pub(crate) trait GrantParser {
	type Grant;
	fn parse_grant<'a>(&self, input: &'a str) -> IResult<&'a str, Self::Grant>;
}

pub(crate) fn aclitem<'a, GP>(
	input: &'a str,
	grant_parser: &'a GP,
) -> AclItem<GP::Grant>
where
	GP: GrantParser,
{
	match parse_aclitem(input, grant_parser) {
		Ok((_, item)) => item,
		Err(_e) => panic!("Postgres returned an ACL item in an unexpected format: {}", input),
	}
}

fn parse_aclitem<'a, GP>(
	input: &'a str,
	grant_parser: &'a GP,
) -> IResult<&'a str, AclItem<GP::Grant>>
where
	GP: GrantParser,
{
	let (input, grantee) = opt(parse_role_name).parse(input)?;
	let (input, _) = char('=').parse(input)?;

	// TODO is this ever actually zero or more?
	let (input, grants) = many0(|i: &'a str| {
		let (i, privilege) = grant_parser.parse_grant(i)?;
		let (i, star) = opt(char('*')).parse(i)?;
		Ok((i, Grant { privilege, with_grant_option: star.is_some() }))
	})
	.parse(input)?;

	let (input, _) = char('/').parse(input)?;
	let (input, grantor) = all_consuming(parse_role_name).parse(input)?;

	Ok((input, AclItem { grantee, grantor, grants }))
}

fn parse_role_name(input: &str) -> IResult<&str, Str> {
	alt((parse_quoted_name, parse_unquoted_name)).parse(input)
}

fn parse_unquoted_name(input: &str) -> IResult<&str, Str> {
	map(
		recognize(pair(
			satisfy(|c: char| c.is_alphabetic() || c == '_'),
			many0(satisfy(|c: char| c.is_alphanumeric() || c == '_' || c == '$')),
		)),
		|s: &str| s.into(),
	)
	.parse(input)
}

fn parse_quoted_name(input: &str) -> IResult<&str, Str> {
	let (mut rest, _) = char('"').parse(input)?;
	let mut name = String::new();
	loop {
		let (r, segment) = take_until("\"").parse(rest)?;
		name.push_str(segment);
		let (r, _) = char('"').parse(r)?;
		// A doubled `""` is an escaped quote inside the identifier.
		match char::<&str, nom::error::Error<&str>>('"').parse(r) {
			Ok((r, _)) => { name.push('"'); rest = r; }
			Err(_)     => { rest = r; break; }
		}
	}
	Ok((rest, name.into()))
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn table_with_grant_option() {
		let (rest, item) = parse_aclitem("calvin=r*w/hobbes", &TableGrantParser).unwrap();
		assert_eq!(rest, "");
		assert_eq!(item.grantee.as_deref(), Some("calvin"));
		assert_eq!(item.grantor, "hobbes");
		assert_eq!(item.grants, vec![
			Grant { privilege: TablePrivilege::Select, with_grant_option: true },
			Grant { privilege: TablePrivilege::Update, with_grant_option: false },
		]);
	}

	#[test]
	fn public_grantee() {
		let (_, item) = parse_aclitem("=r/miriam", &TableGrantParser).unwrap();
		assert_eq!(item.grantee, None);
		assert_eq!(item.grantor, "miriam");
		assert_eq!(item.grants[0].privilege, TablePrivilege::Select);
	}

	#[test]
	fn no_grants() {
		let (_, item) = parse_aclitem("alice=/bob", &TableGrantParser).unwrap();
		assert!(item.grants.is_empty());
	}

	#[test]
	fn full_table_acl() {
		let (rest, item) = parse_aclitem("miriam=arwdDxtm/miriam", &TableGrantParser).unwrap();
		assert_eq!(rest, "");
		assert_eq!(item.grants.len(), 8);
	}

	#[test]
	fn database_grants() {
		let (_, item) = parse_aclitem("app=Tc/admin", &DbGrantParser).unwrap();
		assert_eq!(item.grants, vec![
			Grant { privilege: DbPrivilege::Temporary, with_grant_option: false },
			Grant { privilege: DbPrivilege::Connect, with_grant_option: false },
		]);
	}

	#[test]
	fn quoted_role_name() {
		let (_, item) = parse_aclitem(r#""my role"=r/"the owner""#, &TableGrantParser).unwrap();
		assert_eq!(item.grantee.as_deref(), Some("my role"));
		assert_eq!(item.grantor, "the owner");
	}

	#[test]
	fn quoted_role_name_with_escaped_quote() {
		let (_, item) = parse_aclitem(r#""it's""complex"=U/admin"#, &LanguageGrantParser).unwrap();
		assert_eq!(item.grantee.as_deref(), Some(r#"it's"complex"#));
	}

	#[test]
	fn function_execute_with_grant() {
		let (_, item) = parse_aclitem("analyst=X*/dba", &FunctionGrantParser).unwrap();
		assert_eq!(item.grants, vec![
			Grant { privilege: FunctionPrivilege::Execute, with_grant_option: true },
		]);
	}
}
