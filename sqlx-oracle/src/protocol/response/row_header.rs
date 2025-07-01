use sqlx_core::bytes::Buf;

use crate::{io::OraBufExt, protocol::response::Response};

pub struct RowHeader {
    pub bit_vector: Vec<u8>,
}

impl Response for RowHeader {
    fn decode_body_with(
        mut buf: sqlx_core::bytes::Bytes,
        _: &mut crate::OracleConnection,
    ) -> Result<Self, sqlx_core::Error> {
        let _ = buf.get_u8(); // flags
        let _ = buf.get_u16(); // num requests
        let _ = buf.get_u32(); // iteration number
        let _ = buf.get_u32(); // number iters
        let _ = buf.get_u16(); // buffer length
        let num_bytes = buf.get_u32();
        let mut bit_vector = Vec::new();
        if num_bytes > 0 {
            let _ = buf.get_u8(); // skip repeated length
            bit_vector = buf._read_raw_bytes_and_length(num_bytes as usize).to_vec();
        }
        let num_bytes = buf.get_u32();
        if num_bytes > 0 {
            buf.skip_raw_bytes_chunked(); // rxhrid
        }

        Ok(RowHeader { bit_vector })
    }
}
