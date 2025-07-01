use crate::{
    message::{FrontendMessage, FrontendMessageFormat},
    OracleConnection,
};

pub(crate) struct EndPipelineMessage {}

impl FrontendMessage for EndPipelineMessage {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::PipelineEnd;

    fn encode_body_with(
        &self,
        buf: &mut Vec<u8>,
        _conn: &mut OracleConnection,
    ) -> Result<(), sqlx_core::Error> {
        todo!()
    }
}
