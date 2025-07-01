use crate::{
    message::{FrontendMessage, FrontendMessageFormat},
    OracleConnection,
};

// for tcp_commit
// for tcp_prepare
// for tcp_rollback

pub(crate) struct TransactionChangeStateMessage;

impl FrontendMessage for TransactionChangeStateMessage {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::TpcTxnChangeState;

    fn encode_body_with(
        &self,
        buf: &mut Vec<u8>,
        context: &mut OracleConnection,
    ) -> Result<(), sqlx_core::Error> {
        todo!()
    }
}
