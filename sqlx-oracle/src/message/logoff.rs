use crate::{
    message::{FrontendMessage, FrontendMessageFormat},
    OracleConnection,
};

pub(crate) struct LogoffMessage {}

impl FrontendMessage for LogoffMessage {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::Logoff;

    fn encode_body_with(
        &self,
        buf: &mut Vec<u8>,
        _conn: &mut OracleConnection,
    ) -> Result<(), sqlx_core::Error> {
        todo!()
    }
}
