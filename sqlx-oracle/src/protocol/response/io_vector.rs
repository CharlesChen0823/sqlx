use sqlx_core::bytes::Buf;

use crate::{constants::TNS_BIND_DIR_INPUT, io::OraBufExt, protocol::response::Response};

pub struct IoVector {
    out_var_imples: Vec<u8>,
}

impl Response for IoVector {
    fn decode_body_with(
        mut buf: sqlx_core::bytes::Bytes,
        _: &mut crate::OracleConnection,
    ) -> Result<Self, sqlx_core::Error> {
        let _ = buf.get_u8(); // flags
        let temp16 = buf.get_u16(); // num requests
        let temp32 = buf.get_u32(); // num iter
        let num_binds = temp32 * 256 + (temp16 as u32);
        let _ = buf.get_u32(); // num iters this time
        let _ = buf.get_u16(); // uac buffer length
        let num_bytes = buf.get_u16(); // bit vector for fast fetch
        if num_bytes > 0 {
            buf.skip_raw_bytes(num_bytes);
        }
        let num_bytes = buf.get_u16(); // rowid
        if num_bytes > 0 {
            buf.skip_raw_bytes(num_bytes);
        }
        let mut out_var_imples = Vec::new();
        for _ in 0..num_binds {
            let bind_dir = buf.get_u8();
            if bind_dir == TNS_BIND_DIR_INPUT {
                continue;
            }
            out_var_imples.push(bind_dir); // todo!()
        }
        Ok(IoVector { out_var_imples })
    }
}
