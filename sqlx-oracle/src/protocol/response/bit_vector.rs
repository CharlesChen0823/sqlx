use std::ops::Div;

use sqlx_core::bytes::Buf;

use crate::{io::OraBufExt, protocol::response::Response};

pub struct BitVector {
    pub num_columns_sent: u16,
    pub bit_vector: Vec<u8>,
}

impl Response for BitVector {
    fn decode_body_with(
        mut buf: sqlx_core::bytes::Bytes,
        conn: &mut crate::OracleConnection,
    ) -> Result<Self, sqlx_core::Error> {
        let num_columns_sent = buf.get_u16();
        let mut num_bytes = conn.num_columns().saturating_div(8);
        if conn.num_columns().div(8) > 0 {
            num_bytes += 1;
        }
        let bit_vector = buf._read_raw_bytes_and_length(num_bytes as usize);
        Ok(BitVector {
            num_columns_sent,
            bit_vector: bit_vector.to_vec(),
        })
    }
}
