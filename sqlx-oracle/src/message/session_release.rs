use crate::{
    message::{FrontendMessage, FrontendMessageFormat},
    OracleConnection,
};

pub(crate) struct SessionReleaseMessage {}

impl FrontendMessage for SessionReleaseMessage {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::SessionRelease;

    fn encode_body_with(
        &self,
        buf: &mut Vec<u8>,
        context: &mut OracleConnection,
    ) -> Result<(), sqlx_core::Error> {
        todo!()
    }
}
