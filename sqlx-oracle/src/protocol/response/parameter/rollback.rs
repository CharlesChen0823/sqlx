use sqlx_core::{bytes::Bytes, Error};

use crate::message::FrontendMessageFormat;

pub struct RollbackParameter;

impl super::Parameter<FrontendMessageFormat, ()> for RollbackParameter {
    fn decode_body_with(mut _buf: Bytes, _: ()) -> Result<Self, Error> {
        // should do nothing
        Ok(Self)
    }
}
