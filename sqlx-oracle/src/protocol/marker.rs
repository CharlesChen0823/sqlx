use byteorder::{BigEndian, WriteBytesExt};
use sqlx_core::io::ProtocolEncode;

use crate::{
    constants::{TNS_PACKET_TYPE_MARKER, TNS_VERSION_MIN_LARGE_SDU},
    protocol::{Capabilities, PACKET_HEADER_SIZE},
};

pub struct Marker {
    pub r#type: u8,
}

impl ProtocolEncode<'_, Capabilities> for Marker {
    fn encode_with(&self, buf: &mut Vec<u8>, caps: Capabilities) -> Result<(), sqlx_core::Error> {
        let packet_type = TNS_PACKET_TYPE_MARKER;
        let packet_flags = 0;
        let size = PACKET_HEADER_SIZE;
        if caps.protocol_version >= TNS_VERSION_MIN_LARGE_SDU {
            buf.write_u32::<BigEndian>(size as u32)?;
        } else {
            buf.write_u16::<BigEndian>(size as u16)?;
            buf.write_u16::<BigEndian>(0)?;
        }
        buf.write_u8(packet_type);
        buf.write_u8(packet_flags);
        buf.write_u16::<BigEndian>(0)?;
        buf.write_u8(1);
        buf.write_u8(0);
        buf.write_u8(self.r#type);
        Ok(())
    }
}
