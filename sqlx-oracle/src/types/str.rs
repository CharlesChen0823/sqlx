use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{Oracle, OracleArgumentBuffer, OracleHasArrayType, OracleTypeInfo, OracleValueRef};
use std::borrow::Cow;

impl Type<Oracle> for str {
    fn type_info() -> OracleTypeInfo {
        OracleTypeInfo::TEXT
    }

    fn compatible(ty: &OracleTypeInfo) -> bool {
        [
            OracleTypeInfo::TEXT,
            OracleTypeInfo::NAME,
            OracleTypeInfo::BPCHAR,
            OracleTypeInfo::VARCHAR,
            OracleTypeInfo::UNKNOWN,
        ]
        .contains(ty)
    }
}

impl Type<Oracle> for Cow<'_, str> {
    fn type_info() -> OracleTypeInfo {
        <&str as Type<Oracle>>::type_info()
    }

    fn compatible(ty: &OracleTypeInfo) -> bool {
        <&str as Type<Oracle>>::compatible(ty)
    }
}

impl Type<Oracle> for Box<str> {
    fn type_info() -> OracleTypeInfo {
        <&str as Type<Oracle>>::type_info()
    }

    fn compatible(ty: &OracleTypeInfo) -> bool {
        <&str as Type<Oracle>>::compatible(ty)
    }
}

impl Type<Oracle> for String {
    fn type_info() -> OracleTypeInfo {
        <&str as Type<Oracle>>::type_info()
    }

    fn compatible(ty: &OracleTypeInfo) -> bool {
        <&str as Type<Oracle>>::compatible(ty)
    }
}

impl OracleHasArrayType for &'_ str {
    fn array_type_info() -> OracleTypeInfo {
        OracleTypeInfo::TEXT_ARRAY
    }
}

impl OracleHasArrayType for Cow<'_, str> {
    fn array_type_info() -> OracleTypeInfo {
        <&str as OracleHasArrayType>::array_type_info()
    }

    fn array_compatible(ty: &OracleTypeInfo) -> bool {
        <&str as OracleHasArrayType>::array_compatible(ty)
    }
}

impl OracleHasArrayType for Box<str> {
    fn array_type_info() -> OracleTypeInfo {
        <&str as OracleHasArrayType>::array_type_info()
    }

    fn array_compatible(ty: &OracleTypeInfo) -> bool {
        <&str as OracleHasArrayType>::array_compatible(ty)
    }
}

impl OracleHasArrayType for String {
    fn array_type_info() -> OracleTypeInfo {
        <&str as OracleHasArrayType>::array_type_info()
    }

    fn array_compatible(ty: &OracleTypeInfo) -> bool {
        <&str as OracleHasArrayType>::array_compatible(ty)
    }
}

impl Encode<'_, Oracle> for &'_ str {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        buf.extend(self.as_bytes());

        Ok(IsNull::No)
    }
}

impl Encode<'_, Oracle> for Cow<'_, str> {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        match self {
            Cow::Borrowed(str) => <&str as Encode<Oracle>>::encode(*str, buf),
            Cow::Owned(str) => <&str as Encode<Oracle>>::encode(&**str, buf),
        }
    }
}

impl Encode<'_, Oracle> for Box<str> {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <&str as Encode<Oracle>>::encode(&**self, buf)
    }
}

impl Encode<'_, Oracle> for String {
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <&str as Encode<Oracle>>::encode(&**self, buf)
    }
}

impl<'r> Decode<'r, Oracle> for &'r str {
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        value.as_str()
    }
}

impl<'r> Decode<'r, Oracle> for Cow<'r, str> {
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(Cow::Borrowed(value.as_str()?))
    }
}

impl<'r> Decode<'r, Oracle> for Box<str> {
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(Box::from(value.as_str()?))
    }
}

impl Decode<'_, Oracle> for String {
    fn decode(value: OracleValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(value.as_str()?.to_owned())
    }
}
