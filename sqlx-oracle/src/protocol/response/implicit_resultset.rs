use sqlx_core::bytes::Buf;

use crate::{io::OraBufExt, protocol::response::Response};

pub struct ImplicitResultSet {}

impl Response for ImplicitResultSet {
    fn decode_body_with(
        mut buf: sqlx_core::bytes::Bytes,
        _: &mut crate::OracleConnection,
    ) -> Result<Self, sqlx_core::Error> {
        let num_results = buf.get_u32();
        for _ in 0..num_results {
            let num_bytes = buf.get_u8();
            buf.skip_raw_bytes(num_bytes as u16);
            // todo!() create cursor from describle
            //
            let _ = buf.get_u32(); // max row size
            let num_columns = buf.get_u32();
            if num_columns > 0 {
                let _ = buf.get_u8();
            }
            for _ in 0..num_columns {
                // todo!() process_metadata
            }

            let num_bytes = buf.get_u32();
            if num_bytes > 0 {
                buf.skip_raw_bytes_chunked(); // current date
            }

            let _ = buf.get_u32(); // dcbflag
            let _ = buf.get_u32(); // dcbmdbz
            let _ = buf.get_u32(); // dcbmnpr
            let _ = buf.get_u32(); // dcbmxpr
            let num_bytes = buf.get_u32();
            if num_bytes > 0 {
                buf.skip_raw_bytes_chunked(); // dcbqcky
            }

            let _cursor_id = buf.get_u16();
        }
        Ok(ImplicitResultSet {})
    }
}
