use sqlx_core::net::BufferedSocket;

use crate::connection::OracleStream;
use crate::error::Error;
use crate::net::tls::{self, TlsConfig};
use crate::net::{Socket, WithSocket};

use crate::protocol::Capabilities;
use crate::{OracleConnectOptions, OracleSslMode};

const TNS_VERSION_DESIRED: u8 = 319; // todo!() define in other place

fn calc_sni_data(options: &OracleConnectOptions) -> String {
    let server_type_part = if let Some(ref server_type) = options.server_type {
        format!(".T1.{}", server_type)
    } else {
        "".to_string()
    };

    let service_name = options.service_name.as_ref().unwrap();

    return format!(
        "S{}.{}{}.V3.{}",
        service_name.len(),
        service_name,
        server_type_part,
        TNS_VERSION_DESIRED
    );
}

pub async fn maybe_upgrade<S: Socket>(
    mut stream: OracleStream<S>,
    options: &OracleConnectOptions,
) -> Result<OracleStream, Error> {
    match options.ssl_mode {
        OracleSslMode::Allow | OracleSslMode::Disable => return Ok(stream.boxed_socket()),

        OracleSslMode::Prefer => {
            if !tls::available() {
                return Ok(stream.boxed_socket());
            }
        }

        OracleSslMode::Require | OracleSslMode::VerifyFull | OracleSslMode::VerifyCa => {
            tls::error_if_unavailable()?;
        }
    }

    let accept_invalid_certs = !matches!(
        options.ssl_mode,
        OracleSslMode::VerifyCa | OracleSslMode::VerifyFull
    );
    let accept_invalid_hostnames = !matches!(options.ssl_mode, OracleSslMode::VerifyFull);

    let hostname = calc_sni_data(options);

    let config = TlsConfig {
        accept_invalid_certs,
        accept_invalid_hostnames,
        hostname: hostname.as_str(),
        root_cert_path: options.ssl_root_cert.as_ref(),
        client_cert_path: options.ssl_client_cert.as_ref(),
        client_key_path: options.ssl_client_key.as_ref(),
    };

    tls::handshake(
        stream.socket.into_inner(),
        config,
        MapStream {
            server_version: todo!(),
            capabilities: todo!(),
            sequence_id: todo!(),
        },
    )
    .await
}

struct MapStream {
    server_version: (u16, u16, u16),
    capabilities: Capabilities,
    sequence_id: u8,
}

impl WithSocket for MapStream {
    type Output = OracleStream;

    async fn with_socket<S: Socket>(self, socket: S) -> Self::Output {
        OracleStream {
            socket: BufferedSocket::new(Box::new(socket)),
            server_version: self.server_version,
            capabilities: self.capabilities,
            sequence_id: self.sequence_id,
            is_tls: true,
            is_authed: todo!(),
        }
    }
}
