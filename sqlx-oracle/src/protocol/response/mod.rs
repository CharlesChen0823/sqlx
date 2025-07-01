use sqlx_core::{bytes::Bytes, Error};

use crate::OracleConnection;

mod bit_vector;
mod describe_info;
mod end_of_response;
mod error;
mod flush_out_binds;
mod implicit_resultset;
mod io_vector;
mod ora_error;
mod parameter;
mod protocol;
mod row_data;
mod row_header;
mod server_side_piggyback;
mod status;
mod token;
mod warning;

pub trait Response: Sized {
    fn decode_body_with(buf: Bytes, _: &mut OracleConnection) -> Result<Self, Error>;
}
