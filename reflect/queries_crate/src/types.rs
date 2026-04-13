// This file was generated with `clorinde`. Do not modify.

pub mod pg_temp_1 {
    #[derive(Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
    #[postgres(name = "function_aclgrant")]
    pub struct FunctionAclgrant {
        #[postgres(name = "privilege")]
        pub privilege: postgres_static_analyzer_ddl_catalog_structs::FunctionAclPrivilege,
        #[postgres(name = "with_grant_option")]
        pub with_grant_option: bool,
    }
    impl<'a> postgres_types::ToSql for FunctionAclgrant {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let FunctionAclgrant {
                privilege,
                with_grant_option,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "privilege" => postgres_types::ToSql::to_sql(privilege, field.type_(), out),
                    "with_grant_option" => {
                        postgres_types::ToSql::to_sql(with_grant_option, field.type_(), out)
                    }
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "function_aclgrant" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "privilege" => {
                                <postgres_static_analyzer_ddl_catalog_structs::FunctionAclPrivilege as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            "with_grant_option" => {
                                <bool as postgres_types::ToSql>::accepts(f.type_())
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Clone, PartialEq)]
    #[postgres(name = "function_aclitem")]
    pub struct FunctionAclitem {
        #[postgres(name = "grantee")]
        pub grantee: String,
        #[postgres(name = "grantor")]
        pub grantor: String,
        #[postgres(name = "grants")]
        pub grants: Vec<super::pg_temp_1::FunctionAclgrant>,
    }
    #[derive(Debug)]
    pub struct FunctionAclitemBorrowed<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: crate::ArrayIterator<'a, super::pg_temp_1::FunctionAclgrant>,
    }
    impl<'a> From<FunctionAclitemBorrowed<'a>> for FunctionAclitem {
        fn from(
            FunctionAclitemBorrowed {
                grantee,
                grantor,
                grants,
            }: FunctionAclitemBorrowed<'a>,
        ) -> Self {
            Self {
                grantee: grantee.into(),
                grantor: grantor.into(),
                grants: grants.map(|v| v).collect(),
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for FunctionAclitemBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<FunctionAclitemBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantee = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantor = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grants = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
            Ok(FunctionAclitemBorrowed {
                grantee,
                grantor,
                grants,
            })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "function_aclitem" && ty.schema() == "pg_temp_1"
        }
    }
    #[derive(Debug)]
    pub struct FunctionAclitemParams<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: &'a [super::pg_temp_1::FunctionAclgrant],
    }
    impl<'a> postgres_types::ToSql for FunctionAclitemParams<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let FunctionAclitemParams {
                grantee,
                grantor,
                grants,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "grantee" => postgres_types::ToSql::to_sql(grantee, field.type_(), out),
                    "grantor" => postgres_types::ToSql::to_sql(grantor, field.type_(), out),
                    "grants" => postgres_types::ToSql::to_sql(grants, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "function_aclitem" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 3 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "grantee" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grantor" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grants" => {
                                <&'a [super::pg_temp_1::FunctionAclgrant] as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
    #[postgres(name = "tablecolumn_aclgrant")]
    pub struct TablecolumnAclgrant {
        #[postgres(name = "privilege")]
        pub privilege: postgres_static_analyzer_ddl_catalog_structs::TableColumnAclPrivilege,
        #[postgres(name = "with_grant_option")]
        pub with_grant_option: bool,
    }
    impl<'a> postgres_types::ToSql for TablecolumnAclgrant {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let TablecolumnAclgrant {
                privilege,
                with_grant_option,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "privilege" => postgres_types::ToSql::to_sql(privilege, field.type_(), out),
                    "with_grant_option" => {
                        postgres_types::ToSql::to_sql(with_grant_option, field.type_(), out)
                    }
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "tablecolumn_aclgrant" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "privilege" => {
                                <postgres_static_analyzer_ddl_catalog_structs::TableColumnAclPrivilege as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            "with_grant_option" => {
                                <bool as postgres_types::ToSql>::accepts(f.type_())
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Clone, PartialEq)]
    #[postgres(name = "tablecolumn_aclitem")]
    pub struct TablecolumnAclitem {
        #[postgres(name = "grantee")]
        pub grantee: String,
        #[postgres(name = "grantor")]
        pub grantor: String,
        #[postgres(name = "grants")]
        pub grants: Vec<super::pg_temp_1::TablecolumnAclgrant>,
    }
    #[derive(Debug)]
    pub struct TablecolumnAclitemBorrowed<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: crate::ArrayIterator<'a, super::pg_temp_1::TablecolumnAclgrant>,
    }
    impl<'a> From<TablecolumnAclitemBorrowed<'a>> for TablecolumnAclitem {
        fn from(
            TablecolumnAclitemBorrowed {
                grantee,
                grantor,
                grants,
            }: TablecolumnAclitemBorrowed<'a>,
        ) -> Self {
            Self {
                grantee: grantee.into(),
                grantor: grantor.into(),
                grants: grants.map(|v| v).collect(),
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for TablecolumnAclitemBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<TablecolumnAclitemBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
        {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantee = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantor = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grants = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
            Ok(TablecolumnAclitemBorrowed {
                grantee,
                grantor,
                grants,
            })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "tablecolumn_aclitem" && ty.schema() == "pg_temp_1"
        }
    }
    #[derive(Debug)]
    pub struct TablecolumnAclitemParams<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: &'a [super::pg_temp_1::TablecolumnAclgrant],
    }
    impl<'a> postgres_types::ToSql for TablecolumnAclitemParams<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let TablecolumnAclitemParams {
                grantee,
                grantor,
                grants,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "grantee" => postgres_types::ToSql::to_sql(grantee, field.type_(), out),
                    "grantor" => postgres_types::ToSql::to_sql(grantor, field.type_(), out),
                    "grants" => postgres_types::ToSql::to_sql(grants, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "tablecolumn_aclitem" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 3 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "grantee" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grantor" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grants" => {
                                <&'a [super::pg_temp_1::TablecolumnAclgrant] as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
    #[postgres(name = "table_aclgrant")]
    pub struct TableAclgrant {
        #[postgres(name = "privilege")]
        pub privilege: postgres_static_analyzer_ddl_catalog_structs::TableAclPrivilege,
        #[postgres(name = "with_grant_option")]
        pub with_grant_option: bool,
    }
    impl<'a> postgres_types::ToSql for TableAclgrant {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let TableAclgrant {
                privilege,
                with_grant_option,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "privilege" => postgres_types::ToSql::to_sql(privilege, field.type_(), out),
                    "with_grant_option" => {
                        postgres_types::ToSql::to_sql(with_grant_option, field.type_(), out)
                    }
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "table_aclgrant" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "privilege" => {
                                <postgres_static_analyzer_ddl_catalog_structs::TableAclPrivilege as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            "with_grant_option" => {
                                <bool as postgres_types::ToSql>::accepts(f.type_())
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Clone, PartialEq)]
    #[postgres(name = "table_aclitem")]
    pub struct TableAclitem {
        #[postgres(name = "grantee")]
        pub grantee: String,
        #[postgres(name = "grantor")]
        pub grantor: String,
        #[postgres(name = "grants")]
        pub grants: Vec<super::pg_temp_1::TableAclgrant>,
    }
    #[derive(Debug)]
    pub struct TableAclitemBorrowed<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: crate::ArrayIterator<'a, super::pg_temp_1::TableAclgrant>,
    }
    impl<'a> From<TableAclitemBorrowed<'a>> for TableAclitem {
        fn from(
            TableAclitemBorrowed {
                grantee,
                grantor,
                grants,
            }: TableAclitemBorrowed<'a>,
        ) -> Self {
            Self {
                grantee: grantee.into(),
                grantor: grantor.into(),
                grants: grants.map(|v| v).collect(),
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for TableAclitemBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<TableAclitemBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantee = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantor = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grants = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
            Ok(TableAclitemBorrowed {
                grantee,
                grantor,
                grants,
            })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "table_aclitem" && ty.schema() == "pg_temp_1"
        }
    }
    #[derive(Debug)]
    pub struct TableAclitemParams<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: &'a [super::pg_temp_1::TableAclgrant],
    }
    impl<'a> postgres_types::ToSql for TableAclitemParams<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let TableAclitemParams {
                grantee,
                grantor,
                grants,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "grantee" => postgres_types::ToSql::to_sql(grantee, field.type_(), out),
                    "grantor" => postgres_types::ToSql::to_sql(grantor, field.type_(), out),
                    "grants" => postgres_types::ToSql::to_sql(grants, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "table_aclitem" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 3 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "grantee" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grantor" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grants" => {
                                <&'a [super::pg_temp_1::TableAclgrant] as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
    #[postgres(name = "db_aclgrant")]
    pub struct DbAclgrant {
        #[postgres(name = "privilege")]
        pub privilege: postgres_static_analyzer_ddl_catalog_structs::DbAclPrivilege,
        #[postgres(name = "with_grant_option")]
        pub with_grant_option: bool,
    }
    impl<'a> postgres_types::ToSql for DbAclgrant {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let DbAclgrant {
                privilege,
                with_grant_option,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "privilege" => postgres_types::ToSql::to_sql(privilege, field.type_(), out),
                    "with_grant_option" => {
                        postgres_types::ToSql::to_sql(with_grant_option, field.type_(), out)
                    }
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "db_aclgrant" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "privilege" => {
                                <postgres_static_analyzer_ddl_catalog_structs::DbAclPrivilege as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            "with_grant_option" => {
                                <bool as postgres_types::ToSql>::accepts(f.type_())
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Clone, PartialEq)]
    #[postgres(name = "db_aclitem")]
    pub struct DbAclitem {
        #[postgres(name = "grantee")]
        pub grantee: String,
        #[postgres(name = "grantor")]
        pub grantor: String,
        #[postgres(name = "grants")]
        pub grants: Vec<super::pg_temp_1::DbAclgrant>,
    }
    #[derive(Debug)]
    pub struct DbAclitemBorrowed<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: crate::ArrayIterator<'a, super::pg_temp_1::DbAclgrant>,
    }
    impl<'a> From<DbAclitemBorrowed<'a>> for DbAclitem {
        fn from(
            DbAclitemBorrowed {
                grantee,
                grantor,
                grants,
            }: DbAclitemBorrowed<'a>,
        ) -> Self {
            Self {
                grantee: grantee.into(),
                grantor: grantor.into(),
                grants: grants.map(|v| v).collect(),
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for DbAclitemBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<DbAclitemBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantee = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantor = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grants = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
            Ok(DbAclitemBorrowed {
                grantee,
                grantor,
                grants,
            })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "db_aclitem" && ty.schema() == "pg_temp_1"
        }
    }
    #[derive(Debug)]
    pub struct DbAclitemParams<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: &'a [super::pg_temp_1::DbAclgrant],
    }
    impl<'a> postgres_types::ToSql for DbAclitemParams<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let DbAclitemParams {
                grantee,
                grantor,
                grants,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "grantee" => postgres_types::ToSql::to_sql(grantee, field.type_(), out),
                    "grantor" => postgres_types::ToSql::to_sql(grantor, field.type_(), out),
                    "grants" => postgres_types::ToSql::to_sql(grants, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "db_aclitem" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 3 {
                        return false;
                    }
                    fields.iter().all(|f| match f.name() {
                        "grantee" => <&'a str as postgres_types::ToSql>::accepts(f.type_()),
                        "grantor" => <&'a str as postgres_types::ToSql>::accepts(f.type_()),
                        "grants" => {
                            <&'a [super::pg_temp_1::DbAclgrant] as postgres_types::ToSql>::accepts(
                                f.type_(),
                            )
                        }
                        _ => false,
                    })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
    #[postgres(name = "acldefault_aclgrant")]
    pub struct AcldefaultAclgrant {
        #[postgres(name = "privilege")]
        pub privilege: postgres_static_analyzer_ddl_catalog_structs::AclDefaultAclPrivilege,
        #[postgres(name = "with_grant_option")]
        pub with_grant_option: bool,
    }
    impl<'a> postgres_types::ToSql for AcldefaultAclgrant {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let AcldefaultAclgrant {
                privilege,
                with_grant_option,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "privilege" => postgres_types::ToSql::to_sql(privilege, field.type_(), out),
                    "with_grant_option" => {
                        postgres_types::ToSql::to_sql(with_grant_option, field.type_(), out)
                    }
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "acldefault_aclgrant" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "privilege" => {
                                <postgres_static_analyzer_ddl_catalog_structs::AclDefaultAclPrivilege as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            "with_grant_option" => {
                                <bool as postgres_types::ToSql>::accepts(f.type_())
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Clone, PartialEq)]
    #[postgres(name = "acldefault_aclitem")]
    pub struct AcldefaultAclitem {
        #[postgres(name = "grantee")]
        pub grantee: String,
        #[postgres(name = "grantor")]
        pub grantor: String,
        #[postgres(name = "grants")]
        pub grants: Vec<super::pg_temp_1::AcldefaultAclgrant>,
    }
    #[derive(Debug)]
    pub struct AcldefaultAclitemBorrowed<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: crate::ArrayIterator<'a, super::pg_temp_1::AcldefaultAclgrant>,
    }
    impl<'a> From<AcldefaultAclitemBorrowed<'a>> for AcldefaultAclitem {
        fn from(
            AcldefaultAclitemBorrowed {
                grantee,
                grantor,
                grants,
            }: AcldefaultAclitemBorrowed<'a>,
        ) -> Self {
            Self {
                grantee: grantee.into(),
                grantor: grantor.into(),
                grants: grants.map(|v| v).collect(),
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for AcldefaultAclitemBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<AcldefaultAclitemBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
        {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantee = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantor = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grants = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
            Ok(AcldefaultAclitemBorrowed {
                grantee,
                grantor,
                grants,
            })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "acldefault_aclitem" && ty.schema() == "pg_temp_1"
        }
    }
    #[derive(Debug)]
    pub struct AcldefaultAclitemParams<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: &'a [super::pg_temp_1::AcldefaultAclgrant],
    }
    impl<'a> postgres_types::ToSql for AcldefaultAclitemParams<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let AcldefaultAclitemParams {
                grantee,
                grantor,
                grants,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "grantee" => postgres_types::ToSql::to_sql(grantee, field.type_(), out),
                    "grantor" => postgres_types::ToSql::to_sql(grantor, field.type_(), out),
                    "grants" => postgres_types::ToSql::to_sql(grants, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "acldefault_aclitem" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 3 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "grantee" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grantor" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grants" => {
                                <&'a [super::pg_temp_1::AcldefaultAclgrant] as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
    #[postgres(name = "foreigndatawrapper_aclgrant")]
    pub struct ForeigndatawrapperAclgrant {
        #[postgres(name = "privilege")]
        pub privilege: postgres_static_analyzer_ddl_catalog_structs::ForeignDataWrapperAclPrivilege,
        #[postgres(name = "with_grant_option")]
        pub with_grant_option: bool,
    }
    impl<'a> postgres_types::ToSql for ForeigndatawrapperAclgrant {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let ForeigndatawrapperAclgrant {
                privilege,
                with_grant_option,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "privilege" => postgres_types::ToSql::to_sql(privilege, field.type_(), out),
                    "with_grant_option" => {
                        postgres_types::ToSql::to_sql(with_grant_option, field.type_(), out)
                    }
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "foreigndatawrapper_aclgrant" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "privilege" => {
                                <postgres_static_analyzer_ddl_catalog_structs::ForeignDataWrapperAclPrivilege as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            "with_grant_option" => {
                                <bool as postgres_types::ToSql>::accepts(f.type_())
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Clone, PartialEq)]
    #[postgres(name = "foreigndatawrapper_aclitem")]
    pub struct ForeigndatawrapperAclitem {
        #[postgres(name = "grantee")]
        pub grantee: String,
        #[postgres(name = "grantor")]
        pub grantor: String,
        #[postgres(name = "grants")]
        pub grants: Vec<super::pg_temp_1::ForeigndatawrapperAclgrant>,
    }
    #[derive(Debug)]
    pub struct ForeigndatawrapperAclitemBorrowed<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: crate::ArrayIterator<'a, super::pg_temp_1::ForeigndatawrapperAclgrant>,
    }
    impl<'a> From<ForeigndatawrapperAclitemBorrowed<'a>> for ForeigndatawrapperAclitem {
        fn from(
            ForeigndatawrapperAclitemBorrowed {
                grantee,
                grantor,
                grants,
            }: ForeigndatawrapperAclitemBorrowed<'a>,
        ) -> Self {
            Self {
                grantee: grantee.into(),
                grantor: grantor.into(),
                grants: grants.map(|v| v).collect(),
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for ForeigndatawrapperAclitemBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<ForeigndatawrapperAclitemBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
        {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantee = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantor = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grants = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
            Ok(ForeigndatawrapperAclitemBorrowed {
                grantee,
                grantor,
                grants,
            })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "foreigndatawrapper_aclitem" && ty.schema() == "pg_temp_1"
        }
    }
    #[derive(Debug)]
    pub struct ForeigndatawrapperAclitemParams<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: &'a [super::pg_temp_1::ForeigndatawrapperAclgrant],
    }
    impl<'a> postgres_types::ToSql for ForeigndatawrapperAclitemParams<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let ForeigndatawrapperAclitemParams {
                grantee,
                grantor,
                grants,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "grantee" => postgres_types::ToSql::to_sql(grantee, field.type_(), out),
                    "grantor" => postgres_types::ToSql::to_sql(grantor, field.type_(), out),
                    "grants" => postgres_types::ToSql::to_sql(grants, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "foreigndatawrapper_aclitem" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 3 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "grantee" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grantor" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grants" => {
                                <&'a [super::pg_temp_1::ForeigndatawrapperAclgrant] as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
    #[postgres(name = "foreignserver_aclgrant")]
    pub struct ForeignserverAclgrant {
        #[postgres(name = "privilege")]
        pub privilege: postgres_static_analyzer_ddl_catalog_structs::ForeignServerAclPrivilege,
        #[postgres(name = "with_grant_option")]
        pub with_grant_option: bool,
    }
    impl<'a> postgres_types::ToSql for ForeignserverAclgrant {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let ForeignserverAclgrant {
                privilege,
                with_grant_option,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "privilege" => postgres_types::ToSql::to_sql(privilege, field.type_(), out),
                    "with_grant_option" => {
                        postgres_types::ToSql::to_sql(with_grant_option, field.type_(), out)
                    }
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "foreignserver_aclgrant" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "privilege" => {
                                <postgres_static_analyzer_ddl_catalog_structs::ForeignServerAclPrivilege as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            "with_grant_option" => {
                                <bool as postgres_types::ToSql>::accepts(f.type_())
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Clone, PartialEq)]
    #[postgres(name = "foreignserver_aclitem")]
    pub struct ForeignserverAclitem {
        #[postgres(name = "grantee")]
        pub grantee: String,
        #[postgres(name = "grantor")]
        pub grantor: String,
        #[postgres(name = "grants")]
        pub grants: Vec<super::pg_temp_1::ForeignserverAclgrant>,
    }
    #[derive(Debug)]
    pub struct ForeignserverAclitemBorrowed<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: crate::ArrayIterator<'a, super::pg_temp_1::ForeignserverAclgrant>,
    }
    impl<'a> From<ForeignserverAclitemBorrowed<'a>> for ForeignserverAclitem {
        fn from(
            ForeignserverAclitemBorrowed {
                grantee,
                grantor,
                grants,
            }: ForeignserverAclitemBorrowed<'a>,
        ) -> Self {
            Self {
                grantee: grantee.into(),
                grantor: grantor.into(),
                grants: grants.map(|v| v).collect(),
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for ForeignserverAclitemBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<ForeignserverAclitemBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
        {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantee = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantor = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grants = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
            Ok(ForeignserverAclitemBorrowed {
                grantee,
                grantor,
                grants,
            })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "foreignserver_aclitem" && ty.schema() == "pg_temp_1"
        }
    }
    #[derive(Debug)]
    pub struct ForeignserverAclitemParams<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: &'a [super::pg_temp_1::ForeignserverAclgrant],
    }
    impl<'a> postgres_types::ToSql for ForeignserverAclitemParams<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let ForeignserverAclitemParams {
                grantee,
                grantor,
                grants,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "grantee" => postgres_types::ToSql::to_sql(grantee, field.type_(), out),
                    "grantor" => postgres_types::ToSql::to_sql(grantor, field.type_(), out),
                    "grants" => postgres_types::ToSql::to_sql(grants, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "foreignserver_aclitem" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 3 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "grantee" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grantor" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grants" => {
                                <&'a [super::pg_temp_1::ForeignserverAclgrant] as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
    #[postgres(name = "language_aclgrant")]
    pub struct LanguageAclgrant {
        #[postgres(name = "privilege")]
        pub privilege: postgres_static_analyzer_ddl_catalog_structs::LanguageAclPrivilege,
        #[postgres(name = "with_grant_option")]
        pub with_grant_option: bool,
    }
    impl<'a> postgres_types::ToSql for LanguageAclgrant {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let LanguageAclgrant {
                privilege,
                with_grant_option,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "privilege" => postgres_types::ToSql::to_sql(privilege, field.type_(), out),
                    "with_grant_option" => {
                        postgres_types::ToSql::to_sql(with_grant_option, field.type_(), out)
                    }
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "language_aclgrant" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "privilege" => {
                                <postgres_static_analyzer_ddl_catalog_structs::LanguageAclPrivilege as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            "with_grant_option" => {
                                <bool as postgres_types::ToSql>::accepts(f.type_())
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Clone, PartialEq)]
    #[postgres(name = "language_aclitem")]
    pub struct LanguageAclitem {
        #[postgres(name = "grantee")]
        pub grantee: String,
        #[postgres(name = "grantor")]
        pub grantor: String,
        #[postgres(name = "grants")]
        pub grants: Vec<super::pg_temp_1::LanguageAclgrant>,
    }
    #[derive(Debug)]
    pub struct LanguageAclitemBorrowed<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: crate::ArrayIterator<'a, super::pg_temp_1::LanguageAclgrant>,
    }
    impl<'a> From<LanguageAclitemBorrowed<'a>> for LanguageAclitem {
        fn from(
            LanguageAclitemBorrowed {
                grantee,
                grantor,
                grants,
            }: LanguageAclitemBorrowed<'a>,
        ) -> Self {
            Self {
                grantee: grantee.into(),
                grantor: grantor.into(),
                grants: grants.map(|v| v).collect(),
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for LanguageAclitemBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<LanguageAclitemBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantee = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantor = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grants = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
            Ok(LanguageAclitemBorrowed {
                grantee,
                grantor,
                grants,
            })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "language_aclitem" && ty.schema() == "pg_temp_1"
        }
    }
    #[derive(Debug)]
    pub struct LanguageAclitemParams<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: &'a [super::pg_temp_1::LanguageAclgrant],
    }
    impl<'a> postgres_types::ToSql for LanguageAclitemParams<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let LanguageAclitemParams {
                grantee,
                grantor,
                grants,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "grantee" => postgres_types::ToSql::to_sql(grantee, field.type_(), out),
                    "grantor" => postgres_types::ToSql::to_sql(grantor, field.type_(), out),
                    "grants" => postgres_types::ToSql::to_sql(grants, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "language_aclitem" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 3 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "grantee" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grantor" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grants" => {
                                <&'a [super::pg_temp_1::LanguageAclgrant] as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
    #[postgres(name = "schema_aclgrant")]
    pub struct SchemaAclgrant {
        #[postgres(name = "privilege")]
        pub privilege: postgres_static_analyzer_ddl_catalog_structs::SchemaAclPrivilege,
        #[postgres(name = "with_grant_option")]
        pub with_grant_option: bool,
    }
    impl<'a> postgres_types::ToSql for SchemaAclgrant {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let SchemaAclgrant {
                privilege,
                with_grant_option,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "privilege" => postgres_types::ToSql::to_sql(privilege, field.type_(), out),
                    "with_grant_option" => {
                        postgres_types::ToSql::to_sql(with_grant_option, field.type_(), out)
                    }
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "schema_aclgrant" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "privilege" => {
                                <postgres_static_analyzer_ddl_catalog_structs::SchemaAclPrivilege as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            "with_grant_option" => {
                                <bool as postgres_types::ToSql>::accepts(f.type_())
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Clone, PartialEq)]
    #[postgres(name = "schema_aclitem")]
    pub struct SchemaAclitem {
        #[postgres(name = "grantee")]
        pub grantee: String,
        #[postgres(name = "grantor")]
        pub grantor: String,
        #[postgres(name = "grants")]
        pub grants: Vec<super::pg_temp_1::SchemaAclgrant>,
    }
    #[derive(Debug)]
    pub struct SchemaAclitemBorrowed<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: crate::ArrayIterator<'a, super::pg_temp_1::SchemaAclgrant>,
    }
    impl<'a> From<SchemaAclitemBorrowed<'a>> for SchemaAclitem {
        fn from(
            SchemaAclitemBorrowed {
                grantee,
                grantor,
                grants,
            }: SchemaAclitemBorrowed<'a>,
        ) -> Self {
            Self {
                grantee: grantee.into(),
                grantor: grantor.into(),
                grants: grants.map(|v| v).collect(),
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for SchemaAclitemBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<SchemaAclitemBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantee = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantor = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grants = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
            Ok(SchemaAclitemBorrowed {
                grantee,
                grantor,
                grants,
            })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "schema_aclitem" && ty.schema() == "pg_temp_1"
        }
    }
    #[derive(Debug)]
    pub struct SchemaAclitemParams<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: &'a [super::pg_temp_1::SchemaAclgrant],
    }
    impl<'a> postgres_types::ToSql for SchemaAclitemParams<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let SchemaAclitemParams {
                grantee,
                grantor,
                grants,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "grantee" => postgres_types::ToSql::to_sql(grantee, field.type_(), out),
                    "grantor" => postgres_types::ToSql::to_sql(grantor, field.type_(), out),
                    "grants" => postgres_types::ToSql::to_sql(grants, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "schema_aclitem" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 3 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "grantee" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grantor" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grants" => {
                                <&'a [super::pg_temp_1::SchemaAclgrant] as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
    #[postgres(name = "parameter_aclgrant")]
    pub struct ParameterAclgrant {
        #[postgres(name = "privilege")]
        pub privilege: postgres_static_analyzer_ddl_catalog_structs::ParameterAclPrivilege,
        #[postgres(name = "with_grant_option")]
        pub with_grant_option: bool,
    }
    impl<'a> postgres_types::ToSql for ParameterAclgrant {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let ParameterAclgrant {
                privilege,
                with_grant_option,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "privilege" => postgres_types::ToSql::to_sql(privilege, field.type_(), out),
                    "with_grant_option" => {
                        postgres_types::ToSql::to_sql(with_grant_option, field.type_(), out)
                    }
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "parameter_aclgrant" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "privilege" => {
                                <postgres_static_analyzer_ddl_catalog_structs::ParameterAclPrivilege as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            "with_grant_option" => {
                                <bool as postgres_types::ToSql>::accepts(f.type_())
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Clone, PartialEq)]
    #[postgres(name = "parameter_aclitem")]
    pub struct ParameterAclitem {
        #[postgres(name = "grantee")]
        pub grantee: String,
        #[postgres(name = "grantor")]
        pub grantor: String,
        #[postgres(name = "grants")]
        pub grants: Vec<super::pg_temp_1::ParameterAclgrant>,
    }
    #[derive(Debug)]
    pub struct ParameterAclitemBorrowed<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: crate::ArrayIterator<'a, super::pg_temp_1::ParameterAclgrant>,
    }
    impl<'a> From<ParameterAclitemBorrowed<'a>> for ParameterAclitem {
        fn from(
            ParameterAclitemBorrowed {
                grantee,
                grantor,
                grants,
            }: ParameterAclitemBorrowed<'a>,
        ) -> Self {
            Self {
                grantee: grantee.into(),
                grantor: grantor.into(),
                grants: grants.map(|v| v).collect(),
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for ParameterAclitemBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<ParameterAclitemBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
        {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantee = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantor = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grants = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
            Ok(ParameterAclitemBorrowed {
                grantee,
                grantor,
                grants,
            })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "parameter_aclitem" && ty.schema() == "pg_temp_1"
        }
    }
    #[derive(Debug)]
    pub struct ParameterAclitemParams<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: &'a [super::pg_temp_1::ParameterAclgrant],
    }
    impl<'a> postgres_types::ToSql for ParameterAclitemParams<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let ParameterAclitemParams {
                grantee,
                grantor,
                grants,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "grantee" => postgres_types::ToSql::to_sql(grantee, field.type_(), out),
                    "grantor" => postgres_types::ToSql::to_sql(grantor, field.type_(), out),
                    "grants" => postgres_types::ToSql::to_sql(grants, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "parameter_aclitem" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 3 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "grantee" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grantor" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grants" => {
                                <&'a [super::pg_temp_1::ParameterAclgrant] as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
    #[postgres(name = "type_aclgrant")]
    pub struct TypeAclgrant {
        #[postgres(name = "privilege")]
        pub privilege: postgres_static_analyzer_ddl_catalog_structs::TypeAclPrivilege,
        #[postgres(name = "with_grant_option")]
        pub with_grant_option: bool,
    }
    impl<'a> postgres_types::ToSql for TypeAclgrant {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let TypeAclgrant {
                privilege,
                with_grant_option,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "privilege" => postgres_types::ToSql::to_sql(privilege, field.type_(), out),
                    "with_grant_option" => {
                        postgres_types::ToSql::to_sql(with_grant_option, field.type_(), out)
                    }
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "type_aclgrant" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "privilege" => {
                                <postgres_static_analyzer_ddl_catalog_structs::TypeAclPrivilege as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            "with_grant_option" => {
                                <bool as postgres_types::ToSql>::accepts(f.type_())
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(Debug, postgres_types::FromSql, Clone, PartialEq)]
    #[postgres(name = "type_aclitem")]
    pub struct TypeAclitem {
        #[postgres(name = "grantee")]
        pub grantee: String,
        #[postgres(name = "grantor")]
        pub grantor: String,
        #[postgres(name = "grants")]
        pub grants: Vec<super::pg_temp_1::TypeAclgrant>,
    }
    #[derive(Debug)]
    pub struct TypeAclitemBorrowed<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: crate::ArrayIterator<'a, super::pg_temp_1::TypeAclgrant>,
    }
    impl<'a> From<TypeAclitemBorrowed<'a>> for TypeAclitem {
        fn from(
            TypeAclitemBorrowed {
                grantee,
                grantor,
                grants,
            }: TypeAclitemBorrowed<'a>,
        ) -> Self {
            Self {
                grantee: grantee.into(),
                grantor: grantor.into(),
                grants: grants.map(|v| v).collect(),
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for TypeAclitemBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<TypeAclitemBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantee = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grantor = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let grants = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
            Ok(TypeAclitemBorrowed {
                grantee,
                grantor,
                grants,
            })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "type_aclitem" && ty.schema() == "pg_temp_1"
        }
    }
    #[derive(Debug)]
    pub struct TypeAclitemParams<'a> {
        pub grantee: &'a str,
        pub grantor: &'a str,
        pub grants: &'a [super::pg_temp_1::TypeAclgrant],
    }
    impl<'a> postgres_types::ToSql for TypeAclitemParams<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let TypeAclitemParams {
                grantee,
                grantor,
                grants,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "grantee" => postgres_types::ToSql::to_sql(grantee, field.type_(), out),
                    "grantor" => postgres_types::ToSql::to_sql(grantor, field.type_(), out),
                    "grants" => postgres_types::ToSql::to_sql(grants, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::MAX as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "type_aclitem" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 3 {
                        return false;
                    }
                    fields
                        .iter()
                        .all(|f| match f.name() {
                            "grantee" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grantor" => {
                                <&'a str as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "grants" => {
                                <&'a [super::pg_temp_1::TypeAclgrant] as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            _ => false,
                        })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
}
