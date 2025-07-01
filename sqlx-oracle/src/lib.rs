//! **Oracle** database driver.

#[macro_use]
extern crate sqlx_core;

use crate::executor::Executor;

mod arguments;
mod column;
mod connection;
mod constants;
mod database;
mod error;
mod io;
mod listener;
mod message;
mod options;
mod protocol;
mod query_result;
mod row;
mod statement;
mod transaction;
mod type_checking;
mod type_info;
pub mod types;
mod value;

#[cfg(feature = "any")]
// We are hiding the any module with its AnyConnectionBackend trait
// so that IDEs don't show it in the autocompletion list
// and end users don't accidentally use it. This can result in
// nested transactions not behaving as expected.
// For more information, see https://github.com/launchbadge/sqlx/pull/3254#issuecomment-2144043823
#[doc(hidden)]
pub mod any;

#[cfg(feature = "migrate")]
mod migrate;

#[cfg(feature = "migrate")]
mod testing;

pub(crate) use sqlx_core::driver_prelude::*;

pub use arguments::{OracleArgumentBuffer, OracleArguments};
pub use column::OracleColumn;
pub use connection::OracleConnection;
pub use database::Oracle;
pub use error::{OracleDatabaseError, OracleErrorPosition};
pub use listener::{OracleListener, OracleNotification};
pub use options::{OracleConnectOptions, OracleSslMode};
pub use query_result::OracleQueryResult;
pub use row::OracleRow;
pub use statement::OracleStatement;
pub use transaction::OracleTransactionManager;
pub use type_info::{OracleTypeInfo, OracleTypeKind};
pub use types::OracleHasArrayType;
pub use value::{OracleValue, OracleValueFormat, OracleValueRef};

/// An alias for [`Pool`][crate::pool::Pool], specialized for Oracle.
pub type OraclePool = crate::pool::Pool<Oracle>;

/// An alias for [`PoolOptions`][crate::pool::PoolOptions], specialized for Oracle.
pub type OraclePoolOptions = crate::pool::PoolOptions<Oracle>;

/// An alias for [`Executor<'_, Database = Oracle>`][Executor].
pub trait OracleExecutor<'c>: Executor<'c, Database = Oracle> {}
impl<'c, T: Executor<'c, Database = Oracle>> OracleExecutor<'c> for T {}

/// An alias for [`Transaction`][crate::transaction::Transaction], specialized for Oracle.
pub type OracleTransaction<'c> = crate::transaction::Transaction<'c, Oracle>;

impl_into_arguments_for_arguments!(OracleArguments);
impl_acquire!(Oracle, OracleConnection);
impl_column_index_for_row!(OracleRow);
impl_column_index_for_statement!(OracleStatement);
impl_encode_for_option!(Oracle);
