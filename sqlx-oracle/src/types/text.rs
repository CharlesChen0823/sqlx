use crate::{Oracle, OracleArgumentBuffer, OracleTypeInfo, OracleValueRef};
use sqlx_core::decode::Decode;
use sqlx_core::encode::{Encode, IsNull};
use sqlx_core::error::BoxDynError;
use sqlx_core::types::{Text, Type};
use std::fmt::Display;
use std::str::FromStr;

use std::io::Write;

impl<T> Type<Oracle> for Text<T> {
    fn type_info() -> OracleTypeInfo {
        <String as Type<Oracle>>::type_info()
    }

    fn compatible(ty: &OracleTypeInfo) -> bool {
        <String as Type<Oracle>>::compatible(ty)
    }
}

impl<'q, T> Encode<'q, Oracle> for Text<T>
where
    T: Display,
{
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        write!(**buf, "{}", self.0)?;
        Ok(IsNull::No)
    }
}

impl<'r, T> Decode<'r, Oracle> for Text<T>
where
    T: FromStr,
    BoxDynError: From<<T as FromStr>::Err>,
{
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        let s: &str = Decode::<Oracle>::decode(value)?;
        Ok(Self(s.parse()?))
    }
}
