//! Conversions between Rust and **Oracle** types.
//!
//! # Types
//!
//! | Rust type                             | Oracle type(s)                                     |
//! |---------------------------------------|------------------------------------------------------|
//! | `bool`                                | BOOL                                                 |
//! | `i32`                                 | BINARY_INT                                           |
//! | `f32`                                 | FLOAT, BINARY_FLOAT                                  |
//! | `f64`                                 | DOUBLE, BINARY_DOUBLE                                |
//! | `&str`, [`String`]                    | VARCHAR2, CHAR(N), NCHAR(N), NVARCHAR2               |
//! | `&[u8]`, `Vec<u8>`                    | LONG, BLOB, CLOB, BFILE, RAW, LONG RAW               |
//! | `OraRowId`                            | ROWID                                                |
//! | `OraURowId`                           | UROWID                                               |
//! | `OraIntervalYM`                       | Interval Year TO Month                               |
//! | `OraIntervalDS`                       | Interval Date TO Second                              |
//!
//!
//! ### [`bigdecimal`](https://crates.io/crates/bigdecimal)
//! Requires the `bigdecimal` Cargo feature flag.
//!
//! | Rust type                             | Oracle type(s)                                       |
//! |---------------------------------------|------------------------------------------------------|
//! | `bigdecimal::BigDecimal`              | NUMBER                                               |
//!
#![doc=include_str!("bigdecimal-range.md")]
//!
//! ### [`rust_decimal`](https://crates.io/crates/rust_decimal)
//! Requires the `rust_decimal` Cargo feature flag.
//!
//! | Rust type                             | Oracle type(s)                                       |
//! |---------------------------------------|------------------------------------------------------|
//! | `rust_decimal::Decimal`               | NUMERIC                                              |
//!
#![doc=include_str!("rust_decimal-range.md")]
//!
//! ### [`chrono`](https://crates.io/crates/chrono)
//!
//! Requires the `chrono` Cargo feature flag.
//!
//! | Rust type                             | Oracle type(s)                                       |
//! |---------------------------------------|------------------------------------------------------|
//! | `chrono::DateTime<Utc>`               | TIMESTAMPWTZ                                         |
//! | `chrono::DateTime<Local>`             | TIMESTAMPWLTZ                                        |
//! | `chrono::NaiveDateTime`               | TIMESTAMP                                            |
//! | `chrono::NaiveDate`                   | DATE                                                 |
//!
//! ### [`time`](https://crates.io/crates/time)
//!
//! Requires the `time` Cargo feature flag.
//!
//! | Rust type                             | Oracle type(s)                                     |
//! |---------------------------------------|------------------------------------------------------|
//! | `time::PrimitiveDateTime`             | TIMESTAMP                                            |
//! | `time::OffsetDateTime`                | TIMESTAMPWTZ                                         |
//! | `time::OffsetDateTime`                | TIMESTAMPWLTZ                                        |
//! | `time::Date`                          | DATE                                                 |
//!
//! ### [`json`](https://crates.io/crates/serde_json)
//!
//! Requires the `json` Cargo feature flag.
//!
//! | Rust type                             | Oracle type(s)                                     |
//! |---------------------------------------|------------------------------------------------------|
//! | [`Json<T>`]                           | JSON                                                 |
//! | `serde_json::Value`                   | JSON                                                 |
//! | `&serde_json::value::RawValue`        | JSON                                                 |
//!
//! `Value` and `RawValue` from `serde_json` can be used for unstructured JSON data with
//! Oracle.
//!
//! [`Json<T>`](crate::types::Json) can be used for structured JSON data with Oracle.
//!
//! # [Composite types](https://www.postgresql.org/docs/current/rowtypes.html)
//!
//! User-defined composite types are supported through a derive for `Type`.
//!
//! ```text
//! CREATE TYPE inventory_item AS (
//!     name            text,
//!     supplier_id     integer,
//!     price           numeric
//! );
//! ```
//!
//! ```rust,ignore
//! #[derive(sqlx::Type)]
//! #[sqlx(type_name = "inventory_item")]
//! struct InventoryItem {
//!     name: String,
//!     supplier_id: i32,
//!     price: BigDecimal,
//! }
//! ```
//!
//! Anonymous composite types are represented as tuples. Note that anonymous composites may only
//! be returned and not sent to Oracle (this is a limitation of oracle).
//!
//! # Arrays
//!
//! One-dimensional arrays are supported as `Vec<T>` or `&[T]` where `T` implements `Type`.
//!
//! # [Enumerations](https://www.postgresql.org/docs/current/datatype-enum.html)
//!
//! User-defined enumerations are supported through a derive for `Type`.
//!
//! ```text
//! CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');
//! ```
//!
//! ```rust,ignore
//! #[derive(sqlx::Type)]
//! #[sqlx(type_name = "mood", rename_all = "lowercase")]
//! enum Mood { Sad, Ok, Happy }
//! ```
//!
//! Rust enumerations may also be defined to be represented as an integer using `repr`.
//! The following type expects a SQL type of `INTEGER` or `INT4` and will convert to/from the
//! Rust enumeration.
//!
//! ```rust,ignore
//! #[derive(sqlx::Type)]
//! #[repr(i32)]
//! enum Mood { Sad = 0, Ok = 1, Happy = 2 }
//! ```
//!
//! # Arrays
//!
//! One-dimensional arrays are supported as `Vec<T>` or `&[T]` where `T` implements `Type`.
//!
//! Note that an error can occur if you attempt to decode a value not contained within the enum
//! definition.
//!

use crate::type_info::OracleTypeKind;
use crate::{Oracle, OracleTypeInfo};

pub(crate) use sqlx_core::types::{Json, Type};

mod array;
mod bool;
mod bytes;
mod float;
mod int;
mod interval;
// Not behind a Cargo feature because we require JSON in the driver implementation.
mod json;
mod rowid;
mod str;

#[cfg(any(feature = "chrono", feature = "time"))]
mod time_tz;

#[cfg(feature = "bigdecimal")]
mod bigdecimal;

#[cfg(any(feature = "bigdecimal", feature = "rust_decimal"))]
mod numeric;

#[cfg(feature = "rust_decimal")]
mod rust_decimal;

#[cfg(feature = "chrono")]
mod chrono;

#[cfg(feature = "time")]
mod time;

pub use array::OracleHasArrayType;
pub use interval::{OraIntervalDS, OraIntervalYM};
pub use rowid::{OraRowId, OraURowId};

#[cfg(any(feature = "chrono", feature = "time"))]
pub use time_tz::OracleTimeTz;
