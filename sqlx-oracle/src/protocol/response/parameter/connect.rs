use sqlx_core::{bytes::Bytes, Error};

use crate::message::FrontendMessageFormat;

pub struct ConnectParameter;

impl super::Parameter<FrontendMessageFormat, ()> for ConnectParameter {
    fn decode_body_with(mut _buf: Bytes, _: ()) -> Result<Self, Error> {
        // should do nothing
        Ok(ConnectParameter)
    }
}
