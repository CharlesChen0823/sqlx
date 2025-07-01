use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{
    Oracle, OracleArgumentBuffer, OracleHasArrayType, OracleTypeInfo, OracleValueFormat,
    OracleValueRef,
};

impl OracleHasArrayType for u8 {
    fn array_type_info() -> OracleTypeInfo {
        OracleTypeInfo::BYTEA
    }
}

impl OracleHasArrayType for &'_ [u8] {
    fn array_type_info() -> OracleTypeInfo {
        OracleTypeInfo::BYTEA_ARRAY
    }
}

impl OracleHasArrayType for Box<[u8]> {
    fn array_type_info() -> OracleTypeInfo {
        <[&[u8]] as Type<Oracle>>::type_info()
    }
}

impl OracleHasArrayType for Vec<u8> {
    fn array_type_info() -> OracleTypeInfo {
        <[&[u8]] as Type<Oracle>>::type_info()
    }
}

impl<const N: usize> OracleHasArrayType for [u8; N] {
    fn array_type_info() -> OracleTypeInfo {
        <[&[u8]] as Type<Oracle>>::type_info()
    }
}

impl Encode<'_, Oracle> for &'_ [u8] {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        buf.extend_from_slice(self);

        Ok(IsNull::No)
    }
}

impl Encode<'_, Oracle> for Box<[u8]> {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <&[u8] as Encode<Oracle>>::encode(self.as_ref(), buf)
    }
}

impl Encode<'_, Oracle> for Vec<u8> {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <&[u8] as Encode<Oracle>>::encode(self, buf)
    }
}

impl<const N: usize> Encode<'_, Oracle> for [u8; N] {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <&[u8] as Encode<Oracle>>::encode(self.as_slice(), buf)
    }
}

impl<'r> Decode<'r, Oracle> for &'r [u8] {
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            OracleValueFormat::Binary => value.as_bytes(),
            OracleValueFormat::Text => {
                Err("unsupported decode to `&[u8]` of BYTEA in a simple query; use a prepared query or decode to `Vec<u8>`".into())
            }
        }
    }
}

fn text_hex_decode_input(value: OracleValueRef<'_>) -> Result<&[u8], BoxDynError> {
    // BYTEA is formatted as \x followed by hex characters
    value
        .as_bytes()?
        .strip_prefix(b"\\x")
        .ok_or("text does not start with \\x")
        .map_err(Into::into)
}

impl Decode<'_, Oracle> for Box<[u8]> {
    fn decode(value: OracleValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            OracleValueFormat::Binary => Box::from(value.as_bytes()?),
            OracleValueFormat::Text => Box::from(hex::decode(text_hex_decode_input(value)?)?),
        })
    }
}

impl Decode<'_, Oracle> for Vec<u8> {
    fn decode(value: OracleValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            OracleValueFormat::Binary => value.as_bytes()?.to_owned(),
            OracleValueFormat::Text => hex::decode(text_hex_decode_input(value)?)?,
        })
    }
}

impl<const N: usize> Decode<'_, Oracle> for [u8; N] {
    fn decode(value: OracleValueRef<'_>) -> Result<Self, BoxDynError> {
        let mut bytes = [0u8; N];
        match value.format() {
            OracleValueFormat::Binary => {
                bytes = value.as_bytes()?.try_into()?;
            }
            OracleValueFormat::Text => {
                hex::decode_to_slice(text_hex_decode_input(value)?, &mut bytes)?
            }
        };
        Ok(bytes)
    }
}
