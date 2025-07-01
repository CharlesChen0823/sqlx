use crate::{constants::CS_FORM_IMPLICIT, error::Error, io::RowId};
use sqlx_core::bytes::{Buf, Bytes};

const TNS_LONG_LENGTH_INDICATOR: u8 = 254;
const TNS_NULL_LENGTH_INDICATOR: u8 = 255;

pub trait OraBufExt: Buf {
    fn _get_int_length_and_sign(
        &mut self,
        max_length: u8,
        is_negative: Option<bool>,
    ) -> Result<u8, Error>;
    fn _read_raw_bytes_and_length(&mut self, num_bytes: usize) -> Bytes;
    fn skip_int(&mut self, max_length: u8, is_negative: Option<bool>) -> Result<(), Error>;
    fn read_oracle_data(&mut self) -> Option<Bytes>;
    fn read_bytes(&mut self) -> Option<Bytes>;
    fn read_bytes_with_length(&mut self) -> Option<Bytes>;
    fn read_raw_bytes_and_length(&mut self) -> Option<Bytes>;
    fn read_null_terminated_bytes(&mut self) -> Bytes;
    fn read_str(&mut self, csfrm: u8) -> Option<String>;
    fn read_str_with_length(&mut self) -> Option<String>;
    fn skip_raw_bytes(&mut self, length: u16);
    fn skip_raw_bytes_chunked(&mut self);
    fn read_rowid(&mut self) -> RowId;
    fn read_urowid(&mut self) -> Option<Bytes>;
}

impl OraBufExt for Bytes {
    fn _get_int_length_and_sign(
        &mut self,
        max_length: u8,
        mut is_negative: Option<bool>,
    ) -> Result<u8, Error> {
        let length = self.get_u8();
        if length & 0x80 != 0 {
            if is_negative.is_none() {
                return Err(err_protocol!("Missing sign indicator"));
            }
            is_negative = Some(true);
            return Ok(length & 0x7f);
        } else {
            if !is_negative.is_none() {
                is_negative = Some(false);
            }
            if length > max_length {
                return Err(err_protocol!("Length exceeds maximum"));
            }
            return Ok(length);
        }
    }

    fn _read_raw_bytes_and_length(&mut self, num_bytes: usize) -> Bytes {
        let v = self.slice(..num_bytes);
        self.advance(num_bytes);
        v
    }

    fn skip_int(&mut self, max_length: u8, is_negative: Option<bool>) -> Result<(), Error> {
        let length = self._get_int_length_and_sign(max_length, is_negative)?;
        self.advance(length as usize);
        Ok(())
    }

    fn read_oracle_data(&mut self) -> Option<Bytes> {
        None
    }

    fn read_bytes(&mut self) -> Option<Bytes> {
        return self.read_raw_bytes_and_length();
    }

    fn skip_raw_bytes_chunked(&mut self) {
        let length = self.get_u8();
        if length != TNS_LONG_LENGTH_INDICATOR {
            self.advance(length as usize);
        } else {
            loop {
                let num_bytes = self.get_u32();
                if num_bytes == 0 {
                    break;
                }
                self.advance(num_bytes as usize);
            }
        }
    }

    fn read_str(&mut self, csfrm: u8) -> Option<String> {
        let v = self.read_raw_bytes_and_length();
        match v {
            Some(v) => {
                if csfrm == CS_FORM_IMPLICIT {
                    let r = String::from_utf8(v.into());
                    r.ok()
                } else {
                    let iter = (0..v.len()).map(|i| u16::from_be_bytes([v[2 * i], v[2 * i + 1]]));
                    let v = std::char::decode_utf16(iter).collect::<Result<String, _>>();
                    v.ok()
                }
            }
            None => None,
        }
    }

    fn read_str_with_length(&mut self) -> Option<String> {
        let length = self.get_u32();
        if length > 0 {
            self.read_str(CS_FORM_IMPLICIT)
        } else {
            None
        }
    }

    fn read_rowid(&mut self) -> RowId {
        let rba = self.get_u32();
        let partition_id = self.get_u16();
        let _ = self.advance(1);
        let block_num = self.get_u32();
        let slot_num = self.get_u16();
        RowId {
            rba,
            partition_id,
            block_num,
            slot_num,
        }
    }

    fn read_urowid(&mut self) -> Option<Bytes> {
        None
    }

    fn read_null_terminated_bytes(&mut self) -> Bytes {
        let mut v = Vec::new();
        loop {
            let b = self.get_u8();
            if b == b'\0' {
                v.push(b);
                break;
            }
            v.push(b);
        }
        v.into()
    }

    fn skip_raw_bytes(&mut self, length: u16) {
        let mut length = length;
        while length > 0 {
            let _ = self.advance(1);
            length -= 1;
        }
    }

    fn read_raw_bytes_and_length(&mut self) -> Option<Bytes> {
        let length = self.get_u8();
        if length == 0 || length == TNS_NULL_LENGTH_INDICATOR {
            return None;
        } else {
            Some(self._read_raw_bytes_and_length(length as usize))
        }
    }

    fn read_bytes_with_length(&mut self) -> Option<Bytes> {
        let length = self.get_u32();
        if length > 0 {
            self.read_bytes()
        } else {
            None
        }
    }
}
