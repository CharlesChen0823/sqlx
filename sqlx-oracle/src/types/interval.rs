use std::mem;

use byteorder::{NetworkEndian, ReadBytesExt};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{Oracle, OracleArgumentBuffer, OracleTypeInfo, OracleValueFormat, OracleValueRef};

// `OracleInterval` is available for direct access to the INTERVAL type

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Default)]
pub struct OraIntervalYM {
    pub years: i32,
    pub months: i32,
}

pub struct OraIntervalDS {
    pub days: i32,
    pub microseconds: i64,
}

impl Type<Oracle> for OraIntervalYM {
    fn type_info() -> OracleTypeInfo {
        OracleTypeInfo::INTERVAL
    }
}

impl<'de> Decode<'de, Oracle> for OraIntervalYM {
    fn decode(value: OracleValueRef<'de>) -> Result<Self, BoxDynError> {
        match value.format() {
            OracleValueFormat::Binary => {
                let mut buf = value.as_bytes()?;
                let years = buf.read_i32::<NetworkEndian>()?;
                let months = buf.read_i32::<NetworkEndian>()?;

                Ok(OraIntervalYM { years, months })
            }

            // TODO: Implement parsing of text mode
            OracleValueFormat::Text => {
                Err("not implemented: decode `INTERVAL` in text mode (unprepared queries)".into())
            }
        }
    }
}

impl Encode<'_, Oracle> for OraIntervalYM {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.years.to_be_bytes());
        buf.extend(&self.months.to_be_bytes());

        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        2 * mem::size_of::<i64>()
    }
}
