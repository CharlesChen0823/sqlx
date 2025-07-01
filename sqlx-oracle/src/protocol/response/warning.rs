use sqlx_core::bytes::Buf;

use crate::{constants::CS_FORM_IMPLICIT, io::OraBufExt, protocol::response::Response};

pub struct Warning {
    pub error_num: u16,
    pub message: Option<String>,
}

impl Response for Warning {
    fn decode_body_with(
        mut buf: sqlx_core::bytes::Bytes,
        _: &mut crate::OracleConnection,
    ) -> Result<Self, sqlx_core::Error> {
        let mut message = None;
        let error_num = buf.get_u16();
        let num_bytes = buf.get_u16();
        let _ = buf.get_u16(); // flags
        if error_num != 0 && num_bytes > 0 {
            message = buf.read_str(CS_FORM_IMPLICIT);
        }

        Ok(Warning { error_num, message })
    }
}
