use sqlx_core::{bytes::Bytes, Error};

use crate::message::FrontendMessageFormat;

pub struct AQDeqParameter;

impl super::Parameter<FrontendMessageFormat, ()> for AQDeqParameter {
    fn decode_body_with(mut _buf: Bytes, _: ()) -> Result<Self, Error> {
        // Implementation goes here
        unimplemented!()
    }
}
