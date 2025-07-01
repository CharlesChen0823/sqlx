use crate::error::{BoxDynError, UnexpectedNullError};
use crate::{Oracle, OracleTypeInfo};
use sqlx_core::bytes::{Buf, Bytes};
pub(crate) use sqlx_core::value::{Value, ValueRef};
use std::borrow::Cow;
use std::str::from_utf8;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum OracleValueFormat {
    Text = 0,
    Binary = 1,
}

/// Implementation of [`ValueRef`] for Oracle.
#[derive(Clone)]
pub struct OracleValueRef<'r> {
    pub(crate) value: Option<&'r [u8]>,
    pub(crate) row: Option<&'r Bytes>,
    pub(crate) type_info: OracleTypeInfo,
    pub(crate) format: OracleValueFormat,
}

/// Implementation of [`Value`] for Oracle.
#[derive(Clone)]
pub struct OracleValue {
    pub(crate) value: Option<Bytes>,
    pub(crate) type_info: OracleTypeInfo,
    pub(crate) format: OracleValueFormat,
}

impl<'r> OracleValueRef<'r> {
    pub(crate) fn get(
        buf: &mut &'r [u8],
        format: OracleValueFormat,
        ty: OracleTypeInfo,
    ) -> Result<Self, String> {
        let element_len = buf.get_i32();

        let element_val = if element_len == -1 {
            None
        } else {
            let element_len: usize = element_len
                .try_into()
                .map_err(|_| format!("overflow converting element_len ({element_len}) to usize"))?;

            let val = &buf[..element_len];
            buf.advance(element_len);
            Some(val)
        };

        Ok(OracleValueRef {
            value: element_val,
            row: None,
            type_info: ty,
            format,
        })
    }

    pub fn format(&self) -> OracleValueFormat {
        self.format
    }

    pub fn as_bytes(&self) -> Result<&'r [u8], BoxDynError> {
        match &self.value {
            Some(v) => Ok(v),
            None => Err(UnexpectedNullError.into()),
        }
    }

    pub fn as_str(&self) -> Result<&'r str, BoxDynError> {
        Ok(from_utf8(self.as_bytes()?)?)
    }
}

impl Value for OracleValue {
    type Database = Oracle;

    #[inline]
    fn as_ref(&self) -> OracleValueRef<'_> {
        OracleValueRef {
            value: self.value.as_deref(),
            row: None,
            type_info: self.type_info.clone(),
            format: self.format,
        }
    }

    fn type_info(&self) -> Cow<'_, OracleTypeInfo> {
        Cow::Borrowed(&self.type_info)
    }

    fn is_null(&self) -> bool {
        self.value.is_none()
    }
}

impl<'r> ValueRef<'r> for OracleValueRef<'r> {
    type Database = Oracle;

    fn to_owned(&self) -> OracleValue {
        let value = match (self.row, self.value) {
            (Some(row), Some(value)) => Some(row.slice_ref(value)),

            (None, Some(value)) => Some(Bytes::copy_from_slice(value)),

            _ => None,
        };

        OracleValue {
            value,
            format: self.format,
            type_info: self.type_info.clone(),
        }
    }

    fn type_info(&self) -> Cow<'_, OracleTypeInfo> {
        Cow::Borrowed(&self.type_info)
    }

    fn is_null(&self) -> bool {
        self.value.is_none()
    }
}
