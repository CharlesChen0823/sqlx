use crate::error::Error;
use crate::{OracleConnectOptions, OracleSslMode};
use sqlx_core::percent_encoding::percent_decode_str;
use sqlx_core::Url;
use std::net::IpAddr;
use std::str::FromStr;

impl OracleConnectOptions {
    pub(crate) fn parse_from_url(url: &Url) -> Result<Self, Error> {
        let mut options = Self::new_without_pgpass();

        if let Some(host) = url.host_str() {
            let host_decoded = percent_decode_str(host);
            options = match host_decoded.clone().next() {
                _ => options.host(host),
            }
        }

        if let Some(port) = url.port() {
            options = options.port(port);
        }

        let username = url.username();
        if !username.is_empty() {
            options = options.username(
                &percent_decode_str(username)
                    .decode_utf8()
                    .map_err(Error::config)?,
            );
        }

        if let Some(password) = url.password() {
            options = options.password(
                &percent_decode_str(password)
                    .decode_utf8()
                    .map_err(Error::config)?,
            );
        }

        let path = url.path().trim_start_matches('/');
        if !path.is_empty() {
            options = options.database(
                &percent_decode_str(path)
                    .decode_utf8()
                    .map_err(Error::config)?,
            );
        }

        for (key, value) in url.query_pairs().into_iter() {
            match &*key {
                "sslmode" | "ssl-mode" => {
                    options = options.ssl_mode(value.parse().map_err(Error::config)?);
                }

                "sslrootcert" | "ssl-root-cert" | "ssl-ca" => {
                    options = options.ssl_root_cert(&*value);
                }

                "sslcert" | "ssl-cert" => options = options.ssl_client_cert(&*value),

                "sslkey" | "ssl-key" => options = options.ssl_client_key(&*value),

                "statement-cache-capacity" => {
                    options =
                        options.statement_cache_capacity(value.parse().map_err(Error::config)?);
                }

                "host" => {
                    options = options.host(&value);
                }

                "hostaddr" => {
                    value.parse::<IpAddr>().map_err(Error::config)?;
                    options = options.host(&value)
                }

                "port" => options = options.port(value.parse().map_err(Error::config)?),

                "dbname" => options = options.database(&value),

                "user" => options = options.username(&value),

                "password" => options = options.password(&value),

                "options" => {
                    if let Some(options) = options.options.as_mut() {
                        options.push(' ');
                        options.push_str(&value);
                    } else {
                        options.options = Some(value.to_string());
                    }
                }

                k if k.starts_with("options[") => {
                    if let Some(key) = k.strip_prefix("options[").unwrap().strip_suffix(']') {
                        options = options.options([(key, &*value)]);
                    }
                }

                _ => tracing::warn!(%key, %value, "ignoring unrecognized connect parameter"),
            }
        }

        Ok(options)
    }

    pub(crate) fn build_url(&self) -> Url {
        let host = self.host.to_owned();

        let mut url = Url::parse(&format!(
            "oracle://{}@{}:{}",
            self.username, host, self.port
        ))
        .expect("BUG: generated un-parseable URL");

        let password = self.password.to_owned();

        if let Some(database) = &self.database {
            url.set_path(database);
        }

        let ssl_mode = match self.ssl_mode {
            OracleSslMode::Allow => "allow",
            OracleSslMode::Disable => "disable",
            OracleSslMode::Prefer => "prefer",
            OracleSslMode::Require => "require",
            OracleSslMode::VerifyCa => "verify-ca",
            OracleSslMode::VerifyFull => "verify-full",
        };
        url.query_pairs_mut().append_pair("sslmode", ssl_mode);

        if let Some(ssl_root_cert) = &self.ssl_root_cert {
            url.query_pairs_mut()
                .append_pair("sslrootcert", &ssl_root_cert.to_string());
        }

        if let Some(ssl_client_cert) = &self.ssl_client_cert {
            url.query_pairs_mut()
                .append_pair("sslcert", &ssl_client_cert.to_string());
        }

        if let Some(ssl_client_key) = &self.ssl_client_key {
            url.query_pairs_mut()
                .append_pair("sslkey", &ssl_client_key.to_string());
        }

        url.query_pairs_mut().append_pair(
            "statement-cache-capacity",
            &self.statement_cache_capacity.to_string(),
        );

        url
    }
}

impl FromStr for OracleConnectOptions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let url: Url = s.parse().map_err(Error::config)?;

        Self::parse_from_url(&url)
    }
}
