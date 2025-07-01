use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{
    Oracle, OracleArgumentBuffer, OracleHasArrayType, OracleTypeInfo, OracleValueFormat,
    OracleValueRef,
};
use std::mem;
use time::macros::format_description;
use time::{Duration, Time};

impl Type<Oracle> for Time {
    fn type_info() -> OracleTypeInfo {
        OracleTypeInfo::TIME
    }
}

impl OracleHasArrayType for Time {
    fn array_type_info() -> OracleTypeInfo {
        OracleTypeInfo::TIME_ARRAY
    }
}

impl Encode<'_, Oracle> for Time {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // TIME is encoded as the microseconds since midnight.
        //
        // A truncating cast is fine because `self - Time::MIDNIGHT` cannot exceed a span of 24 hours.
        #[allow(clippy::cast_possible_truncation)]
        let micros: i64 = (*self - Time::MIDNIGHT).whole_microseconds() as i64;
        Encode::<Oracle>::encode(micros, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<u64>()
    }
}

impl<'r> Decode<'r, Oracle> for Time {
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            OracleValueFormat::Binary => {
                // TIME is encoded as the microseconds since midnight
                let us = Decode::<Oracle>::decode(value)?;
                Time::MIDNIGHT + Duration::microseconds(us)
            }

            OracleValueFormat::Text => Time::parse(
                value.as_str()?,
                // Oracle will not include the subsecond part if it's zero.
                &format_description!("[hour]:[minute]:[second][optional [.[subsecond]]]"),
            )?,
        })
    }
}
