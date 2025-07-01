use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::{Oracle, OracleArgumentBuffer, OracleTypeInfo, OracleValueFormat, OracleValueRef};
use serde::{Deserialize, Serialize};
pub(crate) use sqlx_core::types::{Json, Type};

// <https://www.postgresql.org/docs/12/datatype-json.html>

// In general, most applications should prefer to store JSON data as jsonb,
// unless there are quite specialized needs, such as legacy assumptions
// about ordering of object keys.

impl<T> Type<Oracle> for Json<T> {
    fn type_info() -> OracleTypeInfo {
        OracleTypeInfo::JSONB
    }

    fn compatible(ty: &OracleTypeInfo) -> bool {
        *ty == OracleTypeInfo::JSON
    }
}

impl<'q, T> Encode<'q, Oracle> for Json<T>
where
    T: Serialize,
{
    fn encode_by_ref(&self, buf: &mut OracleArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // we have a tiny amount of dynamic behavior depending if we are resolved to be JSON
        // instead of JSONB
        buf.patch(|buf, ty: &OracleTypeInfo| {
            if *ty == OracleTypeInfo::JSON {
                buf[0] = b' ';
            }
        });

        // JSONB version (as of 2020-03-20)
        buf.push(1);

        // the JSON data written to the buffer is the same regardless of parameter type
        serde_json::to_writer(&mut **buf, &self.0)?;

        Ok(IsNull::No)
    }
}

impl<'r, T: 'r> Decode<'r, Oracle> for Json<T>
where
    T: Deserialize<'r>,
{
    fn decode(value: OracleValueRef<'r>) -> Result<Self, BoxDynError> {
        let mut buf = value.as_bytes()?;

        if value.format() == OracleValueFormat::Binary && value.type_info == OracleTypeInfo::JSONB {
            assert_eq!(
                buf[0], 1,
                "unsupported JSONB format version {}; please open an issue",
                buf[0]
            );

            buf = &buf[1..];
        }

        serde_json::from_slice(buf).map(Json).map_err(Into::into)
    }
}
