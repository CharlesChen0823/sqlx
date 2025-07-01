use sqlx_core::{
    bytes::{Buf, Bytes},
    Error,
};

use crate::{io::OraBufExt, message::FrontendMessageFormat};

pub struct TpcSwitchParameter {
    pub application_value: u32,
    pub context: Bytes,
}

impl super::Parameter<FrontendMessageFormat, ()> for TpcSwitchParameter {
    fn decode_body_with(mut buf: Bytes, _: ()) -> Result<Self, Error> {
        let application_value = buf.get_u32();
        let context_len = buf.get_u16();
        let context = buf._read_raw_bytes_and_length(context_len as usize);
        Ok(Self {
            application_value,
            context,
        })
    }
}
