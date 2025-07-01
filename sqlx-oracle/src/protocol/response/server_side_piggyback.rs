use sqlx_core::{
    bytes::{Buf, Bytes},
    Error,
};

use crate::{constants::*, io::OraBufExt, protocol::response::Response, OracleConnection};

pub struct ServerSidePiggyback {
    pub ltxid: Option<Bytes>,
    pub session_id: Option<u32>,
    pub serial_num: Option<u16>,
}

const TNS_SESSGET_SESSION_CHANGED: u8 = 4;

impl Response for ServerSidePiggyback {
    fn decode_body_with(mut buf: Bytes, conn: &mut OracleConnection) -> Result<Self, Error> {
        let op_code = buf.get_u8();
        let mut ltxid = None;
        let mut session_id = None;
        let mut serial_num = None;

        match op_code {
            TNS_SERVER_PIGGYBACK_LTXID => {
                ltxid = buf.read_bytes_with_length();
            }
            TNS_SERVER_PIGGYBACK_QUERY_CACHE_INVALIDATION | TNS_SERVER_PIGGYBACK_TRACE_EVENT => {}
            TNS_SERVER_PIGGYBACK_OS_PID_MTS => {
                let _temp = buf.get_u16();
                buf.skip_raw_bytes_chunked();
            }
            TNS_SERVER_PIGGYBACK_SYNC => {
                let _ = buf.get_u16(); // skip number of DTYs
                let _ = buf.get_u8(); // skip length of DTYs
                let num_elements = buf.get_u16();
                let _ = buf.get_u8(); // skip length
                for _ in 0..num_elements {
                    let temp = buf.get_u16();
                    if temp > 0 {
                        buf.skip_raw_bytes_chunked(); // skip key
                    }
                    let temp = buf.get_u16();
                    if temp > 0 {
                        buf.skip_raw_bytes_chunked(); // skip value
                    }
                    let _ = buf.get_u16(); // skip flags
                }
                let _ = buf.get_u32(); // skip overall flags
            }
            TNS_SERVER_PIGGYBACK_EXT_SYNC => {
                let _ = buf.get_u16(); // skip number of DTYs
                let _ = buf.get_u8(); // skip length of DTYs
            }
            TNS_SERVER_PIGGYBACK_AC_REPLAY_CONTEXT => {
                let _ = buf.get_u16(); // skip number of DTYs
                let _ = buf.get_u8(); // skip length of DTYs
                let _ = buf.get_u32(); // skip flags
                let _ = buf.get_u32(); // skip error code
                let _ = buf.get_u8(); // skip queue
                let num_bytes = buf.get_u32(); // skip replay context
                if num_bytes > 0 {
                    buf.skip_raw_bytes_chunked();
                }
            }
            TNS_SERVER_PIGGYBACK_SESS_RET => {
                let _ = buf.get_u16();
                let _ = buf.get_u8();
                let num_elements = buf.get_u16();
                if num_elements > 0 {
                    let _ = buf.get_u8();
                    for _ in 0..num_elements {
                        let temp = buf.get_u16();
                        if temp > 0 {
                            // skip key
                            buf.skip_raw_bytes_chunked();
                        }
                        let temp = buf.get_u16();
                        if temp > 0 {
                            // skip value
                            buf.skip_raw_bytes_chunked();
                        }
                        let _ = buf.get_u16(); // skip flags
                    }
                }
                let flags = buf.get_u32(); // session flags
                if flags & (TNS_SESSGET_SESSION_CHANGED as u32) != 0 {
                    if conn.inner.drcp_establish_session {
                        conn.clear_open_cursors();
                    }
                }
                conn.inner.drcp_establish_session = false;
                session_id = Some(buf.get_u32());
                serial_num = Some(buf.get_u16());
            }
            TNS_SERVER_PIGGYBACK_SESS_SIGNATURE => {
                let _ = buf.get_u16(); // number of DTYs
                let _ = buf.get_u8(); // length of DTYs
                let _ = buf.get_u64(); // signature flags
                let _ = buf.get_u64(); // client signature
                let _ = buf.get_u64(); // server signature
            }
            _ => {
                err_protocol!("Unsupported server_piggyback op {}", op_code);
            }
        }

        Ok(ServerSidePiggyback {
            ltxid,
            session_id,
            serial_num,
        })
    }
}
