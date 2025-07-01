use crate::protocol::response::Response;

pub struct FlushOutBinds {
    pub flush_out_binds: bool,
    pub end_of_response: bool,
}

impl Response for FlushOutBinds {
    fn decode_body_with(
        _buf: sqlx_core::bytes::Bytes,
        _: &mut crate::OracleConnection,
    ) -> Result<Self, sqlx_core::Error> {
        Ok(FlushOutBinds {
            flush_out_binds: true,
            end_of_response: true,
        })
    }
}
