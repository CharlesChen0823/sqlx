use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{
    Oracle, OracleArgumentBuffer, OracleHasArrayType, OracleTypeInfo, OracleValueFormat,
    OracleValueRef,
};
use chrono::{Duration, NaiveTime};
use std::mem;

impl Type<Oracle> for NaiveTime {
    fn type_info() -> OracleTypeInfo {
        OracleTypeInfo::TIME
    }
}

impl OracleHasArrayType for NaiveTime {
    fn array_type_info() -> OracleTypeInfo {
        OracleTypeInfo::TIME_ARRAY
    }
}

impl Encode<'_, Oracle> for NaiveTime {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // TIME is encoded as the microseconds since midnight
        let micros = (*self - NaiveTime::default())
            .num_microseconds()
            .ok_or_else(|| format!("Time out of range for Oracle: {self}"))?;

        Encode::<Oracle>::encode(micros, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<u64>()
    }
}

impl<'r> Decode<'r, Oracle> for NaiveTime {
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            OracleValueFormat::Binary => {
                // TIME is encoded as the microseconds since midnight
                let us: i64 = Decode::<Oracle>::decode(value)?;
                NaiveTime::default() + Duration::microseconds(us)
            }

            OracleValueFormat::Text => NaiveTime::parse_from_str(value.as_str()?, "%H:%M:%S%.f")?,
        })
    }
}

#[test]
fn check_naive_time_default_is_midnight() {
    // Just a canary in case this changes.
    assert_eq!(
        NaiveTime::from_hms_opt(0, 0, 0),
        Some(NaiveTime::default()),
        "implementation assumes `NaiveTime::default()` equals midnight"
    );
}
