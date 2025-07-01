use sqlx_core::{bytes::Bytes, Error};

use crate::message::FrontendMessageFormat;

pub struct DataTypeParameter;

impl super::Parameter<FrontendMessageFormat, ()> for DataTypeParameter {
    fn decode_body_with(mut _buf: Bytes, _: ()) -> Result<Self, Error> {
        // should do nothing
        Ok(DataTypeParameter)
    }
}
