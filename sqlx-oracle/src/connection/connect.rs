use crate::error::Error;

use crate::constants::*;
use crate::io::OraBufMutExt;
use crate::protocol::Capabilities;
use crate::protocol::PACKET_HEADER_SIZE;

use byteorder::BigEndian;
use byteorder::WriteBytesExt;
use sqlx_core::bytes::Buf;
use sqlx_core::bytes::Bytes;
use sqlx_core::io::ProtocolDecode;
use sqlx_core::io::ProtocolEncode;

pub struct ConnectMessage<'a> {
    pub packet_flags: u8,
    pub connect_string: &'a str,
}

pub struct ConnectMessage2<'a> {
    pub packet_flags: u8,
    pub connect_string: &'a str,
}

pub struct ConnectResponse {}

impl ProtocolEncode<'_, &Capabilities> for ConnectMessage<'_> {
    fn encode_with(&self, buf: &mut Vec<u8>, caps: &Capabilities) -> Result<(), Error> {
        let mut service_options: u16 = TNS_GSO_DONT_CARE;
        let connect_flags_1 = 0;
        let mut connect_flags_2 = 0;
        let connect_string_len = self.connect_string.as_bytes().len();
        let nsi_flags = TNS_NSI_SUPPORT_SECURITY_RENEG | TNS_NSI_DISABLE_NA;
        if caps.supports_oob {
            service_options |= TNS_GSO_CAN_RECV_ATTENTION;
            connect_flags_2 |= TNS_CHECK_OOB;
        }
        // start_request
        let packet_type = TNS_PACKET_TYPE_CONNECT;
        let size = PACKET_HEADER_SIZE;
        if caps.protocol_version >= TNS_VERSION_MIN_LARGE_SDU {
            buf.write_u32::<BigEndian>(size as u32);
        } else {
            buf.write_u16::<BigEndian>(size as u16);
            buf.write_u16::<BigEndian>(0);
        }
        buf.write_u8(packet_type);
        buf.write_u8(self.packet_flags);
        buf.write_u16::<BigEndian>(0);
        //
        buf.write_u16::<BigEndian>(TNS_VERSION_DESIRED);
        buf.write_u16::<BigEndian>(TNS_VERSION_MINIMUM);
        buf.write_u16::<BigEndian>(service_options);
        buf.write_u16::<BigEndian>(caps.sdu);
        buf.write_u16::<BigEndian>(caps.sdu);
        buf.write_u16::<BigEndian>(TNS_PROTOCOL_CHARACTERISTICS);
        buf.write_u16::<BigEndian>(0); // line turnaround
        buf.write_u16::<BigEndian>(1); // value of 1
        buf.write_u16::<BigEndian>(connect_string_len as u16);
        buf.write_u16::<BigEndian>(74); // offset to connect data
        buf.write_u16::<BigEndian>(0); // max receivable data
        buf.write_u8(nsi_flags as u8);
        buf.write_u8(nsi_flags as u8);
        buf.write_u64::<BigEndian>(0); // obsolete
        buf.write_u64::<BigEndian>(0); // obsolete
        buf.write_u64::<BigEndian>(0); // obsolete
        buf.write_u32::<BigEndian>(caps.sdu as u32);
        buf.write_u32::<BigEndian>(caps.sdu as u32);
        buf.write_u32::<BigEndian>(connect_flags_1);
        buf.write_u32::<BigEndian>(connect_flags_2 as u32);
        if connect_string_len > TNS_MAX_CONNECT_DATA as usize {
            return Ok(());
        }
        buf.write_bytes(self.connect_string.as_bytes());

        Ok(())
    }
}

impl ProtocolEncode<'_, &Capabilities> for ConnectMessage2<'_> {
    fn encode_with(&self, buf: &mut Vec<u8>, caps: &Capabilities) -> Result<(), Error> {
        // start_request
        let packet_type = TNS_PACKET_TYPE_DATA;
        let size = PACKET_HEADER_SIZE + std::mem::size_of::<u16>();
        if caps.protocol_version >= TNS_VERSION_MIN_LARGE_SDU {
            buf.write_u32::<BigEndian>(size as u32);
        } else {
            buf.write_u16::<BigEndian>(size as u16);
            buf.write_u16::<BigEndian>(0);
        }
        buf.write_u8(packet_type);
        buf.write_u8(self.packet_flags);
        buf.write_u16::<BigEndian>(0);
        buf.write_u16::<BigEndian>(0);
        //
        buf.write_bytes(self.connect_string.as_bytes());

        Ok(())
    }
}

impl ProtocolDecode<'_, ()> for ConnectResponse {
    fn decode_with(mut buf: Bytes, _: ()) -> Result<Self, Error> {
        let status = buf.get_u16();

        Ok(ConnectResponse {})
    }
}
