use sqlx_core::{
    bytes::{Buf, Bytes},
    Error, HashMap,
};

use crate::{io::OraBufExt, message::FrontendMessageFormat};

pub struct AuthOneParameter {
    verifier_type: u32,
    session_data: HashMap<String, String>,
}

impl super::Parameter<FrontendMessageFormat, ()> for AuthOneParameter {
    fn decode_body_with(mut buf: Bytes, _: ()) -> Result<Self, Error> {
        // Implementation goes here
        let num_params = buf.get_u16();
        let mut verifier_type = 0;
        let mut session_data: HashMap<String, String> = HashMap::default();
        for _ in 0..num_params {
            let key = buf.read_str_with_length();
            let value = buf.read_str_with_length().unwrap_or_default();
            if key.is_none() {
                continue;
            }
            let key = key.unwrap();
            if key.eq("AUTH_VFR_DATA") {
                verifier_type = buf.get_u32();
            } else {
                let _ = buf.get_u32();
            }
            session_data.insert(key, value);
        }

        Ok(AuthOneParameter {
            verifier_type,
            session_data,
        })
    }
}

pub struct AuthTwoParameter {
    verifier_type: u32,
    session_data: HashMap<String, String>,
}

impl super::Parameter<FrontendMessageFormat, ()> for AuthTwoParameter {
    fn decode_body_with(mut buf: Bytes, _: ()) -> Result<Self, Error> {
        // Implementation goes here
        let num_params = buf.get_u16();
        let mut verifier_type = 0;
        let mut session_data: HashMap<String, String> = HashMap::default();
        for _ in 0..num_params {
            let key = buf.read_str_with_length();
            let value = buf.read_str_with_length().unwrap_or_default();
            if key.is_none() {
                continue;
            }
            let key = key.unwrap();
            if key.eq("AUTH_VFR_DATA") {
                verifier_type = buf.get_u32();
            } else {
                let _ = buf.get_u32();
            }
            session_data.insert(key, value);
        }

        Ok(AuthTwoParameter {
            verifier_type,
            session_data,
        })
    }
}
