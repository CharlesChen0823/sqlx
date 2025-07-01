use sqlx_core::{
    bytes::{Buf, Bytes},
    Error,
};

use crate::message::FrontendMessageFormat;

pub struct TpcChangeStateParameter {
    state: u32,
}

impl super::Parameter<FrontendMessageFormat, ()> for TpcChangeStateParameter {
    fn decode_body_with(mut buf: Bytes, _: ()) -> Result<Self, Error> {
        // Implementation goes here
        let state = buf.get_u32();
        Ok(TpcChangeStateParameter { state })
    }
}
