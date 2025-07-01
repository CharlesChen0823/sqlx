use crate::{
    message::{FrontendMessage, FrontendMessageFormat},
    OracleConnection,
};

pub(crate) struct RollbackMessage {}

impl FrontendMessage for RollbackMessage {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::Rollback;

    fn encode_body_with(
        &self,
        buf: &mut Vec<u8>,
        _conn: &mut OracleConnection,
    ) -> Result<(), sqlx_core::Error> {
        todo!()
    }
}
