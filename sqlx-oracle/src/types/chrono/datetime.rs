use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{
    Oracle, OracleArgumentBuffer, OracleHasArrayType, OracleTypeInfo, OracleValueFormat,
    OracleValueRef,
};
use chrono::{
    DateTime, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, Offset, TimeZone, Utc,
};
use std::mem;

impl Type<Oracle> for NaiveDateTime {
    fn type_info() -> OracleTypeInfo {
        OracleTypeInfo::TIMESTAMP
    }
}

impl<Tz: TimeZone> Type<Oracle> for DateTime<Tz> {
    fn type_info() -> OracleTypeInfo {
        OracleTypeInfo::TIMESTAMPTZ
    }
}

impl OracleHasArrayType for NaiveDateTime {
    fn array_type_info() -> OracleTypeInfo {
        OracleTypeInfo::TIMESTAMP_ARRAY
    }
}

impl<Tz: TimeZone> OracleHasArrayType for DateTime<Tz> {
    fn array_type_info() -> OracleTypeInfo {
        OracleTypeInfo::TIMESTAMPTZ_ARRAY
    }
}

impl Encode<'_, Oracle> for NaiveDateTime {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // TIMESTAMP is encoded as the microseconds since the epoch
        let micros = (*self - postgres_epoch_datetime())
            .num_microseconds()
            .ok_or_else(|| format!("NaiveDateTime out of range for Oracle: {self:?}"))?;

        Encode::<Oracle>::encode(micros, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<i64>()
    }
}

impl<'r> Decode<'r, Oracle> for NaiveDateTime {
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            OracleValueFormat::Binary => {
                // TIMESTAMP is encoded as the microseconds since the epoch
                let us = Decode::<Oracle>::decode(value)?;
                postgres_epoch_datetime() + Duration::microseconds(us)
            }

            OracleValueFormat::Text => {
                let s = value.as_str()?;
                NaiveDateTime::parse_from_str(
                    s,
                    if s.contains('+') {
                        // Contains a time-zone specifier
                        // This is given for timestamptz for some reason
                        // Oracle already guarantees this to always be UTC
                        "%Y-%m-%d %H:%M:%S%.f%#z"
                    } else {
                        "%Y-%m-%d %H:%M:%S%.f"
                    },
                )?
            }
        })
    }
}

impl<Tz: TimeZone> Encode<'_, Oracle> for DateTime<Tz> {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        Encode::<Oracle>::encode(self.naive_utc(), buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<i64>()
    }
}

impl<'r> Decode<'r, Oracle> for DateTime<Local> {
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        let fixed = <DateTime<FixedOffset> as Decode<Oracle>>::decode(value)?;
        Ok(Local.from_utc_datetime(&fixed.naive_utc()))
    }
}

impl<'r> Decode<'r, Oracle> for DateTime<Utc> {
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        let fixed = <DateTime<FixedOffset> as Decode<Oracle>>::decode(value)?;
        Ok(Utc.from_utc_datetime(&fixed.naive_utc()))
    }
}

impl<'r> Decode<'r, Oracle> for DateTime<FixedOffset> {
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            OracleValueFormat::Binary => {
                let naive = <NaiveDateTime as Decode<Oracle>>::decode(value)?;
                Utc.fix().from_utc_datetime(&naive)
            }

            OracleValueFormat::Text => {
                let s = value.as_str()?;
                DateTime::parse_from_str(
                    s,
                    if s.contains('+') || s.contains('-') {
                        // Contains a time-zone specifier
                        // This is given for timestamptz for some reason
                        // Oracle already guarantees this to always be UTC
                        "%Y-%m-%d %H:%M:%S%.f%#z"
                    } else {
                        "%Y-%m-%d %H:%M:%S%.f"
                    },
                )?
            }
        })
    }
}

#[inline]
fn postgres_epoch_datetime() -> NaiveDateTime {
    NaiveDate::from_ymd_opt(2000, 1, 1)
        .expect("expected 2000-01-01 to be a valid NaiveDate")
        .and_hms_opt(0, 0, 0)
        .expect("expected 2000-01-01T00:00:00 to be a valid NaiveDateTime")
}
