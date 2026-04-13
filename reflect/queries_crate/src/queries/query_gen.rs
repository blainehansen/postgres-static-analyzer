// This file was generated with `clorinde`. Do not modify.

#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgAggregate {
    pub aggfnoid: String,
    pub aggkind: i8,
    pub aggnumdirectargs: i16,
    pub aggtransfn: String,
    pub aggfinalfn: Option<String>,
    pub aggcombinefn: Option<String>,
    pub aggserialfn: Option<String>,
    pub aggdeserialfn: Option<String>,
    pub aggmtransfn: Option<String>,
    pub aggminvtransfn: Option<String>,
    pub aggmfinalfn: Option<String>,
    pub aggfinalextra: bool,
    pub aggmfinalextra: bool,
    pub aggfinalmodify: i8,
    pub aggmfinalmodify: i8,
    pub aggsortop: Option<String>,
    pub aggtranstype: String,
    pub aggmtranstype: Option<String>,
    pub agginitval: Option<String>,
    pub aggminitval: Option<String>,
}
pub struct ReflectPgAggregateBorrowed<'a> {
    pub aggfnoid: &'a str,
    pub aggkind: i8,
    pub aggnumdirectargs: i16,
    pub aggtransfn: &'a str,
    pub aggfinalfn: Option<&'a str>,
    pub aggcombinefn: Option<&'a str>,
    pub aggserialfn: Option<&'a str>,
    pub aggdeserialfn: Option<&'a str>,
    pub aggmtransfn: Option<&'a str>,
    pub aggminvtransfn: Option<&'a str>,
    pub aggmfinalfn: Option<&'a str>,
    pub aggfinalextra: bool,
    pub aggmfinalextra: bool,
    pub aggfinalmodify: i8,
    pub aggmfinalmodify: i8,
    pub aggsortop: Option<&'a str>,
    pub aggtranstype: &'a str,
    pub aggmtranstype: Option<&'a str>,
    pub agginitval: Option<&'a str>,
    pub aggminitval: Option<&'a str>,
}
impl<'a> From<ReflectPgAggregateBorrowed<'a>> for ReflectPgAggregate {
    fn from(
        ReflectPgAggregateBorrowed {
            aggfnoid,
            aggkind,
            aggnumdirectargs,
            aggtransfn,
            aggfinalfn,
            aggcombinefn,
            aggserialfn,
            aggdeserialfn,
            aggmtransfn,
            aggminvtransfn,
            aggmfinalfn,
            aggfinalextra,
            aggmfinalextra,
            aggfinalmodify,
            aggmfinalmodify,
            aggsortop,
            aggtranstype,
            aggmtranstype,
            agginitval,
            aggminitval,
        }: ReflectPgAggregateBorrowed<'a>,
    ) -> Self {
        Self {
            aggfnoid: aggfnoid.into(),
            aggkind,
            aggnumdirectargs,
            aggtransfn: aggtransfn.into(),
            aggfinalfn: aggfinalfn.map(|v| v.into()),
            aggcombinefn: aggcombinefn.map(|v| v.into()),
            aggserialfn: aggserialfn.map(|v| v.into()),
            aggdeserialfn: aggdeserialfn.map(|v| v.into()),
            aggmtransfn: aggmtransfn.map(|v| v.into()),
            aggminvtransfn: aggminvtransfn.map(|v| v.into()),
            aggmfinalfn: aggmfinalfn.map(|v| v.into()),
            aggfinalextra,
            aggmfinalextra,
            aggfinalmodify,
            aggmfinalmodify,
            aggsortop: aggsortop.map(|v| v.into()),
            aggtranstype: aggtranstype.into(),
            aggmtranstype: aggmtranstype.map(|v| v.into()),
            agginitval: agginitval.map(|v| v.into()),
            aggminitval: aggminitval.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgAm {
    pub amname: String,
    pub amhandler: String,
    pub amtype: i8,
    pub description: Option<String>,
}
pub struct ReflectPgAmBorrowed<'a> {
    pub amname: &'a str,
    pub amhandler: &'a str,
    pub amtype: i8,
    pub description: Option<&'a str>,
}
impl<'a> From<ReflectPgAmBorrowed<'a>> for ReflectPgAm {
    fn from(
        ReflectPgAmBorrowed {
            amname,
            amhandler,
            amtype,
            description,
        }: ReflectPgAmBorrowed<'a>,
    ) -> Self {
        Self {
            amname: amname.into(),
            amhandler: amhandler.into(),
            amtype,
            description: description.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgAmop {
    pub amopfamily: String,
    pub amoplefttype: String,
    pub amoprighttype: String,
    pub amopstrategy: i16,
    pub amoppurpose: i8,
    pub amopopr: String,
    pub amopmethod: String,
    pub amopsortfamily: Option<String>,
}
pub struct ReflectPgAmopBorrowed<'a> {
    pub amopfamily: &'a str,
    pub amoplefttype: &'a str,
    pub amoprighttype: &'a str,
    pub amopstrategy: i16,
    pub amoppurpose: i8,
    pub amopopr: &'a str,
    pub amopmethod: &'a str,
    pub amopsortfamily: Option<&'a str>,
}
impl<'a> From<ReflectPgAmopBorrowed<'a>> for ReflectPgAmop {
    fn from(
        ReflectPgAmopBorrowed {
            amopfamily,
            amoplefttype,
            amoprighttype,
            amopstrategy,
            amoppurpose,
            amopopr,
            amopmethod,
            amopsortfamily,
        }: ReflectPgAmopBorrowed<'a>,
    ) -> Self {
        Self {
            amopfamily: amopfamily.into(),
            amoplefttype: amoplefttype.into(),
            amoprighttype: amoprighttype.into(),
            amopstrategy,
            amoppurpose,
            amopopr: amopopr.into(),
            amopmethod: amopmethod.into(),
            amopsortfamily: amopsortfamily.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgAmproc {
    pub amprocfamily: String,
    pub amproclefttype: String,
    pub amprocrighttype: String,
    pub amprocnum: i16,
    pub amproc: String,
}
pub struct ReflectPgAmprocBorrowed<'a> {
    pub amprocfamily: &'a str,
    pub amproclefttype: &'a str,
    pub amprocrighttype: &'a str,
    pub amprocnum: i16,
    pub amproc: &'a str,
}
impl<'a> From<ReflectPgAmprocBorrowed<'a>> for ReflectPgAmproc {
    fn from(
        ReflectPgAmprocBorrowed {
            amprocfamily,
            amproclefttype,
            amprocrighttype,
            amprocnum,
            amproc,
        }: ReflectPgAmprocBorrowed<'a>,
    ) -> Self {
        Self {
            amprocfamily: amprocfamily.into(),
            amproclefttype: amproclefttype.into(),
            amprocrighttype: amprocrighttype.into(),
            amprocnum,
            amproc: amproc.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgAttrdef {
    pub adrelid: String,
    pub adnum: i16,
    pub adbin: String,
}
pub struct ReflectPgAttrdefBorrowed<'a> {
    pub adrelid: &'a str,
    pub adnum: i16,
    pub adbin: &'a str,
}
impl<'a> From<ReflectPgAttrdefBorrowed<'a>> for ReflectPgAttrdef {
    fn from(
        ReflectPgAttrdefBorrowed {
            adrelid,
            adnum,
            adbin,
        }: ReflectPgAttrdefBorrowed<'a>,
    ) -> Self {
        Self {
            adrelid: adrelid.into(),
            adnum,
            adbin: adbin.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgAttribute {
    pub attrelid: String,
    pub attname: String,
    pub atttypid: String,
    pub attnum: i16,
    pub atttypmod: Option<i32>,
    pub attndims: i16,
    pub attcompression: Option<i8>,
    pub attnotnull: bool,
    pub atthasdef: bool,
    pub attidentity: Option<i8>,
    pub attgenerated: Option<i8>,
    pub attisdropped: bool,
    pub attislocal: bool,
    pub attinhcount: i16,
    pub attcollation: Option<String>,
    pub attstattarget: Option<i16>,
    pub attacl: Option<Vec<crate::types::pg_temp_1::TablecolumnAclitem>>,
    pub attoptions: Option<Vec<String>>,
    pub attfdwoptions: Option<Vec<String>>,
    pub description: Option<String>,
    pub seclabel: Option<String>,
    pub seclabel_provider: Option<String>,
    pub initprivs: Option<Vec<crate::types::pg_temp_1::TablecolumnAclitem>>,
    pub initprivs_type: Option<i8>,
}
pub struct ReflectPgAttributeBorrowed<'a> {
    pub attrelid: &'a str,
    pub attname: &'a str,
    pub atttypid: &'a str,
    pub attnum: i16,
    pub atttypmod: Option<i32>,
    pub attndims: i16,
    pub attcompression: Option<i8>,
    pub attnotnull: bool,
    pub atthasdef: bool,
    pub attidentity: Option<i8>,
    pub attgenerated: Option<i8>,
    pub attisdropped: bool,
    pub attislocal: bool,
    pub attinhcount: i16,
    pub attcollation: Option<&'a str>,
    pub attstattarget: Option<i16>,
    pub attacl:
        Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::TablecolumnAclitemBorrowed<'a>>>,
    pub attoptions: Option<crate::ArrayIterator<'a, &'a str>>,
    pub attfdwoptions: Option<crate::ArrayIterator<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub seclabel: Option<&'a str>,
    pub seclabel_provider: Option<&'a str>,
    pub initprivs:
        Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::TablecolumnAclitemBorrowed<'a>>>,
    pub initprivs_type: Option<i8>,
}
impl<'a> From<ReflectPgAttributeBorrowed<'a>> for ReflectPgAttribute {
    fn from(
        ReflectPgAttributeBorrowed {
            attrelid,
            attname,
            atttypid,
            attnum,
            atttypmod,
            attndims,
            attcompression,
            attnotnull,
            atthasdef,
            attidentity,
            attgenerated,
            attisdropped,
            attislocal,
            attinhcount,
            attcollation,
            attstattarget,
            attacl,
            attoptions,
            attfdwoptions,
            description,
            seclabel,
            seclabel_provider,
            initprivs,
            initprivs_type,
        }: ReflectPgAttributeBorrowed<'a>,
    ) -> Self {
        Self {
            attrelid: attrelid.into(),
            attname: attname.into(),
            atttypid: atttypid.into(),
            attnum,
            atttypmod,
            attndims,
            attcompression,
            attnotnull,
            atthasdef,
            attidentity,
            attgenerated,
            attisdropped,
            attislocal,
            attinhcount,
            attcollation: attcollation.map(|v| v.into()),
            attstattarget,
            attacl: attacl.map(|v| v.map(|v| v.into()).collect()),
            attoptions: attoptions.map(|v| v.map(|v| v.into()).collect()),
            attfdwoptions: attfdwoptions.map(|v| v.map(|v| v.into()).collect()),
            description: description.map(|v| v.into()),
            seclabel: seclabel.map(|v| v.into()),
            seclabel_provider: seclabel_provider.map(|v| v.into()),
            initprivs: initprivs.map(|v| v.map(|v| v.into()).collect()),
            initprivs_type,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgRoles {
    pub rolname: String,
    pub rolsuper: bool,
    pub rolinherit: bool,
    pub rolcreaterole: bool,
    pub rolcreatedb: bool,
    pub rolcanlogin: bool,
    pub rolreplication: bool,
    pub rolconnlimit: Option<i32>,
    pub rolvaliduntil: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub rolbypassrls: bool,
    pub rolconfig: Option<Vec<String>>,
    pub description: Option<String>,
    pub seclabel: Option<String>,
    pub seclabel_provider: Option<String>,
}
pub struct ReflectPgRolesBorrowed<'a> {
    pub rolname: &'a str,
    pub rolsuper: bool,
    pub rolinherit: bool,
    pub rolcreaterole: bool,
    pub rolcreatedb: bool,
    pub rolcanlogin: bool,
    pub rolreplication: bool,
    pub rolconnlimit: Option<i32>,
    pub rolvaliduntil: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub rolbypassrls: bool,
    pub rolconfig: Option<crate::ArrayIterator<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub seclabel: Option<&'a str>,
    pub seclabel_provider: Option<&'a str>,
}
impl<'a> From<ReflectPgRolesBorrowed<'a>> for ReflectPgRoles {
    fn from(
        ReflectPgRolesBorrowed {
            rolname,
            rolsuper,
            rolinherit,
            rolcreaterole,
            rolcreatedb,
            rolcanlogin,
            rolreplication,
            rolconnlimit,
            rolvaliduntil,
            rolbypassrls,
            rolconfig,
            description,
            seclabel,
            seclabel_provider,
        }: ReflectPgRolesBorrowed<'a>,
    ) -> Self {
        Self {
            rolname: rolname.into(),
            rolsuper,
            rolinherit,
            rolcreaterole,
            rolcreatedb,
            rolcanlogin,
            rolreplication,
            rolconnlimit,
            rolvaliduntil,
            rolbypassrls,
            rolconfig: rolconfig.map(|v| v.map(|v| v.into()).collect()),
            description: description.map(|v| v.into()),
            seclabel: seclabel.map(|v| v.into()),
            seclabel_provider: seclabel_provider.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgAuthMembers {
    pub roleid: String,
    pub member: String,
    pub grantor: String,
    pub admin_option: bool,
    pub inherit_option: bool,
    pub set_option: bool,
}
pub struct ReflectPgAuthMembersBorrowed<'a> {
    pub roleid: &'a str,
    pub member: &'a str,
    pub grantor: &'a str,
    pub admin_option: bool,
    pub inherit_option: bool,
    pub set_option: bool,
}
impl<'a> From<ReflectPgAuthMembersBorrowed<'a>> for ReflectPgAuthMembers {
    fn from(
        ReflectPgAuthMembersBorrowed {
            roleid,
            member,
            grantor,
            admin_option,
            inherit_option,
            set_option,
        }: ReflectPgAuthMembersBorrowed<'a>,
    ) -> Self {
        Self {
            roleid: roleid.into(),
            member: member.into(),
            grantor: grantor.into(),
            admin_option,
            inherit_option,
            set_option,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgCast {
    pub castsource: String,
    pub casttarget: String,
    pub castfunc: Option<String>,
    pub castcontext: i8,
    pub castmethod: i8,
    pub description: Option<String>,
}
pub struct ReflectPgCastBorrowed<'a> {
    pub castsource: &'a str,
    pub casttarget: &'a str,
    pub castfunc: Option<&'a str>,
    pub castcontext: i8,
    pub castmethod: i8,
    pub description: Option<&'a str>,
}
impl<'a> From<ReflectPgCastBorrowed<'a>> for ReflectPgCast {
    fn from(
        ReflectPgCastBorrowed {
            castsource,
            casttarget,
            castfunc,
            castcontext,
            castmethod,
            description,
        }: ReflectPgCastBorrowed<'a>,
    ) -> Self {
        Self {
            castsource: castsource.into(),
            casttarget: casttarget.into(),
            castfunc: castfunc.map(|v| v.into()),
            castcontext,
            castmethod,
            description: description.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgClass {
    pub oid: String,
    pub relname: String,
    pub relnamespace: String,
    pub reltype: Option<String>,
    pub reloftype: Option<String>,
    pub relowner: String,
    pub relam: Option<String>,
    pub relisshared: bool,
    pub relpersistence: i8,
    pub relkind: i8,
    pub relnatts: i16,
    pub relchecks: i16,
    pub relrowsecurity: bool,
    pub relforcerowsecurity: bool,
    pub relreplident: i8,
    pub relispartition: bool,
    pub relacl: Option<Vec<crate::types::pg_temp_1::TableAclitem>>,
    pub reloptions: Option<Vec<String>>,
    pub relpartbound: Option<String>,
    pub description: Option<String>,
    pub seclabel: Option<String>,
    pub seclabel_provider: Option<String>,
    pub initprivs: Option<Vec<crate::types::pg_temp_1::TableAclitem>>,
    pub initprivs_type: Option<i8>,
}
pub struct ReflectPgClassBorrowed<'a> {
    pub oid: &'a str,
    pub relname: &'a str,
    pub relnamespace: &'a str,
    pub reltype: Option<&'a str>,
    pub reloftype: Option<&'a str>,
    pub relowner: &'a str,
    pub relam: Option<&'a str>,
    pub relisshared: bool,
    pub relpersistence: i8,
    pub relkind: i8,
    pub relnatts: i16,
    pub relchecks: i16,
    pub relrowsecurity: bool,
    pub relforcerowsecurity: bool,
    pub relreplident: i8,
    pub relispartition: bool,
    pub relacl: Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::TableAclitemBorrowed<'a>>>,
    pub reloptions: Option<crate::ArrayIterator<'a, &'a str>>,
    pub relpartbound: Option<&'a str>,
    pub description: Option<&'a str>,
    pub seclabel: Option<&'a str>,
    pub seclabel_provider: Option<&'a str>,
    pub initprivs:
        Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::TableAclitemBorrowed<'a>>>,
    pub initprivs_type: Option<i8>,
}
impl<'a> From<ReflectPgClassBorrowed<'a>> for ReflectPgClass {
    fn from(
        ReflectPgClassBorrowed {
            oid,
            relname,
            relnamespace,
            reltype,
            reloftype,
            relowner,
            relam,
            relisshared,
            relpersistence,
            relkind,
            relnatts,
            relchecks,
            relrowsecurity,
            relforcerowsecurity,
            relreplident,
            relispartition,
            relacl,
            reloptions,
            relpartbound,
            description,
            seclabel,
            seclabel_provider,
            initprivs,
            initprivs_type,
        }: ReflectPgClassBorrowed<'a>,
    ) -> Self {
        Self {
            oid: oid.into(),
            relname: relname.into(),
            relnamespace: relnamespace.into(),
            reltype: reltype.map(|v| v.into()),
            reloftype: reloftype.map(|v| v.into()),
            relowner: relowner.into(),
            relam: relam.map(|v| v.into()),
            relisshared,
            relpersistence,
            relkind,
            relnatts,
            relchecks,
            relrowsecurity,
            relforcerowsecurity,
            relreplident,
            relispartition,
            relacl: relacl.map(|v| v.map(|v| v.into()).collect()),
            reloptions: reloptions.map(|v| v.map(|v| v.into()).collect()),
            relpartbound: relpartbound.map(|v| v.into()),
            description: description.map(|v| v.into()),
            seclabel: seclabel.map(|v| v.into()),
            seclabel_provider: seclabel_provider.map(|v| v.into()),
            initprivs: initprivs.map(|v| v.map(|v| v.into()).collect()),
            initprivs_type,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgCollation {
    pub oid: String,
    pub collname: String,
    pub collnamespace: String,
    pub collowner: String,
    pub collprovider: i8,
    pub collisdeterministic: bool,
    pub collencoding: Option<String>,
    pub collcollate: Option<String>,
    pub collctype: Option<String>,
    pub colllocale: Option<String>,
    pub collicurules: Option<String>,
    pub description: Option<String>,
}
pub struct ReflectPgCollationBorrowed<'a> {
    pub oid: &'a str,
    pub collname: &'a str,
    pub collnamespace: &'a str,
    pub collowner: &'a str,
    pub collprovider: i8,
    pub collisdeterministic: bool,
    pub collencoding: Option<&'a str>,
    pub collcollate: Option<&'a str>,
    pub collctype: Option<&'a str>,
    pub colllocale: Option<&'a str>,
    pub collicurules: Option<&'a str>,
    pub description: Option<&'a str>,
}
impl<'a> From<ReflectPgCollationBorrowed<'a>> for ReflectPgCollation {
    fn from(
        ReflectPgCollationBorrowed {
            oid,
            collname,
            collnamespace,
            collowner,
            collprovider,
            collisdeterministic,
            collencoding,
            collcollate,
            collctype,
            colllocale,
            collicurules,
            description,
        }: ReflectPgCollationBorrowed<'a>,
    ) -> Self {
        Self {
            oid: oid.into(),
            collname: collname.into(),
            collnamespace: collnamespace.into(),
            collowner: collowner.into(),
            collprovider,
            collisdeterministic,
            collencoding: collencoding.map(|v| v.into()),
            collcollate: collcollate.map(|v| v.into()),
            collctype: collctype.map(|v| v.into()),
            colllocale: colllocale.map(|v| v.into()),
            collicurules: collicurules.map(|v| v.into()),
            description: description.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgConstraint {
    pub conname: String,
    pub connamespace: String,
    pub contype: i8,
    pub condeferrable: bool,
    pub condeferred: bool,
    pub convalidated: bool,
    pub conrelid: Option<String>,
    pub contypid: Option<String>,
    pub conindid: Option<String>,
    pub conparentid: Option<String>,
    pub confrelid: Option<String>,
    pub confupdtype: Option<i8>,
    pub confdeltype: Option<i8>,
    pub confmatchtype: Option<i8>,
    pub conislocal: bool,
    pub coninhcount: i16,
    pub connoinherit: bool,
    pub conkey: Option<Vec<i16>>,
    pub confkey: Option<Vec<i16>>,
    pub conpfeqop: Option<Vec<String>>,
    pub conppeqop: Option<Vec<String>>,
    pub conffeqop: Option<Vec<String>>,
    pub confdelsetcols: Option<Vec<i16>>,
    pub conexclop: Option<Vec<String>>,
    pub conbin: Option<String>,
    pub description: Option<String>,
}
pub struct ReflectPgConstraintBorrowed<'a> {
    pub conname: &'a str,
    pub connamespace: &'a str,
    pub contype: i8,
    pub condeferrable: bool,
    pub condeferred: bool,
    pub convalidated: bool,
    pub conrelid: Option<&'a str>,
    pub contypid: Option<&'a str>,
    pub conindid: Option<&'a str>,
    pub conparentid: Option<&'a str>,
    pub confrelid: Option<&'a str>,
    pub confupdtype: Option<i8>,
    pub confdeltype: Option<i8>,
    pub confmatchtype: Option<i8>,
    pub conislocal: bool,
    pub coninhcount: i16,
    pub connoinherit: bool,
    pub conkey: Option<crate::ArrayIterator<'a, i16>>,
    pub confkey: Option<crate::ArrayIterator<'a, i16>>,
    pub conpfeqop: Option<crate::ArrayIterator<'a, &'a str>>,
    pub conppeqop: Option<crate::ArrayIterator<'a, &'a str>>,
    pub conffeqop: Option<crate::ArrayIterator<'a, &'a str>>,
    pub confdelsetcols: Option<crate::ArrayIterator<'a, i16>>,
    pub conexclop: Option<crate::ArrayIterator<'a, &'a str>>,
    pub conbin: Option<&'a str>,
    pub description: Option<&'a str>,
}
impl<'a> From<ReflectPgConstraintBorrowed<'a>> for ReflectPgConstraint {
    fn from(
        ReflectPgConstraintBorrowed {
            conname,
            connamespace,
            contype,
            condeferrable,
            condeferred,
            convalidated,
            conrelid,
            contypid,
            conindid,
            conparentid,
            confrelid,
            confupdtype,
            confdeltype,
            confmatchtype,
            conislocal,
            coninhcount,
            connoinherit,
            conkey,
            confkey,
            conpfeqop,
            conppeqop,
            conffeqop,
            confdelsetcols,
            conexclop,
            conbin,
            description,
        }: ReflectPgConstraintBorrowed<'a>,
    ) -> Self {
        Self {
            conname: conname.into(),
            connamespace: connamespace.into(),
            contype,
            condeferrable,
            condeferred,
            convalidated,
            conrelid: conrelid.map(|v| v.into()),
            contypid: contypid.map(|v| v.into()),
            conindid: conindid.map(|v| v.into()),
            conparentid: conparentid.map(|v| v.into()),
            confrelid: confrelid.map(|v| v.into()),
            confupdtype,
            confdeltype,
            confmatchtype,
            conislocal,
            coninhcount,
            connoinherit,
            conkey: conkey.map(|v| v.map(|v| v).collect()),
            confkey: confkey.map(|v| v.map(|v| v).collect()),
            conpfeqop: conpfeqop.map(|v| v.map(|v| v.into()).collect()),
            conppeqop: conppeqop.map(|v| v.map(|v| v.into()).collect()),
            conffeqop: conffeqop.map(|v| v.map(|v| v.into()).collect()),
            confdelsetcols: confdelsetcols.map(|v| v.map(|v| v).collect()),
            conexclop: conexclop.map(|v| v.map(|v| v.into()).collect()),
            conbin: conbin.map(|v| v.into()),
            description: description.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgConversion {
    pub conname: String,
    pub connamespace: String,
    pub conowner: String,
    pub conforencoding: String,
    pub contoencoding: String,
    pub conproc: String,
    pub condefault: bool,
    pub description: Option<String>,
}
pub struct ReflectPgConversionBorrowed<'a> {
    pub conname: &'a str,
    pub connamespace: &'a str,
    pub conowner: &'a str,
    pub conforencoding: &'a str,
    pub contoencoding: &'a str,
    pub conproc: &'a str,
    pub condefault: bool,
    pub description: Option<&'a str>,
}
impl<'a> From<ReflectPgConversionBorrowed<'a>> for ReflectPgConversion {
    fn from(
        ReflectPgConversionBorrowed {
            conname,
            connamespace,
            conowner,
            conforencoding,
            contoencoding,
            conproc,
            condefault,
            description,
        }: ReflectPgConversionBorrowed<'a>,
    ) -> Self {
        Self {
            conname: conname.into(),
            connamespace: connamespace.into(),
            conowner: conowner.into(),
            conforencoding: conforencoding.into(),
            contoencoding: contoencoding.into(),
            conproc: conproc.into(),
            condefault,
            description: description.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgDatabase {
    pub datname: String,
    pub datdba: String,
    pub encoding: String,
    pub datlocprovider: i8,
    pub datistemplate: bool,
    pub datallowconn: bool,
    pub datconnlimit: Option<i32>,
    pub datcollate: Option<String>,
    pub datctype: Option<String>,
    pub datlocale: Option<String>,
    pub daticurules: Option<String>,
    pub datacl: Option<Vec<crate::types::pg_temp_1::DbAclitem>>,
    pub description: Option<String>,
    pub seclabel: Option<String>,
    pub seclabel_provider: Option<String>,
}
pub struct ReflectPgDatabaseBorrowed<'a> {
    pub datname: &'a str,
    pub datdba: &'a str,
    pub encoding: &'a str,
    pub datlocprovider: i8,
    pub datistemplate: bool,
    pub datallowconn: bool,
    pub datconnlimit: Option<i32>,
    pub datcollate: Option<&'a str>,
    pub datctype: Option<&'a str>,
    pub datlocale: Option<&'a str>,
    pub daticurules: Option<&'a str>,
    pub datacl: Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::DbAclitemBorrowed<'a>>>,
    pub description: Option<&'a str>,
    pub seclabel: Option<&'a str>,
    pub seclabel_provider: Option<&'a str>,
}
impl<'a> From<ReflectPgDatabaseBorrowed<'a>> for ReflectPgDatabase {
    fn from(
        ReflectPgDatabaseBorrowed {
            datname,
            datdba,
            encoding,
            datlocprovider,
            datistemplate,
            datallowconn,
            datconnlimit,
            datcollate,
            datctype,
            datlocale,
            daticurules,
            datacl,
            description,
            seclabel,
            seclabel_provider,
        }: ReflectPgDatabaseBorrowed<'a>,
    ) -> Self {
        Self {
            datname: datname.into(),
            datdba: datdba.into(),
            encoding: encoding.into(),
            datlocprovider,
            datistemplate,
            datallowconn,
            datconnlimit,
            datcollate: datcollate.map(|v| v.into()),
            datctype: datctype.map(|v| v.into()),
            datlocale: datlocale.map(|v| v.into()),
            daticurules: daticurules.map(|v| v.into()),
            datacl: datacl.map(|v| v.map(|v| v.into()).collect()),
            description: description.map(|v| v.into()),
            seclabel: seclabel.map(|v| v.into()),
            seclabel_provider: seclabel_provider.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgDefaultAcl {
    pub defaclrole: String,
    pub defaclnamespace: Option<String>,
    pub defaclobjtype: i8,
    pub defaclacl: Option<Vec<crate::types::pg_temp_1::AcldefaultAclitem>>,
}
pub struct ReflectPgDefaultAclBorrowed<'a> {
    pub defaclrole: &'a str,
    pub defaclnamespace: Option<&'a str>,
    pub defaclobjtype: i8,
    pub defaclacl:
        Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::AcldefaultAclitemBorrowed<'a>>>,
}
impl<'a> From<ReflectPgDefaultAclBorrowed<'a>> for ReflectPgDefaultAcl {
    fn from(
        ReflectPgDefaultAclBorrowed {
            defaclrole,
            defaclnamespace,
            defaclobjtype,
            defaclacl,
        }: ReflectPgDefaultAclBorrowed<'a>,
    ) -> Self {
        Self {
            defaclrole: defaclrole.into(),
            defaclnamespace: defaclnamespace.map(|v| v.into()),
            defaclobjtype,
            defaclacl: defaclacl.map(|v| v.map(|v| v.into()).collect()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgEventTrigger {
    pub evtname: String,
    pub evtevent: String,
    pub evtowner: String,
    pub evtfoid: String,
    pub evtenabled: i8,
    pub evttags: Option<Vec<String>>,
    pub description: Option<String>,
    pub seclabel: Option<String>,
    pub seclabel_provider: Option<String>,
}
pub struct ReflectPgEventTriggerBorrowed<'a> {
    pub evtname: &'a str,
    pub evtevent: &'a str,
    pub evtowner: &'a str,
    pub evtfoid: &'a str,
    pub evtenabled: i8,
    pub evttags: Option<crate::ArrayIterator<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub seclabel: Option<&'a str>,
    pub seclabel_provider: Option<&'a str>,
}
impl<'a> From<ReflectPgEventTriggerBorrowed<'a>> for ReflectPgEventTrigger {
    fn from(
        ReflectPgEventTriggerBorrowed {
            evtname,
            evtevent,
            evtowner,
            evtfoid,
            evtenabled,
            evttags,
            description,
            seclabel,
            seclabel_provider,
        }: ReflectPgEventTriggerBorrowed<'a>,
    ) -> Self {
        Self {
            evtname: evtname.into(),
            evtevent: evtevent.into(),
            evtowner: evtowner.into(),
            evtfoid: evtfoid.into(),
            evtenabled,
            evttags: evttags.map(|v| v.map(|v| v.into()).collect()),
            description: description.map(|v| v.into()),
            seclabel: seclabel.map(|v| v.into()),
            seclabel_provider: seclabel_provider.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgExtension {
    pub extname: String,
    pub extowner: String,
    pub extnamespace: String,
    pub extrelocatable: bool,
    pub extversion: String,
    pub extconfig: Option<Vec<String>>,
    pub extcondition: Option<Vec<String>>,
    pub description: Option<String>,
}
pub struct ReflectPgExtensionBorrowed<'a> {
    pub extname: &'a str,
    pub extowner: &'a str,
    pub extnamespace: &'a str,
    pub extrelocatable: bool,
    pub extversion: &'a str,
    pub extconfig: Option<crate::ArrayIterator<'a, &'a str>>,
    pub extcondition: Option<crate::ArrayIterator<'a, &'a str>>,
    pub description: Option<&'a str>,
}
impl<'a> From<ReflectPgExtensionBorrowed<'a>> for ReflectPgExtension {
    fn from(
        ReflectPgExtensionBorrowed {
            extname,
            extowner,
            extnamespace,
            extrelocatable,
            extversion,
            extconfig,
            extcondition,
            description,
        }: ReflectPgExtensionBorrowed<'a>,
    ) -> Self {
        Self {
            extname: extname.into(),
            extowner: extowner.into(),
            extnamespace: extnamespace.into(),
            extrelocatable,
            extversion: extversion.into(),
            extconfig: extconfig.map(|v| v.map(|v| v.into()).collect()),
            extcondition: extcondition.map(|v| v.map(|v| v.into()).collect()),
            description: description.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgForeignDataWrapper {
    pub fdwname: String,
    pub fdwowner: String,
    pub fdwhandler: Option<String>,
    pub fdwvalidator: Option<String>,
    pub fdwacl: Option<Vec<crate::types::pg_temp_1::ForeigndatawrapperAclitem>>,
    pub fdwoptions: Option<Vec<String>>,
    pub description: Option<String>,
    pub initprivs: Option<Vec<crate::types::pg_temp_1::ForeigndatawrapperAclitem>>,
    pub initprivs_type: Option<i8>,
}
pub struct ReflectPgForeignDataWrapperBorrowed<'a> {
    pub fdwname: &'a str,
    pub fdwowner: &'a str,
    pub fdwhandler: Option<&'a str>,
    pub fdwvalidator: Option<&'a str>,
    pub fdwacl: Option<
        crate::ArrayIterator<'a, crate::types::pg_temp_1::ForeigndatawrapperAclitemBorrowed<'a>>,
    >,
    pub fdwoptions: Option<crate::ArrayIterator<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub initprivs: Option<
        crate::ArrayIterator<'a, crate::types::pg_temp_1::ForeigndatawrapperAclitemBorrowed<'a>>,
    >,
    pub initprivs_type: Option<i8>,
}
impl<'a> From<ReflectPgForeignDataWrapperBorrowed<'a>> for ReflectPgForeignDataWrapper {
    fn from(
        ReflectPgForeignDataWrapperBorrowed {
            fdwname,
            fdwowner,
            fdwhandler,
            fdwvalidator,
            fdwacl,
            fdwoptions,
            description,
            initprivs,
            initprivs_type,
        }: ReflectPgForeignDataWrapperBorrowed<'a>,
    ) -> Self {
        Self {
            fdwname: fdwname.into(),
            fdwowner: fdwowner.into(),
            fdwhandler: fdwhandler.map(|v| v.into()),
            fdwvalidator: fdwvalidator.map(|v| v.into()),
            fdwacl: fdwacl.map(|v| v.map(|v| v.into()).collect()),
            fdwoptions: fdwoptions.map(|v| v.map(|v| v.into()).collect()),
            description: description.map(|v| v.into()),
            initprivs: initprivs.map(|v| v.map(|v| v.into()).collect()),
            initprivs_type,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgForeignServer {
    pub srvname: String,
    pub srvowner: String,
    pub srvfdw: String,
    pub srvtype: Option<String>,
    pub srvversion: Option<String>,
    pub srvacl: Option<Vec<crate::types::pg_temp_1::ForeignserverAclitem>>,
    pub srvoptions: Option<Vec<String>>,
    pub description: Option<String>,
    pub initprivs: Option<Vec<crate::types::pg_temp_1::ForeignserverAclitem>>,
    pub initprivs_type: Option<i8>,
}
pub struct ReflectPgForeignServerBorrowed<'a> {
    pub srvname: &'a str,
    pub srvowner: &'a str,
    pub srvfdw: &'a str,
    pub srvtype: Option<&'a str>,
    pub srvversion: Option<&'a str>,
    pub srvacl:
        Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::ForeignserverAclitemBorrowed<'a>>>,
    pub srvoptions: Option<crate::ArrayIterator<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub initprivs:
        Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::ForeignserverAclitemBorrowed<'a>>>,
    pub initprivs_type: Option<i8>,
}
impl<'a> From<ReflectPgForeignServerBorrowed<'a>> for ReflectPgForeignServer {
    fn from(
        ReflectPgForeignServerBorrowed {
            srvname,
            srvowner,
            srvfdw,
            srvtype,
            srvversion,
            srvacl,
            srvoptions,
            description,
            initprivs,
            initprivs_type,
        }: ReflectPgForeignServerBorrowed<'a>,
    ) -> Self {
        Self {
            srvname: srvname.into(),
            srvowner: srvowner.into(),
            srvfdw: srvfdw.into(),
            srvtype: srvtype.map(|v| v.into()),
            srvversion: srvversion.map(|v| v.into()),
            srvacl: srvacl.map(|v| v.map(|v| v.into()).collect()),
            srvoptions: srvoptions.map(|v| v.map(|v| v.into()).collect()),
            description: description.map(|v| v.into()),
            initprivs: initprivs.map(|v| v.map(|v| v.into()).collect()),
            initprivs_type,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgForeignTable {
    pub ftrelid: String,
    pub ftserver: String,
    pub ftoptions: Option<Vec<String>>,
}
pub struct ReflectPgForeignTableBorrowed<'a> {
    pub ftrelid: &'a str,
    pub ftserver: &'a str,
    pub ftoptions: Option<crate::ArrayIterator<'a, &'a str>>,
}
impl<'a> From<ReflectPgForeignTableBorrowed<'a>> for ReflectPgForeignTable {
    fn from(
        ReflectPgForeignTableBorrowed {
            ftrelid,
            ftserver,
            ftoptions,
        }: ReflectPgForeignTableBorrowed<'a>,
    ) -> Self {
        Self {
            ftrelid: ftrelid.into(),
            ftserver: ftserver.into(),
            ftoptions: ftoptions.map(|v| v.map(|v| v.into()).collect()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgIndex {
    pub indexrelid: String,
    pub indrelid: String,
    pub indnatts: i16,
    pub indnkeyatts: i16,
    pub indisunique: bool,
    pub indnullsnotdistinct: bool,
    pub indisprimary: bool,
    pub indisexclusion: bool,
    pub indimmediate: bool,
    pub indisreplident: bool,
    pub indkey: Vec<i16>,
    pub indcollation: Vec<Option<String>>,
    pub indclass: Vec<String>,
    pub indoption: Vec<i16>,
    pub indexprs: Option<String>,
    pub indpred: Option<String>,
}
pub struct ReflectPgIndexBorrowed<'a> {
    pub indexrelid: &'a str,
    pub indrelid: &'a str,
    pub indnatts: i16,
    pub indnkeyatts: i16,
    pub indisunique: bool,
    pub indnullsnotdistinct: bool,
    pub indisprimary: bool,
    pub indisexclusion: bool,
    pub indimmediate: bool,
    pub indisreplident: bool,
    pub indkey: crate::ArrayIterator<'a, i16>,
    pub indcollation: crate::ArrayIterator<'a, Option<&'a str>>,
    pub indclass: crate::ArrayIterator<'a, &'a str>,
    pub indoption: crate::ArrayIterator<'a, i16>,
    pub indexprs: Option<&'a str>,
    pub indpred: Option<&'a str>,
}
impl<'a> From<ReflectPgIndexBorrowed<'a>> for ReflectPgIndex {
    fn from(
        ReflectPgIndexBorrowed {
            indexrelid,
            indrelid,
            indnatts,
            indnkeyatts,
            indisunique,
            indnullsnotdistinct,
            indisprimary,
            indisexclusion,
            indimmediate,
            indisreplident,
            indkey,
            indcollation,
            indclass,
            indoption,
            indexprs,
            indpred,
        }: ReflectPgIndexBorrowed<'a>,
    ) -> Self {
        Self {
            indexrelid: indexrelid.into(),
            indrelid: indrelid.into(),
            indnatts,
            indnkeyatts,
            indisunique,
            indnullsnotdistinct,
            indisprimary,
            indisexclusion,
            indimmediate,
            indisreplident,
            indkey: indkey.map(|v| v).collect(),
            indcollation: indcollation.map(|v| v.map(|v| v.into())).collect(),
            indclass: indclass.map(|v| v.into()).collect(),
            indoption: indoption.map(|v| v).collect(),
            indexprs: indexprs.map(|v| v.into()),
            indpred: indpred.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgInherits {
    pub inhrelid: String,
    pub inhparent: String,
    pub inhseqno: i32,
}
pub struct ReflectPgInheritsBorrowed<'a> {
    pub inhrelid: &'a str,
    pub inhparent: &'a str,
    pub inhseqno: i32,
}
impl<'a> From<ReflectPgInheritsBorrowed<'a>> for ReflectPgInherits {
    fn from(
        ReflectPgInheritsBorrowed {
            inhrelid,
            inhparent,
            inhseqno,
        }: ReflectPgInheritsBorrowed<'a>,
    ) -> Self {
        Self {
            inhrelid: inhrelid.into(),
            inhparent: inhparent.into(),
            inhseqno,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgLanguage {
    pub lanname: String,
    pub lanowner: String,
    pub lanispl: bool,
    pub lanpltrusted: bool,
    pub lanplcallfoid: Option<String>,
    pub laninline: Option<String>,
    pub lanvalidator: Option<String>,
    pub lanacl: Option<Vec<crate::types::pg_temp_1::LanguageAclitem>>,
    pub description: Option<String>,
    pub seclabel: Option<String>,
    pub seclabel_provider: Option<String>,
    pub initprivs: Option<Vec<crate::types::pg_temp_1::LanguageAclitem>>,
    pub initprivs_type: Option<i8>,
}
pub struct ReflectPgLanguageBorrowed<'a> {
    pub lanname: &'a str,
    pub lanowner: &'a str,
    pub lanispl: bool,
    pub lanpltrusted: bool,
    pub lanplcallfoid: Option<&'a str>,
    pub laninline: Option<&'a str>,
    pub lanvalidator: Option<&'a str>,
    pub lanacl:
        Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::LanguageAclitemBorrowed<'a>>>,
    pub description: Option<&'a str>,
    pub seclabel: Option<&'a str>,
    pub seclabel_provider: Option<&'a str>,
    pub initprivs:
        Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::LanguageAclitemBorrowed<'a>>>,
    pub initprivs_type: Option<i8>,
}
impl<'a> From<ReflectPgLanguageBorrowed<'a>> for ReflectPgLanguage {
    fn from(
        ReflectPgLanguageBorrowed {
            lanname,
            lanowner,
            lanispl,
            lanpltrusted,
            lanplcallfoid,
            laninline,
            lanvalidator,
            lanacl,
            description,
            seclabel,
            seclabel_provider,
            initprivs,
            initprivs_type,
        }: ReflectPgLanguageBorrowed<'a>,
    ) -> Self {
        Self {
            lanname: lanname.into(),
            lanowner: lanowner.into(),
            lanispl,
            lanpltrusted,
            lanplcallfoid: lanplcallfoid.map(|v| v.into()),
            laninline: laninline.map(|v| v.into()),
            lanvalidator: lanvalidator.map(|v| v.into()),
            lanacl: lanacl.map(|v| v.map(|v| v.into()).collect()),
            description: description.map(|v| v.into()),
            seclabel: seclabel.map(|v| v.into()),
            seclabel_provider: seclabel_provider.map(|v| v.into()),
            initprivs: initprivs.map(|v| v.map(|v| v.into()).collect()),
            initprivs_type,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgNamespace {
    pub nspname: String,
    pub nspowner: String,
    pub nspacl: Option<Vec<crate::types::pg_temp_1::SchemaAclitem>>,
    pub description: Option<String>,
    pub seclabel: Option<String>,
    pub seclabel_provider: Option<String>,
    pub initprivs: Option<Vec<crate::types::pg_temp_1::SchemaAclitem>>,
    pub initprivs_type: Option<i8>,
}
pub struct ReflectPgNamespaceBorrowed<'a> {
    pub nspname: &'a str,
    pub nspowner: &'a str,
    pub nspacl:
        Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::SchemaAclitemBorrowed<'a>>>,
    pub description: Option<&'a str>,
    pub seclabel: Option<&'a str>,
    pub seclabel_provider: Option<&'a str>,
    pub initprivs:
        Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::SchemaAclitemBorrowed<'a>>>,
    pub initprivs_type: Option<i8>,
}
impl<'a> From<ReflectPgNamespaceBorrowed<'a>> for ReflectPgNamespace {
    fn from(
        ReflectPgNamespaceBorrowed {
            nspname,
            nspowner,
            nspacl,
            description,
            seclabel,
            seclabel_provider,
            initprivs,
            initprivs_type,
        }: ReflectPgNamespaceBorrowed<'a>,
    ) -> Self {
        Self {
            nspname: nspname.into(),
            nspowner: nspowner.into(),
            nspacl: nspacl.map(|v| v.map(|v| v.into()).collect()),
            description: description.map(|v| v.into()),
            seclabel: seclabel.map(|v| v.into()),
            seclabel_provider: seclabel_provider.map(|v| v.into()),
            initprivs: initprivs.map(|v| v.map(|v| v.into()).collect()),
            initprivs_type,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgOpclass {
    pub opcmethod: String,
    pub opcname: String,
    pub opcnamespace: String,
    pub opcowner: String,
    pub opcfamily: String,
    pub opcintype: String,
    pub opcdefault: bool,
    pub opckeytype: Option<String>,
    pub description: Option<String>,
}
pub struct ReflectPgOpclassBorrowed<'a> {
    pub opcmethod: &'a str,
    pub opcname: &'a str,
    pub opcnamespace: &'a str,
    pub opcowner: &'a str,
    pub opcfamily: &'a str,
    pub opcintype: &'a str,
    pub opcdefault: bool,
    pub opckeytype: Option<&'a str>,
    pub description: Option<&'a str>,
}
impl<'a> From<ReflectPgOpclassBorrowed<'a>> for ReflectPgOpclass {
    fn from(
        ReflectPgOpclassBorrowed {
            opcmethod,
            opcname,
            opcnamespace,
            opcowner,
            opcfamily,
            opcintype,
            opcdefault,
            opckeytype,
            description,
        }: ReflectPgOpclassBorrowed<'a>,
    ) -> Self {
        Self {
            opcmethod: opcmethod.into(),
            opcname: opcname.into(),
            opcnamespace: opcnamespace.into(),
            opcowner: opcowner.into(),
            opcfamily: opcfamily.into(),
            opcintype: opcintype.into(),
            opcdefault,
            opckeytype: opckeytype.map(|v| v.into()),
            description: description.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgOperator {
    pub oid: String,
    pub oprname: String,
    pub oprnamespace: String,
    pub oprowner: String,
    pub oprkind: i8,
    pub oprcanmerge: bool,
    pub oprcanhash: bool,
    pub oprleft: Option<String>,
    pub oprright: String,
    pub oprresult: Option<String>,
    pub oprcom: Option<String>,
    pub oprnegate: Option<String>,
    pub oprcode: Option<String>,
    pub oprrest: Option<String>,
    pub oprjoin: Option<String>,
    pub description: Option<String>,
}
pub struct ReflectPgOperatorBorrowed<'a> {
    pub oid: &'a str,
    pub oprname: &'a str,
    pub oprnamespace: &'a str,
    pub oprowner: &'a str,
    pub oprkind: i8,
    pub oprcanmerge: bool,
    pub oprcanhash: bool,
    pub oprleft: Option<&'a str>,
    pub oprright: &'a str,
    pub oprresult: Option<&'a str>,
    pub oprcom: Option<&'a str>,
    pub oprnegate: Option<&'a str>,
    pub oprcode: Option<&'a str>,
    pub oprrest: Option<&'a str>,
    pub oprjoin: Option<&'a str>,
    pub description: Option<&'a str>,
}
impl<'a> From<ReflectPgOperatorBorrowed<'a>> for ReflectPgOperator {
    fn from(
        ReflectPgOperatorBorrowed {
            oid,
            oprname,
            oprnamespace,
            oprowner,
            oprkind,
            oprcanmerge,
            oprcanhash,
            oprleft,
            oprright,
            oprresult,
            oprcom,
            oprnegate,
            oprcode,
            oprrest,
            oprjoin,
            description,
        }: ReflectPgOperatorBorrowed<'a>,
    ) -> Self {
        Self {
            oid: oid.into(),
            oprname: oprname.into(),
            oprnamespace: oprnamespace.into(),
            oprowner: oprowner.into(),
            oprkind,
            oprcanmerge,
            oprcanhash,
            oprleft: oprleft.map(|v| v.into()),
            oprright: oprright.into(),
            oprresult: oprresult.map(|v| v.into()),
            oprcom: oprcom.map(|v| v.into()),
            oprnegate: oprnegate.map(|v| v.into()),
            oprcode: oprcode.map(|v| v.into()),
            oprrest: oprrest.map(|v| v.into()),
            oprjoin: oprjoin.map(|v| v.into()),
            description: description.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgOpfamily {
    pub opfmethod: String,
    pub opfname: String,
    pub opfnamespace: String,
    pub opfowner: String,
    pub description: Option<String>,
}
pub struct ReflectPgOpfamilyBorrowed<'a> {
    pub opfmethod: &'a str,
    pub opfname: &'a str,
    pub opfnamespace: &'a str,
    pub opfowner: &'a str,
    pub description: Option<&'a str>,
}
impl<'a> From<ReflectPgOpfamilyBorrowed<'a>> for ReflectPgOpfamily {
    fn from(
        ReflectPgOpfamilyBorrowed {
            opfmethod,
            opfname,
            opfnamespace,
            opfowner,
            description,
        }: ReflectPgOpfamilyBorrowed<'a>,
    ) -> Self {
        Self {
            opfmethod: opfmethod.into(),
            opfname: opfname.into(),
            opfnamespace: opfnamespace.into(),
            opfowner: opfowner.into(),
            description: description.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgParameterAcl {
    pub parname: String,
    pub paracl: Option<Vec<crate::types::pg_temp_1::ParameterAclitem>>,
    pub initprivs: Option<Vec<crate::types::pg_temp_1::ParameterAclitem>>,
    pub initprivs_type: Option<i8>,
}
pub struct ReflectPgParameterAclBorrowed<'a> {
    pub parname: &'a str,
    pub paracl:
        Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::ParameterAclitemBorrowed<'a>>>,
    pub initprivs:
        Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::ParameterAclitemBorrowed<'a>>>,
    pub initprivs_type: Option<i8>,
}
impl<'a> From<ReflectPgParameterAclBorrowed<'a>> for ReflectPgParameterAcl {
    fn from(
        ReflectPgParameterAclBorrowed {
            parname,
            paracl,
            initprivs,
            initprivs_type,
        }: ReflectPgParameterAclBorrowed<'a>,
    ) -> Self {
        Self {
            parname: parname.into(),
            paracl: paracl.map(|v| v.map(|v| v.into()).collect()),
            initprivs: initprivs.map(|v| v.map(|v| v.into()).collect()),
            initprivs_type,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgPartitionedTable {
    pub partrelid: String,
    pub partstrat: i8,
    pub partnatts: i16,
    pub partdefid: Option<String>,
    pub partattrs: Vec<i16>,
    pub partclass: Vec<String>,
    pub partcollation: Vec<Option<String>>,
    pub partexprs: Option<String>,
}
pub struct ReflectPgPartitionedTableBorrowed<'a> {
    pub partrelid: &'a str,
    pub partstrat: i8,
    pub partnatts: i16,
    pub partdefid: Option<&'a str>,
    pub partattrs: crate::ArrayIterator<'a, i16>,
    pub partclass: crate::ArrayIterator<'a, &'a str>,
    pub partcollation: crate::ArrayIterator<'a, Option<&'a str>>,
    pub partexprs: Option<&'a str>,
}
impl<'a> From<ReflectPgPartitionedTableBorrowed<'a>> for ReflectPgPartitionedTable {
    fn from(
        ReflectPgPartitionedTableBorrowed {
            partrelid,
            partstrat,
            partnatts,
            partdefid,
            partattrs,
            partclass,
            partcollation,
            partexprs,
        }: ReflectPgPartitionedTableBorrowed<'a>,
    ) -> Self {
        Self {
            partrelid: partrelid.into(),
            partstrat,
            partnatts,
            partdefid: partdefid.map(|v| v.into()),
            partattrs: partattrs.map(|v| v).collect(),
            partclass: partclass.map(|v| v.into()).collect(),
            partcollation: partcollation.map(|v| v.map(|v| v.into())).collect(),
            partexprs: partexprs.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgPolicy {
    pub polname: String,
    pub polrelid: String,
    pub polcmd: i8,
    pub polpermissive: bool,
    pub polroles: Vec<Option<String>>,
    pub polqual: Option<String>,
    pub polwithcheck: Option<String>,
    pub description: Option<String>,
}
pub struct ReflectPgPolicyBorrowed<'a> {
    pub polname: &'a str,
    pub polrelid: &'a str,
    pub polcmd: i8,
    pub polpermissive: bool,
    pub polroles: crate::ArrayIterator<'a, Option<&'a str>>,
    pub polqual: Option<&'a str>,
    pub polwithcheck: Option<&'a str>,
    pub description: Option<&'a str>,
}
impl<'a> From<ReflectPgPolicyBorrowed<'a>> for ReflectPgPolicy {
    fn from(
        ReflectPgPolicyBorrowed {
            polname,
            polrelid,
            polcmd,
            polpermissive,
            polroles,
            polqual,
            polwithcheck,
            description,
        }: ReflectPgPolicyBorrowed<'a>,
    ) -> Self {
        Self {
            polname: polname.into(),
            polrelid: polrelid.into(),
            polcmd,
            polpermissive,
            polroles: polroles.map(|v| v.map(|v| v.into())).collect(),
            polqual: polqual.map(|v| v.into()),
            polwithcheck: polwithcheck.map(|v| v.into()),
            description: description.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgPublication {
    pub pubname: String,
    pub pubowner: String,
    pub puballtables: bool,
    pub pubinsert: bool,
    pub pubupdate: bool,
    pub pubdelete: bool,
    pub pubtruncate: bool,
    pub pubviaroot: bool,
    pub description: Option<String>,
    pub seclabel: Option<String>,
    pub seclabel_provider: Option<String>,
}
pub struct ReflectPgPublicationBorrowed<'a> {
    pub pubname: &'a str,
    pub pubowner: &'a str,
    pub puballtables: bool,
    pub pubinsert: bool,
    pub pubupdate: bool,
    pub pubdelete: bool,
    pub pubtruncate: bool,
    pub pubviaroot: bool,
    pub description: Option<&'a str>,
    pub seclabel: Option<&'a str>,
    pub seclabel_provider: Option<&'a str>,
}
impl<'a> From<ReflectPgPublicationBorrowed<'a>> for ReflectPgPublication {
    fn from(
        ReflectPgPublicationBorrowed {
            pubname,
            pubowner,
            puballtables,
            pubinsert,
            pubupdate,
            pubdelete,
            pubtruncate,
            pubviaroot,
            description,
            seclabel,
            seclabel_provider,
        }: ReflectPgPublicationBorrowed<'a>,
    ) -> Self {
        Self {
            pubname: pubname.into(),
            pubowner: pubowner.into(),
            puballtables,
            pubinsert,
            pubupdate,
            pubdelete,
            pubtruncate,
            pubviaroot,
            description: description.map(|v| v.into()),
            seclabel: seclabel.map(|v| v.into()),
            seclabel_provider: seclabel_provider.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgPublicationNamespace {
    pub pnpubid: String,
    pub pnnspid: String,
}
pub struct ReflectPgPublicationNamespaceBorrowed<'a> {
    pub pnpubid: &'a str,
    pub pnnspid: &'a str,
}
impl<'a> From<ReflectPgPublicationNamespaceBorrowed<'a>> for ReflectPgPublicationNamespace {
    fn from(
        ReflectPgPublicationNamespaceBorrowed {
            pnpubid,
            pnnspid,
        }: ReflectPgPublicationNamespaceBorrowed<'a>,
    ) -> Self {
        Self {
            pnpubid: pnpubid.into(),
            pnnspid: pnnspid.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgPublicationRel {
    pub prpubid: String,
    pub prrelid: String,
    pub prqual: Option<String>,
    pub prattrs: Option<Vec<i16>>,
}
pub struct ReflectPgPublicationRelBorrowed<'a> {
    pub prpubid: &'a str,
    pub prrelid: &'a str,
    pub prqual: Option<&'a str>,
    pub prattrs: Option<crate::ArrayIterator<'a, i16>>,
}
impl<'a> From<ReflectPgPublicationRelBorrowed<'a>> for ReflectPgPublicationRel {
    fn from(
        ReflectPgPublicationRelBorrowed {
            prpubid,
            prrelid,
            prqual,
            prattrs,
        }: ReflectPgPublicationRelBorrowed<'a>,
    ) -> Self {
        Self {
            prpubid: prpubid.into(),
            prrelid: prrelid.into(),
            prqual: prqual.map(|v| v.into()),
            prattrs: prattrs.map(|v| v.map(|v| v).collect()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgRange {
    pub rngtypid: String,
    pub rngsubtype: String,
    pub rngmultitypid: String,
    pub rngcollation: Option<String>,
    pub rngsubopc: String,
    pub rngcanonical: Option<String>,
    pub rngsubdiff: Option<String>,
}
pub struct ReflectPgRangeBorrowed<'a> {
    pub rngtypid: &'a str,
    pub rngsubtype: &'a str,
    pub rngmultitypid: &'a str,
    pub rngcollation: Option<&'a str>,
    pub rngsubopc: &'a str,
    pub rngcanonical: Option<&'a str>,
    pub rngsubdiff: Option<&'a str>,
}
impl<'a> From<ReflectPgRangeBorrowed<'a>> for ReflectPgRange {
    fn from(
        ReflectPgRangeBorrowed {
            rngtypid,
            rngsubtype,
            rngmultitypid,
            rngcollation,
            rngsubopc,
            rngcanonical,
            rngsubdiff,
        }: ReflectPgRangeBorrowed<'a>,
    ) -> Self {
        Self {
            rngtypid: rngtypid.into(),
            rngsubtype: rngsubtype.into(),
            rngmultitypid: rngmultitypid.into(),
            rngcollation: rngcollation.map(|v| v.into()),
            rngsubopc: rngsubopc.into(),
            rngcanonical: rngcanonical.map(|v| v.into()),
            rngsubdiff: rngsubdiff.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgRules {
    pub schemaname: String,
    pub tablename: String,
    pub rulename: String,
    pub definition: String,
    pub description: Option<String>,
}
pub struct ReflectPgRulesBorrowed<'a> {
    pub schemaname: &'a str,
    pub tablename: &'a str,
    pub rulename: &'a str,
    pub definition: &'a str,
    pub description: Option<&'a str>,
}
impl<'a> From<ReflectPgRulesBorrowed<'a>> for ReflectPgRules {
    fn from(
        ReflectPgRulesBorrowed {
            schemaname,
            tablename,
            rulename,
            definition,
            description,
        }: ReflectPgRulesBorrowed<'a>,
    ) -> Self {
        Self {
            schemaname: schemaname.into(),
            tablename: tablename.into(),
            rulename: rulename.into(),
            definition: definition.into(),
            description: description.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgViews {
    pub schemaname: String,
    pub viewname: String,
    pub viewowner: String,
    pub definition: String,
}
pub struct ReflectPgViewsBorrowed<'a> {
    pub schemaname: &'a str,
    pub viewname: &'a str,
    pub viewowner: &'a str,
    pub definition: &'a str,
}
impl<'a> From<ReflectPgViewsBorrowed<'a>> for ReflectPgViews {
    fn from(
        ReflectPgViewsBorrowed {
            schemaname,
            viewname,
            viewowner,
            definition,
        }: ReflectPgViewsBorrowed<'a>,
    ) -> Self {
        Self {
            schemaname: schemaname.into(),
            viewname: viewname.into(),
            viewowner: viewowner.into(),
            definition: definition.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgMatviews {
    pub schemaname: String,
    pub matviewname: String,
    pub matviewowner: String,
    pub definition: String,
}
pub struct ReflectPgMatviewsBorrowed<'a> {
    pub schemaname: &'a str,
    pub matviewname: &'a str,
    pub matviewowner: &'a str,
    pub definition: &'a str,
}
impl<'a> From<ReflectPgMatviewsBorrowed<'a>> for ReflectPgMatviews {
    fn from(
        ReflectPgMatviewsBorrowed {
            schemaname,
            matviewname,
            matviewowner,
            definition,
        }: ReflectPgMatviewsBorrowed<'a>,
    ) -> Self {
        Self {
            schemaname: schemaname.into(),
            matviewname: matviewname.into(),
            matviewowner: matviewowner.into(),
            definition: definition.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgSequence {
    pub seqrelid: String,
    pub seqtypid: String,
    pub seqstart: i64,
    pub seqincrement: i64,
    pub seqmax: i64,
    pub seqmin: i64,
    pub seqcache: i64,
    pub seqcycle: bool,
}
pub struct ReflectPgSequenceBorrowed<'a> {
    pub seqrelid: &'a str,
    pub seqtypid: &'a str,
    pub seqstart: i64,
    pub seqincrement: i64,
    pub seqmax: i64,
    pub seqmin: i64,
    pub seqcache: i64,
    pub seqcycle: bool,
}
impl<'a> From<ReflectPgSequenceBorrowed<'a>> for ReflectPgSequence {
    fn from(
        ReflectPgSequenceBorrowed {
            seqrelid,
            seqtypid,
            seqstart,
            seqincrement,
            seqmax,
            seqmin,
            seqcache,
            seqcycle,
        }: ReflectPgSequenceBorrowed<'a>,
    ) -> Self {
        Self {
            seqrelid: seqrelid.into(),
            seqtypid: seqtypid.into(),
            seqstart,
            seqincrement,
            seqmax,
            seqmin,
            seqcache,
            seqcycle,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgStatisticExt {
    pub stxrelid: String,
    pub stxname: String,
    pub stxnamespace: String,
    pub stxowner: String,
    pub stxkeys: Vec<i16>,
    pub stxstattarget: Option<i16>,
    pub stxkind: Vec<i8>,
    pub stxexprs: Option<String>,
    pub description: Option<String>,
}
pub struct ReflectPgStatisticExtBorrowed<'a> {
    pub stxrelid: &'a str,
    pub stxname: &'a str,
    pub stxnamespace: &'a str,
    pub stxowner: &'a str,
    pub stxkeys: crate::ArrayIterator<'a, i16>,
    pub stxstattarget: Option<i16>,
    pub stxkind: crate::ArrayIterator<'a, i8>,
    pub stxexprs: Option<&'a str>,
    pub description: Option<&'a str>,
}
impl<'a> From<ReflectPgStatisticExtBorrowed<'a>> for ReflectPgStatisticExt {
    fn from(
        ReflectPgStatisticExtBorrowed {
            stxrelid,
            stxname,
            stxnamespace,
            stxowner,
            stxkeys,
            stxstattarget,
            stxkind,
            stxexprs,
            description,
        }: ReflectPgStatisticExtBorrowed<'a>,
    ) -> Self {
        Self {
            stxrelid: stxrelid.into(),
            stxname: stxname.into(),
            stxnamespace: stxnamespace.into(),
            stxowner: stxowner.into(),
            stxkeys: stxkeys.map(|v| v).collect(),
            stxstattarget,
            stxkind: stxkind.map(|v| v).collect(),
            stxexprs: stxexprs.map(|v| v.into()),
            description: description.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgSubscription {
    pub subname: String,
    pub subowner: String,
    pub subenabled: bool,
    pub subbinary: bool,
    pub substream: i8,
    pub subtwophasestate: i8,
    pub subdisableonerr: bool,
    pub subpasswordrequired: bool,
    pub subrunasowner: bool,
    pub subfailover: bool,
    pub subconninfo: String,
    pub subslotname: Option<String>,
    pub subsynccommit: String,
    pub subpublications: Vec<String>,
    pub suborigin: Option<String>,
    pub description: Option<String>,
    pub seclabel: Option<String>,
    pub seclabel_provider: Option<String>,
}
pub struct ReflectPgSubscriptionBorrowed<'a> {
    pub subname: &'a str,
    pub subowner: &'a str,
    pub subenabled: bool,
    pub subbinary: bool,
    pub substream: i8,
    pub subtwophasestate: i8,
    pub subdisableonerr: bool,
    pub subpasswordrequired: bool,
    pub subrunasowner: bool,
    pub subfailover: bool,
    pub subconninfo: &'a str,
    pub subslotname: Option<&'a str>,
    pub subsynccommit: &'a str,
    pub subpublications: crate::ArrayIterator<'a, &'a str>,
    pub suborigin: Option<&'a str>,
    pub description: Option<&'a str>,
    pub seclabel: Option<&'a str>,
    pub seclabel_provider: Option<&'a str>,
}
impl<'a> From<ReflectPgSubscriptionBorrowed<'a>> for ReflectPgSubscription {
    fn from(
        ReflectPgSubscriptionBorrowed {
            subname,
            subowner,
            subenabled,
            subbinary,
            substream,
            subtwophasestate,
            subdisableonerr,
            subpasswordrequired,
            subrunasowner,
            subfailover,
            subconninfo,
            subslotname,
            subsynccommit,
            subpublications,
            suborigin,
            description,
            seclabel,
            seclabel_provider,
        }: ReflectPgSubscriptionBorrowed<'a>,
    ) -> Self {
        Self {
            subname: subname.into(),
            subowner: subowner.into(),
            subenabled,
            subbinary,
            substream,
            subtwophasestate,
            subdisableonerr,
            subpasswordrequired,
            subrunasowner,
            subfailover,
            subconninfo: subconninfo.into(),
            subslotname: subslotname.map(|v| v.into()),
            subsynccommit: subsynccommit.into(),
            subpublications: subpublications.map(|v| v.into()).collect(),
            suborigin: suborigin.map(|v| v.into()),
            description: description.map(|v| v.into()),
            seclabel: seclabel.map(|v| v.into()),
            seclabel_provider: seclabel_provider.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgTransform {
    pub trftype: String,
    pub trflang: String,
    pub trffromsql: Option<String>,
    pub trftosql: Option<String>,
}
pub struct ReflectPgTransformBorrowed<'a> {
    pub trftype: &'a str,
    pub trflang: &'a str,
    pub trffromsql: Option<&'a str>,
    pub trftosql: Option<&'a str>,
}
impl<'a> From<ReflectPgTransformBorrowed<'a>> for ReflectPgTransform {
    fn from(
        ReflectPgTransformBorrowed {
            trftype,
            trflang,
            trffromsql,
            trftosql,
        }: ReflectPgTransformBorrowed<'a>,
    ) -> Self {
        Self {
            trftype: trftype.into(),
            trflang: trflang.into(),
            trffromsql: trffromsql.map(|v| v.into()),
            trftosql: trftosql.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgTrigger {
    pub tgrelid: String,
    pub tgparentid: Option<String>,
    pub tgname: String,
    pub tgfoid: String,
    pub tgtype: i16,
    pub tgenabled: i8,
    pub tgisinternal: bool,
    pub tgconstrrelid: Option<String>,
    pub tgconstrindid: Option<String>,
    pub tgconstraint: Option<String>,
    pub tgdeferrable: bool,
    pub tginitdeferred: bool,
    pub tgnargs: i16,
    pub tgattr: Vec<i16>,
    pub tgargs: Vec<u8>,
    pub tgqual: Option<String>,
    pub tgoldtable: Option<String>,
    pub tgnewtable: Option<String>,
    pub description: Option<String>,
    pub seclabel: Option<String>,
    pub seclabel_provider: Option<String>,
}
pub struct ReflectPgTriggerBorrowed<'a> {
    pub tgrelid: &'a str,
    pub tgparentid: Option<&'a str>,
    pub tgname: &'a str,
    pub tgfoid: &'a str,
    pub tgtype: i16,
    pub tgenabled: i8,
    pub tgisinternal: bool,
    pub tgconstrrelid: Option<&'a str>,
    pub tgconstrindid: Option<&'a str>,
    pub tgconstraint: Option<&'a str>,
    pub tgdeferrable: bool,
    pub tginitdeferred: bool,
    pub tgnargs: i16,
    pub tgattr: crate::ArrayIterator<'a, i16>,
    pub tgargs: &'a [u8],
    pub tgqual: Option<&'a str>,
    pub tgoldtable: Option<&'a str>,
    pub tgnewtable: Option<&'a str>,
    pub description: Option<&'a str>,
    pub seclabel: Option<&'a str>,
    pub seclabel_provider: Option<&'a str>,
}
impl<'a> From<ReflectPgTriggerBorrowed<'a>> for ReflectPgTrigger {
    fn from(
        ReflectPgTriggerBorrowed {
            tgrelid,
            tgparentid,
            tgname,
            tgfoid,
            tgtype,
            tgenabled,
            tgisinternal,
            tgconstrrelid,
            tgconstrindid,
            tgconstraint,
            tgdeferrable,
            tginitdeferred,
            tgnargs,
            tgattr,
            tgargs,
            tgqual,
            tgoldtable,
            tgnewtable,
            description,
            seclabel,
            seclabel_provider,
        }: ReflectPgTriggerBorrowed<'a>,
    ) -> Self {
        Self {
            tgrelid: tgrelid.into(),
            tgparentid: tgparentid.map(|v| v.into()),
            tgname: tgname.into(),
            tgfoid: tgfoid.into(),
            tgtype,
            tgenabled,
            tgisinternal,
            tgconstrrelid: tgconstrrelid.map(|v| v.into()),
            tgconstrindid: tgconstrindid.map(|v| v.into()),
            tgconstraint: tgconstraint.map(|v| v.into()),
            tgdeferrable,
            tginitdeferred,
            tgnargs,
            tgattr: tgattr.map(|v| v).collect(),
            tgargs: tgargs.into(),
            tgqual: tgqual.map(|v| v.into()),
            tgoldtable: tgoldtable.map(|v| v.into()),
            tgnewtable: tgnewtable.map(|v| v.into()),
            description: description.map(|v| v.into()),
            seclabel: seclabel.map(|v| v.into()),
            seclabel_provider: seclabel_provider.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgTsConfig {
    pub oid: String,
    pub cfgname: String,
    pub cfgnamespace: String,
    pub cfgowner: String,
    pub description: Option<String>,
    pub seclabel: Option<String>,
    pub seclabel_provider: Option<String>,
}
pub struct ReflectPgTsConfigBorrowed<'a> {
    pub oid: &'a str,
    pub cfgname: &'a str,
    pub cfgnamespace: &'a str,
    pub cfgowner: &'a str,
    pub description: Option<&'a str>,
    pub seclabel: Option<&'a str>,
    pub seclabel_provider: Option<&'a str>,
}
impl<'a> From<ReflectPgTsConfigBorrowed<'a>> for ReflectPgTsConfig {
    fn from(
        ReflectPgTsConfigBorrowed {
            oid,
            cfgname,
            cfgnamespace,
            cfgowner,
            description,
            seclabel,
            seclabel_provider,
        }: ReflectPgTsConfigBorrowed<'a>,
    ) -> Self {
        Self {
            oid: oid.into(),
            cfgname: cfgname.into(),
            cfgnamespace: cfgnamespace.into(),
            cfgowner: cfgowner.into(),
            description: description.map(|v| v.into()),
            seclabel: seclabel.map(|v| v.into()),
            seclabel_provider: seclabel_provider.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgTsConfigMap {
    pub mapcfg: String,
    pub maptokentype: i32,
    pub mapseqno: i32,
    pub mapdict: String,
}
pub struct ReflectPgTsConfigMapBorrowed<'a> {
    pub mapcfg: &'a str,
    pub maptokentype: i32,
    pub mapseqno: i32,
    pub mapdict: &'a str,
}
impl<'a> From<ReflectPgTsConfigMapBorrowed<'a>> for ReflectPgTsConfigMap {
    fn from(
        ReflectPgTsConfigMapBorrowed {
            mapcfg,
            maptokentype,
            mapseqno,
            mapdict,
        }: ReflectPgTsConfigMapBorrowed<'a>,
    ) -> Self {
        Self {
            mapcfg: mapcfg.into(),
            maptokentype,
            mapseqno,
            mapdict: mapdict.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgTsDict {
    pub oid: String,
    pub dictname: String,
    pub dictnamespace: String,
    pub dictowner: String,
    pub dictinitoption: Option<String>,
    pub description: Option<String>,
    pub seclabel: Option<String>,
    pub seclabel_provider: Option<String>,
}
pub struct ReflectPgTsDictBorrowed<'a> {
    pub oid: &'a str,
    pub dictname: &'a str,
    pub dictnamespace: &'a str,
    pub dictowner: &'a str,
    pub dictinitoption: Option<&'a str>,
    pub description: Option<&'a str>,
    pub seclabel: Option<&'a str>,
    pub seclabel_provider: Option<&'a str>,
}
impl<'a> From<ReflectPgTsDictBorrowed<'a>> for ReflectPgTsDict {
    fn from(
        ReflectPgTsDictBorrowed {
            oid,
            dictname,
            dictnamespace,
            dictowner,
            dictinitoption,
            description,
            seclabel,
            seclabel_provider,
        }: ReflectPgTsDictBorrowed<'a>,
    ) -> Self {
        Self {
            oid: oid.into(),
            dictname: dictname.into(),
            dictnamespace: dictnamespace.into(),
            dictowner: dictowner.into(),
            dictinitoption: dictinitoption.map(|v| v.into()),
            description: description.map(|v| v.into()),
            seclabel: seclabel.map(|v| v.into()),
            seclabel_provider: seclabel_provider.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgTsParser {
    pub prsname: String,
    pub prsnamespace: String,
    pub prsstart: String,
    pub prstoken: String,
    pub prsend: String,
    pub prsheadline: Option<String>,
    pub prslextype: String,
}
pub struct ReflectPgTsParserBorrowed<'a> {
    pub prsname: &'a str,
    pub prsnamespace: &'a str,
    pub prsstart: &'a str,
    pub prstoken: &'a str,
    pub prsend: &'a str,
    pub prsheadline: Option<&'a str>,
    pub prslextype: &'a str,
}
impl<'a> From<ReflectPgTsParserBorrowed<'a>> for ReflectPgTsParser {
    fn from(
        ReflectPgTsParserBorrowed {
            prsname,
            prsnamespace,
            prsstart,
            prstoken,
            prsend,
            prsheadline,
            prslextype,
        }: ReflectPgTsParserBorrowed<'a>,
    ) -> Self {
        Self {
            prsname: prsname.into(),
            prsnamespace: prsnamespace.into(),
            prsstart: prsstart.into(),
            prstoken: prstoken.into(),
            prsend: prsend.into(),
            prsheadline: prsheadline.map(|v| v.into()),
            prslextype: prslextype.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgTsTemplate {
    pub tmplname: String,
    pub tmplnamespace: String,
    pub tmplinit: Option<String>,
    pub tmpllexize: String,
}
pub struct ReflectPgTsTemplateBorrowed<'a> {
    pub tmplname: &'a str,
    pub tmplnamespace: &'a str,
    pub tmplinit: Option<&'a str>,
    pub tmpllexize: &'a str,
}
impl<'a> From<ReflectPgTsTemplateBorrowed<'a>> for ReflectPgTsTemplate {
    fn from(
        ReflectPgTsTemplateBorrowed {
            tmplname,
            tmplnamespace,
            tmplinit,
            tmpllexize,
        }: ReflectPgTsTemplateBorrowed<'a>,
    ) -> Self {
        Self {
            tmplname: tmplname.into(),
            tmplnamespace: tmplnamespace.into(),
            tmplinit: tmplinit.map(|v| v.into()),
            tmpllexize: tmpllexize.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgType {
    pub oid: String,
    pub typname: String,
    pub typnamespace: String,
    pub typowner: String,
    pub typlen: i16,
    pub typbyval: bool,
    pub typtype: i8,
    pub typispreferred: bool,
    pub typisdefined: bool,
    pub typdelim: i8,
    pub typrelid: Option<String>,
    pub typsubscript: Option<String>,
    pub typelem: Option<String>,
    pub typarray: Option<String>,
    pub typinput: String,
    pub typoutput: String,
    pub typreceive: Option<String>,
    pub typsend: Option<String>,
    pub typmodin: Option<String>,
    pub typmodout: Option<String>,
    pub typanalyze: Option<String>,
    pub typalign: i8,
    pub typstorage: i8,
    pub typnotnull: bool,
    pub typbasetype: Option<String>,
    pub typtypmod: Option<i32>,
    pub typndims: i32,
    pub typcollation: Option<String>,
    pub typdefaultbin: Option<String>,
    pub typdefault: Option<String>,
    pub typacl: Option<Vec<crate::types::pg_temp_1::TypeAclitem>>,
    pub description: Option<String>,
    pub seclabel: Option<String>,
    pub seclabel_provider: Option<String>,
    pub initprivs: Option<Vec<crate::types::pg_temp_1::TypeAclitem>>,
    pub initprivs_type: Option<i8>,
}
pub struct ReflectPgTypeBorrowed<'a> {
    pub oid: &'a str,
    pub typname: &'a str,
    pub typnamespace: &'a str,
    pub typowner: &'a str,
    pub typlen: i16,
    pub typbyval: bool,
    pub typtype: i8,
    pub typispreferred: bool,
    pub typisdefined: bool,
    pub typdelim: i8,
    pub typrelid: Option<&'a str>,
    pub typsubscript: Option<&'a str>,
    pub typelem: Option<&'a str>,
    pub typarray: Option<&'a str>,
    pub typinput: &'a str,
    pub typoutput: &'a str,
    pub typreceive: Option<&'a str>,
    pub typsend: Option<&'a str>,
    pub typmodin: Option<&'a str>,
    pub typmodout: Option<&'a str>,
    pub typanalyze: Option<&'a str>,
    pub typalign: i8,
    pub typstorage: i8,
    pub typnotnull: bool,
    pub typbasetype: Option<&'a str>,
    pub typtypmod: Option<i32>,
    pub typndims: i32,
    pub typcollation: Option<&'a str>,
    pub typdefaultbin: Option<&'a str>,
    pub typdefault: Option<&'a str>,
    pub typacl: Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::TypeAclitemBorrowed<'a>>>,
    pub description: Option<&'a str>,
    pub seclabel: Option<&'a str>,
    pub seclabel_provider: Option<&'a str>,
    pub initprivs:
        Option<crate::ArrayIterator<'a, crate::types::pg_temp_1::TypeAclitemBorrowed<'a>>>,
    pub initprivs_type: Option<i8>,
}
impl<'a> From<ReflectPgTypeBorrowed<'a>> for ReflectPgType {
    fn from(
        ReflectPgTypeBorrowed {
            oid,
            typname,
            typnamespace,
            typowner,
            typlen,
            typbyval,
            typtype,
            typispreferred,
            typisdefined,
            typdelim,
            typrelid,
            typsubscript,
            typelem,
            typarray,
            typinput,
            typoutput,
            typreceive,
            typsend,
            typmodin,
            typmodout,
            typanalyze,
            typalign,
            typstorage,
            typnotnull,
            typbasetype,
            typtypmod,
            typndims,
            typcollation,
            typdefaultbin,
            typdefault,
            typacl,
            description,
            seclabel,
            seclabel_provider,
            initprivs,
            initprivs_type,
        }: ReflectPgTypeBorrowed<'a>,
    ) -> Self {
        Self {
            oid: oid.into(),
            typname: typname.into(),
            typnamespace: typnamespace.into(),
            typowner: typowner.into(),
            typlen,
            typbyval,
            typtype,
            typispreferred,
            typisdefined,
            typdelim,
            typrelid: typrelid.map(|v| v.into()),
            typsubscript: typsubscript.map(|v| v.into()),
            typelem: typelem.map(|v| v.into()),
            typarray: typarray.map(|v| v.into()),
            typinput: typinput.into(),
            typoutput: typoutput.into(),
            typreceive: typreceive.map(|v| v.into()),
            typsend: typsend.map(|v| v.into()),
            typmodin: typmodin.map(|v| v.into()),
            typmodout: typmodout.map(|v| v.into()),
            typanalyze: typanalyze.map(|v| v.into()),
            typalign,
            typstorage,
            typnotnull,
            typbasetype: typbasetype.map(|v| v.into()),
            typtypmod,
            typndims,
            typcollation: typcollation.map(|v| v.into()),
            typdefaultbin: typdefaultbin.map(|v| v.into()),
            typdefault: typdefault.map(|v| v.into()),
            typacl: typacl.map(|v| v.map(|v| v.into()).collect()),
            description: description.map(|v| v.into()),
            seclabel: seclabel.map(|v| v.into()),
            seclabel_provider: seclabel_provider.map(|v| v.into()),
            initprivs: initprivs.map(|v| v.map(|v| v.into()).collect()),
            initprivs_type,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReflectPgUserMappings {
    pub srvname: String,
    pub umuser: Option<String>,
    pub usename: String,
    pub umoptions: Option<Vec<String>>,
}
pub struct ReflectPgUserMappingsBorrowed<'a> {
    pub srvname: &'a str,
    pub umuser: Option<&'a str>,
    pub usename: &'a str,
    pub umoptions: Option<crate::ArrayIterator<'a, &'a str>>,
}
impl<'a> From<ReflectPgUserMappingsBorrowed<'a>> for ReflectPgUserMappings {
    fn from(
        ReflectPgUserMappingsBorrowed {
            srvname,
            umuser,
            usename,
            umoptions,
        }: ReflectPgUserMappingsBorrowed<'a>,
    ) -> Self {
        Self {
            srvname: srvname.into(),
            umuser: umuser.map(|v| v.into()),
            usename: usename.into(),
            umoptions: umoptions.map(|v| v.map(|v| v.into()).collect()),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct ReflectPgAggregateQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgAggregateBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgAggregateBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgAggregateQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgAggregateBorrowed) -> R,
    ) -> ReflectPgAggregateQuery<'c, 'a, 's, C, R, N> {
        ReflectPgAggregateQuery {
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
pub struct ReflectPgAmQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgAmBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgAmBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgAmQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgAmBorrowed) -> R,
    ) -> ReflectPgAmQuery<'c, 'a, 's, C, R, N> {
        ReflectPgAmQuery {
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
pub struct ReflectPgAmopQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgAmopBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgAmopBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgAmopQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgAmopBorrowed) -> R,
    ) -> ReflectPgAmopQuery<'c, 'a, 's, C, R, N> {
        ReflectPgAmopQuery {
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
pub struct ReflectPgAmprocQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgAmprocBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgAmprocBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgAmprocQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgAmprocBorrowed) -> R,
    ) -> ReflectPgAmprocQuery<'c, 'a, 's, C, R, N> {
        ReflectPgAmprocQuery {
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
pub struct ReflectPgAttrdefQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgAttrdefBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgAttrdefBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgAttrdefQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgAttrdefBorrowed) -> R,
    ) -> ReflectPgAttrdefQuery<'c, 'a, 's, C, R, N> {
        ReflectPgAttrdefQuery {
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
pub struct ReflectPgAttributeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgAttributeBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgAttributeBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgAttributeQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgAttributeBorrowed) -> R,
    ) -> ReflectPgAttributeQuery<'c, 'a, 's, C, R, N> {
        ReflectPgAttributeQuery {
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
pub struct ReflectPgRolesQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgRolesBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgRolesBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgRolesQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgRolesBorrowed) -> R,
    ) -> ReflectPgRolesQuery<'c, 'a, 's, C, R, N> {
        ReflectPgRolesQuery {
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
pub struct ReflectPgAuthMembersQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgAuthMembersBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgAuthMembersBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgAuthMembersQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgAuthMembersBorrowed) -> R,
    ) -> ReflectPgAuthMembersQuery<'c, 'a, 's, C, R, N> {
        ReflectPgAuthMembersQuery {
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
pub struct ReflectPgCastQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgCastBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgCastBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgCastQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgCastBorrowed) -> R,
    ) -> ReflectPgCastQuery<'c, 'a, 's, C, R, N> {
        ReflectPgCastQuery {
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
pub struct ReflectPgClassQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgClassBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgClassBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgClassQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgClassBorrowed) -> R,
    ) -> ReflectPgClassQuery<'c, 'a, 's, C, R, N> {
        ReflectPgClassQuery {
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
pub struct ReflectPgCollationQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgCollationBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgCollationBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgCollationQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgCollationBorrowed) -> R,
    ) -> ReflectPgCollationQuery<'c, 'a, 's, C, R, N> {
        ReflectPgCollationQuery {
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
pub struct ReflectPgConstraintQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgConstraintBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgConstraintBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgConstraintQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgConstraintBorrowed) -> R,
    ) -> ReflectPgConstraintQuery<'c, 'a, 's, C, R, N> {
        ReflectPgConstraintQuery {
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
pub struct ReflectPgConversionQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgConversionBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgConversionBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgConversionQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgConversionBorrowed) -> R,
    ) -> ReflectPgConversionQuery<'c, 'a, 's, C, R, N> {
        ReflectPgConversionQuery {
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
pub struct ReflectPgDatabaseQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgDatabaseBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgDatabaseBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgDatabaseQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgDatabaseBorrowed) -> R,
    ) -> ReflectPgDatabaseQuery<'c, 'a, 's, C, R, N> {
        ReflectPgDatabaseQuery {
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
pub struct ReflectPgDefaultAclQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgDefaultAclBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgDefaultAclBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgDefaultAclQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgDefaultAclBorrowed) -> R,
    ) -> ReflectPgDefaultAclQuery<'c, 'a, 's, C, R, N> {
        ReflectPgDefaultAclQuery {
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
pub struct ReflectPgEventTriggerQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgEventTriggerBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgEventTriggerBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgEventTriggerQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgEventTriggerBorrowed) -> R,
    ) -> ReflectPgEventTriggerQuery<'c, 'a, 's, C, R, N> {
        ReflectPgEventTriggerQuery {
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
pub struct ReflectPgExtensionQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgExtensionBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgExtensionBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgExtensionQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgExtensionBorrowed) -> R,
    ) -> ReflectPgExtensionQuery<'c, 'a, 's, C, R, N> {
        ReflectPgExtensionQuery {
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
pub struct ReflectPgForeignDataWrapperQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(
        &tokio_postgres::Row,
    ) -> Result<ReflectPgForeignDataWrapperBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgForeignDataWrapperBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgForeignDataWrapperQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgForeignDataWrapperBorrowed) -> R,
    ) -> ReflectPgForeignDataWrapperQuery<'c, 'a, 's, C, R, N> {
        ReflectPgForeignDataWrapperQuery {
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
pub struct ReflectPgForeignServerQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgForeignServerBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgForeignServerBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgForeignServerQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgForeignServerBorrowed) -> R,
    ) -> ReflectPgForeignServerQuery<'c, 'a, 's, C, R, N> {
        ReflectPgForeignServerQuery {
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
pub struct ReflectPgForeignTableQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgForeignTableBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgForeignTableBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgForeignTableQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgForeignTableBorrowed) -> R,
    ) -> ReflectPgForeignTableQuery<'c, 'a, 's, C, R, N> {
        ReflectPgForeignTableQuery {
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
pub struct ReflectPgIndexQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgIndexBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgIndexBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgIndexQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgIndexBorrowed) -> R,
    ) -> ReflectPgIndexQuery<'c, 'a, 's, C, R, N> {
        ReflectPgIndexQuery {
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
pub struct ReflectPgInheritsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgInheritsBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgInheritsBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgInheritsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgInheritsBorrowed) -> R,
    ) -> ReflectPgInheritsQuery<'c, 'a, 's, C, R, N> {
        ReflectPgInheritsQuery {
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
pub struct ReflectPgLanguageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgLanguageBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgLanguageBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgLanguageQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgLanguageBorrowed) -> R,
    ) -> ReflectPgLanguageQuery<'c, 'a, 's, C, R, N> {
        ReflectPgLanguageQuery {
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
pub struct ReflectPgNamespaceQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgNamespaceBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgNamespaceBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgNamespaceQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgNamespaceBorrowed) -> R,
    ) -> ReflectPgNamespaceQuery<'c, 'a, 's, C, R, N> {
        ReflectPgNamespaceQuery {
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
pub struct ReflectPgOpclassQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgOpclassBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgOpclassBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgOpclassQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgOpclassBorrowed) -> R,
    ) -> ReflectPgOpclassQuery<'c, 'a, 's, C, R, N> {
        ReflectPgOpclassQuery {
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
pub struct ReflectPgOperatorQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgOperatorBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgOperatorBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgOperatorQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgOperatorBorrowed) -> R,
    ) -> ReflectPgOperatorQuery<'c, 'a, 's, C, R, N> {
        ReflectPgOperatorQuery {
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
pub struct ReflectPgOpfamilyQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgOpfamilyBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgOpfamilyBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgOpfamilyQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgOpfamilyBorrowed) -> R,
    ) -> ReflectPgOpfamilyQuery<'c, 'a, 's, C, R, N> {
        ReflectPgOpfamilyQuery {
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
pub struct ReflectPgParameterAclQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgParameterAclBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgParameterAclBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgParameterAclQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgParameterAclBorrowed) -> R,
    ) -> ReflectPgParameterAclQuery<'c, 'a, 's, C, R, N> {
        ReflectPgParameterAclQuery {
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
pub struct ReflectPgPartitionedTableQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(
        &tokio_postgres::Row,
    ) -> Result<ReflectPgPartitionedTableBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgPartitionedTableBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgPartitionedTableQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgPartitionedTableBorrowed) -> R,
    ) -> ReflectPgPartitionedTableQuery<'c, 'a, 's, C, R, N> {
        ReflectPgPartitionedTableQuery {
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
pub struct ReflectPgPolicyQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgPolicyBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgPolicyBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgPolicyQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgPolicyBorrowed) -> R,
    ) -> ReflectPgPolicyQuery<'c, 'a, 's, C, R, N> {
        ReflectPgPolicyQuery {
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
pub struct ReflectPgPublicationQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgPublicationBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgPublicationBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgPublicationQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgPublicationBorrowed) -> R,
    ) -> ReflectPgPublicationQuery<'c, 'a, 's, C, R, N> {
        ReflectPgPublicationQuery {
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
pub struct ReflectPgPublicationNamespaceQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(
        &tokio_postgres::Row,
    ) -> Result<ReflectPgPublicationNamespaceBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgPublicationNamespaceBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgPublicationNamespaceQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgPublicationNamespaceBorrowed) -> R,
    ) -> ReflectPgPublicationNamespaceQuery<'c, 'a, 's, C, R, N> {
        ReflectPgPublicationNamespaceQuery {
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
pub struct ReflectPgPublicationRelQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgPublicationRelBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgPublicationRelBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgPublicationRelQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgPublicationRelBorrowed) -> R,
    ) -> ReflectPgPublicationRelQuery<'c, 'a, 's, C, R, N> {
        ReflectPgPublicationRelQuery {
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
pub struct ReflectPgRangeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgRangeBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgRangeBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgRangeQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgRangeBorrowed) -> R,
    ) -> ReflectPgRangeQuery<'c, 'a, 's, C, R, N> {
        ReflectPgRangeQuery {
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
pub struct ReflectPgRulesQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgRulesBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgRulesBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgRulesQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgRulesBorrowed) -> R,
    ) -> ReflectPgRulesQuery<'c, 'a, 's, C, R, N> {
        ReflectPgRulesQuery {
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
pub struct ReflectPgViewsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgViewsBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgViewsBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgViewsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgViewsBorrowed) -> R,
    ) -> ReflectPgViewsQuery<'c, 'a, 's, C, R, N> {
        ReflectPgViewsQuery {
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
pub struct ReflectPgMatviewsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgMatviewsBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgMatviewsBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgMatviewsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgMatviewsBorrowed) -> R,
    ) -> ReflectPgMatviewsQuery<'c, 'a, 's, C, R, N> {
        ReflectPgMatviewsQuery {
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
pub struct ReflectPgSequenceQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgSequenceBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgSequenceBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgSequenceQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgSequenceBorrowed) -> R,
    ) -> ReflectPgSequenceQuery<'c, 'a, 's, C, R, N> {
        ReflectPgSequenceQuery {
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
pub struct ReflectPgStatisticExtQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgStatisticExtBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgStatisticExtBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgStatisticExtQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgStatisticExtBorrowed) -> R,
    ) -> ReflectPgStatisticExtQuery<'c, 'a, 's, C, R, N> {
        ReflectPgStatisticExtQuery {
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
pub struct ReflectPgSubscriptionQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgSubscriptionBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgSubscriptionBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgSubscriptionQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgSubscriptionBorrowed) -> R,
    ) -> ReflectPgSubscriptionQuery<'c, 'a, 's, C, R, N> {
        ReflectPgSubscriptionQuery {
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
pub struct ReflectPgTransformQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgTransformBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgTransformBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgTransformQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgTransformBorrowed) -> R,
    ) -> ReflectPgTransformQuery<'c, 'a, 's, C, R, N> {
        ReflectPgTransformQuery {
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
pub struct ReflectPgTriggerQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgTriggerBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgTriggerBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgTriggerQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgTriggerBorrowed) -> R,
    ) -> ReflectPgTriggerQuery<'c, 'a, 's, C, R, N> {
        ReflectPgTriggerQuery {
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
pub struct ReflectPgTsConfigQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgTsConfigBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgTsConfigBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgTsConfigQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgTsConfigBorrowed) -> R,
    ) -> ReflectPgTsConfigQuery<'c, 'a, 's, C, R, N> {
        ReflectPgTsConfigQuery {
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
pub struct ReflectPgTsConfigMapQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgTsConfigMapBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgTsConfigMapBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgTsConfigMapQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgTsConfigMapBorrowed) -> R,
    ) -> ReflectPgTsConfigMapQuery<'c, 'a, 's, C, R, N> {
        ReflectPgTsConfigMapQuery {
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
pub struct ReflectPgTsDictQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgTsDictBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgTsDictBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgTsDictQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgTsDictBorrowed) -> R,
    ) -> ReflectPgTsDictQuery<'c, 'a, 's, C, R, N> {
        ReflectPgTsDictQuery {
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
pub struct ReflectPgTsParserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgTsParserBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgTsParserBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgTsParserQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgTsParserBorrowed) -> R,
    ) -> ReflectPgTsParserQuery<'c, 'a, 's, C, R, N> {
        ReflectPgTsParserQuery {
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
pub struct ReflectPgTsTemplateQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgTsTemplateBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgTsTemplateBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgTsTemplateQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgTsTemplateBorrowed) -> R,
    ) -> ReflectPgTsTemplateQuery<'c, 'a, 's, C, R, N> {
        ReflectPgTsTemplateQuery {
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
pub struct ReflectPgTypeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ReflectPgTypeBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgTypeBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgTypeQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgTypeBorrowed) -> R,
    ) -> ReflectPgTypeQuery<'c, 'a, 's, C, R, N> {
        ReflectPgTypeQuery {
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
pub struct ReflectPgUserMappingsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReflectPgUserMappingsBorrowed, tokio_postgres::Error>,
    mapper: fn(ReflectPgUserMappingsBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReflectPgUserMappingsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReflectPgUserMappingsBorrowed) -> R,
    ) -> ReflectPgUserMappingsQuery<'c, 'a, 's, C, R, N> {
        ReflectPgUserMappingsQuery {
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
pub struct ReflectPgAggregateStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_aggregate() -> ReflectPgAggregateStmt {
    ReflectPgAggregateStmt(
        "select aggfnoid::regprocedure::text as aggfnoid, pg_aggregate.aggkind as aggkind, pg_aggregate.aggnumdirectargs as aggnumdirectargs, aggtransfn::regprocedure::text as aggtransfn, case when aggfinalfn = 0 then null else aggfinalfn::regprocedure::text end as aggfinalfn, case when aggcombinefn = 0 then null else aggcombinefn::regprocedure::text end as aggcombinefn, case when aggserialfn = 0 then null else aggserialfn::regprocedure::text end as aggserialfn, case when aggdeserialfn = 0 then null else aggdeserialfn::regprocedure::text end as aggdeserialfn, case when aggmtransfn = 0 then null else aggmtransfn::regprocedure::text end as aggmtransfn, case when aggminvtransfn = 0 then null else aggminvtransfn::regprocedure::text end as aggminvtransfn, case when aggmfinalfn = 0 then null else aggmfinalfn::regprocedure::text end as aggmfinalfn, pg_aggregate.aggfinalextra as aggfinalextra, pg_aggregate.aggmfinalextra as aggmfinalextra, pg_aggregate.aggfinalmodify as aggfinalmodify, pg_aggregate.aggmfinalmodify as aggmfinalmodify, case when pg_aggregate.aggsortop = 0 then null else pg_aggregate.aggsortop::regoperator::text end as aggsortop, pg_aggregate.aggtranstype::regtype::text as aggtranstype, case when pg_aggregate.aggmtranstype = 0 then null else pg_aggregate.aggmtranstype::regtype::text end as aggmtranstype, pg_aggregate.agginitval as agginitval, pg_aggregate.aggminitval as aggminitval from pg_aggregate",
        None,
    )
}
impl ReflectPgAggregateStmt {
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
    ) -> ReflectPgAggregateQuery<'c, 'a, 's, C, ReflectPgAggregate, 0> {
        ReflectPgAggregateQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgAggregateBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgAggregateBorrowed {
                    aggfnoid: row.try_get(0)?,
                    aggkind: row.try_get(1)?,
                    aggnumdirectargs: row.try_get(2)?,
                    aggtransfn: row.try_get(3)?,
                    aggfinalfn: row.try_get(4)?,
                    aggcombinefn: row.try_get(5)?,
                    aggserialfn: row.try_get(6)?,
                    aggdeserialfn: row.try_get(7)?,
                    aggmtransfn: row.try_get(8)?,
                    aggminvtransfn: row.try_get(9)?,
                    aggmfinalfn: row.try_get(10)?,
                    aggfinalextra: row.try_get(11)?,
                    aggmfinalextra: row.try_get(12)?,
                    aggfinalmodify: row.try_get(13)?,
                    aggmfinalmodify: row.try_get(14)?,
                    aggsortop: row.try_get(15)?,
                    aggtranstype: row.try_get(16)?,
                    aggmtranstype: row.try_get(17)?,
                    agginitval: row.try_get(18)?,
                    aggminitval: row.try_get(19)?,
                })
            },
            mapper: |it| ReflectPgAggregate::from(it),
        }
    }
}
pub struct ReflectPgAmStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_am() -> ReflectPgAmStmt {
    ReflectPgAmStmt(
        "select pg_am.amname::text as amname, amhandler::regprocedure::text as amhandler, pg_am.amtype as amtype, pg_description.description as description from pg_am left join pg_description on pg_description.objoid = pg_am.oid and pg_description.objsubid = 0",
        None,
    )
}
impl ReflectPgAmStmt {
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
    ) -> ReflectPgAmQuery<'c, 'a, 's, C, ReflectPgAm, 0> {
        ReflectPgAmQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ReflectPgAmBorrowed, tokio_postgres::Error> {
                    Ok(ReflectPgAmBorrowed {
                        amname: row.try_get(0)?,
                        amhandler: row.try_get(1)?,
                        amtype: row.try_get(2)?,
                        description: row.try_get(3)?,
                    })
                },
            mapper: |it| ReflectPgAm::from(it),
        }
    }
}
pub struct ReflectPgAmopStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_amop() -> ReflectPgAmopStmt {
    ReflectPgAmopStmt(
        "select pg_temp.quote_with_namespace(amopfamily_pg_namespace.nspname, amopfamily_pg_opfamily.opfname) as amopfamily, pg_amop.amoplefttype::regtype::text as amoplefttype, pg_amop.amoprighttype::regtype::text as amoprighttype, pg_amop.amopstrategy as amopstrategy, pg_amop.amoppurpose as amoppurpose, pg_amop.amopopr::regoperator::text as amopopr, amopmethod_pg_am.amname::text as amopmethod, pg_temp.quote_with_namespace(amopsortfamily_pg_namespace.nspname, amopsortfamily_pg_opfamily.opfname) as amopsortfamily from pg_amop join pg_opfamily as amopfamily_pg_opfamily on pg_amop.amopfamily = amopfamily_pg_opfamily.oid join pg_namespace as amopfamily_pg_namespace on amopfamily_pg_opfamily.opfnamespace = amopfamily_pg_namespace.oid join pg_am as amopmethod_pg_am on pg_amop.amopmethod = amopmethod_pg_am.oid left join pg_opfamily as amopsortfamily_pg_opfamily on pg_amop.amopsortfamily = amopsortfamily_pg_opfamily.oid left join pg_namespace as amopsortfamily_pg_namespace on amopsortfamily_pg_opfamily.opfnamespace = amopsortfamily_pg_namespace.oid",
        None,
    )
}
impl ReflectPgAmopStmt {
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
    ) -> ReflectPgAmopQuery<'c, 'a, 's, C, ReflectPgAmop, 0> {
        ReflectPgAmopQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ReflectPgAmopBorrowed, tokio_postgres::Error> {
                    Ok(ReflectPgAmopBorrowed {
                        amopfamily: row.try_get(0)?,
                        amoplefttype: row.try_get(1)?,
                        amoprighttype: row.try_get(2)?,
                        amopstrategy: row.try_get(3)?,
                        amoppurpose: row.try_get(4)?,
                        amopopr: row.try_get(5)?,
                        amopmethod: row.try_get(6)?,
                        amopsortfamily: row.try_get(7)?,
                    })
                },
            mapper: |it| ReflectPgAmop::from(it),
        }
    }
}
pub struct ReflectPgAmprocStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_amproc() -> ReflectPgAmprocStmt {
    ReflectPgAmprocStmt(
        "select pg_temp.quote_with_namespace(amprocfamily_pg_namespace.nspname, amprocfamily_pg_opfamily.opfname) as amprocfamily, pg_amproc.amproclefttype::regtype::text as amproclefttype, pg_amproc.amprocrighttype::regtype::text as amprocrighttype, pg_amproc.amprocnum as amprocnum, amproc::regprocedure::text as amproc from pg_amproc join pg_opfamily as amprocfamily_pg_opfamily on pg_amproc.amprocfamily = amprocfamily_pg_opfamily.oid join pg_namespace as amprocfamily_pg_namespace on amprocfamily_pg_opfamily.opfnamespace = amprocfamily_pg_namespace.oid",
        None,
    )
}
impl ReflectPgAmprocStmt {
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
    ) -> ReflectPgAmprocQuery<'c, 'a, 's, C, ReflectPgAmproc, 0> {
        ReflectPgAmprocQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgAmprocBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgAmprocBorrowed {
                    amprocfamily: row.try_get(0)?,
                    amproclefttype: row.try_get(1)?,
                    amprocrighttype: row.try_get(2)?,
                    amprocnum: row.try_get(3)?,
                    amproc: row.try_get(4)?,
                })
            },
            mapper: |it| ReflectPgAmproc::from(it),
        }
    }
}
pub struct ReflectPgAttrdefStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_attrdef() -> ReflectPgAttrdefStmt {
    ReflectPgAttrdefStmt(
        "select pg_attrdef.adrelid::regclass::text as adrelid, pg_attrdef.adnum as adnum, pg_get_expr(pg_attrdef.adbin, pg_attrdef.adrelid) as adbin from pg_attrdef where adnum > 0",
        None,
    )
}
impl ReflectPgAttrdefStmt {
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
    ) -> ReflectPgAttrdefQuery<'c, 'a, 's, C, ReflectPgAttrdef, 0> {
        ReflectPgAttrdefQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgAttrdefBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgAttrdefBorrowed {
                    adrelid: row.try_get(0)?,
                    adnum: row.try_get(1)?,
                    adbin: row.try_get(2)?,
                })
            },
            mapper: |it| ReflectPgAttrdef::from(it),
        }
    }
}
pub struct ReflectPgAttributeStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_attribute() -> ReflectPgAttributeStmt {
    ReflectPgAttributeStmt(
        "select pg_attribute.attrelid::regclass::text as attrelid, pg_attribute.attname::text as attname, case when pg_attribute.atttypid = 0 then null else pg_attribute.atttypid::regtype::text end as atttypid, pg_attribute.attnum as attnum, case when atttypmod < 0 then null else atttypmod end as atttypmod, pg_attribute.attndims as attndims, case when pg_attribute.attcompression = '' then null else pg_attribute.attcompression end as attcompression, pg_attribute.attnotnull as attnotnull, pg_attribute.atthasdef as atthasdef, case when pg_attribute.attidentity = '' then null else pg_attribute.attidentity end as attidentity, case when pg_attribute.attgenerated = '' then null else pg_attribute.attgenerated end as attgenerated, pg_attribute.attisdropped as attisdropped, pg_attribute.attislocal as attislocal, pg_attribute.attinhcount as attinhcount, case when pg_attribute.attcollation = 0 then null else pg_attribute.attcollation::regcollation::text end as attcollation, pg_attribute.attstattarget as attstattarget, pg_temp.format_tablecolumn_aclitems(pg_attribute.attacl) as attacl, pg_attribute.attoptions as attoptions, pg_attribute.attfdwoptions as attfdwoptions, pg_description.description as description, pg_seclabel.label as seclabel, pg_seclabel.provider as seclabel_provider, pg_temp.format_tablecolumn_aclitems(pg_init_privs.initprivs) as initprivs, pg_init_privs.privtype as initprivs_type from pg_attribute left join pg_description on pg_description.objoid = pg_attribute.attrelid and pg_description.objsubid = pg_attribute.attnum left join pg_seclabel on pg_seclabel.objoid = pg_attribute.attrelid and pg_seclabel.objsubid = pg_attribute.attnum left join pg_init_privs on pg_init_privs.objoid = pg_attribute.attrelid and pg_init_privs.objsubid = pg_attribute.attnum where not starts_with(pg_attribute.attrelid::regclass::text, 'pg_toast') and attnum > 0 and not pg_attribute.attisdropped",
        None,
    )
}
impl ReflectPgAttributeStmt {
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
    ) -> ReflectPgAttributeQuery<'c, 'a, 's, C, ReflectPgAttribute, 0> {
        ReflectPgAttributeQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgAttributeBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgAttributeBorrowed {
                    attrelid: row.try_get(0)?,
                    attname: row.try_get(1)?,
                    atttypid: row.try_get(2)?,
                    attnum: row.try_get(3)?,
                    atttypmod: row.try_get(4)?,
                    attndims: row.try_get(5)?,
                    attcompression: row.try_get(6)?,
                    attnotnull: row.try_get(7)?,
                    atthasdef: row.try_get(8)?,
                    attidentity: row.try_get(9)?,
                    attgenerated: row.try_get(10)?,
                    attisdropped: row.try_get(11)?,
                    attislocal: row.try_get(12)?,
                    attinhcount: row.try_get(13)?,
                    attcollation: row.try_get(14)?,
                    attstattarget: row.try_get(15)?,
                    attacl: row.try_get(16)?,
                    attoptions: row.try_get(17)?,
                    attfdwoptions: row.try_get(18)?,
                    description: row.try_get(19)?,
                    seclabel: row.try_get(20)?,
                    seclabel_provider: row.try_get(21)?,
                    initprivs: row.try_get(22)?,
                    initprivs_type: row.try_get(23)?,
                })
            },
            mapper: |it| ReflectPgAttribute::from(it),
        }
    }
}
pub struct ReflectPgRolesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_roles() -> ReflectPgRolesStmt {
    ReflectPgRolesStmt(
        "select pg_roles.rolname::text as rolname, pg_roles.rolsuper as rolsuper, pg_roles.rolinherit as rolinherit, pg_roles.rolcreaterole as rolcreaterole, pg_roles.rolcreatedb as rolcreatedb, pg_roles.rolcanlogin as rolcanlogin, pg_roles.rolreplication as rolreplication, case when rolconnlimit < 0 then null else rolconnlimit end as rolconnlimit, pg_roles.rolvaliduntil as rolvaliduntil, pg_roles.rolbypassrls as rolbypassrls, pg_roles.rolconfig as rolconfig, pg_shdescription.description as description, pg_shseclabel.label as seclabel, pg_shseclabel.provider as seclabel_provider from pg_roles left join pg_shdescription on pg_shdescription.objoid = pg_roles.oid left join pg_shseclabel on pg_shseclabel.objoid = pg_roles.oid",
        None,
    )
}
impl ReflectPgRolesStmt {
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
    ) -> ReflectPgRolesQuery<'c, 'a, 's, C, ReflectPgRoles, 0> {
        ReflectPgRolesQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgRolesBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgRolesBorrowed {
                    rolname: row.try_get(0)?,
                    rolsuper: row.try_get(1)?,
                    rolinherit: row.try_get(2)?,
                    rolcreaterole: row.try_get(3)?,
                    rolcreatedb: row.try_get(4)?,
                    rolcanlogin: row.try_get(5)?,
                    rolreplication: row.try_get(6)?,
                    rolconnlimit: row.try_get(7)?,
                    rolvaliduntil: row.try_get(8)?,
                    rolbypassrls: row.try_get(9)?,
                    rolconfig: row.try_get(10)?,
                    description: row.try_get(11)?,
                    seclabel: row.try_get(12)?,
                    seclabel_provider: row.try_get(13)?,
                })
            },
            mapper: |it| ReflectPgRoles::from(it),
        }
    }
}
pub struct ReflectPgAuthMembersStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_auth_members() -> ReflectPgAuthMembersStmt {
    ReflectPgAuthMembersStmt(
        "select pg_get_userbyid(pg_auth_members.roleid)::text as roleid, pg_get_userbyid(pg_auth_members.member)::text as member, pg_get_userbyid(pg_auth_members.grantor)::text as grantor, pg_auth_members.admin_option as admin_option, pg_auth_members.inherit_option as inherit_option, pg_auth_members.set_option as set_option from pg_auth_members",
        None,
    )
}
impl ReflectPgAuthMembersStmt {
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
    ) -> ReflectPgAuthMembersQuery<'c, 'a, 's, C, ReflectPgAuthMembers, 0> {
        ReflectPgAuthMembersQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgAuthMembersBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgAuthMembersBorrowed {
                    roleid: row.try_get(0)?,
                    member: row.try_get(1)?,
                    grantor: row.try_get(2)?,
                    admin_option: row.try_get(3)?,
                    inherit_option: row.try_get(4)?,
                    set_option: row.try_get(5)?,
                })
            },
            mapper: |it| ReflectPgAuthMembers::from(it),
        }
    }
}
pub struct ReflectPgCastStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_cast() -> ReflectPgCastStmt {
    ReflectPgCastStmt(
        "select pg_cast.castsource::regtype::text as castsource, pg_cast.casttarget::regtype::text as casttarget, case when pg_cast.castfunc = 0 then null else pg_cast.castfunc::regprocedure::text end as castfunc, pg_cast.castcontext as castcontext, pg_cast.castmethod as castmethod, pg_description.description as description from pg_cast left join pg_description on pg_description.objoid = pg_cast.oid and pg_description.objsubid = 0",
        None,
    )
}
impl ReflectPgCastStmt {
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
    ) -> ReflectPgCastQuery<'c, 'a, 's, C, ReflectPgCast, 0> {
        ReflectPgCastQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ReflectPgCastBorrowed, tokio_postgres::Error> {
                    Ok(ReflectPgCastBorrowed {
                        castsource: row.try_get(0)?,
                        casttarget: row.try_get(1)?,
                        castfunc: row.try_get(2)?,
                        castcontext: row.try_get(3)?,
                        castmethod: row.try_get(4)?,
                        description: row.try_get(5)?,
                    })
                },
            mapper: |it| ReflectPgCast::from(it),
        }
    }
}
pub struct ReflectPgClassStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_class() -> ReflectPgClassStmt {
    ReflectPgClassStmt(
        "select pg_class.oid::regclass::text as oid, pg_class.relname::text as relname, pg_class.relnamespace::regnamespace::text as relnamespace, case when pg_class.reltype = 0 then null else pg_class.reltype::regtype::text end as reltype, case when pg_class.reloftype = 0 then null else pg_class.reloftype::regtype::text end as reloftype, pg_get_userbyid(pg_class.relowner)::text as relowner, relam_pg_am.amname::text as relam, pg_class.relisshared as relisshared, pg_class.relpersistence as relpersistence, pg_class.relkind as relkind, pg_class.relnatts as relnatts, pg_class.relchecks as relchecks, pg_class.relrowsecurity as relrowsecurity, pg_class.relforcerowsecurity as relforcerowsecurity, pg_class.relreplident as relreplident, pg_class.relispartition as relispartition, pg_temp.format_table_aclitems(pg_class.relacl) as relacl, pg_class.reloptions as reloptions, pg_get_expr(relpartbound, pg_class.oid) as relpartbound, pg_description.description as description, pg_seclabel.label as seclabel, pg_seclabel.provider as seclabel_provider, pg_temp.format_table_aclitems(pg_init_privs.initprivs) as initprivs, pg_init_privs.privtype as initprivs_type from pg_class left join pg_am as relam_pg_am on pg_class.relam = relam_pg_am.oid left join pg_description on pg_description.objoid = pg_class.oid and pg_description.objsubid = 0 left join pg_seclabel on pg_seclabel.objoid = pg_class.oid and pg_seclabel.objsubid = 0 left join pg_init_privs on pg_init_privs.objoid = pg_class.oid and pg_init_privs.objsubid = 0 where relnamespace::regnamespace != 'pg_toast'::regnamespace",
        None,
    )
}
impl ReflectPgClassStmt {
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
    ) -> ReflectPgClassQuery<'c, 'a, 's, C, ReflectPgClass, 0> {
        ReflectPgClassQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgClassBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgClassBorrowed {
                    oid: row.try_get(0)?,
                    relname: row.try_get(1)?,
                    relnamespace: row.try_get(2)?,
                    reltype: row.try_get(3)?,
                    reloftype: row.try_get(4)?,
                    relowner: row.try_get(5)?,
                    relam: row.try_get(6)?,
                    relisshared: row.try_get(7)?,
                    relpersistence: row.try_get(8)?,
                    relkind: row.try_get(9)?,
                    relnatts: row.try_get(10)?,
                    relchecks: row.try_get(11)?,
                    relrowsecurity: row.try_get(12)?,
                    relforcerowsecurity: row.try_get(13)?,
                    relreplident: row.try_get(14)?,
                    relispartition: row.try_get(15)?,
                    relacl: row.try_get(16)?,
                    reloptions: row.try_get(17)?,
                    relpartbound: row.try_get(18)?,
                    description: row.try_get(19)?,
                    seclabel: row.try_get(20)?,
                    seclabel_provider: row.try_get(21)?,
                    initprivs: row.try_get(22)?,
                    initprivs_type: row.try_get(23)?,
                })
            },
            mapper: |it| ReflectPgClass::from(it),
        }
    }
}
pub struct ReflectPgCollationStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_collation() -> ReflectPgCollationStmt {
    ReflectPgCollationStmt(
        "select pg_collation.oid::regcollation::text as oid, pg_collation.collname::text as collname, pg_collation.collnamespace::regnamespace::text as collnamespace, pg_get_userbyid(pg_collation.collowner)::text as collowner, pg_collation.collprovider as collprovider, pg_collation.collisdeterministic as collisdeterministic, case when collencoding < 0 then null else pg_encoding_to_char(collencoding)::text end as collencoding, pg_collation.collcollate as collcollate, pg_collation.collctype as collctype, pg_collation.colllocale as colllocale, pg_collation.collicurules as collicurules, pg_description.description as description from pg_collation left join pg_description on pg_description.objoid = pg_collation.oid and pg_description.objsubid = 0",
        None,
    )
}
impl ReflectPgCollationStmt {
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
    ) -> ReflectPgCollationQuery<'c, 'a, 's, C, ReflectPgCollation, 0> {
        ReflectPgCollationQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgCollationBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgCollationBorrowed {
                    oid: row.try_get(0)?,
                    collname: row.try_get(1)?,
                    collnamespace: row.try_get(2)?,
                    collowner: row.try_get(3)?,
                    collprovider: row.try_get(4)?,
                    collisdeterministic: row.try_get(5)?,
                    collencoding: row.try_get(6)?,
                    collcollate: row.try_get(7)?,
                    collctype: row.try_get(8)?,
                    colllocale: row.try_get(9)?,
                    collicurules: row.try_get(10)?,
                    description: row.try_get(11)?,
                })
            },
            mapper: |it| ReflectPgCollation::from(it),
        }
    }
}
pub struct ReflectPgConstraintStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_constraint() -> ReflectPgConstraintStmt {
    ReflectPgConstraintStmt(
        "select pg_constraint.conname::text as conname, pg_constraint.connamespace::regnamespace::text as connamespace, pg_constraint.contype as contype, pg_constraint.condeferrable as condeferrable, pg_constraint.condeferred as condeferred, pg_constraint.convalidated as convalidated, case when pg_constraint.conrelid = 0 then null else pg_constraint.conrelid::regclass::text end as conrelid, case when pg_constraint.contypid = 0 then null else pg_constraint.contypid::regtype::text end as contypid, case when pg_constraint.conindid = 0 then null else pg_constraint.conindid::regclass::text end as conindid, pg_temp.quote_with_namespace(conparentid_pg_namespace.nspname, conparentid_pg_constraint.conname) as conparentid, case when pg_constraint.confrelid = 0 then null else pg_constraint.confrelid::regclass::text end as confrelid, case when pg_constraint.confupdtype = ' ' then null else pg_constraint.confupdtype end as confupdtype, case when pg_constraint.confdeltype = ' ' then null else pg_constraint.confdeltype end as confdeltype, case when pg_constraint.confmatchtype = ' ' then null else pg_constraint.confmatchtype end as confmatchtype, pg_constraint.conislocal as conislocal, pg_constraint.coninhcount as coninhcount, pg_constraint.connoinherit as connoinherit, pg_constraint.conkey as conkey, pg_constraint.confkey as confkey, pg_constraint.conpfeqop::regoperator[]::text[] as conpfeqop, pg_constraint.conppeqop::regoperator[]::text[] as conppeqop, pg_constraint.conffeqop::regoperator[]::text[] as conffeqop, pg_constraint.confdelsetcols as confdelsetcols, pg_constraint.conexclop::regoperator[]::text[] as conexclop, pg_get_expr(pg_constraint.conbin, pg_constraint.conrelid) as conbin, pg_description.description as description from pg_constraint left join pg_constraint as conparentid_pg_constraint on pg_constraint.conparentid = conparentid_pg_constraint.oid left join pg_namespace as conparentid_pg_namespace on conparentid_pg_constraint.connamespace = conparentid_pg_namespace.oid left join pg_description on pg_description.objoid = pg_constraint.oid and pg_description.objsubid = 0 where (pg_constraint.conkey is null or not (0 >= any(pg_constraint.conkey))) and (pg_constraint.confkey is null or not (0 >= any(pg_constraint.confkey))) and (pg_constraint.confdelsetcols is null or not (0 >= any(pg_constraint.confdelsetcols)))",
        None,
    )
}
impl ReflectPgConstraintStmt {
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
    ) -> ReflectPgConstraintQuery<'c, 'a, 's, C, ReflectPgConstraint, 0> {
        ReflectPgConstraintQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgConstraintBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgConstraintBorrowed {
                    conname: row.try_get(0)?,
                    connamespace: row.try_get(1)?,
                    contype: row.try_get(2)?,
                    condeferrable: row.try_get(3)?,
                    condeferred: row.try_get(4)?,
                    convalidated: row.try_get(5)?,
                    conrelid: row.try_get(6)?,
                    contypid: row.try_get(7)?,
                    conindid: row.try_get(8)?,
                    conparentid: row.try_get(9)?,
                    confrelid: row.try_get(10)?,
                    confupdtype: row.try_get(11)?,
                    confdeltype: row.try_get(12)?,
                    confmatchtype: row.try_get(13)?,
                    conislocal: row.try_get(14)?,
                    coninhcount: row.try_get(15)?,
                    connoinherit: row.try_get(16)?,
                    conkey: row.try_get(17)?,
                    confkey: row.try_get(18)?,
                    conpfeqop: row.try_get(19)?,
                    conppeqop: row.try_get(20)?,
                    conffeqop: row.try_get(21)?,
                    confdelsetcols: row.try_get(22)?,
                    conexclop: row.try_get(23)?,
                    conbin: row.try_get(24)?,
                    description: row.try_get(25)?,
                })
            },
            mapper: |it| ReflectPgConstraint::from(it),
        }
    }
}
pub struct ReflectPgConversionStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_conversion() -> ReflectPgConversionStmt {
    ReflectPgConversionStmt(
        "select pg_conversion.conname::text as conname, pg_conversion.connamespace::regnamespace::text as connamespace, pg_get_userbyid(pg_conversion.conowner)::text as conowner, pg_encoding_to_char(conforencoding)::text as conforencoding, pg_encoding_to_char(contoencoding)::text as contoencoding, conproc::regprocedure::text as conproc, pg_conversion.condefault as condefault, pg_description.description as description from pg_conversion left join pg_description on pg_description.objoid = pg_conversion.oid and pg_description.objsubid = 0",
        None,
    )
}
impl ReflectPgConversionStmt {
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
    ) -> ReflectPgConversionQuery<'c, 'a, 's, C, ReflectPgConversion, 0> {
        ReflectPgConversionQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgConversionBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgConversionBorrowed {
                    conname: row.try_get(0)?,
                    connamespace: row.try_get(1)?,
                    conowner: row.try_get(2)?,
                    conforencoding: row.try_get(3)?,
                    contoencoding: row.try_get(4)?,
                    conproc: row.try_get(5)?,
                    condefault: row.try_get(6)?,
                    description: row.try_get(7)?,
                })
            },
            mapper: |it| ReflectPgConversion::from(it),
        }
    }
}
pub struct ReflectPgDatabaseStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_database() -> ReflectPgDatabaseStmt {
    ReflectPgDatabaseStmt(
        "select pg_database.datname::text as datname, pg_get_userbyid(pg_database.datdba)::text as datdba, pg_encoding_to_char(encoding)::text as encoding, pg_database.datlocprovider as datlocprovider, pg_database.datistemplate as datistemplate, pg_database.datallowconn as datallowconn, case when datconnlimit < 0 then null else datconnlimit end as datconnlimit, pg_database.datcollate as datcollate, pg_database.datctype as datctype, pg_database.datlocale as datlocale, pg_database.daticurules as daticurules, pg_temp.format_db_aclitems(pg_database.datacl) as datacl, pg_shdescription.description as description, pg_shseclabel.label as seclabel, pg_shseclabel.provider as seclabel_provider from pg_database left join pg_shdescription on pg_shdescription.objoid = pg_database.oid left join pg_shseclabel on pg_shseclabel.objoid = pg_database.oid where pg_database.datname = current_database()",
        None,
    )
}
impl ReflectPgDatabaseStmt {
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
    ) -> ReflectPgDatabaseQuery<'c, 'a, 's, C, ReflectPgDatabase, 0> {
        ReflectPgDatabaseQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgDatabaseBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgDatabaseBorrowed {
                    datname: row.try_get(0)?,
                    datdba: row.try_get(1)?,
                    encoding: row.try_get(2)?,
                    datlocprovider: row.try_get(3)?,
                    datistemplate: row.try_get(4)?,
                    datallowconn: row.try_get(5)?,
                    datconnlimit: row.try_get(6)?,
                    datcollate: row.try_get(7)?,
                    datctype: row.try_get(8)?,
                    datlocale: row.try_get(9)?,
                    daticurules: row.try_get(10)?,
                    datacl: row.try_get(11)?,
                    description: row.try_get(12)?,
                    seclabel: row.try_get(13)?,
                    seclabel_provider: row.try_get(14)?,
                })
            },
            mapper: |it| ReflectPgDatabase::from(it),
        }
    }
}
pub struct ReflectPgDefaultAclStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_default_acl() -> ReflectPgDefaultAclStmt {
    ReflectPgDefaultAclStmt(
        "select pg_get_userbyid(pg_default_acl.defaclrole)::text as defaclrole, case when pg_default_acl.defaclnamespace = 0 then null else pg_default_acl.defaclnamespace::regnamespace::text end as defaclnamespace, pg_default_acl.defaclobjtype as defaclobjtype, pg_temp.format_acldefault_aclitems(pg_default_acl.defaclacl) as defaclacl from pg_default_acl",
        None,
    )
}
impl ReflectPgDefaultAclStmt {
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
    ) -> ReflectPgDefaultAclQuery<'c, 'a, 's, C, ReflectPgDefaultAcl, 0> {
        ReflectPgDefaultAclQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgDefaultAclBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgDefaultAclBorrowed {
                    defaclrole: row.try_get(0)?,
                    defaclnamespace: row.try_get(1)?,
                    defaclobjtype: row.try_get(2)?,
                    defaclacl: row.try_get(3)?,
                })
            },
            mapper: |it| ReflectPgDefaultAcl::from(it),
        }
    }
}
pub struct ReflectPgEventTriggerStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_event_trigger() -> ReflectPgEventTriggerStmt {
    ReflectPgEventTriggerStmt(
        "select pg_event_trigger.evtname::text as evtname, pg_event_trigger.evtevent::text as evtevent, pg_get_userbyid(pg_event_trigger.evtowner)::text as evtowner, pg_event_trigger.evtfoid::regprocedure::text as evtfoid, pg_event_trigger.evtenabled as evtenabled, pg_event_trigger.evttags as evttags, pg_description.description as description, pg_seclabel.label as seclabel, pg_seclabel.provider as seclabel_provider from pg_event_trigger left join pg_description on pg_description.objoid = pg_event_trigger.oid and pg_description.objsubid = 0 left join pg_seclabel on pg_seclabel.objoid = pg_event_trigger.oid and pg_seclabel.objsubid = 0",
        None,
    )
}
impl ReflectPgEventTriggerStmt {
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
    ) -> ReflectPgEventTriggerQuery<'c, 'a, 's, C, ReflectPgEventTrigger, 0> {
        ReflectPgEventTriggerQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgEventTriggerBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgEventTriggerBorrowed {
                    evtname: row.try_get(0)?,
                    evtevent: row.try_get(1)?,
                    evtowner: row.try_get(2)?,
                    evtfoid: row.try_get(3)?,
                    evtenabled: row.try_get(4)?,
                    evttags: row.try_get(5)?,
                    description: row.try_get(6)?,
                    seclabel: row.try_get(7)?,
                    seclabel_provider: row.try_get(8)?,
                })
            },
            mapper: |it| ReflectPgEventTrigger::from(it),
        }
    }
}
pub struct ReflectPgExtensionStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_extension() -> ReflectPgExtensionStmt {
    ReflectPgExtensionStmt(
        "select pg_extension.extname::text as extname, pg_get_userbyid(pg_extension.extowner)::text as extowner, pg_extension.extnamespace::regnamespace::text as extnamespace, pg_extension.extrelocatable as extrelocatable, pg_extension.extversion as extversion, pg_extension.extconfig::regclass[]::text[] as extconfig, pg_extension.extcondition as extcondition, pg_description.description as description from pg_extension left join pg_description on pg_description.objoid = pg_extension.oid and pg_description.objsubid = 0",
        None,
    )
}
impl ReflectPgExtensionStmt {
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
    ) -> ReflectPgExtensionQuery<'c, 'a, 's, C, ReflectPgExtension, 0> {
        ReflectPgExtensionQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgExtensionBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgExtensionBorrowed {
                    extname: row.try_get(0)?,
                    extowner: row.try_get(1)?,
                    extnamespace: row.try_get(2)?,
                    extrelocatable: row.try_get(3)?,
                    extversion: row.try_get(4)?,
                    extconfig: row.try_get(5)?,
                    extcondition: row.try_get(6)?,
                    description: row.try_get(7)?,
                })
            },
            mapper: |it| ReflectPgExtension::from(it),
        }
    }
}
pub struct ReflectPgForeignDataWrapperStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_foreign_data_wrapper() -> ReflectPgForeignDataWrapperStmt {
    ReflectPgForeignDataWrapperStmt(
        "select pg_foreign_data_wrapper.fdwname::text as fdwname, pg_get_userbyid(pg_foreign_data_wrapper.fdwowner)::text as fdwowner, case when pg_foreign_data_wrapper.fdwhandler = 0 then null else pg_foreign_data_wrapper.fdwhandler::regprocedure::text end as fdwhandler, case when pg_foreign_data_wrapper.fdwvalidator = 0 then null else pg_foreign_data_wrapper.fdwvalidator::regprocedure::text end as fdwvalidator, pg_temp.format_foreigndatawrapper_aclitems(pg_foreign_data_wrapper.fdwacl) as fdwacl, pg_foreign_data_wrapper.fdwoptions as fdwoptions, pg_description.description as description, pg_temp.format_foreigndatawrapper_aclitems(pg_init_privs.initprivs) as initprivs, pg_init_privs.privtype as initprivs_type from pg_foreign_data_wrapper left join pg_description on pg_description.objoid = pg_foreign_data_wrapper.oid and pg_description.objsubid = 0 left join pg_init_privs on pg_init_privs.objoid = pg_foreign_data_wrapper.oid and pg_init_privs.objsubid = 0",
        None,
    )
}
impl ReflectPgForeignDataWrapperStmt {
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
    ) -> ReflectPgForeignDataWrapperQuery<'c, 'a, 's, C, ReflectPgForeignDataWrapper, 0> {
        ReflectPgForeignDataWrapperQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgForeignDataWrapperBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgForeignDataWrapperBorrowed {
                    fdwname: row.try_get(0)?,
                    fdwowner: row.try_get(1)?,
                    fdwhandler: row.try_get(2)?,
                    fdwvalidator: row.try_get(3)?,
                    fdwacl: row.try_get(4)?,
                    fdwoptions: row.try_get(5)?,
                    description: row.try_get(6)?,
                    initprivs: row.try_get(7)?,
                    initprivs_type: row.try_get(8)?,
                })
            },
            mapper: |it| ReflectPgForeignDataWrapper::from(it),
        }
    }
}
pub struct ReflectPgForeignServerStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_foreign_server() -> ReflectPgForeignServerStmt {
    ReflectPgForeignServerStmt(
        "select pg_foreign_server.srvname::text as srvname, pg_get_userbyid(pg_foreign_server.srvowner)::text as srvowner, srvfdw_pg_foreign_data_wrapper.fdwname::text as srvfdw, pg_foreign_server.srvtype as srvtype, pg_foreign_server.srvversion as srvversion, pg_temp.format_foreignserver_aclitems(pg_foreign_server.srvacl) as srvacl, pg_foreign_server.srvoptions as srvoptions, pg_description.description as description, pg_temp.format_foreignserver_aclitems(pg_init_privs.initprivs) as initprivs, pg_init_privs.privtype as initprivs_type from pg_foreign_server join pg_foreign_data_wrapper as srvfdw_pg_foreign_data_wrapper on pg_foreign_server.srvfdw = srvfdw_pg_foreign_data_wrapper.oid left join pg_description on pg_description.objoid = pg_foreign_server.oid and pg_description.objsubid = 0 left join pg_init_privs on pg_init_privs.objoid = pg_foreign_server.oid and pg_init_privs.objsubid = 0",
        None,
    )
}
impl ReflectPgForeignServerStmt {
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
    ) -> ReflectPgForeignServerQuery<'c, 'a, 's, C, ReflectPgForeignServer, 0> {
        ReflectPgForeignServerQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgForeignServerBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgForeignServerBorrowed {
                    srvname: row.try_get(0)?,
                    srvowner: row.try_get(1)?,
                    srvfdw: row.try_get(2)?,
                    srvtype: row.try_get(3)?,
                    srvversion: row.try_get(4)?,
                    srvacl: row.try_get(5)?,
                    srvoptions: row.try_get(6)?,
                    description: row.try_get(7)?,
                    initprivs: row.try_get(8)?,
                    initprivs_type: row.try_get(9)?,
                })
            },
            mapper: |it| ReflectPgForeignServer::from(it),
        }
    }
}
pub struct ReflectPgForeignTableStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_foreign_table() -> ReflectPgForeignTableStmt {
    ReflectPgForeignTableStmt(
        "select pg_foreign_table.ftrelid::regclass::text as ftrelid, ftserver_pg_foreign_server.srvname::text as ftserver, pg_foreign_table.ftoptions as ftoptions from pg_foreign_table join pg_foreign_server as ftserver_pg_foreign_server on pg_foreign_table.ftserver = ftserver_pg_foreign_server.oid",
        None,
    )
}
impl ReflectPgForeignTableStmt {
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
    ) -> ReflectPgForeignTableQuery<'c, 'a, 's, C, ReflectPgForeignTable, 0> {
        ReflectPgForeignTableQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgForeignTableBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgForeignTableBorrowed {
                    ftrelid: row.try_get(0)?,
                    ftserver: row.try_get(1)?,
                    ftoptions: row.try_get(2)?,
                })
            },
            mapper: |it| ReflectPgForeignTable::from(it),
        }
    }
}
pub struct ReflectPgIndexStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_index() -> ReflectPgIndexStmt {
    ReflectPgIndexStmt(
        "select pg_index.indexrelid::regclass::text as indexrelid, pg_index.indrelid::regclass::text as indrelid, pg_index.indnatts as indnatts, pg_index.indnkeyatts as indnkeyatts, pg_index.indisunique as indisunique, pg_index.indnullsnotdistinct as indnullsnotdistinct, pg_index.indisprimary as indisprimary, pg_index.indisexclusion as indisexclusion, pg_index.indimmediate as indimmediate, pg_index.indisreplident as indisreplident, pg_index.indkey as indkey, pg_temp.format_pg_collation_oidvector(pg_index.indcollation) as indcollation, pg_temp.format_pg_opclass_oidvector(pg_index.indclass) as indclass, pg_index.indoption::int2[] as indoption, pg_get_expr(pg_index.indexprs, pg_index.indrelid) as indexprs, pg_get_expr(pg_index.indpred, pg_index.indrelid) as indpred from pg_index where not starts_with(pg_index.indrelid::regclass::text, 'pg_toast')",
        None,
    )
}
impl ReflectPgIndexStmt {
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
    ) -> ReflectPgIndexQuery<'c, 'a, 's, C, ReflectPgIndex, 0> {
        ReflectPgIndexQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgIndexBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgIndexBorrowed {
                    indexrelid: row.try_get(0)?,
                    indrelid: row.try_get(1)?,
                    indnatts: row.try_get(2)?,
                    indnkeyatts: row.try_get(3)?,
                    indisunique: row.try_get(4)?,
                    indnullsnotdistinct: row.try_get(5)?,
                    indisprimary: row.try_get(6)?,
                    indisexclusion: row.try_get(7)?,
                    indimmediate: row.try_get(8)?,
                    indisreplident: row.try_get(9)?,
                    indkey: row.try_get(10)?,
                    indcollation: row.try_get(11)?,
                    indclass: row.try_get(12)?,
                    indoption: row.try_get(13)?,
                    indexprs: row.try_get(14)?,
                    indpred: row.try_get(15)?,
                })
            },
            mapper: |it| ReflectPgIndex::from(it),
        }
    }
}
pub struct ReflectPgInheritsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_inherits() -> ReflectPgInheritsStmt {
    ReflectPgInheritsStmt(
        "select pg_inherits.inhrelid::regclass::text as inhrelid, pg_inherits.inhparent::regclass::text as inhparent, pg_inherits.inhseqno as inhseqno from pg_inherits",
        None,
    )
}
impl ReflectPgInheritsStmt {
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
    ) -> ReflectPgInheritsQuery<'c, 'a, 's, C, ReflectPgInherits, 0> {
        ReflectPgInheritsQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgInheritsBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgInheritsBorrowed {
                    inhrelid: row.try_get(0)?,
                    inhparent: row.try_get(1)?,
                    inhseqno: row.try_get(2)?,
                })
            },
            mapper: |it| ReflectPgInherits::from(it),
        }
    }
}
pub struct ReflectPgLanguageStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_language() -> ReflectPgLanguageStmt {
    ReflectPgLanguageStmt(
        "select pg_language.lanname::text as lanname, pg_get_userbyid(pg_language.lanowner)::text as lanowner, pg_language.lanispl as lanispl, pg_language.lanpltrusted as lanpltrusted, case when pg_language.lanplcallfoid = 0 then null else pg_language.lanplcallfoid::regprocedure::text end as lanplcallfoid, case when pg_language.laninline = 0 then null else pg_language.laninline::regprocedure::text end as laninline, case when pg_language.lanvalidator = 0 then null else pg_language.lanvalidator::regprocedure::text end as lanvalidator, pg_temp.format_language_aclitems(pg_language.lanacl) as lanacl, pg_description.description as description, pg_seclabel.label as seclabel, pg_seclabel.provider as seclabel_provider, pg_temp.format_language_aclitems(pg_init_privs.initprivs) as initprivs, pg_init_privs.privtype as initprivs_type from pg_language left join pg_description on pg_description.objoid = pg_language.oid and pg_description.objsubid = 0 left join pg_seclabel on pg_seclabel.objoid = pg_language.oid and pg_seclabel.objsubid = 0 left join pg_init_privs on pg_init_privs.objoid = pg_language.oid and pg_init_privs.objsubid = 0",
        None,
    )
}
impl ReflectPgLanguageStmt {
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
    ) -> ReflectPgLanguageQuery<'c, 'a, 's, C, ReflectPgLanguage, 0> {
        ReflectPgLanguageQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgLanguageBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgLanguageBorrowed {
                    lanname: row.try_get(0)?,
                    lanowner: row.try_get(1)?,
                    lanispl: row.try_get(2)?,
                    lanpltrusted: row.try_get(3)?,
                    lanplcallfoid: row.try_get(4)?,
                    laninline: row.try_get(5)?,
                    lanvalidator: row.try_get(6)?,
                    lanacl: row.try_get(7)?,
                    description: row.try_get(8)?,
                    seclabel: row.try_get(9)?,
                    seclabel_provider: row.try_get(10)?,
                    initprivs: row.try_get(11)?,
                    initprivs_type: row.try_get(12)?,
                })
            },
            mapper: |it| ReflectPgLanguage::from(it),
        }
    }
}
pub struct ReflectPgNamespaceStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_namespace() -> ReflectPgNamespaceStmt {
    ReflectPgNamespaceStmt(
        "select pg_namespace.nspname::text as nspname, pg_get_userbyid(pg_namespace.nspowner)::text as nspowner, pg_temp.format_schema_aclitems(pg_namespace.nspacl) as nspacl, pg_description.description as description, pg_seclabel.label as seclabel, pg_seclabel.provider as seclabel_provider, pg_temp.format_schema_aclitems(pg_init_privs.initprivs) as initprivs, pg_init_privs.privtype as initprivs_type from pg_namespace left join pg_description on pg_description.objoid = pg_namespace.oid and pg_description.objsubid = 0 left join pg_seclabel on pg_seclabel.objoid = pg_namespace.oid and pg_seclabel.objsubid = 0 left join pg_init_privs on pg_init_privs.objoid = pg_namespace.oid and pg_init_privs.objsubid = 0 where not starts_with(nspname, 'pg_temp') and not starts_with(nspname, 'pg_toast')",
        None,
    )
}
impl ReflectPgNamespaceStmt {
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
    ) -> ReflectPgNamespaceQuery<'c, 'a, 's, C, ReflectPgNamespace, 0> {
        ReflectPgNamespaceQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgNamespaceBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgNamespaceBorrowed {
                    nspname: row.try_get(0)?,
                    nspowner: row.try_get(1)?,
                    nspacl: row.try_get(2)?,
                    description: row.try_get(3)?,
                    seclabel: row.try_get(4)?,
                    seclabel_provider: row.try_get(5)?,
                    initprivs: row.try_get(6)?,
                    initprivs_type: row.try_get(7)?,
                })
            },
            mapper: |it| ReflectPgNamespace::from(it),
        }
    }
}
pub struct ReflectPgOpclassStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_opclass() -> ReflectPgOpclassStmt {
    ReflectPgOpclassStmt(
        "select opcmethod_pg_am.amname::text as opcmethod, pg_opclass.opcname::text as opcname, pg_opclass.opcnamespace::regnamespace::text as opcnamespace, pg_get_userbyid(pg_opclass.opcowner)::text as opcowner, pg_temp.quote_with_namespace(opcfamily_pg_namespace.nspname, opcfamily_pg_opfamily.opfname) as opcfamily, pg_opclass.opcintype::regtype::text as opcintype, pg_opclass.opcdefault as opcdefault, case when pg_opclass.opckeytype = 0 then null else pg_opclass.opckeytype::regtype::text end as opckeytype, pg_description.description as description from pg_opclass join pg_am as opcmethod_pg_am on pg_opclass.opcmethod = opcmethod_pg_am.oid join pg_opfamily as opcfamily_pg_opfamily on pg_opclass.opcfamily = opcfamily_pg_opfamily.oid join pg_namespace as opcfamily_pg_namespace on opcfamily_pg_opfamily.opfnamespace = opcfamily_pg_namespace.oid left join pg_description on pg_description.objoid = pg_opclass.oid and pg_description.objsubid = 0",
        None,
    )
}
impl ReflectPgOpclassStmt {
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
    ) -> ReflectPgOpclassQuery<'c, 'a, 's, C, ReflectPgOpclass, 0> {
        ReflectPgOpclassQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgOpclassBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgOpclassBorrowed {
                    opcmethod: row.try_get(0)?,
                    opcname: row.try_get(1)?,
                    opcnamespace: row.try_get(2)?,
                    opcowner: row.try_get(3)?,
                    opcfamily: row.try_get(4)?,
                    opcintype: row.try_get(5)?,
                    opcdefault: row.try_get(6)?,
                    opckeytype: row.try_get(7)?,
                    description: row.try_get(8)?,
                })
            },
            mapper: |it| ReflectPgOpclass::from(it),
        }
    }
}
pub struct ReflectPgOperatorStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_operator() -> ReflectPgOperatorStmt {
    ReflectPgOperatorStmt(
        "select pg_operator.oid::regoperator::text as oid, pg_operator.oprname::text as oprname, pg_operator.oprnamespace::regnamespace::text as oprnamespace, pg_get_userbyid(pg_operator.oprowner)::text as oprowner, pg_operator.oprkind as oprkind, pg_operator.oprcanmerge as oprcanmerge, pg_operator.oprcanhash as oprcanhash, case when pg_operator.oprleft = 0 then null else pg_operator.oprleft::regtype::text end as oprleft, pg_operator.oprright::regtype::text as oprright, case when pg_operator.oprresult = 0 then null else pg_operator.oprresult::regtype::text end as oprresult, case when pg_operator.oprcom = 0 then null else pg_operator.oprcom::regoperator::text end as oprcom, case when pg_operator.oprnegate = 0 then null else pg_operator.oprnegate::regoperator::text end as oprnegate, case when oprcode = 0 then null else oprcode::regprocedure::text end as oprcode, case when oprrest = 0 then null else oprrest::regprocedure::text end as oprrest, case when oprjoin = 0 then null else oprjoin::regprocedure::text end as oprjoin, pg_description.description as description from pg_operator left join pg_description on pg_description.objoid = pg_operator.oid and pg_description.objsubid = 0",
        None,
    )
}
impl ReflectPgOperatorStmt {
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
    ) -> ReflectPgOperatorQuery<'c, 'a, 's, C, ReflectPgOperator, 0> {
        ReflectPgOperatorQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgOperatorBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgOperatorBorrowed {
                    oid: row.try_get(0)?,
                    oprname: row.try_get(1)?,
                    oprnamespace: row.try_get(2)?,
                    oprowner: row.try_get(3)?,
                    oprkind: row.try_get(4)?,
                    oprcanmerge: row.try_get(5)?,
                    oprcanhash: row.try_get(6)?,
                    oprleft: row.try_get(7)?,
                    oprright: row.try_get(8)?,
                    oprresult: row.try_get(9)?,
                    oprcom: row.try_get(10)?,
                    oprnegate: row.try_get(11)?,
                    oprcode: row.try_get(12)?,
                    oprrest: row.try_get(13)?,
                    oprjoin: row.try_get(14)?,
                    description: row.try_get(15)?,
                })
            },
            mapper: |it| ReflectPgOperator::from(it),
        }
    }
}
pub struct ReflectPgOpfamilyStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_opfamily() -> ReflectPgOpfamilyStmt {
    ReflectPgOpfamilyStmt(
        "select opfmethod_pg_am.amname::text as opfmethod, pg_opfamily.opfname::text as opfname, pg_opfamily.opfnamespace::regnamespace::text as opfnamespace, pg_get_userbyid(pg_opfamily.opfowner)::text as opfowner, pg_description.description as description from pg_opfamily join pg_am as opfmethod_pg_am on pg_opfamily.opfmethod = opfmethod_pg_am.oid left join pg_description on pg_description.objoid = pg_opfamily.oid and pg_description.objsubid = 0",
        None,
    )
}
impl ReflectPgOpfamilyStmt {
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
    ) -> ReflectPgOpfamilyQuery<'c, 'a, 's, C, ReflectPgOpfamily, 0> {
        ReflectPgOpfamilyQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgOpfamilyBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgOpfamilyBorrowed {
                    opfmethod: row.try_get(0)?,
                    opfname: row.try_get(1)?,
                    opfnamespace: row.try_get(2)?,
                    opfowner: row.try_get(3)?,
                    description: row.try_get(4)?,
                })
            },
            mapper: |it| ReflectPgOpfamily::from(it),
        }
    }
}
pub struct ReflectPgParameterAclStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_parameter_acl() -> ReflectPgParameterAclStmt {
    ReflectPgParameterAclStmt(
        "select pg_parameter_acl.parname as parname, pg_temp.format_parameter_aclitems(pg_parameter_acl.paracl) as paracl, pg_temp.format_parameter_aclitems(pg_init_privs.initprivs) as initprivs, pg_init_privs.privtype as initprivs_type from pg_parameter_acl left join pg_init_privs on pg_init_privs.objoid = pg_parameter_acl.oid and pg_init_privs.objsubid = 0",
        None,
    )
}
impl ReflectPgParameterAclStmt {
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
    ) -> ReflectPgParameterAclQuery<'c, 'a, 's, C, ReflectPgParameterAcl, 0> {
        ReflectPgParameterAclQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgParameterAclBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgParameterAclBorrowed {
                    parname: row.try_get(0)?,
                    paracl: row.try_get(1)?,
                    initprivs: row.try_get(2)?,
                    initprivs_type: row.try_get(3)?,
                })
            },
            mapper: |it| ReflectPgParameterAcl::from(it),
        }
    }
}
pub struct ReflectPgPartitionedTableStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_partitioned_table() -> ReflectPgPartitionedTableStmt {
    ReflectPgPartitionedTableStmt(
        "select pg_partitioned_table.partrelid::regclass::text as partrelid, pg_partitioned_table.partstrat as partstrat, pg_partitioned_table.partnatts as partnatts, case when pg_partitioned_table.partdefid = 0 then null else pg_partitioned_table.partdefid::regclass::text end as partdefid, pg_partitioned_table.partattrs as partattrs, pg_temp.format_pg_opclass_oidvector(pg_partitioned_table.partclass) as partclass, pg_temp.format_pg_collation_oidvector(pg_partitioned_table.partcollation) as partcollation, pg_get_expr(pg_partitioned_table.partexprs, pg_partitioned_table.partrelid) as partexprs from pg_partitioned_table",
        None,
    )
}
impl ReflectPgPartitionedTableStmt {
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
    ) -> ReflectPgPartitionedTableQuery<'c, 'a, 's, C, ReflectPgPartitionedTable, 0> {
        ReflectPgPartitionedTableQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgPartitionedTableBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgPartitionedTableBorrowed {
                    partrelid: row.try_get(0)?,
                    partstrat: row.try_get(1)?,
                    partnatts: row.try_get(2)?,
                    partdefid: row.try_get(3)?,
                    partattrs: row.try_get(4)?,
                    partclass: row.try_get(5)?,
                    partcollation: row.try_get(6)?,
                    partexprs: row.try_get(7)?,
                })
            },
            mapper: |it| ReflectPgPartitionedTable::from(it),
        }
    }
}
pub struct ReflectPgPolicyStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_policy() -> ReflectPgPolicyStmt {
    ReflectPgPolicyStmt(
        "select pg_policy.polname::text as polname, pg_policy.polrelid::regclass::text as polrelid, pg_policy.polcmd as polcmd, pg_policy.polpermissive as polpermissive, pg_temp.format_role_oid_array(pg_policy.polroles) as polroles, pg_get_expr(pg_policy.polqual, pg_policy.polrelid) as polqual, pg_get_expr(pg_policy.polwithcheck, pg_policy.polrelid) as polwithcheck, pg_description.description as description from pg_policy left join pg_description on pg_description.objoid = pg_policy.oid and pg_description.objsubid = 0",
        None,
    )
}
impl ReflectPgPolicyStmt {
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
    ) -> ReflectPgPolicyQuery<'c, 'a, 's, C, ReflectPgPolicy, 0> {
        ReflectPgPolicyQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgPolicyBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgPolicyBorrowed {
                    polname: row.try_get(0)?,
                    polrelid: row.try_get(1)?,
                    polcmd: row.try_get(2)?,
                    polpermissive: row.try_get(3)?,
                    polroles: row.try_get(4)?,
                    polqual: row.try_get(5)?,
                    polwithcheck: row.try_get(6)?,
                    description: row.try_get(7)?,
                })
            },
            mapper: |it| ReflectPgPolicy::from(it),
        }
    }
}
pub struct ReflectPgPublicationStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_publication() -> ReflectPgPublicationStmt {
    ReflectPgPublicationStmt(
        "select pg_publication.pubname::text as pubname, pg_get_userbyid(pg_publication.pubowner)::text as pubowner, pg_publication.puballtables as puballtables, pg_publication.pubinsert as pubinsert, pg_publication.pubupdate as pubupdate, pg_publication.pubdelete as pubdelete, pg_publication.pubtruncate as pubtruncate, pg_publication.pubviaroot as pubviaroot, pg_description.description as description, pg_seclabel.label as seclabel, pg_seclabel.provider as seclabel_provider from pg_publication left join pg_description on pg_description.objoid = pg_publication.oid and pg_description.objsubid = 0 left join pg_seclabel on pg_seclabel.objoid = pg_publication.oid and pg_seclabel.objsubid = 0",
        None,
    )
}
impl ReflectPgPublicationStmt {
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
    ) -> ReflectPgPublicationQuery<'c, 'a, 's, C, ReflectPgPublication, 0> {
        ReflectPgPublicationQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgPublicationBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgPublicationBorrowed {
                    pubname: row.try_get(0)?,
                    pubowner: row.try_get(1)?,
                    puballtables: row.try_get(2)?,
                    pubinsert: row.try_get(3)?,
                    pubupdate: row.try_get(4)?,
                    pubdelete: row.try_get(5)?,
                    pubtruncate: row.try_get(6)?,
                    pubviaroot: row.try_get(7)?,
                    description: row.try_get(8)?,
                    seclabel: row.try_get(9)?,
                    seclabel_provider: row.try_get(10)?,
                })
            },
            mapper: |it| ReflectPgPublication::from(it),
        }
    }
}
pub struct ReflectPgPublicationNamespaceStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_publication_namespace() -> ReflectPgPublicationNamespaceStmt {
    ReflectPgPublicationNamespaceStmt(
        "select pnpubid_pg_publication.pubname::text as pnpubid, pg_publication_namespace.pnnspid::regnamespace::text as pnnspid from pg_publication_namespace join pg_publication as pnpubid_pg_publication on pg_publication_namespace.pnpubid = pnpubid_pg_publication.oid",
        None,
    )
}
impl ReflectPgPublicationNamespaceStmt {
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
    ) -> ReflectPgPublicationNamespaceQuery<'c, 'a, 's, C, ReflectPgPublicationNamespace, 0> {
        ReflectPgPublicationNamespaceQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgPublicationNamespaceBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgPublicationNamespaceBorrowed {
                    pnpubid: row.try_get(0)?,
                    pnnspid: row.try_get(1)?,
                })
            },
            mapper: |it| ReflectPgPublicationNamespace::from(it),
        }
    }
}
pub struct ReflectPgPublicationRelStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_publication_rel() -> ReflectPgPublicationRelStmt {
    ReflectPgPublicationRelStmt(
        "select prpubid_pg_publication.pubname::text as prpubid, pg_publication_rel.prrelid::regclass::text as prrelid, pg_get_expr(pg_publication_rel.prqual, pg_publication_rel.prrelid) as prqual, pg_publication_rel.prattrs as prattrs from pg_publication_rel join pg_publication as prpubid_pg_publication on pg_publication_rel.prpubid = prpubid_pg_publication.oid",
        None,
    )
}
impl ReflectPgPublicationRelStmt {
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
    ) -> ReflectPgPublicationRelQuery<'c, 'a, 's, C, ReflectPgPublicationRel, 0> {
        ReflectPgPublicationRelQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgPublicationRelBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgPublicationRelBorrowed {
                    prpubid: row.try_get(0)?,
                    prrelid: row.try_get(1)?,
                    prqual: row.try_get(2)?,
                    prattrs: row.try_get(3)?,
                })
            },
            mapper: |it| ReflectPgPublicationRel::from(it),
        }
    }
}
pub struct ReflectPgRangeStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_range() -> ReflectPgRangeStmt {
    ReflectPgRangeStmt(
        "select pg_range.rngtypid::regtype::text as rngtypid, pg_range.rngsubtype::regtype::text as rngsubtype, pg_range.rngmultitypid::regtype::text as rngmultitypid, case when pg_range.rngcollation = 0 then null else pg_range.rngcollation::regcollation::text end as rngcollation, pg_temp.quote_with_namespace(rngsubopc_pg_namespace.nspname, rngsubopc_pg_opclass.opcname) as rngsubopc, case when rngcanonical = 0 then null else rngcanonical::regprocedure::text end as rngcanonical, case when rngsubdiff = 0 then null else rngsubdiff::regprocedure::text end as rngsubdiff from pg_range join pg_opclass as rngsubopc_pg_opclass on pg_range.rngsubopc = rngsubopc_pg_opclass.oid join pg_namespace as rngsubopc_pg_namespace on rngsubopc_pg_opclass.opcnamespace = rngsubopc_pg_namespace.oid",
        None,
    )
}
impl ReflectPgRangeStmt {
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
    ) -> ReflectPgRangeQuery<'c, 'a, 's, C, ReflectPgRange, 0> {
        ReflectPgRangeQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgRangeBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgRangeBorrowed {
                    rngtypid: row.try_get(0)?,
                    rngsubtype: row.try_get(1)?,
                    rngmultitypid: row.try_get(2)?,
                    rngcollation: row.try_get(3)?,
                    rngsubopc: row.try_get(4)?,
                    rngcanonical: row.try_get(5)?,
                    rngsubdiff: row.try_get(6)?,
                })
            },
            mapper: |it| ReflectPgRange::from(it),
        }
    }
}
pub struct ReflectPgRulesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_rules() -> ReflectPgRulesStmt {
    ReflectPgRulesStmt(
        "select pg_rules.schemaname::text as schemaname, pg_temp.quote_with_namespace(pg_rules.schemaname, pg_rules.tablename) as tablename, pg_rules.rulename::text as rulename, pg_rules.definition as definition, pg_description.description as description from pg_rules join pg_rewrite on pg_rewrite.ev_type != '1' and pg_rules.rulename = pg_rewrite.rulename and (quote_ident(pg_rules.schemaname) || '.' || quote_ident(pg_rules.tablename))::regclass::oid = pg_rewrite.ev_class left join pg_description on pg_description.objoid = pg_rewrite.oid and pg_description.objsubid = 0",
        None,
    )
}
impl ReflectPgRulesStmt {
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
    ) -> ReflectPgRulesQuery<'c, 'a, 's, C, ReflectPgRules, 0> {
        ReflectPgRulesQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgRulesBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgRulesBorrowed {
                    schemaname: row.try_get(0)?,
                    tablename: row.try_get(1)?,
                    rulename: row.try_get(2)?,
                    definition: row.try_get(3)?,
                    description: row.try_get(4)?,
                })
            },
            mapper: |it| ReflectPgRules::from(it),
        }
    }
}
pub struct ReflectPgViewsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_views() -> ReflectPgViewsStmt {
    ReflectPgViewsStmt(
        "select pg_views.schemaname::text as schemaname, pg_temp.quote_with_namespace(pg_views.schemaname, pg_views.viewname) as viewname, pg_views.viewowner::text as viewowner, pg_views.definition as definition from pg_views where not starts_with(pg_views.schemaname, 'pg_toast')",
        None,
    )
}
impl ReflectPgViewsStmt {
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
    ) -> ReflectPgViewsQuery<'c, 'a, 's, C, ReflectPgViews, 0> {
        ReflectPgViewsQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgViewsBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgViewsBorrowed {
                    schemaname: row.try_get(0)?,
                    viewname: row.try_get(1)?,
                    viewowner: row.try_get(2)?,
                    definition: row.try_get(3)?,
                })
            },
            mapper: |it| ReflectPgViews::from(it),
        }
    }
}
pub struct ReflectPgMatviewsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_matviews() -> ReflectPgMatviewsStmt {
    ReflectPgMatviewsStmt(
        "select pg_matviews.schemaname::text as schemaname, pg_temp.quote_with_namespace(pg_matviews.schemaname, pg_matviews.matviewname) as matviewname, pg_matviews.matviewowner::text as matviewowner, pg_matviews.definition as definition from pg_matviews",
        None,
    )
}
impl ReflectPgMatviewsStmt {
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
    ) -> ReflectPgMatviewsQuery<'c, 'a, 's, C, ReflectPgMatviews, 0> {
        ReflectPgMatviewsQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgMatviewsBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgMatviewsBorrowed {
                    schemaname: row.try_get(0)?,
                    matviewname: row.try_get(1)?,
                    matviewowner: row.try_get(2)?,
                    definition: row.try_get(3)?,
                })
            },
            mapper: |it| ReflectPgMatviews::from(it),
        }
    }
}
pub struct ReflectPgSequenceStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_sequence() -> ReflectPgSequenceStmt {
    ReflectPgSequenceStmt(
        "select pg_sequence.seqrelid::regclass::text as seqrelid, pg_sequence.seqtypid::regtype::text as seqtypid, pg_sequence.seqstart as seqstart, pg_sequence.seqincrement as seqincrement, pg_sequence.seqmax as seqmax, pg_sequence.seqmin as seqmin, pg_sequence.seqcache as seqcache, pg_sequence.seqcycle as seqcycle from pg_sequence",
        None,
    )
}
impl ReflectPgSequenceStmt {
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
    ) -> ReflectPgSequenceQuery<'c, 'a, 's, C, ReflectPgSequence, 0> {
        ReflectPgSequenceQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgSequenceBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgSequenceBorrowed {
                    seqrelid: row.try_get(0)?,
                    seqtypid: row.try_get(1)?,
                    seqstart: row.try_get(2)?,
                    seqincrement: row.try_get(3)?,
                    seqmax: row.try_get(4)?,
                    seqmin: row.try_get(5)?,
                    seqcache: row.try_get(6)?,
                    seqcycle: row.try_get(7)?,
                })
            },
            mapper: |it| ReflectPgSequence::from(it),
        }
    }
}
pub struct ReflectPgStatisticExtStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_statistic_ext() -> ReflectPgStatisticExtStmt {
    ReflectPgStatisticExtStmt(
        "select pg_statistic_ext.stxrelid::regclass::text as stxrelid, pg_statistic_ext.stxname::text as stxname, pg_statistic_ext.stxnamespace::regnamespace::text as stxnamespace, pg_get_userbyid(pg_statistic_ext.stxowner)::text as stxowner, pg_statistic_ext.stxkeys as stxkeys, pg_statistic_ext.stxstattarget as stxstattarget, pg_statistic_ext.stxkind as stxkind, pg_get_expr(pg_statistic_ext.stxexprs, pg_statistic_ext.stxrelid) as stxexprs, pg_description.description as description from pg_statistic_ext left join pg_description on pg_description.objoid = pg_statistic_ext.oid and pg_description.objsubid = 0",
        None,
    )
}
impl ReflectPgStatisticExtStmt {
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
    ) -> ReflectPgStatisticExtQuery<'c, 'a, 's, C, ReflectPgStatisticExt, 0> {
        ReflectPgStatisticExtQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgStatisticExtBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgStatisticExtBorrowed {
                    stxrelid: row.try_get(0)?,
                    stxname: row.try_get(1)?,
                    stxnamespace: row.try_get(2)?,
                    stxowner: row.try_get(3)?,
                    stxkeys: row.try_get(4)?,
                    stxstattarget: row.try_get(5)?,
                    stxkind: row.try_get(6)?,
                    stxexprs: row.try_get(7)?,
                    description: row.try_get(8)?,
                })
            },
            mapper: |it| ReflectPgStatisticExt::from(it),
        }
    }
}
pub struct ReflectPgSubscriptionStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_subscription() -> ReflectPgSubscriptionStmt {
    ReflectPgSubscriptionStmt(
        "select pg_subscription.subname::text as subname, pg_get_userbyid(pg_subscription.subowner)::text as subowner, pg_subscription.subenabled as subenabled, pg_subscription.subbinary as subbinary, pg_subscription.substream as substream, pg_subscription.subtwophasestate as subtwophasestate, pg_subscription.subdisableonerr as subdisableonerr, pg_subscription.subpasswordrequired as subpasswordrequired, pg_subscription.subrunasowner as subrunasowner, pg_subscription.subfailover as subfailover, pg_subscription.subconninfo as subconninfo, pg_subscription.subslotname::text as subslotname, pg_subscription.subsynccommit as subsynccommit, pg_subscription.subpublications as subpublications, pg_subscription.suborigin as suborigin, pg_description.description as description, pg_seclabel.label as seclabel, pg_seclabel.provider as seclabel_provider from pg_subscription left join pg_description on pg_description.objoid = pg_subscription.oid and pg_description.objsubid = 0 left join pg_seclabel on pg_seclabel.objoid = pg_subscription.oid and pg_seclabel.objsubid = 0 where pg_subscription.subdbid = (select oid from pg_database where datname = current_database())",
        None,
    )
}
impl ReflectPgSubscriptionStmt {
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
    ) -> ReflectPgSubscriptionQuery<'c, 'a, 's, C, ReflectPgSubscription, 0> {
        ReflectPgSubscriptionQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgSubscriptionBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgSubscriptionBorrowed {
                    subname: row.try_get(0)?,
                    subowner: row.try_get(1)?,
                    subenabled: row.try_get(2)?,
                    subbinary: row.try_get(3)?,
                    substream: row.try_get(4)?,
                    subtwophasestate: row.try_get(5)?,
                    subdisableonerr: row.try_get(6)?,
                    subpasswordrequired: row.try_get(7)?,
                    subrunasowner: row.try_get(8)?,
                    subfailover: row.try_get(9)?,
                    subconninfo: row.try_get(10)?,
                    subslotname: row.try_get(11)?,
                    subsynccommit: row.try_get(12)?,
                    subpublications: row.try_get(13)?,
                    suborigin: row.try_get(14)?,
                    description: row.try_get(15)?,
                    seclabel: row.try_get(16)?,
                    seclabel_provider: row.try_get(17)?,
                })
            },
            mapper: |it| ReflectPgSubscription::from(it),
        }
    }
}
pub struct ReflectPgTransformStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_transform() -> ReflectPgTransformStmt {
    ReflectPgTransformStmt(
        "select pg_transform.trftype::regtype::text as trftype, trflang_pg_language.lanname::text as trflang, case when trffromsql = 0 then null else trffromsql::regprocedure::text end as trffromsql, case when trftosql = 0 then null else trftosql::regprocedure::text end as trftosql from pg_transform join pg_language as trflang_pg_language on pg_transform.trflang = trflang_pg_language.oid",
        None,
    )
}
impl ReflectPgTransformStmt {
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
    ) -> ReflectPgTransformQuery<'c, 'a, 's, C, ReflectPgTransform, 0> {
        ReflectPgTransformQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgTransformBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgTransformBorrowed {
                    trftype: row.try_get(0)?,
                    trflang: row.try_get(1)?,
                    trffromsql: row.try_get(2)?,
                    trftosql: row.try_get(3)?,
                })
            },
            mapper: |it| ReflectPgTransform::from(it),
        }
    }
}
pub struct ReflectPgTriggerStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_trigger() -> ReflectPgTriggerStmt {
    ReflectPgTriggerStmt(
        "select pg_trigger.tgrelid::regclass::text as tgrelid, tgparentid_pg_trigger.tgname::text as tgparentid, pg_trigger.tgname::text as tgname, pg_trigger.tgfoid::regprocedure::text as tgfoid, pg_trigger.tgtype as tgtype, pg_trigger.tgenabled as tgenabled, pg_trigger.tgisinternal as tgisinternal, case when pg_trigger.tgconstrrelid = 0 then null else pg_trigger.tgconstrrelid::regclass::text end as tgconstrrelid, case when pg_trigger.tgconstrindid = 0 then null else pg_trigger.tgconstrindid::regclass::text end as tgconstrindid, pg_temp.quote_with_namespace(tgconstraint_pg_namespace.nspname, tgconstraint_pg_constraint.conname) as tgconstraint, pg_trigger.tgdeferrable as tgdeferrable, pg_trigger.tginitdeferred as tginitdeferred, pg_trigger.tgnargs as tgnargs, pg_trigger.tgattr as tgattr, pg_trigger.tgargs as tgargs, pg_get_expr(pg_trigger.tgqual, pg_trigger.tgrelid) as tgqual, pg_trigger.tgoldtable::text as tgoldtable, pg_trigger.tgnewtable::text as tgnewtable, pg_description.description as description, pg_seclabel.label as seclabel, pg_seclabel.provider as seclabel_provider from pg_trigger left join pg_trigger as tgparentid_pg_trigger on pg_trigger.tgparentid = tgparentid_pg_trigger.oid left join pg_constraint as tgconstraint_pg_constraint on pg_trigger.tgconstraint = tgconstraint_pg_constraint.oid left join pg_namespace as tgconstraint_pg_namespace on tgconstraint_pg_constraint.connamespace = tgconstraint_pg_namespace.oid left join pg_description on pg_description.objoid = pg_trigger.oid and pg_description.objsubid = 0 left join pg_seclabel on pg_seclabel.objoid = pg_trigger.oid and pg_seclabel.objsubid = 0",
        None,
    )
}
impl ReflectPgTriggerStmt {
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
    ) -> ReflectPgTriggerQuery<'c, 'a, 's, C, ReflectPgTrigger, 0> {
        ReflectPgTriggerQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgTriggerBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgTriggerBorrowed {
                    tgrelid: row.try_get(0)?,
                    tgparentid: row.try_get(1)?,
                    tgname: row.try_get(2)?,
                    tgfoid: row.try_get(3)?,
                    tgtype: row.try_get(4)?,
                    tgenabled: row.try_get(5)?,
                    tgisinternal: row.try_get(6)?,
                    tgconstrrelid: row.try_get(7)?,
                    tgconstrindid: row.try_get(8)?,
                    tgconstraint: row.try_get(9)?,
                    tgdeferrable: row.try_get(10)?,
                    tginitdeferred: row.try_get(11)?,
                    tgnargs: row.try_get(12)?,
                    tgattr: row.try_get(13)?,
                    tgargs: row.try_get(14)?,
                    tgqual: row.try_get(15)?,
                    tgoldtable: row.try_get(16)?,
                    tgnewtable: row.try_get(17)?,
                    description: row.try_get(18)?,
                    seclabel: row.try_get(19)?,
                    seclabel_provider: row.try_get(20)?,
                })
            },
            mapper: |it| ReflectPgTrigger::from(it),
        }
    }
}
pub struct ReflectPgTsConfigStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_ts_config() -> ReflectPgTsConfigStmt {
    ReflectPgTsConfigStmt(
        "select pg_ts_config.oid::regconfig::text as oid, pg_ts_config.cfgname::text as cfgname, pg_ts_config.cfgnamespace::regnamespace::text as cfgnamespace, pg_get_userbyid(pg_ts_config.cfgowner)::text as cfgowner, pg_description.description as description, pg_seclabel.label as seclabel, pg_seclabel.provider as seclabel_provider from pg_ts_config left join pg_description on pg_description.objoid = pg_ts_config.oid and pg_description.objsubid = 0 left join pg_seclabel on pg_seclabel.objoid = pg_ts_config.oid and pg_seclabel.objsubid = 0",
        None,
    )
}
impl ReflectPgTsConfigStmt {
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
    ) -> ReflectPgTsConfigQuery<'c, 'a, 's, C, ReflectPgTsConfig, 0> {
        ReflectPgTsConfigQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgTsConfigBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgTsConfigBorrowed {
                    oid: row.try_get(0)?,
                    cfgname: row.try_get(1)?,
                    cfgnamespace: row.try_get(2)?,
                    cfgowner: row.try_get(3)?,
                    description: row.try_get(4)?,
                    seclabel: row.try_get(5)?,
                    seclabel_provider: row.try_get(6)?,
                })
            },
            mapper: |it| ReflectPgTsConfig::from(it),
        }
    }
}
pub struct ReflectPgTsConfigMapStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_ts_config_map() -> ReflectPgTsConfigMapStmt {
    ReflectPgTsConfigMapStmt(
        "select pg_ts_config_map.mapcfg::regconfig::text as mapcfg, pg_ts_config_map.maptokentype as maptokentype, pg_ts_config_map.mapseqno as mapseqno, pg_ts_config_map.mapdict::regdictionary::text as mapdict from pg_ts_config_map",
        None,
    )
}
impl ReflectPgTsConfigMapStmt {
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
    ) -> ReflectPgTsConfigMapQuery<'c, 'a, 's, C, ReflectPgTsConfigMap, 0> {
        ReflectPgTsConfigMapQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgTsConfigMapBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgTsConfigMapBorrowed {
                    mapcfg: row.try_get(0)?,
                    maptokentype: row.try_get(1)?,
                    mapseqno: row.try_get(2)?,
                    mapdict: row.try_get(3)?,
                })
            },
            mapper: |it| ReflectPgTsConfigMap::from(it),
        }
    }
}
pub struct ReflectPgTsDictStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_ts_dict() -> ReflectPgTsDictStmt {
    ReflectPgTsDictStmt(
        "select pg_ts_dict.oid::regdictionary::text as oid, pg_ts_dict.dictname::text as dictname, pg_ts_dict.dictnamespace::regnamespace::text as dictnamespace, pg_get_userbyid(pg_ts_dict.dictowner)::text as dictowner, pg_ts_dict.dictinitoption as dictinitoption, pg_description.description as description, pg_seclabel.label as seclabel, pg_seclabel.provider as seclabel_provider from pg_ts_dict left join pg_description on pg_description.objoid = pg_ts_dict.oid and pg_description.objsubid = 0 left join pg_seclabel on pg_seclabel.objoid = pg_ts_dict.oid and pg_seclabel.objsubid = 0",
        None,
    )
}
impl ReflectPgTsDictStmt {
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
    ) -> ReflectPgTsDictQuery<'c, 'a, 's, C, ReflectPgTsDict, 0> {
        ReflectPgTsDictQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgTsDictBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgTsDictBorrowed {
                    oid: row.try_get(0)?,
                    dictname: row.try_get(1)?,
                    dictnamespace: row.try_get(2)?,
                    dictowner: row.try_get(3)?,
                    dictinitoption: row.try_get(4)?,
                    description: row.try_get(5)?,
                    seclabel: row.try_get(6)?,
                    seclabel_provider: row.try_get(7)?,
                })
            },
            mapper: |it| ReflectPgTsDict::from(it),
        }
    }
}
pub struct ReflectPgTsParserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_ts_parser() -> ReflectPgTsParserStmt {
    ReflectPgTsParserStmt(
        "select pg_ts_parser.prsname::text as prsname, pg_ts_parser.prsnamespace::regnamespace::text as prsnamespace, prsstart::regprocedure::text as prsstart, prstoken::regprocedure::text as prstoken, prsend::regprocedure::text as prsend, case when prsheadline = 0 then null else prsheadline::regprocedure::text end as prsheadline, prslextype::regprocedure::text as prslextype from pg_ts_parser",
        None,
    )
}
impl ReflectPgTsParserStmt {
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
    ) -> ReflectPgTsParserQuery<'c, 'a, 's, C, ReflectPgTsParser, 0> {
        ReflectPgTsParserQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgTsParserBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgTsParserBorrowed {
                    prsname: row.try_get(0)?,
                    prsnamespace: row.try_get(1)?,
                    prsstart: row.try_get(2)?,
                    prstoken: row.try_get(3)?,
                    prsend: row.try_get(4)?,
                    prsheadline: row.try_get(5)?,
                    prslextype: row.try_get(6)?,
                })
            },
            mapper: |it| ReflectPgTsParser::from(it),
        }
    }
}
pub struct ReflectPgTsTemplateStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_ts_template() -> ReflectPgTsTemplateStmt {
    ReflectPgTsTemplateStmt(
        "select pg_ts_template.tmplname::text as tmplname, pg_ts_template.tmplnamespace::regnamespace::text as tmplnamespace, case when tmplinit = 0 then null else tmplinit::regprocedure::text end as tmplinit, tmpllexize::regprocedure::text as tmpllexize from pg_ts_template",
        None,
    )
}
impl ReflectPgTsTemplateStmt {
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
    ) -> ReflectPgTsTemplateQuery<'c, 'a, 's, C, ReflectPgTsTemplate, 0> {
        ReflectPgTsTemplateQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgTsTemplateBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgTsTemplateBorrowed {
                    tmplname: row.try_get(0)?,
                    tmplnamespace: row.try_get(1)?,
                    tmplinit: row.try_get(2)?,
                    tmpllexize: row.try_get(3)?,
                })
            },
            mapper: |it| ReflectPgTsTemplate::from(it),
        }
    }
}
pub struct ReflectPgTypeStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_type() -> ReflectPgTypeStmt {
    ReflectPgTypeStmt(
        "select pg_type.oid::regtype::text as oid, pg_type.typname::text as typname, pg_type.typnamespace::regnamespace::text as typnamespace, pg_get_userbyid(pg_type.typowner)::text as typowner, pg_type.typlen as typlen, pg_type.typbyval as typbyval, pg_type.typtype as typtype, pg_type.typispreferred as typispreferred, pg_type.typisdefined as typisdefined, pg_type.typdelim as typdelim, case when pg_type.typrelid = 0 then null else pg_type.typrelid::regclass::text end as typrelid, case when typsubscript = 0 then null else typsubscript::regprocedure::text end as typsubscript, case when pg_type.typelem = 0 then null else pg_type.typelem::regtype::text end as typelem, case when pg_type.typarray = 0 then null else pg_type.typarray::regtype::text end as typarray, typinput::regprocedure::text as typinput, typoutput::regprocedure::text as typoutput, case when typreceive = 0 then null else typreceive::regprocedure::text end as typreceive, case when typsend = 0 then null else typsend::regprocedure::text end as typsend, case when typmodin = 0 then null else typmodin::regprocedure::text end as typmodin, case when typmodout = 0 then null else typmodout::regprocedure::text end as typmodout, case when typanalyze = 0 then null else typanalyze::regprocedure::text end as typanalyze, pg_type.typalign as typalign, pg_type.typstorage as typstorage, pg_type.typnotnull as typnotnull, case when pg_type.typbasetype = 0 then null else pg_type.typbasetype::regtype::text end as typbasetype, case when typtypmod < 0 then null else typtypmod end as typtypmod, pg_type.typndims as typndims, case when pg_type.typcollation = 0 then null else pg_type.typcollation::regcollation::text end as typcollation, pg_get_expr(typdefaultbin, 0) as typdefaultbin, pg_type.typdefault as typdefault, pg_temp.format_type_aclitems(pg_type.typacl) as typacl, pg_description.description as description, pg_seclabel.label as seclabel, pg_seclabel.provider as seclabel_provider, pg_temp.format_type_aclitems(pg_init_privs.initprivs) as initprivs, pg_init_privs.privtype as initprivs_type from pg_type left join pg_description on pg_description.objoid = pg_type.oid and pg_description.objsubid = 0 left join pg_seclabel on pg_seclabel.objoid = pg_type.oid and pg_seclabel.objsubid = 0 left join pg_init_privs on pg_init_privs.objoid = pg_type.oid and pg_init_privs.objsubid = 0",
        None,
    )
}
impl ReflectPgTypeStmt {
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
    ) -> ReflectPgTypeQuery<'c, 'a, 's, C, ReflectPgType, 0> {
        ReflectPgTypeQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ReflectPgTypeBorrowed, tokio_postgres::Error> {
                    Ok(ReflectPgTypeBorrowed {
                        oid: row.try_get(0)?,
                        typname: row.try_get(1)?,
                        typnamespace: row.try_get(2)?,
                        typowner: row.try_get(3)?,
                        typlen: row.try_get(4)?,
                        typbyval: row.try_get(5)?,
                        typtype: row.try_get(6)?,
                        typispreferred: row.try_get(7)?,
                        typisdefined: row.try_get(8)?,
                        typdelim: row.try_get(9)?,
                        typrelid: row.try_get(10)?,
                        typsubscript: row.try_get(11)?,
                        typelem: row.try_get(12)?,
                        typarray: row.try_get(13)?,
                        typinput: row.try_get(14)?,
                        typoutput: row.try_get(15)?,
                        typreceive: row.try_get(16)?,
                        typsend: row.try_get(17)?,
                        typmodin: row.try_get(18)?,
                        typmodout: row.try_get(19)?,
                        typanalyze: row.try_get(20)?,
                        typalign: row.try_get(21)?,
                        typstorage: row.try_get(22)?,
                        typnotnull: row.try_get(23)?,
                        typbasetype: row.try_get(24)?,
                        typtypmod: row.try_get(25)?,
                        typndims: row.try_get(26)?,
                        typcollation: row.try_get(27)?,
                        typdefaultbin: row.try_get(28)?,
                        typdefault: row.try_get(29)?,
                        typacl: row.try_get(30)?,
                        description: row.try_get(31)?,
                        seclabel: row.try_get(32)?,
                        seclabel_provider: row.try_get(33)?,
                        initprivs: row.try_get(34)?,
                        initprivs_type: row.try_get(35)?,
                    })
                },
            mapper: |it| ReflectPgType::from(it),
        }
    }
}
pub struct ReflectPgUserMappingsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reflect_pg_user_mappings() -> ReflectPgUserMappingsStmt {
    ReflectPgUserMappingsStmt(
        "select pg_user_mappings.srvname::text as srvname, case when pg_user_mappings.umuser = 0 then null else pg_get_userbyid(pg_user_mappings.umuser)::text end as umuser, pg_user_mappings.usename::text as usename, pg_user_mappings.umoptions as umoptions from pg_user_mappings",
        None,
    )
}
impl ReflectPgUserMappingsStmt {
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
    ) -> ReflectPgUserMappingsQuery<'c, 'a, 's, C, ReflectPgUserMappings, 0> {
        ReflectPgUserMappingsQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReflectPgUserMappingsBorrowed, tokio_postgres::Error> {
                Ok(ReflectPgUserMappingsBorrowed {
                    srvname: row.try_get(0)?,
                    umuser: row.try_get(1)?,
                    usename: row.try_get(2)?,
                    umoptions: row.try_get(3)?,
                })
            },
            mapper: |it| ReflectPgUserMappings::from(it),
        }
    }
}
