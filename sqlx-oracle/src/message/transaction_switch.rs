use crate::{
    message::{FrontendMessage, FrontendMessageFormat},
    OracleConnection,
};

pub(crate) struct TransactionSwitchMessage;

// for tcp_begin
// for tcp_end

impl FrontendMessage for TransactionSwitchMessage {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::TpcTxnSwitch;

    fn encode_body_with(
        &self,
        buf: &mut Vec<u8>,
        context: &mut OracleConnection,
    ) -> Result<(), sqlx_core::Error> {
        todo!()
    }
}
