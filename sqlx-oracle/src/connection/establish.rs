use sqlx_core::net::{Socket, WithSocket};

use crate::connection::tls;
use crate::HashMap;

use crate::connection::{sasl, stream::OracleStream};
use crate::error::Error;
use crate::{OracleConnectOptions, OracleConnection};

use super::OracleConnectionInner;

impl OracleConnection {
    pub(crate) async fn establish(options: &OracleConnectOptions) -> Result<Self, Error> {
        let do_connect = DoConnect::new(options);
        let conn = crate::net::connect_tcp(&options.host, options.port, do_connect).await?;
        let stream = conn?;
        Ok(OracleConnection {
            inner: Box::new(OracleConnectionInner {
                stream,
                process_id: 0,
                secret_key: todo!(),
                transaction_status: todo!(),
                transaction_depth: 0,
                pending_ready_for_query_count: 0,
                cache_statement: todo!(),
                cache_type_oid: HashMap::new(),
                cache_type_info: HashMap::new(),
                cache_elem_type_to_array: HashMap::new(),
                log_settings: options.log_settings.clone(),
                token_num: todo!(),
                pipeline_mode: todo!(),
                drcp_establish_session: todo!(),
                action_modified: todo!(),
                action: todo!(),
                client_identifier_modified: todo!(),
                client_info_modified: todo!(),
                dbop_modified: todo!(),
                module_modified: todo!(),
                temp_lobs_to_close: todo!(),
                temp_lobs_total_size: todo!(),
                session_state_desired: todo!(),
                data_flags: todo!(),
                current_schema: todo!(),
                module: todo!(),
                client_identifier: todo!(),
                client_info: todo!(),
                dbop: todo!(),
                seq_num: todo!(),
                server_version_num: todo!(),
                cursors_to_close: todo!(),
                caps: todo!(),
                auth_mode: todo!(),
                cclass: todo!(),
                arraydmlrowcounts: todo!(),
                next_statement_id: todo!(),
            }),
            current_schema_modified: todo!(),
        })
    }
}

struct DoConnect<'a> {
    options: &'a OracleConnectOptions,
}

impl<'a> DoConnect<'a> {
    fn new(options: &'a OracleConnectOptions) -> Self {
        Self { options }
    }

    async fn do_connect<S: Socket>(self, socket: S) -> Result<OracleStream, Error> {
        let DoConnect { options } = self;
        let mut stream = OracleStream::with_socket(options, socket);

        let mut stream = tls::maybe_upgrade(stream, self.options).await?;

        Ok(stream)
    }
}

impl WithSocket for DoConnect<'_> {
    type Output = Result<OracleStream, Error>;
    async fn with_socket<S: Socket>(self, socket: S) -> Self::Output {
        self.do_connect(socket).await
    }
}
