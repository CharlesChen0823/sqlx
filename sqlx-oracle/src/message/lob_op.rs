use crate::{
    message::{FrontendMessage, FrontendMessageFormat},
    OracleConnection,
};

pub(crate) struct LobOpMessage {}

impl FrontendMessage for LobOpMessage {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::LobOp;

    fn encode_body_with(
        &self,
        buf: &mut Vec<u8>,
        _conn: &mut OracleConnection,
    ) -> Result<(), sqlx_core::Error> {
        todo!()
    }
}
