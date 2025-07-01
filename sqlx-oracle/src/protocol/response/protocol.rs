use sqlx_core::bytes::{Buf, Bytes};

use crate::{io::OraBufExt, protocol::response::Response};

pub struct ProtocolInfo {
    pub server_version: u8,
    pub server_banner: Bytes,
    pub charset_id: u16,
    pub server_flags: u8,
    pub ncharset_id: u16,
    pub server_compile_caps: Option<Bytes>,
    pub server_runtime_caps: Option<Bytes>,
}

impl Response for ProtocolInfo {
    fn decode_body_with(
        mut buf: sqlx_core::bytes::Bytes,
        _: &mut crate::OracleConnection,
    ) -> Result<Self, sqlx_core::Error> {
        let server_version = buf.get_u8();
        let _ = buf.get_u8(); // skip zero byte
        let server_banner = buf.read_null_terminated_bytes();
        let charset_id = buf.get_u16(); // todo little endian
        let server_flags = buf.get_u8();
        let num_elem = buf.get_u16(); // todo!() little endian
        if num_elem > 0 {
            buf.skip_raw_bytes(num_elem * 5);
        }
        let fdo_length = buf.get_u16(); // todo!() bigger endian
        let fdo = buf._read_raw_bytes_and_length(fdo_length as usize);
        let ix = (6 + fdo[5] + fdo[6]) as usize;
        let first = fdo[ix + 3] << 8;
        let second = fdo[ix + 4] as u16;
        let ncharset_id = first as u16 + second;
        let server_compile_caps = buf.read_bytes();
        let server_runtime_caps = buf.read_bytes();
        Ok(ProtocolInfo {
            server_version,
            server_banner,
            charset_id,
            server_flags,
            server_compile_caps,
            server_runtime_caps,
            ncharset_id,
        })
    }
}
