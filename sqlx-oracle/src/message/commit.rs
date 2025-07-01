use sqlx_core::Error;

use crate::{
    message::{FrontendMessage, FrontendMessageFormat},
    OracleConnection,
};

pub(crate) struct CommitMessage {}

impl FrontendMessage for CommitMessage {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::Commit;

    fn encode_body_with(
        &self,
        buf: &mut Vec<u8>,
        _conn: &mut OracleConnection,
    ) -> Result<(), Error> {
        todo!()
    }
}
