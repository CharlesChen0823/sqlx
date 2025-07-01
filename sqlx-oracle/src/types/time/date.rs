use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::time::ORACLE_EPOCH;
use crate::types::Type;
use crate::{
    Oracle, OracleArgumentBuffer, OracleHasArrayType, OracleTypeInfo, OracleValueFormat,
    OracleValueRef,
};
use std::mem;
use time::macros::format_description;
use time::{Date, Duration};

impl Type<Oracle> for Date {
    fn type_info() -> OracleTypeInfo {
        OracleTypeInfo::DATE
    }
}

impl OracleHasArrayType for Date {
    fn array_type_info() -> OracleTypeInfo {
        OracleTypeInfo::DATE_ARRAY
    }
}

impl Encode<'_, Oracle> for Date {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // DATE is encoded as number of days since epoch (2000-01-01)
        let days: i32 = (*self - ORACLE_EPOCH)
            .whole_days()
            .try_into()
            .map_err(|_| {
                format!("value {self:?} would overflow binary encoding for Oracle DATE")
            })?;
        Encode::<Oracle>::encode(days, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<i32>()
    }
}

impl<'r> Decode<'r, Oracle> for Date {
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            OracleValueFormat::Binary => {
                // DATE is encoded as the days since epoch
                let days: i32 = Decode::<Oracle>::decode(value)?;
                ORACLE_EPOCH + Duration::days(days.into())
            }

            OracleValueFormat::Text => Date::parse(
                value.as_str()?,
                &format_description!("[year]-[month]-[day]"),
            )?,
        })
    }
}
