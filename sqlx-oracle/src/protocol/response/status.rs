use sqlx_core::bytes::{Buf, Bytes};

use crate::protocol::response::Response;

pub struct Status {
    pub call_status: u32,
    pub end_to_end_seq_num: u16,
}

impl Response for Status {
    fn decode_body_with(
        mut buf: Bytes,
        _: &mut crate::OracleConnection,
    ) -> Result<Self, sqlx_core::Error> {
        let call_status = buf.get_u32();
        let end_to_end_seq_num = buf.get_u16();
        Ok(Status {
            call_status,
            end_to_end_seq_num,
        })
    }
}
