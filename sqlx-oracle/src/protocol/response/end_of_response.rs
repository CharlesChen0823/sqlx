use crate::protocol::response::Response;

pub struct EndOfResponse {
    pub end_of_response: bool,
}

impl Response for EndOfResponse {
    fn decode_body_with(
        _buf: sqlx_core::bytes::Bytes,
        _: &mut crate::OracleConnection,
    ) -> Result<Self, sqlx_core::Error> {
        Ok(EndOfResponse {
            end_of_response: true,
        })
    }
}
