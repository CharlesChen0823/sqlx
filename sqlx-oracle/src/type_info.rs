#![allow(dead_code)]

use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;

use crate::ext::ustr::UStr;
use crate::types::Oid;

pub(crate) use sqlx_core::type_info::TypeInfo;

/// Type information for a Oracle type.
///
/// ### Note: Implementation of `==` ([`PartialEq::eq()`])
/// Because `==` on [`TypeInfo`]s has been used throughout the SQLx API as a synonym for type compatibility,
/// e.g. in the default impl of [`Type::compatible()`][sqlx_core::types::Type::compatible],
/// some concessions have been made in the implementation.
///
/// When comparing two `OracleTypeInfo`s using the `==` operator ([`PartialEq::eq()`]),
/// if one was constructed with [`Self::with_oid()`] and the other with [`Self::with_name()`] or
/// [`Self::array_of()`], `==` will return `true`:
///
/// ```
/// # use sqlx::oracle::{types::Oid, OracleTypeInfo};
/// // Potentially surprising result, this assert will pass:
/// assert_eq!(OracleTypeInfo::with_oid(Oid(1)), OracleTypeInfo::with_name("definitely_not_real"));
/// ```
///
/// Since it is not possible in this case to prove the types are _not_ compatible (because
/// both `OracleTypeInfo`s need to be resolved by an active connection to know for sure)
/// and type compatibility is mainly done as a sanity check anyway,
/// it was deemed acceptable to fudge equality in this very specific case.
///
/// This also applies when querying with the text protocol (not using prepared statements,
/// e.g. [`sqlx::raw_sql()`][sqlx_core::raw_sql::raw_sql]), as the connection will be unable
/// to look up the type info like it normally does when preparing a statement: it won't know
/// what the OIDs of the output columns will be until it's in the middle of reading the result,
/// and by that time it's too late.
///
/// To compare types for exact equality, use [`Self::type_eq()`] instead.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub struct OracleTypeInfo(pub(crate) OracleType);

impl Deref for OracleTypeInfo {
    type Target = OracleType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
#[repr(u32)]
pub enum OracleType {
    Bool,
    Bytea,
    Char,
    Name,
    Int8,
    Int2,
    Int4,
    Text,
    Oid,
    Json,
    JsonArray,
    Box,
    Float4,
    Float8,
    Unknown,
    BoolArray,
    ByteaArray,
    CharArray,
    NameArray,
    Int2Array,
    Int4Array,
    TextArray,
    BpcharArray,
    VarcharArray,
    Int8Array,
    BoxArray,
    Float4Array,
    Float8Array,
    OidArray,
    Bpchar,
    Varchar,
    Date,
    Time,
    Timestamp,
    TimestampArray,
    DateArray,
    TimeArray,
    Timestamptz,
    TimestamptzArray,
    Interval,
    IntervalArray,
    NumericArray,
    Timetz,
    TimetzArray,
    Bit,
    BitArray,
    Varbit,
    VarbitArray,
    Numeric,
    Uuid,
    UuidArray,
    Jsonb,
    JsonbArray,
    Int4Range,
    Int4RangeArray,
    NumRange,
    NumRangeArray,
    TsRange,
    TsRangeArray,
    TstzRange,
    TstzRangeArray,
    DateRange,
    DateRangeArray,
    Int8Range,
    Int8RangeArray,

    // https://www.postgresql.org/docs/9.3/datatype-pseudo.html
    Void,

    // A realized user-defined type. When a connection sees a DeclareXX variant it resolves
    // into this one before passing it along to `accepts` or inside of `Value` objects.
    Custom(Arc<OracleCustomType>),

    // From [`OracleTypeInfo::with_name`]
    DeclareWithName(UStr),

    // NOTE: Do we want to bring back type declaration by ID? It's notoriously fragile but
    //       someone may have a user for it
    DeclareWithOid(Oid),

    DeclareArrayOf(Arc<OracleArrayOf>),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub struct OracleCustomType {
    #[cfg_attr(feature = "offline", serde(skip))]
    pub(crate) oid: Oid,
    pub(crate) name: UStr,
    pub(crate) kind: OracleTypeKind,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub enum OracleTypeKind {
    Simple,
    Pseudo,
    Domain(OracleTypeInfo),
    Composite(Arc<[(String, OracleTypeInfo)]>),
    Array(OracleTypeInfo),
    Enum(Arc<[String]>),
    Range(OracleTypeInfo),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub struct OracleArrayOf {
    pub(crate) elem_name: UStr,
    pub(crate) name: Box<str>,
}

impl OracleTypeInfo {
    /// Returns the corresponding `OracleTypeInfo` if the OID is a built-in type and recognized by SQLx.
    pub(crate) fn try_from_oid(oid: Oid) -> Option<Self> {
        OracleType::try_from_oid(oid).map(Self)
    }

    /// Returns the _kind_ (simple, array, enum, etc.) for this type.
    pub fn kind(&self) -> &OracleTypeKind {
        self.0.kind()
    }

    /// Returns the OID for this type, if available.
    ///
    /// The OID may not be available if SQLx only knows the type by name.
    /// It will have to be resolved by a `OracleConnection` at runtime which
    /// will yield a new and semantically distinct `TypeInfo` instance.
    ///
    /// This method does not perform any such lookup.
    ///
    /// ### Note
    /// With the exception of [the default `oracle_type` catalog][oracle_type], type OIDs are *not* stable in Oracle.
    /// If a type is added by an extension, its OID will be assigned when the `CREATE EXTENSION` statement is executed,
    /// and so can change depending on what extensions are installed and in what order, as well as the exact
    /// version of Oracle.
    ///
    /// [oracle_type]: https://github.com/oracle/oracle/blob/master/src/include/catalog/oracle_type.dat
    pub fn oid(&self) -> Option<Oid> {
        self.0.try_oid()
    }

    #[doc(hidden)]
    pub fn __type_feature_gate(&self) -> Option<&'static str> {
        if [
            OracleTypeInfo::DATE,
            OracleTypeInfo::TIME,
            OracleTypeInfo::TIMESTAMP,
            OracleTypeInfo::TIMESTAMPTZ,
            OracleTypeInfo::DATE_ARRAY,
            OracleTypeInfo::TIME_ARRAY,
            OracleTypeInfo::TIMESTAMP_ARRAY,
            OracleTypeInfo::TIMESTAMPTZ_ARRAY,
        ]
        .contains(self)
        {
            Some("time")
        } else if [OracleTypeInfo::UUID, OracleTypeInfo::UUID_ARRAY].contains(self) {
            Some("uuid")
        } else if [
            OracleTypeInfo::JSON,
            OracleTypeInfo::JSONB,
            OracleTypeInfo::JSON_ARRAY,
            OracleTypeInfo::JSONB_ARRAY,
        ]
        .contains(self)
        {
            Some("json")
        } else if [OracleTypeInfo::NUMERIC, OracleTypeInfo::NUMERIC_ARRAY].contains(self) {
            Some("bigdecimal")
        } else {
            None
        }
    }

    /// Create a `OracleTypeInfo` from a type name.
    ///
    /// The OID for the type will be fetched from Oracle on use of
    /// a value of this type. The fetched OID will be cached per-connection.
    ///
    /// ### Note: Type Names Prefixed with `_`
    /// In `oracle_catalog.oracle_type`, Oracle prefixes a type name with `_` to denote an array of that
    /// type, e.g. `int4[]` actually exists in `oracle_type` as `_int4`.
    ///
    /// Previously, it was necessary in manual [`OracleHasArrayType`][crate::OracleHasArrayType] impls
    /// to return [`OracleTypeInfo::with_name()`] with the type name prefixed with `_` to denote
    /// an array type, but this would not work with schema-qualified names.
    ///
    /// As of 0.8, [`OracleTypeInfo::array_of()`] is used to declare an array type,
    /// and the Oracle driver is now able to properly resolve arrays of custom types,
    /// even in other schemas, which was not previously supported.
    ///
    /// It is highly recommended to migrate existing usages to [`OracleTypeInfo::array_of()`] where
    /// applicable.
    ///
    /// However, to maintain compatibility, the driver now infers any type name prefixed with `_`
    /// to be an array of that type. This may introduce some breakages for types which use
    /// a `_` prefix but which are not arrays.
    ///
    /// As a workaround, type names with `_` as a prefix but which are not arrays should be wrapped
    /// in quotes, e.g.:
    /// ```
    /// use sqlx::oracle::OracleTypeInfo;
    /// use sqlx::{Type, TypeInfo};
    ///
    /// /// `CREATE TYPE "_foo" AS ENUM ('Bar', 'Baz');`
    /// #[derive(sqlx::Type)]
    /// // Will prevent SQLx from inferring `_foo` as an array type.
    /// #[sqlx(type_name = r#""_foo""#)]
    /// enum Foo {
    ///     Bar,
    ///     Baz
    /// }
    ///
    /// assert_eq!(Foo::type_info().name(), r#""_foo""#);
    /// ```
    pub const fn with_name(name: &'static str) -> Self {
        Self(OracleType::DeclareWithName(UStr::Static(name)))
    }

    /// Create a `OracleTypeInfo` of an array from the name of its element type.
    ///
    /// The array type OID will be fetched from Oracle on use of a value of this type.
    /// The fetched OID will be cached per-connection.
    pub fn array_of(elem_name: &'static str) -> Self {
        // to satisfy `name()` and `display_name()`, we need to construct strings to return
        Self(OracleType::DeclareArrayOf(Arc::new(OracleArrayOf {
            elem_name: elem_name.into(),
            name: format!("{elem_name}[]").into(),
        })))
    }

    /// Create a `OracleTypeInfo` from an OID.
    ///
    /// Note that the OID for a type is very dependent on the environment. If you only ever use
    /// one database or if this is an unhandled built-in type, you should be fine. Otherwise,
    /// you will be better served using [`Self::with_name()`].
    ///
    /// ### Note: Interaction with `==`
    /// This constructor may give surprising results with `==`.
    ///
    /// See [the type-level docs][Self] for details.
    pub const fn with_oid(oid: Oid) -> Self {
        Self(OracleType::DeclareWithOid(oid))
    }

    /// Returns `true` if `self` can be compared exactly to `other`.
    ///
    /// Unlike `==`, this will return false if
    pub fn type_eq(&self, other: &Self) -> bool {
        self.eq_impl(other, false)
    }
}

// DEVELOPER PRO TIP: find builtin type OIDs easily by grepping this file
// https://github.com/oracle/oracle/blob/master/src/include/catalog/oracle_type.dat
//
// If you have Oracle running locally you can also try
// SELECT oid, typarray FROM oracle_type where typname = '<type name>'

impl OracleType {
    /// Returns the corresponding `OracleType` if the OID is a built-in type and recognized by SQLx.
    pub(crate) fn try_from_oid(oid: Oid) -> Option<Self> {
        Some(match oid.0 {
            16 => OracleType::Bool,
            17 => OracleType::Bytea,
            18 => OracleType::Char,
            19 => OracleType::Name,
            20 => OracleType::Int8,
            21 => OracleType::Int2,
            23 => OracleType::Int4,
            25 => OracleType::Text,
            26 => OracleType::Oid,
            114 => OracleType::Json,
            199 => OracleType::JsonArray,
            603 => OracleType::Box,
            700 => OracleType::Float4,
            701 => OracleType::Float8,
            705 => OracleType::Unknown,
            1000 => OracleType::BoolArray,
            1001 => OracleType::ByteaArray,
            1002 => OracleType::CharArray,
            1003 => OracleType::NameArray,
            1005 => OracleType::Int2Array,
            1007 => OracleType::Int4Array,
            1009 => OracleType::TextArray,
            1014 => OracleType::BpcharArray,
            1015 => OracleType::VarcharArray,
            1016 => OracleType::Int8Array,
            1020 => OracleType::BoxArray,
            1021 => OracleType::Float4Array,
            1022 => OracleType::Float8Array,
            1028 => OracleType::OidArray,
            1042 => OracleType::Bpchar,
            1043 => OracleType::Varchar,
            1082 => OracleType::Date,
            1083 => OracleType::Time,
            1114 => OracleType::Timestamp,
            1115 => OracleType::TimestampArray,
            1182 => OracleType::DateArray,
            1183 => OracleType::TimeArray,
            1184 => OracleType::Timestamptz,
            1185 => OracleType::TimestamptzArray,
            1186 => OracleType::Interval,
            1187 => OracleType::IntervalArray,
            1231 => OracleType::NumericArray,
            1266 => OracleType::Timetz,
            1270 => OracleType::TimetzArray,
            1560 => OracleType::Bit,
            1561 => OracleType::BitArray,
            1562 => OracleType::Varbit,
            1563 => OracleType::VarbitArray,
            1700 => OracleType::Numeric,
            2278 => OracleType::Void,
            2950 => OracleType::Uuid,
            2951 => OracleType::UuidArray,
            3802 => OracleType::Jsonb,
            3807 => OracleType::JsonbArray,
            3904 => OracleType::Int4Range,
            3905 => OracleType::Int4RangeArray,
            3906 => OracleType::NumRange,
            3907 => OracleType::NumRangeArray,
            3908 => OracleType::TsRange,
            3909 => OracleType::TsRangeArray,
            3910 => OracleType::TstzRange,
            3911 => OracleType::TstzRangeArray,
            3912 => OracleType::DateRange,
            3913 => OracleType::DateRangeArray,
            3926 => OracleType::Int8Range,
            3927 => OracleType::Int8RangeArray,

            _ => {
                return None;
            }
        })
    }

    pub(crate) fn oid(&self) -> Oid {
        match self.try_oid() {
            Some(oid) => oid,
            None => unreachable!("(bug) use of unresolved type declaration [oid]"),
        }
    }

    pub(crate) fn try_oid(&self) -> Option<Oid> {
        Some(match self {
            OracleType::Bool => Oid(16),
            OracleType::Bytea => Oid(17),
            OracleType::Char => Oid(18),
            OracleType::Name => Oid(19),
            OracleType::Int8 => Oid(20),
            OracleType::Int2 => Oid(21),
            OracleType::Int4 => Oid(23),
            OracleType::Text => Oid(25),
            OracleType::Oid => Oid(26),
            OracleType::Json => Oid(114),
            OracleType::JsonArray => Oid(199),
            OracleType::Box => Oid(603),
            OracleType::Float4 => Oid(700),
            OracleType::Float8 => Oid(701),
            OracleType::Unknown => Oid(705),
            OracleType::BoolArray => Oid(1000),
            OracleType::ByteaArray => Oid(1001),
            OracleType::CharArray => Oid(1002),
            OracleType::NameArray => Oid(1003),
            OracleType::Int2Array => Oid(1005),
            OracleType::Int4Array => Oid(1007),
            OracleType::TextArray => Oid(1009),
            OracleType::BpcharArray => Oid(1014),
            OracleType::VarcharArray => Oid(1015),
            OracleType::Int8Array => Oid(1016),
            OracleType::BoxArray => Oid(1020),
            OracleType::Float4Array => Oid(1021),
            OracleType::Float8Array => Oid(1022),
            OracleType::OidArray => Oid(1028),
            OracleType::Bpchar => Oid(1042),
            OracleType::Varchar => Oid(1043),
            OracleType::Date => Oid(1082),
            OracleType::Time => Oid(1083),
            OracleType::Timestamp => Oid(1114),
            OracleType::TimestampArray => Oid(1115),
            OracleType::DateArray => Oid(1182),
            OracleType::TimeArray => Oid(1183),
            OracleType::Timestamptz => Oid(1184),
            OracleType::TimestamptzArray => Oid(1185),
            OracleType::Interval => Oid(1186),
            OracleType::IntervalArray => Oid(1187),
            OracleType::NumericArray => Oid(1231),
            OracleType::Timetz => Oid(1266),
            OracleType::TimetzArray => Oid(1270),
            OracleType::Bit => Oid(1560),
            OracleType::BitArray => Oid(1561),
            OracleType::Varbit => Oid(1562),
            OracleType::VarbitArray => Oid(1563),
            OracleType::Numeric => Oid(1700),
            OracleType::Void => Oid(2278),
            OracleType::Uuid => Oid(2950),
            OracleType::UuidArray => Oid(2951),
            OracleType::Jsonb => Oid(3802),
            OracleType::JsonbArray => Oid(3807),
            OracleType::Int4Range => Oid(3904),
            OracleType::Int4RangeArray => Oid(3905),
            OracleType::NumRange => Oid(3906),
            OracleType::NumRangeArray => Oid(3907),
            OracleType::TsRange => Oid(3908),
            OracleType::TsRangeArray => Oid(3909),
            OracleType::TstzRange => Oid(3910),
            OracleType::TstzRangeArray => Oid(3911),
            OracleType::DateRange => Oid(3912),
            OracleType::DateRangeArray => Oid(3913),
            OracleType::Int8Range => Oid(3926),
            OracleType::Int8RangeArray => Oid(3927),

            OracleType::Custom(ty) => ty.oid,

            OracleType::DeclareWithOid(oid) => *oid,
            OracleType::DeclareWithName(_) => {
                return None;
            }
            OracleType::DeclareArrayOf(_) => {
                return None;
            }
        })
    }

    pub(crate) fn display_name(&self) -> &str {
        match self {
            OracleType::Bool => "BOOL",
            OracleType::Bytea => "BYTEA",
            OracleType::Char => "\"CHAR\"",
            OracleType::Name => "NAME",
            OracleType::Int8 => "INT8",
            OracleType::Int2 => "INT2",
            OracleType::Int4 => "INT4",
            OracleType::Text => "TEXT",
            OracleType::Oid => "OID",
            OracleType::Json => "JSON",
            OracleType::JsonArray => "JSON[]",
            OracleType::Box => "BOX",
            OracleType::Float4 => "FLOAT4",
            OracleType::Float8 => "FLOAT8",
            OracleType::Unknown => "UNKNOWN",
            OracleType::BoolArray => "BOOL[]",
            OracleType::ByteaArray => "BYTEA[]",
            OracleType::CharArray => "\"CHAR\"[]",
            OracleType::NameArray => "NAME[]",
            OracleType::Int2Array => "INT2[]",
            OracleType::Int4Array => "INT4[]",
            OracleType::TextArray => "TEXT[]",
            OracleType::BpcharArray => "CHAR[]",
            OracleType::VarcharArray => "VARCHAR[]",
            OracleType::Int8Array => "INT8[]",
            OracleType::BoxArray => "BOX[]",
            OracleType::Float4Array => "FLOAT4[]",
            OracleType::Float8Array => "FLOAT8[]",
            OracleType::OidArray => "OID[]",
            OracleType::Bpchar => "CHAR",
            OracleType::Varchar => "VARCHAR",
            OracleType::Date => "DATE",
            OracleType::Time => "TIME",
            OracleType::Timestamp => "TIMESTAMP",
            OracleType::TimestampArray => "TIMESTAMP[]",
            OracleType::DateArray => "DATE[]",
            OracleType::TimeArray => "TIME[]",
            OracleType::Timestamptz => "TIMESTAMPTZ",
            OracleType::TimestamptzArray => "TIMESTAMPTZ[]",
            OracleType::Interval => "INTERVAL",
            OracleType::IntervalArray => "INTERVAL[]",
            OracleType::NumericArray => "NUMERIC[]",
            OracleType::Timetz => "TIMETZ",
            OracleType::TimetzArray => "TIMETZ[]",
            OracleType::Bit => "BIT",
            OracleType::BitArray => "BIT[]",
            OracleType::Varbit => "VARBIT",
            OracleType::VarbitArray => "VARBIT[]",
            OracleType::Numeric => "NUMERIC",
            OracleType::Uuid => "UUID",
            OracleType::UuidArray => "UUID[]",
            OracleType::Jsonb => "JSONB",
            OracleType::JsonbArray => "JSONB[]",
            OracleType::Int4Range => "INT4RANGE",
            OracleType::Int4RangeArray => "INT4RANGE[]",
            OracleType::NumRange => "NUMRANGE",
            OracleType::NumRangeArray => "NUMRANGE[]",
            OracleType::TsRange => "TSRANGE",
            OracleType::TsRangeArray => "TSRANGE[]",
            OracleType::TstzRange => "TSTZRANGE",
            OracleType::TstzRangeArray => "TSTZRANGE[]",
            OracleType::DateRange => "DATERANGE",
            OracleType::DateRangeArray => "DATERANGE[]",
            OracleType::Int8Range => "INT8RANGE",
            OracleType::Int8RangeArray => "INT8RANGE[]",
            OracleType::Void => "VOID",
            OracleType::Custom(ty) => &ty.name,
            OracleType::DeclareWithOid(_) => "?",
            OracleType::DeclareWithName(name) => name,
            OracleType::DeclareArrayOf(array) => &array.name,
        }
    }

    pub(crate) fn name(&self) -> &str {
        match self {
            OracleType::Bool => "bool",
            OracleType::Bytea => "bytea",
            OracleType::Char => "char",
            OracleType::Name => "name",
            OracleType::Int8 => "int8",
            OracleType::Int2 => "int2",
            OracleType::Int4 => "int4",
            OracleType::Text => "text",
            OracleType::Oid => "oid",
            OracleType::Json => "json",
            OracleType::JsonArray => "_json",
            OracleType::Box => "box",
            OracleType::Float4 => "float4",
            OracleType::Float8 => "float8",
            OracleType::Unknown => "unknown",
            OracleType::BoolArray => "_bool",
            OracleType::ByteaArray => "_bytea",
            OracleType::CharArray => "_char",
            OracleType::NameArray => "_name",
            OracleType::Int2Array => "_int2",
            OracleType::Int4Array => "_int4",
            OracleType::TextArray => "_text",
            OracleType::BpcharArray => "_bpchar",
            OracleType::VarcharArray => "_varchar",
            OracleType::Int8Array => "_int8",
            OracleType::BoxArray => "_box",
            OracleType::Float4Array => "_float4",
            OracleType::Float8Array => "_float8",
            OracleType::OidArray => "_oid",
            OracleType::Bpchar => "bpchar",
            OracleType::Varchar => "varchar",
            OracleType::Date => "date",
            OracleType::Time => "time",
            OracleType::Timestamp => "timestamp",
            OracleType::TimestampArray => "_timestamp",
            OracleType::DateArray => "_date",
            OracleType::TimeArray => "_time",
            OracleType::Timestamptz => "timestamptz",
            OracleType::TimestamptzArray => "_timestamptz",
            OracleType::Interval => "interval",
            OracleType::IntervalArray => "_interval",
            OracleType::NumericArray => "_numeric",
            OracleType::Timetz => "timetz",
            OracleType::TimetzArray => "_timetz",
            OracleType::Bit => "bit",
            OracleType::BitArray => "_bit",
            OracleType::Varbit => "varbit",
            OracleType::VarbitArray => "_varbit",
            OracleType::Numeric => "numeric",
            OracleType::Uuid => "uuid",
            OracleType::UuidArray => "_uuid",
            OracleType::Jsonb => "jsonb",
            OracleType::JsonbArray => "_jsonb",
            OracleType::Int4Range => "int4range",
            OracleType::Int4RangeArray => "_int4range",
            OracleType::NumRange => "numrange",
            OracleType::NumRangeArray => "_numrange",
            OracleType::TsRange => "tsrange",
            OracleType::TsRangeArray => "_tsrange",
            OracleType::TstzRange => "tstzrange",
            OracleType::TstzRangeArray => "_tstzrange",
            OracleType::DateRange => "daterange",
            OracleType::DateRangeArray => "_daterange",
            OracleType::Int8Range => "int8range",
            OracleType::Int8RangeArray => "_int8range",
            OracleType::Void => "void",
            OracleType::Custom(ty) => &ty.name,
            OracleType::DeclareWithOid(_) => "?",
            OracleType::DeclareWithName(name) => name,
            OracleType::DeclareArrayOf(array) => &array.name,
        }
    }

    pub(crate) fn kind(&self) -> &OracleTypeKind {
        match self {
            OracleType::Bool => &OracleTypeKind::Simple,
            OracleType::Bytea => &OracleTypeKind::Simple,
            OracleType::Char => &OracleTypeKind::Simple,
            OracleType::Name => &OracleTypeKind::Simple,
            OracleType::Int8 => &OracleTypeKind::Simple,
            OracleType::Int2 => &OracleTypeKind::Simple,
            OracleType::Int4 => &OracleTypeKind::Simple,
            OracleType::Text => &OracleTypeKind::Simple,
            OracleType::Oid => &OracleTypeKind::Simple,
            OracleType::Json => &OracleTypeKind::Simple,
            OracleType::JsonArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Json)),
            OracleType::Box => &OracleTypeKind::Simple,
            OracleType::Float4 => &OracleTypeKind::Simple,
            OracleType::Float8 => &OracleTypeKind::Simple,
            OracleType::Unknown => &OracleTypeKind::Simple,
            OracleType::BoolArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Bool)),
            OracleType::ByteaArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Bytea)),
            OracleType::CharArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Char)),
            OracleType::NameArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Name)),
            OracleType::Int2Array => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Int2)),
            OracleType::Int4Array => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Int4)),
            OracleType::TextArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Text)),
            OracleType::BpcharArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Bpchar)),
            OracleType::VarcharArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Varchar)),
            OracleType::Int8Array => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Int8)),
            OracleType::BoxArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Box)),
            OracleType::Float4Array => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Float4)),
            OracleType::Float8Array => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Float8)),
            OracleType::OidArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Oid)),
            OracleType::Bpchar => &OracleTypeKind::Simple,
            OracleType::Varchar => &OracleTypeKind::Simple,
            OracleType::Date => &OracleTypeKind::Simple,
            OracleType::Time => &OracleTypeKind::Simple,
            OracleType::Timestamp => &OracleTypeKind::Simple,
            OracleType::TimestampArray => {
                &OracleTypeKind::Array(OracleTypeInfo(OracleType::Timestamp))
            }
            OracleType::DateArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Date)),
            OracleType::TimeArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Time)),
            OracleType::Timestamptz => &OracleTypeKind::Simple,
            OracleType::TimestamptzArray => {
                &OracleTypeKind::Array(OracleTypeInfo(OracleType::Timestamptz))
            }
            OracleType::Interval => &OracleTypeKind::Simple,
            OracleType::IntervalArray => {
                &OracleTypeKind::Array(OracleTypeInfo(OracleType::Interval))
            }
            OracleType::NumericArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Numeric)),
            OracleType::Timetz => &OracleTypeKind::Simple,
            OracleType::TimetzArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Timetz)),
            OracleType::Bit => &OracleTypeKind::Simple,
            OracleType::BitArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Bit)),
            OracleType::Varbit => &OracleTypeKind::Simple,
            OracleType::VarbitArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Varbit)),
            OracleType::Numeric => &OracleTypeKind::Simple,
            OracleType::Uuid => &OracleTypeKind::Simple,
            OracleType::UuidArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Uuid)),
            OracleType::Jsonb => &OracleTypeKind::Simple,
            OracleType::JsonbArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::Jsonb)),
            OracleType::Int4Range => &OracleTypeKind::Range(OracleTypeInfo::INT4),
            OracleType::Int4RangeArray => {
                &OracleTypeKind::Array(OracleTypeInfo(OracleType::Int4Range))
            }
            OracleType::NumRange => &OracleTypeKind::Range(OracleTypeInfo::NUMERIC),
            OracleType::NumRangeArray => {
                &OracleTypeKind::Array(OracleTypeInfo(OracleType::NumRange))
            }
            OracleType::TsRange => &OracleTypeKind::Range(OracleTypeInfo::TIMESTAMP),
            OracleType::TsRangeArray => &OracleTypeKind::Array(OracleTypeInfo(OracleType::TsRange)),
            OracleType::TstzRange => &OracleTypeKind::Range(OracleTypeInfo::TIMESTAMPTZ),
            OracleType::TstzRangeArray => {
                &OracleTypeKind::Array(OracleTypeInfo(OracleType::TstzRange))
            }
            OracleType::DateRange => &OracleTypeKind::Range(OracleTypeInfo::DATE),
            OracleType::DateRangeArray => {
                &OracleTypeKind::Array(OracleTypeInfo(OracleType::DateRange))
            }
            OracleType::Int8Range => &OracleTypeKind::Range(OracleTypeInfo::INT8),
            OracleType::Int8RangeArray => {
                &OracleTypeKind::Array(OracleTypeInfo(OracleType::Int8Range))
            }

            OracleType::Void => &OracleTypeKind::Pseudo,

            OracleType::Custom(ty) => &ty.kind,

            OracleType::DeclareWithOid(oid) => {
                unreachable!("(bug) use of unresolved type declaration [oid={}]", oid.0);
            }
            OracleType::DeclareWithName(name) => {
                unreachable!("(bug) use of unresolved type declaration [name={name}]");
            }
            OracleType::DeclareArrayOf(array) => {
                unreachable!(
                    "(bug) use of unresolved type declaration [array of={}]",
                    array.elem_name
                );
            }
        }
    }

    /// If `self` is an array type, return the type info for its element.
    pub(crate) fn try_array_element(&self) -> Option<Cow<'_, OracleTypeInfo>> {
        // We explicitly match on all the `None` cases to ensure an exhaustive match.
        match self {
            OracleType::Bool => None,
            OracleType::BoolArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Bool))),
            OracleType::Bytea => None,
            OracleType::ByteaArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Bytea))),
            OracleType::Char => None,
            OracleType::CharArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Char))),
            OracleType::Name => None,
            OracleType::NameArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Name))),
            OracleType::Int8 => None,
            OracleType::Int8Array => Some(Cow::Owned(OracleTypeInfo(OracleType::Int8))),
            OracleType::Int2 => None,
            OracleType::Int2Array => Some(Cow::Owned(OracleTypeInfo(OracleType::Int2))),
            OracleType::Int4 => None,
            OracleType::Int4Array => Some(Cow::Owned(OracleTypeInfo(OracleType::Int4))),
            OracleType::Text => None,
            OracleType::TextArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Text))),
            OracleType::Oid => None,
            OracleType::OidArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Oid))),
            OracleType::Json => None,
            OracleType::JsonArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Json))),
            OracleType::Box => None,
            OracleType::BoxArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Box))),
            OracleType::Float4 => None,
            OracleType::Float4Array => Some(Cow::Owned(OracleTypeInfo(OracleType::Float4))),
            OracleType::Float8 => None,
            OracleType::Float8Array => Some(Cow::Owned(OracleTypeInfo(OracleType::Float8))),
            OracleType::Bpchar => None,
            OracleType::BpcharArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Bpchar))),
            OracleType::Varchar => None,
            OracleType::VarcharArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Varchar))),
            OracleType::Date => None,
            OracleType::DateArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Date))),
            OracleType::Time => None,
            OracleType::TimeArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Time))),
            OracleType::Timestamp => None,
            OracleType::TimestampArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Timestamp))),
            OracleType::Timestamptz => None,
            OracleType::TimestamptzArray => {
                Some(Cow::Owned(OracleTypeInfo(OracleType::Timestamptz)))
            }
            OracleType::Interval => None,
            OracleType::IntervalArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Interval))),
            OracleType::Timetz => None,
            OracleType::TimetzArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Timetz))),
            OracleType::Bit => None,
            OracleType::BitArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Bit))),
            OracleType::Varbit => None,
            OracleType::VarbitArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Varbit))),
            OracleType::Numeric => None,
            OracleType::NumericArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Numeric))),
            OracleType::Uuid => None,
            OracleType::UuidArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Uuid))),
            OracleType::Jsonb => None,
            OracleType::JsonbArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Jsonb))),
            OracleType::Int4Range => None,
            OracleType::Int4RangeArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Int4Range))),
            OracleType::NumRange => None,
            OracleType::NumRangeArray => Some(Cow::Owned(OracleTypeInfo(OracleType::NumRange))),
            OracleType::TsRange => None,
            OracleType::TsRangeArray => Some(Cow::Owned(OracleTypeInfo(OracleType::TsRange))),
            OracleType::TstzRange => None,
            OracleType::TstzRangeArray => Some(Cow::Owned(OracleTypeInfo(OracleType::TstzRange))),
            OracleType::DateRange => None,
            OracleType::DateRangeArray => Some(Cow::Owned(OracleTypeInfo(OracleType::DateRange))),
            OracleType::Int8Range => None,
            OracleType::Int8RangeArray => Some(Cow::Owned(OracleTypeInfo(OracleType::Int8Range))),
            // There is no `UnknownArray`
            OracleType::Unknown => None,
            // There is no `VoidArray`
            OracleType::Void => None,

            OracleType::Custom(ty) => match &ty.kind {
                OracleTypeKind::Simple => None,
                OracleTypeKind::Pseudo => None,
                OracleTypeKind::Domain(_) => None,
                OracleTypeKind::Composite(_) => None,
                OracleTypeKind::Array(ref elem_type_info) => Some(Cow::Borrowed(elem_type_info)),
                OracleTypeKind::Enum(_) => None,
                OracleTypeKind::Range(_) => None,
            },
            OracleType::DeclareWithOid(_) => None,
            OracleType::DeclareWithName(name) => {
                // LEGACY: infer the array element name from a `_` prefix
                UStr::strip_prefix(name, "_")
                    .map(|elem| Cow::Owned(OracleTypeInfo(OracleType::DeclareWithName(elem))))
            }
            OracleType::DeclareArrayOf(array) => Some(Cow::Owned(OracleTypeInfo(
                OracleType::DeclareWithName(array.elem_name.clone()),
            ))),
        }
    }

    /// Returns `true` if this type cannot be matched by name.
    fn is_declare_with_oid(&self) -> bool {
        matches!(self, Self::DeclareWithOid(_))
    }

    /// Compare two `OracleType`s, first by OID, then by array element, then by name.
    ///
    /// If `soft_eq` is true and `self` or `other` is `DeclareWithOid` but not both, return `true`
    /// before checking names.
    fn eq_impl(&self, other: &Self, soft_eq: bool) -> bool {
        if let (Some(a), Some(b)) = (self.try_oid(), other.try_oid()) {
            // If there are OIDs available, use OIDs to perform a direct match
            return a == b;
        }

        if soft_eq && (self.is_declare_with_oid() || other.is_declare_with_oid()) {
            // If we get to this point, one instance is `DeclareWithOid()` and the other is
            // `DeclareArrayOf()` or `DeclareWithName()`, which means we can't compare the two.
            //
            // Since this is only likely to occur when using the text protocol where we can't
            // resolve type names before executing a query, we can just opt out of typechecking.
            return true;
        }

        if let (Some(elem_a), Some(elem_b)) = (self.try_array_element(), other.try_array_element())
        {
            return elem_a == elem_b;
        }

        // Otherwise, perform a match on the name
        name_eq(self.name(), other.name())
    }
}

impl TypeInfo for OracleTypeInfo {
    fn name(&self) -> &str {
        self.0.display_name()
    }

    fn is_null(&self) -> bool {
        false
    }

    fn is_void(&self) -> bool {
        matches!(self.0, OracleType::Void)
    }

    fn type_compatible(&self, other: &Self) -> bool
    where
        Self: Sized,
    {
        self == other
    }
}

impl PartialEq<OracleCustomType> for OracleCustomType {
    fn eq(&self, other: &OracleCustomType) -> bool {
        other.oid == self.oid
    }
}

impl OracleTypeInfo {
    // boolean, state of true or false
    pub(crate) const BOOL: Self = Self(OracleType::Bool);
    pub(crate) const BOOL_ARRAY: Self = Self(OracleType::BoolArray);

    // binary data types, variable-length binary string
    pub(crate) const BYTEA: Self = Self(OracleType::Bytea);
    pub(crate) const BYTEA_ARRAY: Self = Self(OracleType::ByteaArray);

    // uuid
    pub(crate) const UUID: Self = Self(OracleType::Uuid);
    pub(crate) const UUID_ARRAY: Self = Self(OracleType::UuidArray);

    //
    // JSON types
    // https://www.postgresql.org/docs/current/datatype-json.html
    //

    pub(crate) const JSON: Self = Self(OracleType::Json);
    pub(crate) const JSON_ARRAY: Self = Self(OracleType::JsonArray);

    pub(crate) const JSONB: Self = Self(OracleType::Jsonb);
    pub(crate) const JSONB_ARRAY: Self = Self(OracleType::JsonbArray);

    // network address types
    // https://www.postgresql.org/docs/current/datatype-net-types.html
    //

    // character types
    // https://www.postgresql.org/docs/current/datatype-character.html
    //

    // internal type for object names
    pub(crate) const NAME: Self = Self(OracleType::Name);
    pub(crate) const NAME_ARRAY: Self = Self(OracleType::NameArray);

    // character type, fixed-length, blank-padded
    pub(crate) const BPCHAR: Self = Self(OracleType::Bpchar);
    pub(crate) const BPCHAR_ARRAY: Self = Self(OracleType::BpcharArray);

    // character type, variable-length with limit
    pub(crate) const VARCHAR: Self = Self(OracleType::Varchar);
    pub(crate) const VARCHAR_ARRAY: Self = Self(OracleType::VarcharArray);

    // character type, variable-length
    pub(crate) const TEXT: Self = Self(OracleType::Text);
    pub(crate) const TEXT_ARRAY: Self = Self(OracleType::TextArray);

    // unknown type, transmitted as text
    pub(crate) const UNKNOWN: Self = Self(OracleType::Unknown);

    //
    // numeric types
    // https://www.postgresql.org/docs/current/datatype-numeric.html
    //

    // single-byte internal type
    pub(crate) const CHAR: Self = Self(OracleType::Char);
    pub(crate) const CHAR_ARRAY: Self = Self(OracleType::CharArray);

    // internal type for type ids
    pub(crate) const OID: Self = Self(OracleType::Oid);
    pub(crate) const OID_ARRAY: Self = Self(OracleType::OidArray);

    // small-range integer; -32768 to +32767
    pub(crate) const INT2: Self = Self(OracleType::Int2);
    pub(crate) const INT2_ARRAY: Self = Self(OracleType::Int2Array);

    // typical choice for integer; -2147483648 to +2147483647
    pub(crate) const INT4: Self = Self(OracleType::Int4);
    pub(crate) const INT4_ARRAY: Self = Self(OracleType::Int4Array);

    // large-range integer; -9223372036854775808 to +9223372036854775807
    pub(crate) const INT8: Self = Self(OracleType::Int8);
    pub(crate) const INT8_ARRAY: Self = Self(OracleType::Int8Array);

    // variable-precision, inexact, 6 decimal digits precision
    pub(crate) const FLOAT4: Self = Self(OracleType::Float4);
    pub(crate) const FLOAT4_ARRAY: Self = Self(OracleType::Float4Array);

    // variable-precision, inexact, 15 decimal digits precision
    pub(crate) const FLOAT8: Self = Self(OracleType::Float8);
    pub(crate) const FLOAT8_ARRAY: Self = Self(OracleType::Float8Array);

    // user-specified precision, exact
    pub(crate) const NUMERIC: Self = Self(OracleType::Numeric);
    pub(crate) const NUMERIC_ARRAY: Self = Self(OracleType::NumericArray);

    //
    // date/time types
    // https://www.postgresql.org/docs/current/datatype-datetime.html
    //

    // both date and time (no time zone)
    pub(crate) const TIMESTAMP: Self = Self(OracleType::Timestamp);
    pub(crate) const TIMESTAMP_ARRAY: Self = Self(OracleType::TimestampArray);

    // both date and time (with time zone)
    pub(crate) const TIMESTAMPTZ: Self = Self(OracleType::Timestamptz);
    pub(crate) const TIMESTAMPTZ_ARRAY: Self = Self(OracleType::TimestamptzArray);

    // date (no time of day)
    pub(crate) const DATE: Self = Self(OracleType::Date);
    pub(crate) const DATE_ARRAY: Self = Self(OracleType::DateArray);

    // time of day (no date)
    pub(crate) const TIME: Self = Self(OracleType::Time);
    pub(crate) const TIME_ARRAY: Self = Self(OracleType::TimeArray);

    // time of day (no date), with time zone
    pub(crate) const TIMETZ: Self = Self(OracleType::Timetz);
    pub(crate) const TIMETZ_ARRAY: Self = Self(OracleType::TimetzArray);

    // time interval
    pub(crate) const INTERVAL: Self = Self(OracleType::Interval);
    pub(crate) const INTERVAL_ARRAY: Self = Self(OracleType::IntervalArray);

    //
    // geometric types
    // https://www.postgresql.org/docs/current/datatype-geometric.html
    //

    // rectangular box
    pub(crate) const BOX: Self = Self(OracleType::Box);
    pub(crate) const BOX_ARRAY: Self = Self(OracleType::BoxArray);

    //
    // bit string types
    // https://www.postgresql.org/docs/current/datatype-bit.html
    //

    pub(crate) const BIT: Self = Self(OracleType::Bit);
    pub(crate) const BIT_ARRAY: Self = Self(OracleType::BitArray);

    pub(crate) const VARBIT: Self = Self(OracleType::Varbit);
    pub(crate) const VARBIT_ARRAY: Self = Self(OracleType::VarbitArray);

    //
    // range types
    // https://www.postgresql.org/docs/current/rangetypes.html
    //
    pub(crate) const NUM_RANGE: Self = Self(OracleType::NumRange);
    pub(crate) const NUM_RANGE_ARRAY: Self = Self(OracleType::NumRangeArray);

    pub(crate) const TS_RANGE: Self = Self(OracleType::TsRange);
    pub(crate) const TS_RANGE_ARRAY: Self = Self(OracleType::TsRangeArray);

    pub(crate) const TSTZ_RANGE: Self = Self(OracleType::TstzRange);
    pub(crate) const TSTZ_RANGE_ARRAY: Self = Self(OracleType::TstzRangeArray);

    pub(crate) const DATE_RANGE: Self = Self(OracleType::DateRange);
    pub(crate) const DATE_RANGE_ARRAY: Self = Self(OracleType::DateRangeArray);

    pub(crate) const INT8_RANGE: Self = Self(OracleType::Int8Range);
    pub(crate) const INT8_RANGE_ARRAY: Self = Self(OracleType::Int8RangeArray);

    //
    // pseudo types
    // https://www.postgresql.org/docs/9.3/datatype-pseudo.html
    //

    pub(crate) const VOID: Self = Self(OracleType::Void);
}

impl Display for OracleTypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self.name())
    }
}

impl PartialEq<OracleType> for OracleType {
    fn eq(&self, other: &OracleType) -> bool {
        self.eq_impl(other, true)
    }
}

/// Check type names for equality, respecting Oracle' case sensitivity rules for identifiers.
///
/// https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-IDENTIFIERS
fn name_eq(name1: &str, name2: &str) -> bool {
    // Cop-out of processing Unicode escapes by just using string equality.
    if name1.starts_with("U&") {
        // If `name2` doesn't start with `U&` this will automatically be `false`.
        return name1 == name2;
    }

    let mut chars1 = identifier_chars(name1);
    let mut chars2 = identifier_chars(name2);

    while let (Some(a), Some(b)) = (chars1.next(), chars2.next()) {
        if !a.eq(&b) {
            return false;
        }
    }

    chars1.next().is_none() && chars2.next().is_none()
}

struct IdentifierChar {
    ch: char,
    case_sensitive: bool,
}

impl IdentifierChar {
    fn eq(&self, other: &Self) -> bool {
        if self.case_sensitive || other.case_sensitive {
            self.ch == other.ch
        } else {
            self.ch.eq_ignore_ascii_case(&other.ch)
        }
    }
}

/// Return an iterator over all significant characters of an identifier.
///
/// Ignores non-escaped quotation marks.
fn identifier_chars(ident: &str) -> impl Iterator<Item = IdentifierChar> + '_ {
    let mut case_sensitive = false;
    let mut last_char_quote = false;

    ident.chars().filter_map(move |ch| {
        if ch == '"' {
            if last_char_quote {
                last_char_quote = false;
            } else {
                last_char_quote = true;
                return None;
            }
        } else if last_char_quote {
            last_char_quote = false;
            case_sensitive = !case_sensitive;
        }

        Some(IdentifierChar { ch, case_sensitive })
    })
}

#[test]
fn test_name_eq() {
    let test_values = [
        ("foo", "foo", true),
        ("foo", "Foo", true),
        ("foo", "FOO", true),
        ("foo", r#""foo""#, true),
        ("foo", r#""Foo""#, false),
        ("foo", "foo.foo", false),
        ("foo.foo", "foo.foo", true),
        ("foo.foo", "foo.Foo", true),
        ("foo.foo", "foo.FOO", true),
        ("foo.foo", "Foo.foo", true),
        ("foo.foo", "Foo.Foo", true),
        ("foo.foo", "FOO.FOO", true),
        ("foo.foo", "foo", false),
        ("foo.foo", r#"foo."foo""#, true),
        ("foo.foo", r#"foo."Foo""#, false),
        ("foo.foo", r#"foo."FOO""#, false),
    ];

    for (left, right, eq) in test_values {
        assert_eq!(
            name_eq(left, right),
            eq,
            "failed check for name_eq({left:?}, {right:?})"
        );
        assert_eq!(
            name_eq(right, left),
            eq,
            "failed check for name_eq({right:?}, {left:?})"
        );
    }
}
