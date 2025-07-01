mod aq_array;
mod aq_deq;
mod aq_enq;
mod auth;
mod commit;
mod connect;
mod data_types;
mod end_pipeline;
mod execute;
mod fast_auth;
mod fetch;
mod lob_data;
mod lob_op;
mod logoff;
mod ping;
mod protocol;
mod rollback;
mod session_release;
mod tpc_change_state;
mod tpc_switch;

pub use aq_array::AQArrayParameter;
pub use aq_deq::AQDeqParameter;
pub use aq_enq::AQEnqParameter;
pub use auth::AuthOneParameter;
pub use commit::CommitParameter;
pub use connect::ConnectParameter;
pub use end_pipeline::EndPipelineParameter;
pub use execute::ExecuteParameter;
pub use fast_auth::FastAuthParameter;
pub use fetch::FetchParameter;
pub use lob_op::LobOpParameter;
pub use logoff::LogoffParameter;
pub use ping::PingParameter;
pub use protocol::ProtocolParameter;
pub use rollback::RollbackParameter;
pub use tpc_change_state::TpcChangeStateParameter;
pub use tpc_switch::TpcSwitchParameter;

use sqlx_core::{bytes::Bytes, Error};

use crate::OracleConnection;

pub(crate) trait Parameter<T, C>: Sized {
    fn decode_body_with(bytes: Bytes, context: C) -> Result<Self, Error>;
}
