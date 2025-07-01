use byteorder::{BigEndian, WriteBytesExt};
use serde_json::Number;
use sqlx_core::{types::time::Date, Error};

use crate::{
    constants::TNS_LONG_LENGTH_INDICATOR,
    types::{OraIntervalDS, OraIntervalYM},
};
const TNS_MAX_SHORT_LENGTH: u8 = 255;
const TNS_CHUNK_SIZE: u32 = 32767;

pub trait OraBufMutExt {
    fn write_ub2(&mut self, value: u16) -> Result<(), Error>;
    fn write_ub4(&mut self, value: u32) -> Result<(), Error>;
    fn write_ub8(&mut self, value: u64) -> Result<(), Error>;
    fn write_sb4(&mut self, value: i32) -> Result<(), Error>;
    fn write_binary_double(&mut self, value: f64, write_length: bool) -> Result<(), Error>;
    fn write_binary_float(&mut self, value: f32, write_length: bool) -> Result<(), Error>;
    fn write_bool(&mut self, value: bool) -> Result<(), Error>;
    fn write_bytes(&mut self, value: &[u8]) -> Result<(), Error>;
    fn write_bytes_and_length(&mut self, value: &[u8]) -> Result<(), Error>;
    fn write_interval_ds(&mut self, value: &OraIntervalDS, write_length: bool)
        -> Result<(), Error>;
    fn write_interval_ym(&mut self, value: &OraIntervalYM, write_length: bool)
        -> Result<(), Error>;
    fn write_date(&mut self, value: &Date, write_length: bool) -> Result<(), Error>;
    fn write_number(&mut self, value: &Number) -> Result<(), Error>;
    fn write_raw(&mut self, value: &[u8]) -> Result<(), Error>;
    fn write_string(&mut self, value: &str) -> Result<(), Error>;
}

impl OraBufMutExt for Vec<u8> {
    fn write_binary_double(&mut self, value: f64, write_length: bool) -> Result<(), Error> {
        let v = value.to_le_bytes();
        let mut b7 = v[0] & 0xff;
        let mut b6 = v[1] & 0xff;
        let mut b5 = v[2] & 0xff;
        let mut b4 = v[3] & 0xff;
        let mut b3 = v[4] & 0xff;
        let mut b2 = v[5] & 0xff;
        let mut b1 = v[6] & 0xff;
        let mut b0 = v[7] & 0xff;
        if b0 & 0x80 == 0 {
            b0 = b0 | 0x80;
        } else {
            b0 = !b0;
            b1 = !b1;
            b2 = !b2;
            b3 = !b3;
            b4 = !b4;
            b5 = !b5;
            b6 = !b6;
            b7 = !b7;
        }
        if write_length {
            self.push(8);
        }
        self.push(b0);
        self.push(b1);
        self.push(b2);
        self.push(b3);
        self.push(b4);
        self.push(b5);
        self.push(b6);
        self.push(b7);
        Ok(())
    }

    fn write_binary_float(&mut self, value: f32, write_length: bool) -> Result<(), Error> {
        let v = value.to_le_bytes();
        let mut b3 = v[0] & 0xff;
        let mut b2 = v[1] & 0xff;
        let mut b1 = v[2] & 0xff;
        let mut b0 = v[3] & 0xff;
        if b0 & 0x80 == 0 {
            b0 = b0 | 0x80;
        } else {
            b0 = !b0;
            b1 = !b1;
            b2 = !b2;
            b3 = !b3;
        }
        if write_length {
            self.push(4);
        }
        self.push(b0);
        self.push(b1);
        self.push(b2);
        self.push(b3);
        Ok(())
    }

    fn write_bool(&mut self, value: bool) -> Result<(), Error> {
        if value {
            self.push(2);
            self.write_u16::<BigEndian>(0x0101);
        } else {
            self.write_u16::<BigEndian>(0x0100);
        }
        Ok(())
    }

    fn write_bytes(&mut self, value: &[u8]) -> Result<(), Error> {
        self.extend(value);
        Ok(())
    }

    fn write_bytes_and_length(&mut self, value: &[u8]) -> Result<(), Error> {
        let len = value.len();
        if len <= TNS_MAX_SHORT_LENGTH as usize {
            self.push(len as u8);
            if len > 0 {
                self.extend(value);
            }
        } else {
            let mut length = len;
            self.push(TNS_LONG_LENGTH_INDICATOR);
            // todo!()
            // while length > 0 {
            //     let chunk_len = std::cmp::min(length, TNS_CHUNK_SIZE);
            //     self.write_ub4(chunk_len as u32);
            //     length -= chunk_len;
            //     self.write_raw(value[..chunk_len])
            // }
            self.write_ub4(0 as u32);
        }
        Ok(())
    }

    fn write_interval_ds(
        &mut self,
        value: &OraIntervalDS,
        write_length: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn write_interval_ym(
        &mut self,
        value: &OraIntervalYM,
        write_length: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn write_date(&mut self, value: &Date, write_length: bool) -> Result<(), Error> {
        Ok(())
    }

    fn write_number(&mut self, value: &Number) -> Result<(), Error> {
        Ok(())
    }

    fn write_raw(&mut self, value: &[u8]) -> Result<(), Error> {
        self.extend(value);
        Ok(())
    }

    fn write_string(&mut self, value: &str) -> Result<(), Error> {
        let v = value.as_bytes();
        self.extend(v);
        Ok(())
    }

    fn write_ub2(&mut self, value: u16) -> Result<(), Error> {
        if value == 0 {
            self.push(0 as u8);
        } else if value as u8 <= u8::MAX {
            self.push(1);
            self.push(value as u8);
        } else {
            self.push(2);
            self.write_u16::<BigEndian>(value)?;
        }
        Ok(())
    }

    fn write_ub4(&mut self, value: u32) -> Result<(), Error> {
        if value == 0 {
            self.push(0 as u8);
        } else if value as u8 <= u8::MAX {
            self.push(1);
            self.push(value as u8);
        } else if value as u16 <= u16::MAX {
            self.push(2);
            self.write_u16::<BigEndian>(value as u16)?;
        } else {
            self.push(4);
            self.write_u32::<BigEndian>(value)?;
        }
        Ok(())
    }

    fn write_ub8(&mut self, value: u64) -> Result<(), Error> {
        if value == 0 {
            self.push(0 as u8);
        } else if value as u8 <= u8::MAX {
            self.push(1);
            self.push(value as u8);
        } else if value as u16 <= u16::MAX {
            self.push(2);
            self.write_u16::<BigEndian>(value as u16)?;
        } else if value as u32 <= u32::MAX {
            self.push(4);
            self.write_u32::<BigEndian>(value as u32)?;
        } else {
            self.push(8);
            self.write_u64::<BigEndian>(value)?;
        }
        Ok(())
    }

    fn write_sb4(&mut self, value: i32) -> Result<(), Error> {
        let mut sign = 0;
        let mut value = value;
        if value < 0 {
            sign = 0x80;
            value = -value;
        }
        if value == 0 {
            self.push(0);
        } else if value as u8 <= u8::MAX {
            self.push(1 | sign);
            self.push(value as u8);
        } else if value as u16 <= u16::MAX {
            self.push(2 | sign);
            self.write_u16::<BigEndian>(value as u16)?;
        } else {
            self.push(4 | sign);
            self.write_u32::<BigEndian>(value as u32)?;
        }
        Ok(())
    }
}
