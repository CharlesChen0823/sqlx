use crate::{
    message::{FrontendMessage, FrontendMessageFormat},
    OracleConnection,
};

pub(crate) struct PingMessage {}

impl FrontendMessage for PingMessage {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::Ping;

    fn encode_body_with(
        &self,
        buf: &mut Vec<u8>,
        _context: &mut OracleConnection,
    ) -> Result<(), sqlx_core::Error> {
        todo!()
    }
}
