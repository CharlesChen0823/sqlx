use std::mem;

use chrono::{NaiveDate, TimeDelta};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{
    Oracle, OracleArgumentBuffer, OracleHasArrayType, OracleTypeInfo, OracleValueFormat,
    OracleValueRef,
};

impl Type<Oracle> for NaiveDate {
    fn type_info() -> OracleTypeInfo {
        OracleTypeInfo::DATE
    }
}

impl OracleHasArrayType for NaiveDate {
    fn array_type_info() -> OracleTypeInfo {
        OracleTypeInfo::DATE_ARRAY
    }
}

impl Encode<'_, Oracle> for NaiveDate {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // DATE is encoded as the days since epoch
        let days: i32 = (*self - postgres_epoch_date())
            .num_days()
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

impl<'r> Decode<'r, Oracle> for NaiveDate {
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            OracleValueFormat::Binary => {
                // DATE is encoded as the days since epoch
                let days: i32 = Decode::<Oracle>::decode(value)?;

                let days = TimeDelta::try_days(days.into())
                    .unwrap_or_else(|| {
                        unreachable!("BUG: days ({days}) as `i32` multiplied into seconds should not overflow `i64`")
                    });

                postgres_epoch_date() + days
            }

            OracleValueFormat::Text => NaiveDate::parse_from_str(value.as_str()?, "%Y-%m-%d")?,
        })
    }
}

#[inline]
fn postgres_epoch_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2000, 1, 1).expect("expected 2000-01-01 to be a valid NaiveDate")
}
