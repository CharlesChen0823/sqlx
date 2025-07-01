use std::cmp::min;
use std::ops::{Deref, DerefMut};

use bytes::Bytes;
use sqlx_core::bytes::{self, Buf};

use crate::error::Error;
use crate::io::{ProtocolDecode, ProtocolEncode};
use byteorder::{BigEndian, ByteOrder};

use crate::constants::*;

#[derive(Debug)]
pub struct Packet<T> {
    pub packet_size: u32,
    pub packet_type: u8,
    pub packet_flags: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub body: T,
}

impl Packet<Bytes> {
    pub(crate) fn decode<'de, T>(self) -> Result<T, Error>
    where
        T: ProtocolDecode<'de, ()>,
    {
        self.decode_with(())
    }

    pub(crate) fn decode_with<'de, T, C>(self, context: C) -> Result<T, Error>
    where
        T: ProtocolDecode<'de, C>,
    {
        T::decode_with(self.body, context)
    }

    pub(crate) fn has_end_of_response(&self) -> bool {
        let flags = self.body.slice(0..2).get_u16(); // todo decode_u16be
        if flags & TNS_DATA_FLAGS_END_OF_RESPONSE != 0 || flags & TNS_DATA_FLAGS_EOF != 0 {
            return true;
        }
        let flags = self.body.slice(2..3).get_u8();
        if self.packet_size == PACKET_HEADER_SIZE + 3 && flags == TNS_MSG_TYPE_END_OF_RESPONSE {
            return true;
        }
        false
    }
}

impl Deref for Packet<Bytes> {
    type Target = Bytes;

    fn deref(&self) -> &Bytes {
        &self.body
    }
}

impl DerefMut for Packet<Bytes> {
    fn deref_mut(&mut self) -> &mut Bytes {
        &mut self.body
    }
}

const PACKET_HEADER_SIZE: u32 = 8;
