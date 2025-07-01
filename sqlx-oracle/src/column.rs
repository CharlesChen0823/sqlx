use crate::ext::ustr::UStr;
use crate::{Oracle, OracleTypeInfo};

pub(crate) use sqlx_core::column::{Column, ColumnIndex};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub struct OracleColumn {
    pub(crate) ordinal: usize,
    pub(crate) name: UStr,
    pub(crate) type_info: OracleTypeInfo,
    #[cfg_attr(feature = "offline", serde(skip))]
    pub(crate) relation_attribute_no: Option<i16>,
}

impl OracleColumn {
    /// Returns the OID of the table this column is from, if applicable.
    ///
    /// This will be `None` if the column is the result of an expression.
    ///
    /// Corresponds to column `attrelid` of the `oracle_catalog.oracle_attribute` table:
    /// <https://www.postgresql.org/docs/current/catalog-oracle_attribute.html>

    /// Returns the 1-based index of this column in its parent table, if applicable.
    ///
    /// This will be `None` if the column is the result of an expression.
    ///
    /// Corresponds to column `attnum` of the `oracle_catalog.oracle_attribute` table:
    /// <https://www.postgresql.org/docs/current/catalog-oracle_attribute.html>
    pub fn relation_attribute_no(&self) -> Option<i16> {
        self.relation_attribute_no
    }
}

impl Column for OracleColumn {
    type Database = Oracle;

    fn ordinal(&self) -> usize {
        self.ordinal
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn type_info(&self) -> &OracleTypeInfo {
        &self.type_info
    }
}
