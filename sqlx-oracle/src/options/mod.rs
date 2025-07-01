use std::env::var;

use std::fmt::{self, Display, Formatter, Write};
use std::path::Path;

use sqlx_core::{Error, HashMap};
pub use ssl_mode::OracleSslMode;

use crate::{connection::LogSettings, net::tls::CertificateInput};

mod connect;
mod parse;
mod ssl_mode;

/*
def makedsn(
    host: str,
    port: int,
    sid: str = None,
    service_name: str = None,
    region: str = None,
    sharding_key: str = None,
    super_sharding_key: str = None,
) -> str:
    """
    Return a string suitable for use as the dsn parameter for connect(). This
    string is identical to the strings that are defined in the tnsnames.ora
    file.
    """
    connect_data_parts = []
    _check_arg("host", host)
    if service_name is not None:
        _check_arg("service_name", service_name)
        connect_data_parts.append(f"(SERVICE_NAME={service_name})")
    elif sid is not None:
        _check_arg("sid", sid)
        connect_data_parts.append(f"(SID={sid})")
    if region is not None:
        _check_arg("region", region)
        connect_data_parts.append(f"(REGION={region})")
    if sharding_key is not None:
        _check_arg("sharding_key", sharding_key)
        connect_data_parts.append(f"(SHARDING_KEY={sharding_key})")
    if super_sharding_key is not None:
        _check_arg("super_sharding_key", super_sharding_key)
        connect_data_parts.append(f"(SUPER_SHARDING_KEY={super_sharding_key})")
    connect_data = "".join(connect_data_parts)
    return (
        f"(DESCRIPTION=(ADDRESS=(PROTOCOL=TCP)(HOST={host})"
        f"(PORT={port}))(CONNECT_DATA={connect_data}))"
    )
*/

/*
    """
    Constructor for creating a connection to the database.

    The dsn parameter (data source name) can be a string in the format
    user/password@connect_string or can simply be the connect string (in
    which case authentication credentials such as the username and password
    need to be specified separately). See the documentation on connection
    strings for more information.

    The pool parameter is expected to be a pool object and the use of this
    parameter is the equivalent of calling acquire() on the pool.

    The params parameter is expected to be of type ConnectParams and
    contains connection parameters that will be used when establishing the
    connection. See the documentation on ConnectParams for more
    information. If this parameter is not specified, the additional keyword
    parameters will be used to create an instance of ConnectParams. If both
    the params parameter and additional keyword parameters are specified,
    the values in the keyword parameters have precedence. Note that if a
    dsn is also supplied, then in the python-oracledb Thin mode, the values
    of the parameters specified (if any) within the dsn will override the
    values passed as additional keyword parameters, which themselves
    override the values set in the params parameter object.
    """
*/

/*
    """
    All parameters are optional. A brief description of each parameter
    follows:

    - user: the name of the user to connect to

    - proxy_user: the name of the proxy user to connect to. If this value
        is not specified, it will be parsed out of user if user is in the
        form "user[proxy_user]"

    - password: the password for the user

    - newpassword: the new password for the user. The new password will
        take effect immediately upon a successful connection to the database

    - host: the name or IP address of the machine hosting the database or
        the database listener

    - port: the port number on which the database listener is listening

    - protocol: one of the strings "tcp" or "tcps" indicating whether to
        use unencrypted network traffic or encrypted network traffic (TLS)


    - service_name: the service name of the database

    - instance_name: the instance name of the database

    - sid: the system identifier (SID) of the database. Note using a
        service_name instead is recommended

    - server_type: the type of server connection that should be
        established. If specified, it should be one of "dedicated", "shared"
        or "pooled"

    - purity: purity to use for Database Resident Connection Pooling (DRCP)

    - expire_time: an integer indicating the number of minutes between the
        sending of keepalive probes. If this parameter is set to a value
        greater than zero it enables keepalive

    - retry_count: the number of times that a connection attempt should be
        retried before the attempt is terminated

    - retry_delay: the number of seconds to wait before making a new
        connection attempt

    - tcp_connect_timeout: a float indicating the maximum number of seconds
        to wait for establishing a connection to the database host


    - mode: authorization mode to use. For example
        oracledb.AUTH_MODE_SYSDBA

    - disable_oob: boolean indicating whether out-of-band breaks should be
        disabled. This value is only used in thin mode. It has no effect on
        Windows which does not support this functionality

    - stmtcachesize: identifies the initial size of the statement cache

    - config_dir: directory in which the optional tnsnames.ora
        configuration file is located. This value is only used in thin mode.
        For thick mode use the config_dir parameter of init_oracle_client()

    - debug_jdwp: a string with the format "host=<host>;port=<port>" that
        specifies the host and port of the PL/SQL debugger. This value is
        only used in thin mode.

    - connection_id_prefix: an application specific prefix that is added to
        the connection identifier used for tracing

    - sdu: the requested size of the Session Data Unit (SDU), in bytes. The
        value tunes internal buffers used for communication to the database.
        Bigger values can increase throughput for large queries or bulk data
        loads, but at the cost of higher memory use. The SDU size that will
        actually be used is negotiated down to the lower of this value and
        the database network SDU configuration value

    - pool_boundary: one of the values "statement" or "transaction"
        indicating when pooled DRCP connections can be returned to the pool.
        This requires the use of DRCP with Oracle Database 23.4 or higher

    - use_tcp_fast_open: boolean indicating whether to use TCP fast open.
        This is an Oracle Autonomous Database Serverless (ADB-S) specific
        property for clients connecting from within OCI Cloud network. Please
        refer to the ADB-S documentation for more information

    - use_sni: boolean indicating whether to use the TLS SNI extension to
        bypass the second TLS neogiation that would otherwise be required

    """
*/

#[derive(Debug, Clone)]
pub enum ConnectionClass {
    /// connection class to use for Database Resident Connection
    /// Pooling (DRCP)
    CClass(String),
    /// edition to use for the connection. This parameter cannot be
    /// used simultaneously with the cclass parameter
    Edition(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token: String,
    pub private_key: String,
    /// extra_auth_params: a dictionary containing configuration parameters
    /// necessary for Oracle Database authentication using plugins, such as
    /// the Azure and OCI cloud-native authentication plugins
    pub auth_params: HashMap<String, String>,
}

/// access_token: expected to be a string or a 2-tuple or a callable. If
/// it is a string, it specifies an Azure AD OAuth2 token used for Open
/// Authorization (OAuth 2.0) token based authentication. If it is a
/// 2-tuple, it specifies the token and private key strings used for
/// Oracle Cloud Infrastructure (OCI) Identity and Access Management
/// (IAM) token based authentication. If it is a callable, it returns
/// either a string or a 2-tuple used for OAuth 2.0 or OCI IAM token
/// based authentication and is useful when the pool needs to expand and
/// create new connections but the current authentication token has
/// expired
#[derive(Debug, Clone)]
pub enum AccessToken {
    /// If it is a string, it specifies an Azure AD OAuth2 token used for Open
    /// Authorization (OAuth 2.0) token based authentication.
    OAuth(String),
    /// it specifies the token and private key strings used for
    /// Oracle Cloud Infrastructure (OCI) Identity and Access Management
    /// (IAM) token based authentication.
    Token(Token),
}

#[derive(Debug, Clone)]
pub struct Wallet {
    /// wallet_location: the directory where the wallet can be found. In thin
    /// mode this must be the directory containing the PEM-encoded wallet
    /// file ewallet.pem.
    pub location: String,
    /// wallet_password: the password to use to decrypt the wallet, if it is
    /// encrypted. This value is only used in thin mode
    pub password: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SslServerDn {
    /// ssl_server_dn_match: boolean indicating whether the server
    /// certificate distinguished name (DN) should be matched in addition to
    /// the regular certificate verification that is performed. Note that if
    /// the ssl_server_cert_dn parameter is not privided, host name matching
    /// is performed instead
    pub enabled: bool,
    /// ssl_server_cert_dn: the distinguished name (DN) which should be
    /// matched with the server. This value is ignored if the
    /// ssl_server_dn_match parameter is not set to the value True. If
    /// specified this value is used for any verfication. Otherwise the
    /// hostname will be used
    pub cert_dn: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum ServerType {
    Dedicated,
    Shared,
    Pooled,
}

#[derive(Debug, Clone)]
pub struct SocketParams {
    pub expire_time: u32,
    pub retry_count: u32,
    pub retry_delay: u32,
    pub tcp_connect_timeout: f64,
    pub use_tcp_fast_open: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct DefaultOsParams {
    /// the name of the executable program or application connected
    /// to the Oracle Database
    pub program: String,
    /// the terminal identifier from which the connection originates
    pub terminal: String,
    /// the machine name of the client connecting to the Oracle Database
    pub machine: String,
    /// the operating system user that initiates the database connection
    pub osuser: String,
    /// the driver name used by the client to connect to the Oracle Database
    pub driver_name: String,
    pub pid: u32,
}

#[derive(Debug, Clone)]
pub struct OracleConnectOptions {
    pub username: String,
    pub proxy_user: Option<String>,
    pub password: String,
    pub newpassword: Option<String>,
    pub wallet: Option<Wallet>,
    pub host: String,
    pub port: u16,
    pub protocol: String,
    pub database: Option<String>,
    /// https_proxy: the name or IP address of a proxy host to use for tunneling
    /// secure connections the port on which to communicate with the proxy host
    pub https_proxy: Option<(String, u16)>,
    pub service_name: Option<String>,
    pub instance_name: Option<String>,
    pub sid: Option<String>,
    pub server_type: Option<ServerType>,
    pub purity: u8,
    pub ssl_server_dn: SslServerDn,
    pub mode: u8,
    pub disable_oob: bool,
    pub stmtcachesize: u32,
    pub config_dir: Option<String>,
    pub debug_jdwp: Option<String>,
    pub connection_id_prefix: Option<String>,
    pub sdu: u32,
    pub pool_boundary: Option<String>,
    pub use_sni: bool,
    pub conn_class: Option<ConnectionClass>,
    pub os_params: DefaultOsParams,
    pub access_token: Option<AccessToken>,
    pub socket_params: SocketParams,
    pub(crate) log_settings: LogSettings,
    pub(crate) ssl_mode: OracleSslMode,
    pub(crate) ssl_root_cert: Option<CertificateInput>,
    pub(crate) ssl_client_cert: Option<CertificateInput>,
    pub(crate) ssl_client_key: Option<CertificateInput>,
}

impl DefaultOsParams {
    pub fn new() -> Self {
        DefaultOsParams {
            program: String::from("sqlplus"),
            terminal: String::from("xterm"),
            machine: String::from("localhost"),
            osuser: String::from("oracle"),
            driver_name: String::from("Oracle"),
            pid: std::process::id(),
        }
    }
}

impl TryFrom<&str> for ServerType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "dedicated" => Ok(ServerType::Dedicated),
            "shared" => Ok(ServerType::Shared),
            "pooled" => Ok(ServerType::Pooled),
            _ => Err(Error::InvalidArgument(value.to_string())),
        }
    }
}

impl fmt::Display for ServerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerType::Dedicated => write!(f, "dedicated"),
            ServerType::Shared => write!(f, "shared"),
            ServerType::Pooled => write!(f, "pooled"),
        }
    }
}

impl Default for OracleConnectOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl OracleConnectOptions {
    /// Create a default set of connection options populated from the current environment.
    ///
    /// This behaves as if parsed from the connection string `oracle://`
    ///
    /// See the type-level documentation for details.
    pub fn new() -> Self {
        Self::new_without_pgpass()
    }

    /// Create a default set of connection options _without_ reading from `passfile`.
    ///
    /// Equivalent to [`OracleConnectOptions::new()`] but `passfile` is ignored.
    ///
    /// See the type-level documentation for details.
    pub fn new_without_pgpass() -> Self {
        let port = var("PGPORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5432);

        let host = var("PGHOSTADDR")
            .ok()
            .or_else(|| var("PGHOST").ok())
            .unwrap_or_else(|| default_host(port));

        OracleConnectOptions {
            port,
            host,
            password: todo!(),
            debug_jdwp: todo!(),
            proxy_user: todo!(),
            purity: todo!(),
            username: todo!(),
            newpassword: todo!(),
            wallet: todo!(),
            protocol: todo!(),
            https_proxy: todo!(),
            instance_name: todo!(),
            sid: todo!(),
            ssl_server_dn: todo!(),
            mode: todo!(),
            disable_oob: todo!(),
            stmtcachesize: todo!(),
            config_dir: todo!(),
            connection_id_prefix: todo!(),
            sdu: todo!(),
            pool_boundary: todo!(),
            use_sni: todo!(),
            conn_class: todo!(),
            os_params: todo!(),
            access_token: todo!(),
            socket_params: todo!(),
            server_type: todo!(),
            service_name: todo!(),
            database: todo!(),
        }
    }

    /// Sets the name of the host to connect to.
    ///
    /// If a host name begins with a slash, it specifies
    /// Unix-domain communication rather than TCP/IP communication; the value is the name of
    /// the directory in which the socket file is stored.
    ///
    /// The default behavior when host is not specified, or is empty,
    /// is to connect to a Unix-domain socket
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_postgres::OracleConnectOptions;
    /// let options = OracleConnectOptions::new()
    ///     .host("localhost");
    /// ```
    pub fn host(mut self, host: &str) -> Self {
        host.clone_into(&mut self.host);
        self
    }

    pub fn server_type(mut self, server_type: &str) -> Self {
        let server_type = ServerType::try_from(server_type).ok();
        self.server_type = server_type;
        self
    }

    /// Sets the port to connect to at the server host.
    ///
    /// The default port for Oracle is `5432`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_postgres::OracleConnectOptions;
    /// let options = OracleConnectOptions::new()
    ///     .port(5432);
    /// ```
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets the password to use if the server demands password authentication.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_postgres::OracleConnectOptions;
    /// let options = OracleConnectOptions::new()
    ///     .username("root")
    ///     .password("safe-and-secure");
    /// ```
    pub fn password(mut self, password: &str) -> Self {
        password.clone_into(&mut self.password);
        self
    }

    pub fn default_os_params(&self) -> &DefaultOsParams {
        &self.os_params
    }

    pub(crate) fn connect_string(&self) -> &str {
        todo!()
    }

    pub(crate) fn get_private_key(&self) -> Option<String> {
        todo!()
    }
}

impl OracleConnectOptions {
    /// Get the current host.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_postgres::OracleConnectOptions;
    /// let options = OracleConnectOptions::new()
    ///     .host("127.0.0.1");
    /// assert_eq!(options.get_host(), "127.0.0.1");
    /// ```
    pub fn get_host(&self) -> &str {
        &self.host
    }

    /// Get the server's port.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_postgres::OracleConnectOptions;
    /// let options = OracleConnectOptions::new()
    ///     .port(6543);
    /// assert_eq!(options.get_port(), 6543);
    /// ```
    pub fn get_port(&self) -> u16 {
        self.port
    }
}

fn default_host(port: u16) -> String {
    // fallback to localhost if no socket was found
    "localhost".to_owned()
}
