use sqlx_core::{
    bytes::{Buf, Bytes},
    Error,
};

use crate::{protocol::response::Response, OracleConnection};

pub struct Token {
    pub token_num: u64,
}

impl Response for Token {
    fn decode_body_with(mut buf: Bytes, _: &mut OracleConnection) -> Result<Self, Error> {
        let token_num = buf.get_u64();
        Ok(Token { token_num })
    }
}
