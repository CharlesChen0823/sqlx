use sqlx_core::bytes::Bytes;

use crate::{error::Error, OracleConnection};

mod aq_array;
mod aq_deq;
mod aq_enq;
mod auth;
mod commit;
mod data_types;
mod end_pipeline;
mod execute;
mod fast_auth;
mod fetch;
mod flush_out_binds;
mod lob_op;
mod logoff;
mod ping;
mod protocol;
mod rollback;
mod session_release;
mod transaction_change_state;
mod transaction_switch;

#[derive(Debug, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum TNSPacketType {
    Connect = 1,
    Accept = 2,
    Refuse = 4,
    Redirect = 5,
    Data = 6,
    Resend = 11,
    Marker = 12,
    Control = 14,
}

/// Map to TNS_FUNC (TTC Functions)
#[derive(Debug, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum FrontendMessageFormat {
    AuthPhaseOne = 118,
    AuthPhaseTwo = 115,
    CloseCursors = 105,
    Commit = 14,
    Execute = 94,
    Fetch = 5,
    LobOp = 96,
    AqEnq = 121,
    AqDeq = 122,
    ArrayAq = 145,
    Logoff = 9,
    Ping = 147,
    PipelineBegin = 199,
    PipelineEnd = 200,
    Rollback = 15,
    SetEndToEndAttr = 135,
    Reexecute = 4,
    ReexecuteAndFetch = 78,
    SessionGet = 162,
    SessionRelease = 163,
    SessionState = 176,
    SetSchema = 152,
    TpcTxnSwitch = 103,
    TpcTxnChangeState = 104,
}

impl Into<u8> for FrontendMessageFormat {
    fn into(self) -> u8 {
        self as u8
    }
}

/// map to TNS_MSG_TYPE
#[derive(Debug, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum BackendMessageFormat {
    Protocol = 1,
    DataTypes = 2,
    Function = 3,
    Error = 4,
    RowHeader = 6,
    RowData = 7,
    Parameter = 8,
    Status = 9,
    IoVector = 11,
    LobData = 14,
    Warning = 15,
    DescribeInfo = 16,
    Piggyback = 17,
    FlushOutBinds = 19,
    BitVector = 21,
    ServerSidePiggyback = 23,
    OnewayFn = 26,
    ImplicitResultset = 27,
    Renegotiate = 28,
    EndOfResponse = 29,
    Token = 33,
    FastAuth = 34,
}

#[derive(Debug)]
pub struct ReceivedMessage {
    pub format: BackendMessageFormat,
    pub contents: Bytes,
}

impl ReceivedMessage {
    #[inline]
    pub fn decode_with<T>(self, context: &mut OracleConnection) -> Result<T, Error>
    where
        T: BackendMessage,
    {
        if T::FORMAT != self.format {
            return Err(err_protocol!(
                "Oracle protocol error: expected {:?}, got {:?}",
                T::FORMAT,
                self.format
            ));
        }

        T::decode_body_with(self.contents, context).map_err(|e| match e {
            Error::Protocol(s) => {
                err_protocol!("Oracle protocol error (reading {:?}): {s}", self.format)
            }
            other => other,
        })
    }
}

impl BackendMessageFormat {
    pub fn try_from_u8(v: u8) -> Result<Self, Error> {
        Ok(match v {
            1 => BackendMessageFormat::Protocol,
            2 => BackendMessageFormat::DataTypes,
            3 => BackendMessageFormat::Function,
            4 => BackendMessageFormat::Error,
            6 => BackendMessageFormat::RowHeader,
            7 => BackendMessageFormat::RowData,
            8 => BackendMessageFormat::Parameter,
            9 => BackendMessageFormat::Status,
            11 => BackendMessageFormat::IoVector,
            14 => BackendMessageFormat::LobData,
            15 => BackendMessageFormat::Warning,
            16 => BackendMessageFormat::DescribeInfo,
            17 => BackendMessageFormat::Piggyback,
            19 => BackendMessageFormat::FlushOutBinds,
            21 => BackendMessageFormat::BitVector,
            23 => BackendMessageFormat::ServerSidePiggyback,
            26 => BackendMessageFormat::OnewayFn,
            27 => BackendMessageFormat::ImplicitResultset,
            28 => BackendMessageFormat::Renegotiate,
            29 => BackendMessageFormat::EndOfResponse,
            33 => BackendMessageFormat::Token,
            34 => BackendMessageFormat::FastAuth,
            _ => return Err(err_protocol!("unknown message type: {:?}", v as char)),
        })
    }
}

pub(crate) trait DataMessage: Sized {
    fn decode_body_with(bytes: Bytes, context: &mut OracleConnection) -> Result<Self, Error>;
}

pub(crate) trait FrontendMessage: Sized {
    /// The format prefix of this message.
    const FORMAT: FrontendMessageFormat;

    /// Encode this type as a Frontend message in the Oracle protocol.
    ///
    /// The implementation should *not* include `Self::FORMAT` or the length prefix.
    fn encode_body_with(
        &self,
        buf: &mut Vec<u8>,
        context: &mut OracleConnection,
    ) -> Result<(), Error>;
}

pub(crate) trait BackendMessage: Sized {
    const FORMAT: BackendMessageFormat;

    /// Decode this type from a Backend message in the Oracle protocol.
    ///
    /// The format code and length prefix have already been read and are not at the start of `bytes`.
    fn decode_body_with(buf: Bytes, context: &mut OracleConnection) -> Result<Self, Error>;
}
