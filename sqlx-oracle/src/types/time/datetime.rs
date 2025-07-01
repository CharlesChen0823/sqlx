use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::time::ORACLE_EPOCH;
use crate::types::Type;
use crate::{
    Oracle, OracleArgumentBuffer, OracleHasArrayType, OracleTypeInfo, OracleValueFormat,
    OracleValueRef,
};
use std::borrow::Cow;
use std::mem;
use time::macros::format_description;
use time::macros::offset;
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

impl Type<Oracle> for PrimitiveDateTime {
    fn type_info() -> OracleTypeInfo {
        OracleTypeInfo::TIMESTAMP
    }
}

impl Type<Oracle> for OffsetDateTime {
    fn type_info() -> OracleTypeInfo {
        OracleTypeInfo::TIMESTAMPTZ
    }
}

impl OracleHasArrayType for PrimitiveDateTime {
    fn array_type_info() -> OracleTypeInfo {
        OracleTypeInfo::TIMESTAMP_ARRAY
    }
}

impl OracleHasArrayType for OffsetDateTime {
    fn array_type_info() -> OracleTypeInfo {
        OracleTypeInfo::TIMESTAMPTZ_ARRAY
    }
}

impl Encode<'_, Oracle> for PrimitiveDateTime {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // TIMESTAMP is encoded as the microseconds since the epoch
        let micros: i64 = (*self - ORACLE_EPOCH.midnight())
            .whole_microseconds()
            .try_into()
            .map_err(|_| {
                format!("value {self:?} would overflow binary encoding for Oracle TIME")
            })?;
        Encode::<Oracle>::encode(micros, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<i64>()
    }
}

impl<'r> Decode<'r, Oracle> for PrimitiveDateTime {
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            OracleValueFormat::Binary => {
                // TIMESTAMP is encoded as the microseconds since the epoch
                let us = Decode::<Oracle>::decode(value)?;
                ORACLE_EPOCH.midnight() + Duration::microseconds(us)
            }

            OracleValueFormat::Text => {
                let s = value.as_str()?;

                // If there is no decimal point we need to add one.
                let s = if s.contains('.') {
                    Cow::Borrowed(s)
                } else {
                    Cow::Owned(format!("{s}.0"))
                };

                // Contains a time-zone specifier
                // This is given for timestamptz for some reason
                // Oracle already guarantees this to always be UTC
                if s.contains('+') {
                    PrimitiveDateTime::parse(&s, &format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond][offset_hour]"))?
                } else {
                    PrimitiveDateTime::parse(
                        &s,
                        &format_description!(
                            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"
                        ),
                    )?
                }
            }
        })
    }
}

impl Encode<'_, Oracle> for OffsetDateTime {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let utc = self.to_offset(offset!(UTC));
        let primitive = PrimitiveDateTime::new(utc.date(), utc.time());

        Encode::<Oracle>::encode(primitive, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<i64>()
    }
}

impl<'r> Decode<'r, Oracle> for OffsetDateTime {
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(<PrimitiveDateTime as Decode<Oracle>>::decode(value)?.assume_utc())
    }
}
