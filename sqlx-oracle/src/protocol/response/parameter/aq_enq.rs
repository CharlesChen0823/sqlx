use sqlx_core::{bytes::Bytes, Error};

use crate::message::FrontendMessageFormat;

pub struct AQEnqParameter;

impl super::Parameter<FrontendMessageFormat, ()> for AQEnqParameter {
    fn decode_body_with(mut _buf: Bytes, _: ()) -> Result<Self, Error> {
        // Implementation goes here
        unimplemented!()
    }
}
