use byteorder::WriteBytesExt;

use crate::{
    io::OraBufMutExt,
    message::{FrontendMessage, FrontendMessageFormat},
    OracleConnection,
};

pub(crate) struct FetchMessage {
    pub(crate) cursor_id: u32,
    pub(crate) fetch_array_size: u32,
}

impl FrontendMessage for FetchMessage {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::Fetch;

    fn encode_body_with(
        &self,
        buf: &mut Vec<u8>,
        _conn: &mut OracleConnection,
    ) -> Result<(), sqlx_core::Error> {
        buf.write_u8(Self::FORMAT.into());
        buf.write_ub4(self.cursor_id);
        buf.write_ub4(self.fetch_array_size);
        Ok(())
    }
}
