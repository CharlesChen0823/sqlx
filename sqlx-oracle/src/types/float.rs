use byteorder::{BigEndian, ByteOrder};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{
    Oracle, OracleArgumentBuffer, OracleHasArrayType, OracleTypeInfo, OracleValueFormat,
    OracleValueRef,
};

impl Type<Oracle> for f32 {
    fn type_info() -> OracleTypeInfo {
        OracleTypeInfo::FLOAT4
    }
}

impl OracleHasArrayType for f32 {
    fn array_type_info() -> OracleTypeInfo {
        OracleTypeInfo::FLOAT4_ARRAY
    }
}

impl Encode<'_, Oracle> for f32 {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Decode<'_, Oracle> for f32 {
    fn decode(value: OracleValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            OracleValueFormat::Binary => BigEndian::read_f32(value.as_bytes()?),
            OracleValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Type<Oracle> for f64 {
    fn type_info() -> OracleTypeInfo {
        OracleTypeInfo::FLOAT8
    }
}

impl OracleHasArrayType for f64 {
    fn array_type_info() -> OracleTypeInfo {
        OracleTypeInfo::FLOAT8_ARRAY
    }
}

impl Encode<'_, Oracle> for f64 {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Decode<'_, Oracle> for f64 {
    fn decode(value: OracleValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            OracleValueFormat::Binary => BigEndian::read_f64(value.as_bytes()?),
            OracleValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}
