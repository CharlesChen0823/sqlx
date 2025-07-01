use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{
    Oracle, OracleArgumentBuffer, OracleHasArrayType, OracleTypeInfo, OracleValueFormat,
    OracleValueRef,
};

impl Type<Oracle> for bool {
    fn type_info() -> OracleTypeInfo {
        OracleTypeInfo::BOOL
    }
}

impl OracleHasArrayType for bool {
    fn array_type_info() -> OracleTypeInfo {
        OracleTypeInfo::BOOL_ARRAY
    }
}

impl Encode<'_, Oracle> for bool {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        buf.push(*self as u8);

        Ok(IsNull::No)
    }
}

impl Decode<'_, Oracle> for bool {
    fn decode(value: OracleValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            OracleValueFormat::Binary => value.as_bytes()?[0] != 0,

            OracleValueFormat::Text => match value.as_str()? {
                "t" => true,
                "f" => false,

                s => {
                    return Err(format!("unexpected value {s:?} for boolean").into());
                }
            },
        })
    }
}
