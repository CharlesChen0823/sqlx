use sqlx_core::{
    bytes::{Buf, Bytes},
    Error,
};

use crate::{constants::CS_FORM_IMPLICIT, io::OraBufExt, message::FrontendMessageFormat};

pub struct FetchParameter {
    keyword_num: u16,
    key_value: Option<String>,
    rowcounts: Vec<u64>,
}

impl super::Parameter<FrontendMessageFormat, u32> for FetchParameter {
    fn decode_body_with(mut buf: Bytes, arraydmlrowcounts: u32) -> Result<Self, Error> {
        let mut keyword_num = 0;
        let mut key_value = None;
        let mut rowcounts = vec![];
        let num_params = buf.get_u16(); // al8o4l (ignored)
        for _ in 0..num_params {
            let _ = buf.get_u8();
        }
        let num_params = buf.get_u16();
        if num_params > 0 {
            buf.skip_raw_bytes(num_params); // al8txl (ignored)
        }
        let num_params = buf.get_u16(); // num key/value pairs
        for _ in 0..num_params {
            let num_params = buf.get_u16(); // key
            if num_params > 0 {
                key_value = buf.read_str(CS_FORM_IMPLICIT);
            }
            let num_params = buf.get_u16(); // value
            if num_params > 0 {
                buf.skip_raw_bytes_chunked();
            }
            keyword_num = buf.get_u16();
        }
        let num_bytes = buf.get_u16();
        if num_bytes > 0 {
            buf.skip_raw_bytes(num_bytes);
        }
        if arraydmlrowcounts > 0 {
            let num_rows = buf.get_u32();
            for _ in 0..num_rows {
                rowcounts.push(buf.get_u64());
            }
        }
        Ok(FetchParameter {
            keyword_num,
            key_value,
            rowcounts,
        })
    }
}
