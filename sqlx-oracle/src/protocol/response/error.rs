use crate::error::Error;
use crate::io::{OraBufExt, RowId};
use crate::protocol::response::ora_error::{OraError, ERR_TOO_MANY_BATCH_ERRORS};
use crate::protocol::response::Response;
use crate::{constants::*, OracleConnection};

use sqlx_core::{
    bytes::{Buf, Bytes},
    io::BufExt,
};

#[derive(Debug)]
pub struct OraErrorInfo {
    pub call_status: u32,
    pub num: u32,
    pub cursor_id: u16,
    pub pos: u64,
    pub rowcount: u64,
    pub message: Option<String>,
    pub rowid: RowId,
    pub batcherrors: Vec<OraError>,
    pub warnings: Option<OraError>,
}

impl Response for OraErrorInfo {
    fn decode_body_with(mut buf: Bytes, conn: &mut OracleConnection) -> Result<Self, Error> {
        let mut warnings = None;
        let call_status = buf.get_u32(); // end of call status
        let _ = buf.get_u16(); // end to end seq#
        let _ = buf.get_u32(); // current row number
        let _ = buf.get_u16(); // error number
        let _ = buf.get_u16(); // array elem error
        let _ = buf.get_u16(); // array elem error
        let cursor_id = buf.get_u16(); // cursor id
        let error_pos = buf.get_i16(); // error position
        let _ = buf.get_u8(); // sql type
        let _ = buf.get_u8(); // fatal
        let _ = buf.get_u8(); // flags
        let _ = buf.get_u8(); // user cursor options
        let _ = buf.get_u8(); // UPI parameter
        let flag = buf.get_u8();
        if (flag & 0x20) > 0 {
            warnings = Some(OraError::default().warning());
        }
        let rowid = buf.read_rowid(); // rowid
        let _ = buf.get_u32(); // OS error
        let _ = buf.get_u8(); // statement number
        let _ = buf.get_u8(); // call number
        let _ = buf.get_u16(); // padding
        let _ = buf.get_u32(); // success iters
        let num_bytes = buf.get_u32(); // oerrdd (logical rowid)
        if num_bytes > 0 {
            buf.skip_raw_bytes_chunked();
        }
        // batch error codes
        let num_erros = buf.get_u16(); // batch error codes array
        let mut batcherrors: Vec<OraError> = Vec::with_capacity(num_erros as usize);
        if num_erros > 0 {
            let first_byte = buf.get_u8();
            for _ in 0..num_erros {
                if first_byte == TNS_LONG_LENGTH_INDICATOR {
                    let _ = buf.get_u32(); // chunk length ignored
                }
                let error_code = buf.get_u16();
                let ora_error = OraError::default().with_error_code(error_code);
                batcherrors.push(ora_error);
            }
            if first_byte == TNS_LONG_LENGTH_INDICATOR {
                let _ = buf.get_bytes(1); // ignore end marker
            }
        }
        // batch error offsets
        let num_offsets = buf.get_u32(); // batch error row offset array
        if num_offsets > 0 {
            if num_offsets > 65535 {
                let error = OraError::default().with_error_code(ERR_TOO_MANY_BATCH_ERRORS);
                return Err(err_protocol!("found {}", error));
            }
            let first_byte = buf.get_u8();
            for i in 0..num_offsets {
                if first_byte == TNS_LONG_LENGTH_INDICATOR {
                    let _ = buf.get_u32(); // chunk length ignored
                }
                let offset = buf.get_u32();
                if (i as u16) < num_erros {
                    batcherrors[i as usize].offset = offset;
                }
            }
            if first_byte == TNS_LONG_LENGTH_INDICATOR {
                let _ = buf.get_bytes(1); // ignore end marker
            }
        }

        // batch error message
        let temp = buf.get_u16(); // batch error messages array
        if temp > 0 {
            let _ = buf.get_bytes(1); // ignore packed size
            for i in 0..temp {
                let _ = buf.get_u16(); // skip chunk length
                let message = buf.read_str(CS_FORM_IMPLICIT).unwrap_or_default();
                batcherrors[i as usize].message = message;
                let _ = buf.get_bytes(2); // ignore end marker
            }
        }

        let info_num = buf.get_u32(); // error number (extended)
        let rowcount = buf.get_u64(); // row number (extened)

        // fields added in Oracle Database 20c
        if conn.inner.caps.ttc_field_version >= TNS_CCAP_FIELD_VERSION_20_1 {
            let _ = buf.get_u32(); // sql type
            let _ = buf.get_u32(); // server checksum
        }

        // error message
        let mut pos = 0;
        let mut message = None;
        if info_num != 0 {
            if error_pos > 0 {
                pos = error_pos;
            }
            message = buf.read_str(CS_FORM_IMPLICIT);
        }

        Ok(Self {
            call_status,
            num: info_num,
            cursor_id,
            rowid,
            pos: pos as u64,
            rowcount,
            message,
            batcherrors,
            warnings,
        })
    }
}
