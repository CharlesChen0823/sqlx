use super::{OracleColumn, OracleTypeInfo};
use crate::column::ColumnIndex;
use crate::error::Error;
use crate::ext::ustr::UStr;
use crate::{Oracle, OracleArguments};
use std::borrow::Cow;
use std::sync::Arc;

pub(crate) use sqlx_core::statement::Statement;
use sqlx_core::{Either, HashMap};

#[derive(Debug, Clone)]
pub struct OracleStatement<'q> {
    pub(crate) sql: Cow<'q, str>,
    pub(crate) metadata: Arc<OracleStatementMetadata>,
}

#[derive(Debug, Default)]
pub(crate) struct OracleStatementMetadata {
    pub(crate) columns: Vec<OracleColumn>,
    // This `Arc` is not redundant; it's used to avoid deep-copying this map for the `Any` backend.
    // See `sqlx-oracle/src/any.rs`
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
    pub(crate) parameters: Vec<OracleTypeInfo>,
}

impl<'q> Statement<'q> for OracleStatement<'q> {
    type Database = Oracle;

    fn to_owned(&self) -> OracleStatement<'static> {
        OracleStatement::<'static> {
            sql: Cow::Owned(self.sql.clone().into_owned()),
            metadata: self.metadata.clone(),
        }
    }

    fn sql(&self) -> &str {
        &self.sql
    }

    fn parameters(&self) -> Option<Either<&[OracleTypeInfo], usize>> {
        Some(Either::Left(&self.metadata.parameters))
    }

    fn columns(&self) -> &[OracleColumn] {
        &self.metadata.columns
    }

    impl_statement_query!(OracleArguments);
}

impl ColumnIndex<OracleStatement<'_>> for &'_ str {
    fn index(&self, statement: &OracleStatement<'_>) -> Result<usize, Error> {
        statement
            .metadata
            .column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .copied()
    }
}

// #[cfg(feature = "any")]
// impl<'q> From<OracleStatement<'q>> for crate::any::AnyStatement<'q> {
//     #[inline]
//     fn from(statement: OracleStatement<'q>) -> Self {
//         crate::any::AnyStatement::<'q> {
//             columns: statement
//                 .metadata
//                 .columns
//                 .iter()
//                 .map(|col| col.clone().into())
//                 .collect(),
//             column_names: statement.metadata.column_names.clone(),
//             parameters: Some(Either::Left(
//                 statement
//                     .metadata
//                     .parameters
//                     .iter()
//                     .map(|ty| ty.clone().into())
//                     .collect(),
//             )),
//             sql: statement.sql,
//         }
//     }
// }
