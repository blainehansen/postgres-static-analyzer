// This file was generated with `clorinde`. Do not modify.

#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgDbRoleSetting {
    pub setdatabase: bool,
    pub setrole: Option<String>,
    pub setconfig: Option<Vec<String>>,
}
pub struct ReflectPgDbRoleSettingBorrowed<'a> {
    pub setdatabase: bool,
    pub setrole: Option<&'a str>,
    pub setconfig: Option<crate::ArrayIterator<'a, &'a str>>,
}
impl<'a> From<ReflectPgDbRoleSettingBorrowed<'a>> for ReflectPgDbRoleSetting {
    fn from(
        ReflectPgDbRoleSettingBorrowed {
            setdatabase,
            setrole,
            setconfig,
        }: ReflectPgDbRoleSettingBorrowed<'a>,
    ) -> Self {
        Self {
            setdatabase,
            setrole: setrole.map(|v| v.into()),
            setconfig: setconfig.map(|v| v.map(|v| v.into()).collect()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgEnum {
    pub enumtypid: String,
    pub enumlabels: Vec<String>,
}
pub struct ReflectPgEnumBorrowed<'a> {
    pub enumtypid: &'a str,
    pub enumlabels: crate::ArrayIterator<'a, &'a str>,
}
impl<'a> From<ReflectPgEnumBorrowed<'a>> for ReflectPgEnum {
    fn from(
        ReflectPgEnumBorrowed {
            enumtypid,
            enumlabels,
        }: ReflectPgEnumBorrowed<'a>,
    ) -> Self {
        Self {
            enumtypid: enumtypid.into(),
            enumlabels: enumlabels.map(|v| v.into()).collect(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgProc {
    pub oid: String,
    pub proname: String,
    pub pronamespace: String,
    pub proowner: String,
    pub prolang: String,
    pub procost: Option<f32>,
    pub prorows: Option<f32>,
    pub provariadic: Option<String>,
    pub prosupport: Option<String>,
    pub prokind: i8,
    pub prosecdef: bool,
    pub proleakproof: bool,
    pub proisstrict: bool,
    pub proretset: bool,
    pub provolatile: i8,
    pub proparallel: i8,
    pub pronargs: i16,
    pub pronargdefaults: i16,
    pub prorettype: String,
    pub proargtypes: Vec<String>,
    pub proallargtypes: Option<Vec<String>>,
    pub proargmodes: Option<Vec<i8>>,
    pub proargnames: Option<Vec<String>>,
    pub proargdefaults: Option<Vec<Option<String>>>,
    pub protrftypes: Option<Vec<String>>,
    pub prosrc: Option<String>,
    pub probin: Option<String>,
    pub prosqlbody: Option<String>,
    pub proconfig: Option<Vec<String>>,
    pub proacl: Option<Vec<crate::types::pg_temp_1::FunctionAclitem>>,
    pub description: Option<String>,
}
pub struct ReflectPgProcBorrowed<'a> {
    pub oid: &'a str,
    pub proname: &'a str,
    pub pronamespace: &'a str,
    pub proowner: &'a str,
    pub prolang: &'a str,
    pub procost: Option<f32>,
    pub prorows: Option<f32>,
    pub provariadic: Option<&'a str>,
    pub prosupport: Option<&'a str>,
    pub prokind: i8,
    pub prosecdef: bool,
    pub proleakproof: bool,
    pub proisstrict: bool,
    pub proretset: bool,
    pub provolatile: i8,
    pub proparallel: i8,
    pub pronargs: i16,
    pub pronargdefaults: i16,
    pub prorettype: &'a str,
    pub proargtypes: crate::ArrayIterator<'a, &'a str>,
    pub proallargtypes: Option<crate::ArrayIterator<'a, &'a str>>,
    pub proargmodes: Option<crate::ArrayIterator<'a, i8>>,
    pub proargnames: Option<crate::ArrayIterator<'a, &'a str>>,
    pub proargdefaults: Option<crate::ArrayIterator<'a, Option<&'a str>>>,
    pub protrftypes: Option<crate::ArrayIterator<'a, &'a str>>,
    pub prosrc: Option<&'a str>,
    pub probin: Option<&'a str>,
    pub prosqlbody: Option<&'a str>,
    pub proconfig: Option<crate::ArrayIterator<'a, &'a str>>,
    pub proacl:
        Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::FunctionAclitemBorrowed<'a>>>,
    pub description: Option<&'a str>,
}
impl<'a> From<ReflectPgProcBorrowed<'a>> for ReflectPgProc {
    fn from(
        ReflectPgProcBorrowed {
            oid,
            proname,
            pronamespace,
            proowner,
            prolang,
            procost,
            prorows,
            provariadic,
            prosupport,
            prokind,
            prosecdef,
            proleakproof,
            proisstrict,
            proretset,
            provolatile,
            proparallel,
            pronargs,
            pronargdefaults,
            prorettype,
            proargtypes,
            proallargtypes,
            proargmodes,
            proargnames,
            proargdefaults,
            protrftypes,
            prosrc,
            probin,
            prosqlbody,
            proconfig,
            proacl,
            description,
        }: ReflectPgProcBorrowed<'a>,
    ) -> Self {
        Self {
            oid: oid.into(),
            proname: proname.into(),
            pronamespace: pronamespace.into(),
            proowner: proowner.into(),
            prolang: prolang.into(),
            procost,
            prorows,
            provariadic: provariadic.map(|v| v.into()),
            prosupport: prosupport.map(|v| v.into()),
            prokind,
            prosecdef,
            proleakproof,
            proisstrict,
            proretset,
            provolatile,
            proparallel,
            pronargs,
            pronargdefaults,
            prorettype: prorettype.into(),
            proargtypes: proargtypes.map(|v| v.into()).collect(),
            proallargtypes: proallargtypes.map(|v| v.map(|v| v.into()).collect()),
            proargmodes: proargmodes.map(|v| v.map(|v| v).collect()),
            proargnames: proargnames.map(|v| v.map(|v| v.into()).collect()),
            proargdefaults: proargdefaults.map(|v| v.map(|v| v.map(|v| v.into())).collect()),
            protrftypes: protrftypes.map(|v| v.map(|v| v.into()).collect()),
            prosrc: prosrc.map(|v| v.into()),
            probin: probin.map(|v| v.into()),
            prosqlbody: prosqlbody.map(|v| v.into()),
            proconfig: proconfig.map(|v| v.map(|v| v.into()).collect()),
            proacl: proacl.map(|v| v.map(|v| v.into()).collect()),
            description: description.map(|v| v.into()),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct ReflectPgDbRoleSettingQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgDbRoleSettingBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgDbRoleSettingBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgDbRoleSettingQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgDbRoleSettingBorrowed) -> R,
    ) -> ReflectPgDbRoleSettingQuery<'c, 'a, 's, C, R, N> {
        ReflectPgDbRoleSettingQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct ReflectPgEnumQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgEnumBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgEnumBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgEnumQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgEnumBorrowed) -> R,
    ) -> ReflectPgEnumQuery<'c, 'a, 's, C, R, N> {
        ReflectPgEnumQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct ReflectPgProcQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgProcBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgProcBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgProcQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgProcBorrowed) -> R,
    ) -> ReflectPgProcQuery<'c, 'a, 's, C, R, N> {
        ReflectPgProcQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct ReflectPgDbRoleSettingStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_db_role_setting() -> ReflectPgDbRoleSettingStmt {
    ReflectPgDbRoleSettingStmt(
        "select setdatabase != 0 as setdatabase, case when setrole = 0 then null else pg_get_userbyid(setrole)::text end as setrole, setconfig from pg_db_role_setting where (setdatabase = 0 or setdatabase = (select oid from pg_database where datname = current_database()))",
        None,
    )
}
impl ReflectPgDbRoleSettingStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> ReflectPgDbRoleSettingQuery<'c, 'a, 's, C, ReflectPgDbRoleSetting, 0> {
        ReflectPgDbRoleSettingQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgDbRoleSettingBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgDbRoleSettingBorrowed {
                    setdatabase: row.try_get(0)?,
                    setrole: row.try_get(1)?,
                    setconfig: row.try_get(2)?,
                })
            },
            mapper: |it| ReflectPgDbRoleSetting::from(it),
        }
    }
}
pub struct ReflectPgEnumStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_enum() -> ReflectPgEnumStmt {
    ReflectPgEnumStmt(
        "select enumtypid::regtype::text as enumtypid, array_agg(enumlabel::text order by enumsortorder) as enumlabels from pg_enum group by enumtypid",
        None,
    )
}
impl ReflectPgEnumStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> ReflectPgEnumQuery<'c, 'a, 's, C, ReflectPgEnum, 0> {
        ReflectPgEnumQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ReflectPgEnumBorrowed, tokio_postgres::Error> {
                    Ok(ReflectPgEnumBorrowed {
                        enumtypid: row.try_get(0)?,
                        enumlabels: row.try_get(1)?,
                    })
                },
            mapper: |it| ReflectPgEnum::from(it),
        }
    }
}
pub struct ReflectPgProcStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_proc() -> ReflectPgProcStmt {
    ReflectPgProcStmt(
        "select pg_proc.oid::regprocedure::text as oid, pg_proc.proname::text as proname, pg_proc.pronamespace::regnamespace::text as pronamespace, pg_get_userbyid(proowner)::text as proowner, prolang_pg_language.lanname::text as prolang, pg_proc.procost as procost, pg_proc.prorows as prorows, case when pg_proc.provariadic = 0 then null else pg_proc.provariadic::regtype::text end as provariadic, case when prosupport = 0 then null else prosupport::regproc::text end as prosupport, pg_proc.prokind as prokind, pg_proc.prosecdef as prosecdef, pg_proc.proleakproof as proleakproof, pg_proc.proisstrict as proisstrict, pg_proc.proretset as proretset, pg_proc.provolatile as provolatile, pg_proc.proparallel as proparallel, pg_proc.pronargs as pronargs, pg_proc.pronargdefaults as pronargdefaults, pg_proc.prorettype::regtype::text as prorettype, pg_proc.proargtypes::regtype[]::text[] as proargtypes, pg_proc.proallargtypes::regtype[]::text[] as proallargtypes, pg_proc.proargmodes as proargmodes, pg_proc.proargnames as proargnames, case when pg_proc.proargdefaults is null then null else pg_temp.format_fn_defaults(pg_proc) end as proargdefaults, pg_proc.protrftypes::regtype[]::text[] as protrftypes, case when pg_proc.prosrc = '' then null else pg_proc.prosrc end as prosrc, case when pg_proc.probin = '' then null else pg_proc.probin end as probin, pg_get_function_sqlbody(pg_proc.oid) as prosqlbody, pg_proc.proconfig as proconfig, pg_temp.format_function_aclitems(proacl) as proacl, pg_description.description as description from pg_proc join pg_language as prolang_pg_language on pg_proc.prolang = prolang_pg_language.oid left join pg_description on pg_description.objoid = pg_proc.oid and pg_description.objsubid = 0 where not starts_with(pg_proc.pronamespace::regnamespace::text, 'pg_temp')",
        None,
    )
}
impl ReflectPgProcStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> ReflectPgProcQuery<'c, 'a, 's, C, ReflectPgProc, 0> {
        ReflectPgProcQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ReflectPgProcBorrowed, tokio_postgres::Error> {
                    Ok(ReflectPgProcBorrowed {
                        oid: row.try_get(0)?,
                        proname: row.try_get(1)?,
                        pronamespace: row.try_get(2)?,
                        proowner: row.try_get(3)?,
                        prolang: row.try_get(4)?,
                        procost: row.try_get(5)?,
                        prorows: row.try_get(6)?,
                        provariadic: row.try_get(7)?,
                        prosupport: row.try_get(8)?,
                        prokind: row.try_get(9)?,
                        prosecdef: row.try_get(10)?,
                        proleakproof: row.try_get(11)?,
                        proisstrict: row.try_get(12)?,
                        proretset: row.try_get(13)?,
                        provolatile: row.try_get(14)?,
                        proparallel: row.try_get(15)?,
                        pronargs: row.try_get(16)?,
                        pronargdefaults: row.try_get(17)?,
                        prorettype: row.try_get(18)?,
                        proargtypes: row.try_get(19)?,
                        proallargtypes: row.try_get(20)?,
                        proargmodes: row.try_get(21)?,
                        proargnames: row.try_get(22)?,
                        proargdefaults: row.try_get(23)?,
                        protrftypes: row.try_get(24)?,
                        prosrc: row.try_get(25)?,
                        probin: row.try_get(26)?,
                        prosqlbody: row.try_get(27)?,
                        proconfig: row.try_get(28)?,
                        proacl: row.try_get(29)?,
                        description: row.try_get(30)?,
                    })
                },
            mapper: |it| ReflectPgProc::from(it),
        }
    }
}
