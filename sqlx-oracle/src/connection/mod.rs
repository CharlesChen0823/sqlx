use std::borrow::Cow;
use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;

use crate::protocol::Capabilities;
use crate::HashMap;
use futures_core::future::BoxFuture;
use futures_util::FutureExt;
use sqlx_core::bytes::Bytes;

use crate::common::StatementCache;
use crate::error::Error;
use crate::ext::ustr::UStr;
use crate::statement::OracleStatementMetadata;
use crate::transaction::Transaction;
use crate::{Oracle, OracleConnectOptions, OracleTypeInfo};

pub(crate) use sqlx_core::connection::*;

pub use self::stream::OracleStream;

mod connect;
mod establish;
mod executor;
mod response;
mod stream;
mod tls;

pub struct Packet {
    pub packet_size: u32,
    pub packet_type: u8,
    pub packet_flags: u8,
    pub content: Bytes,
}

/// A connection to a Oracle database.
///
/// See [`OracleConnectOptions`] for connection URL reference.
pub struct OracleConnection {
    pub(crate) inner: Box<OracleConnectionInner>,
    pub current_schema_modified: bool,
}

/*
 *
.conn_impl._action_modified
.conn_impl._call_timeout
.conn_impl._cclass
.conn_impl._client_identifier_modified
.conn_impl._client_info_modified
.conn_impl._combo_key
.conn_impl._current_schema
.conn_impl._current_schema_modified
.conn_impl._db_domain
.conn_impl._db_name
.conn_impl._dbobject_type_cache_num
.conn_impl._dbop_modified
.conn_impl._drcp_enabled
.conn_impl._drcp_establish_session
.conn_impl._edition
.conn_impl._external_name
.conn_impl._force_close()
.conn_impl._get_statement()
.conn_impl._instance_name
.conn_impl._internal_name
.conn_impl._is_pool_extra
.conn_impl._ltxid
.conn_impl._max_identifier_length
.conn_impl._max_open_cursors
.conn_impl._module_modified
.conn_impl._oson_max_fname_size
.conn_impl._pool
.conn_impl._protocol
.conn_impl._serial_num
.conn_impl._service_name
.conn_impl._session_id
.conn_impl._session_state_desired
.conn_impl._statement_cache
.conn_impl._temp_lobs_to_close
.conn_impl._temp_lobs_total_size
.conn_impl.autocommit
.conn_impl.decode_oson()
.conn_impl.ping()
.conn_impl.pipeline_mode
.conn_impl.server_version
.conn_impl.set_call_timeout()
.conn_impl.supports_bool
.conn_impl.username
 */

pub struct OracleConnectionInner {
    // underlying TCP or UDS stream,
    // wrapped in a potentially TLS stream,
    // wrapped in a buffered stream
    pub(crate) stream: OracleStream,

    // process id of this backend
    // used to send cancel requests
    #[allow(dead_code)]
    process_id: u32,

    // secret key of this backend
    // used to send cancel requests
    #[allow(dead_code)]
    secret_key: u32,

    // sequence of statement IDs for use in preparing statements
    // in Oracle, the statement is prepared to a user-supplied identifier
    next_statement_id: StatementId,

    // cache statement by query string to the id and columns
    pub cache_statement: StatementCache<(StatementId, Arc<OracleStatementMetadata>)>,

    // cache user-defined types by id <-> info
    cache_type_info: HashMap<Oid, OracleTypeInfo>,
    cache_type_oid: HashMap<UStr, Oid>,
    cache_elem_type_to_array: HashMap<Oid, Oid>,

    // number of ReadyForQuery messages that we are currently expecting
    pub(crate) pending_ready_for_query_count: usize,

    // current transaction status
    transaction_status: TransactionStatus,
    pub(crate) transaction_depth: usize,

    log_settings: LogSettings,
    pub token_num: u64,
    pub pipeline_mode: u32,
    pub drcp_establish_session: bool,
    pub action_modified: bool,
    pub action: Option<String>,
    pub client_identifier_modified: bool,
    pub client_info_modified: bool,
    pub dbop_modified: bool,
    pub module_modified: bool,
    pub temp_lobs_to_close: Vec<String>,
    pub temp_lobs_total_size: u64,
    pub session_state_desired: u64,
    pub data_flags: u16,
    pub current_schema: String,
    pub module: Option<String>,
    pub client_identifier: Option<String>,
    pub client_info: Option<String>,
    pub dbop: Option<String>,
    pub seq_num: u8,
    pub server_version_num: u32,
    pub cursors_to_close: Vec<u32>,
    pub caps: Capabilities,
    pub auth_mode: u32,
    pub cclass: Option<String>,
    pub arraydmlrowcounts: u32, // todo!()
}

impl OracleConnection {
    pub fn server_version_num(&self) -> u32 {
        self.inner.server_version_num
    }

    pub fn get_host_info(&self) -> (String, String) {
        (String::new(), String::new())
    }

    pub fn get_seq_num(&mut self) -> u8 {
        if self.inner.seq_num >= u8::MAX {
            self.inner.seq_num = 1;
        } else {
            self.inner.seq_num.saturating_add(1);
        }
        let seq_num = self.inner.seq_num;
        return seq_num;
    }

    // will return when the connection is ready for another query
    pub(crate) async fn wait_until_ready(&mut self) -> Result<(), Error> {
        if !self.inner.stream.write_buffer_mut().is_empty() {
            self.inner.stream.flush().await?;
        }

        while self.inner.pending_ready_for_query_count > 0 {
            let message = self.inner.stream.recv().await?;
        }

        Ok(())
    }

    pub(crate) fn clear_open_cursors(&self) {
        todo!()
    }

    pub(crate) fn num_columns(&self) -> u32 {
        todo!()
    }
}

impl Debug for OracleConnection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("OracleConnection").finish()
    }
}

impl Connection for OracleConnection {
    type Database = Oracle;

    type Options = OracleConnectOptions;

    fn close(mut self) -> BoxFuture<'static, Result<(), Error>> {
        // The normal, graceful termination procedure is that the frontend sends a Terminate
        // message and immediately closes the connection.

        // On receipt of this message, the backend closes the
        // connection and terminates.

        Box::pin(async move {
            self.inner.stream.shutdown().await?;

            Ok(())
        })
    }

    fn close_hard(mut self) -> BoxFuture<'static, Result<(), Error>> {
        Box::pin(async move {
            self.inner.stream.shutdown().await?;

            Ok(())
        })
    }

    fn ping(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        // Users were complaining about this showing up in query statistics on the server.
        // By sending a comment we avoid an error if the connection was in the middle of a rowset
        // self.execute("/* SQLx ping */").map_ok(|_| ()).boxed()

        Box::pin(async move {
            // The simplest call-and-response that's possible.
            self.write_sync();
            self.wait_until_ready().await
        })
    }

    fn begin(&mut self) -> BoxFuture<'_, Result<Transaction<'_, Self::Database>, Error>>
    where
        Self: Sized,
    {
        Transaction::begin(self, None)
    }

    fn begin_with(
        &mut self,
        statement: impl Into<Cow<'static, str>>,
    ) -> BoxFuture<'_, Result<Transaction<'_, Self::Database>, Error>>
    where
        Self: Sized,
    {
        Transaction::begin(self, Some(statement.into()))
    }

    fn cached_statements_size(&self) -> usize {
        self.inner.cache_statement.len()
    }

    fn clear_cached_statements(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async move {
            self.inner.cache_type_oid.clear();

            let mut cleared = 0_usize;

            self.wait_until_ready().await?;

            if cleared > 0 {
                self.write_sync();
                self.inner.stream.flush().await?;
            }

            Ok(())
        })
    }

    fn shrink_buffers(&mut self) {
        self.inner.stream.shrink_buffers();
    }

    #[doc(hidden)]
    fn flush(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        self.wait_until_ready().boxed()
    }

    #[doc(hidden)]
    fn should_flush(&self) -> bool {
        !self.inner.stream.write_buffer().is_empty()
    }
}

// Implement `AsMut<Self>` so that `OracleConnection` can be wrapped in
// a `OracleAdvisoryLockGuard`.
//
// See: https://github.com/launchbadge/sqlx/issues/2520
impl AsMut<OracleConnection> for OracleConnection {
    fn as_mut(&mut self) -> &mut OracleConnection {
        self
    }
}
