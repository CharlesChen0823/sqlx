use crate::Oracle;

// The paths used below will also be emitted by the macros so they have to match the final facade.
#[allow(unused_imports, dead_code)]
mod sqlx {
    pub use crate as oracle;
    pub use sqlx_core::*;
}

impl_type_checking!(
        Oracle {
        bool,
        String | &str,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64,
        Vec<u8> | &[u8],

        // sqlx::oracle::types::OracleIntervalYM,
        // sqlx::oracle::types::OracleIntervalDS,


        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::types::chrono::NaiveTime,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::types::chrono::NaiveDate,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::types::chrono::NaiveDateTime,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> | sqlx::types::chrono::DateTime<_>,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::oracle::types::OracleTimeTz<sqlx::types::chrono::NaiveTime, sqlx::types::chrono::FixedOffset>,

        #[cfg(feature = "time")]
        sqlx::types::time::Time,

        #[cfg(feature = "time")]
        sqlx::types::time::Date,

        #[cfg(feature = "time")]
        sqlx::types::time::PrimitiveDateTime,

        #[cfg(feature = "time")]
        sqlx::types::time::OffsetDateTime,

        #[cfg(feature = "time")]
        sqlx::oracle::types::OracleTimeTz<sqlx::types::time::Time, sqlx::types::time::UtcOffset>,

        #[cfg(feature = "bigdecimal")]
        sqlx::types::BigDecimal,

        #[cfg(feature = "rust_decimal")]
        sqlx::types::Decimal,

        #[cfg(feature = "json")]
        sqlx::types::JsonValue,

        #[cfg(feature = "bit-vec")]
        sqlx::types::BitVec,

        // Arrays

        Vec<bool> | &[bool],
        Vec<String> | &[String],
        Vec<Vec<u8>> | &[Vec<u8>],
        Vec<i8> | &[i8],
        Vec<i16> | &[i16],
        Vec<i32> | &[i32],
        Vec<i64> | &[i64],
        Vec<f32> | &[f32],
        Vec<f64> | &[f64],

        #[cfg(feature = "uuid")]
        Vec<sqlx::types::Uuid> | &[sqlx::types::Uuid],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::types::chrono::NaiveTime> | &[sqlx::types::chrono::NaiveTime],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::types::chrono::NaiveDate> | &[sqlx::types::chrono::NaiveDate],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::types::chrono::NaiveDateTime> | &[sqlx::types::chrono::NaiveDateTime],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> | &[sqlx::types::chrono::DateTime<_>],

        #[cfg(feature = "time")]
        Vec<sqlx::types::time::Time> | &[sqlx::types::time::Time],

        #[cfg(feature = "time")]
        Vec<sqlx::types::time::Date> | &[sqlx::types::time::Date],

        #[cfg(feature = "time")]
        Vec<sqlx::types::time::PrimitiveDateTime> | &[sqlx::types::time::PrimitiveDateTime],

        #[cfg(feature = "time")]
        Vec<sqlx::types::time::OffsetDateTime> | &[sqlx::types::time::OffsetDateTime],

        #[cfg(feature = "bigdecimal")]
        Vec<sqlx::types::BigDecimal> | &[sqlx::types::BigDecimal],

        #[cfg(feature = "rust_decimal")]
        Vec<sqlx::types::Decimal> | &[sqlx::types::Decimal],

        // #[cfg(feature = "json")]
        // Vec<sqlx::types::JsonValue> | &[sqlx::types::JsonValue],

        #[cfg(feature = "bigdecimal")]
        sqlx::oracle::types::OracleRange<sqlx::types::BigDecimal>,

        #[cfg(feature = "rust_decimal")]
        sqlx::oracle::types::OracleRange<sqlx::types::Decimal>,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::oracle::types::OracleRange<sqlx::types::chrono::NaiveDate>,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::oracle::types::OracleRange<sqlx::types::chrono::NaiveDateTime>,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::oracle::types::OracleRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> |
            sqlx::oracle::types::OracleRange<sqlx::types::chrono::DateTime<_>>,

        #[cfg(feature = "time")]
        sqlx::oracle::types::OracleRange<sqlx::types::time::Date>,

        #[cfg(feature = "time")]
        sqlx::oracle::types::OracleRange<sqlx::types::time::PrimitiveDateTime>,

        #[cfg(feature = "time")]
        sqlx::oracle::types::OracleRange<sqlx::types::time::OffsetDateTime>,

        #[cfg(feature = "bigdecimal")]
        Vec<sqlx::oracle::types::OracleRange<sqlx::types::BigDecimal>> |
            &[sqlx::oracle::types::OracleRange<sqlx::types::BigDecimal>],

        #[cfg(feature = "rust_decimal")]
        Vec<sqlx::oracle::types::OracleRange<sqlx::types::Decimal>> |
            &[sqlx::oracle::types::OracleRange<sqlx::types::Decimal>],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::oracle::types::OracleRange<sqlx::types::chrono::NaiveDate>> |
            &[sqlx::oracle::types::OracleRange<sqlx::types::chrono::NaiveDate>],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::oracle::types::OracleRange<sqlx::types::chrono::NaiveDateTime>> |
            &[sqlx::oracle::types::OracleRange<sqlx::types::chrono::NaiveDateTime>],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::oracle::types::OracleRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>> |
            &[sqlx::oracle::types::OracleRange<sqlx::types::chrono::DateTime<_>>],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::oracle::types::OracleRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>> |
            &[sqlx::oracle::types::OracleRange<sqlx::types::chrono::DateTime<_>>],

        #[cfg(feature = "time")]
        Vec<sqlx::oracle::types::OracleRange<sqlx::types::time::Date>> |
            &[sqlx::oracle::types::OracleRange<sqlx::types::time::Date>],

        #[cfg(feature = "time")]
        Vec<sqlx::oracle::types::OracleRange<sqlx::types::time::PrimitiveDateTime>> |
            &[sqlx::oracle::types::OracleRange<sqlx::types::time::PrimitiveDateTime>],

        #[cfg(feature = "time")]
        Vec<sqlx::oracle::types::OracleRange<sqlx::types::time::OffsetDateTime>> |
            &[sqlx::oracle::types::OracleRange<sqlx::types::time::OffsetDateTime>],
    },
    ParamChecking::Strong,
    feature-types: info => info.__type_feature_gate(),
);
