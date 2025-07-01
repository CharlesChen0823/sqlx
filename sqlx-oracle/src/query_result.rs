use std::iter::{Extend, IntoIterator};

#[derive(Debug, Default)]
pub struct OracleQueryResult {
    pub(super) rows_affected: u64,
}

impl OracleQueryResult {
    pub fn rows_affected(&self) -> u64 {
        self.rows_affected
    }
}

impl Extend<OracleQueryResult> for OracleQueryResult {
    fn extend<T: IntoIterator<Item = OracleQueryResult>>(&mut self, iter: T) {
        for elem in iter {
            self.rows_affected += elem.rows_affected;
        }
    }
}

#[cfg(feature = "any")]
impl From<OracleQueryResult> for sqlx_core::any::AnyQueryResult {
    fn from(done: OracleQueryResult) -> Self {
        sqlx_core::any::AnyQueryResult {
            rows_affected: done.rows_affected,
            last_insert_id: None,
        }
    }
}
