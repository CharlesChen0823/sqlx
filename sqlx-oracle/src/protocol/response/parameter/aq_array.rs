use sqlx_core::{bytes::Bytes, Error};

use crate::message::FrontendMessageFormat;

pub struct AQArrayParameter;

impl super::Parameter<FrontendMessageFormat, ()> for AQArrayParameter {
    fn decode_body_with(mut _buf: Bytes, _: ()) -> Result<Self, Error> {
        // Implementation goes here
        unimplemented!()
    }
}
