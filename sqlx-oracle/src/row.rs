use crate::column::ColumnIndex;
use crate::error::Error;
use crate::statement::OracleStatementMetadata;
use crate::value::OracleValueFormat;
use crate::{Oracle, OracleColumn, OracleValueRef};
pub(crate) use sqlx_core::row::Row;
use sqlx_core::type_checking::TypeChecking;
use sqlx_core::value::ValueRef;
use std::fmt::Debug;
use std::sync::Arc;

/// Implementation of [`Row`] for Oracle.
pub struct OracleRow {
    pub(crate) data: DataRow,
    pub(crate) format: OracleValueFormat,
    pub(crate) metadata: Arc<OracleStatementMetadata>,
}

impl Row for OracleRow {
    type Database = Oracle;

    fn columns(&self) -> &[OracleColumn] {
        &self.metadata.columns
    }

    fn try_get_raw<I>(&self, index: I) -> Result<OracleValueRef<'_>, Error>
    where
        I: ColumnIndex<Self>,
    {
        let index = index.index(self)?;
        let column = &self.metadata.columns[index];
        let value = self.data.get(index);

        Ok(OracleValueRef {
            format: self.format,
            row: Some(&self.data.storage),
            type_info: column.type_info.clone(),
            value,
        })
    }
}

impl ColumnIndex<OracleRow> for &'_ str {
    fn index(&self, row: &OracleRow) -> Result<usize, Error> {
        row.metadata
            .column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .copied()
    }
}

impl Debug for OracleRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OracleRow ")?;

        let mut debug_map = f.debug_map();
        for (index, column) in self.columns().iter().enumerate() {
            match self.try_get_raw(index) {
                Ok(value) => {
                    debug_map.entry(
                        &column.name,
                        &Oracle::fmt_value_debug(&<OracleValueRef as ValueRef>::to_owned(&value)),
                    );
                }
                Err(error) => {
                    debug_map.entry(&column.name, &format!("decode error: {error:?}"));
                }
            }
        }

        debug_map.finish()
    }
}
