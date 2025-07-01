use sqlx_core::Error;

use crate::constants::*;

#[derive(Debug, Clone, Copy)]
pub struct Capabilities {
    pub protocol_version: u16,
    pub ttc_field_version: u8,
    pub charset_id: u16,
    pub ncharset_id: u16,
    pub compile_caps: Vec<u8>,
    pub runtime_caps: Vec<u8>,
    pub max_string_size: u32,
    pub supports_fast_auth: bool,
    pub supports_oob: bool,
    pub supports_oob_check: bool,
    pub supports_end_of_response: bool,
    pub supports_pipelining: bool,
    pub supports_request_boundaries: bool,
    pub sdu: u16,
}

fn init_compile_caps(ttc_field_version: u8) -> Vec<u8> {
    let mut compile_caps = Vec::with_capacity(TNS_CCAP_MAX as usize);
    compile_caps[TNS_CCAP_SQL_VERSION as usize] = TNS_CCAP_SQL_VERSION_MAX;
    compile_caps[TNS_CCAP_LOGON_TYPES as usize] = TNS_CCAP_O5LOGON
        | TNS_CCAP_O5LOGON_NP
        | TNS_CCAP_O7LOGON
        | TNS_CCAP_O8LOGON_LONG_IDENTIFIER
        | TNS_CCAP_O9LOGON_LONG_PASSWORD;
    compile_caps[TNS_CCAP_FEATURE_BACKPORT as usize] = TNS_CCAP_CTB_IMPLICIT_POOL;
    compile_caps[TNS_CCAP_FIELD_VERSION as usize] = ttc_field_version;
    compile_caps[TNS_CCAP_SERVER_DEFINE_CONV as usize] = 1;
    compile_caps[TNS_CCAP_DEQUEUE_WITH_SELECTOR as usize] = 1;
    compile_caps[TNS_CCAP_TTC1 as usize] =
        TNS_CCAP_FAST_BVEC | TNS_CCAP_END_OF_CALL_STATUS | TNS_CCAP_IND_RCD;
    compile_caps[TNS_CCAP_OCI1 as usize] =
        TNS_CCAP_FAST_SESSION_PROPAGATE | TNS_CCAP_APP_CTX_PIGGYBACK;
    compile_caps[TNS_CCAP_TDS_VERSION as usize] = TNS_CCAP_TDS_VERSION_MAX;
    compile_caps[TNS_CCAP_RPC_VERSION as usize] = TNS_CCAP_RPC_VERSION_MAX;
    compile_caps[TNS_CCAP_RPC_SIG as usize] = TNS_CCAP_RPC_SIG_VALUE;
    compile_caps[TNS_CCAP_DBF_VERSION as usize] = TNS_CCAP_DBF_VERSION_MAX;
    compile_caps[TNS_CCAP_LOB as usize] = TNS_CCAP_LOB_UB8_SIZE
        | TNS_CCAP_LOB_ENCS
        | TNS_CCAP_LOB_PREFETCH_LENGTH
        | TNS_CCAP_LOB_TEMP_SIZE
        | TNS_CCAP_LOB_12C
        | TNS_CCAP_LOB_PREFETCH_DATA;
    compile_caps[TNS_CCAP_UB2_DTY as usize] = 1;
    compile_caps[TNS_CCAP_LOB2 as usize] = TNS_CCAP_LOB2_QUASI | TNS_CCAP_LOB2_2GB_PREFETCH;
    compile_caps[TNS_CCAP_TTC3 as usize] = TNS_CCAP_IMPLICIT_RESULTS
        | TNS_CCAP_BIG_CHUNK_CLR
        | TNS_CCAP_KEEP_OUT_ORDER
        | TNS_CCAP_LTXID;
    compile_caps[TNS_CCAP_TTC2 as usize] = TNS_CCAP_ZLNP;
    compile_caps[TNS_CCAP_OCI2 as usize] = TNS_CCAP_DRCP;
    compile_caps[TNS_CCAP_CLIENT_FN as usize] = TNS_CCAP_CLIENT_FN_MAX;
    compile_caps[TNS_CCAP_SESS_SIGNATURE_VERSION as usize] = TNS_CCAP_FIELD_VERSION_12_2;
    compile_caps[TNS_CCAP_TTC4 as usize] =
        TNS_CCAP_INBAND_NOTIFICATION | TNS_CCAP_EXPLICIT_BOUNDARY;
    compile_caps[TNS_CCAP_TTC5 as usize] = TNS_CCAP_VECTOR_SUPPORT
        | TNS_CCAP_TOKEN_SUPPORTED
        | TNS_CCAP_PIPELINING_SUPPORT
        | TNS_CCAP_PIPELINING_BREAK;
    compile_caps[TNS_CCAP_VECTOR_FEATURES as usize] =
        TNS_CCAP_VECTOR_FEATURE_BINARY | TNS_CCAP_VECTOR_FEATURE_SPARSE;
    return compile_caps;
}

fn init_runtime_caps() -> Vec<u8> {
    let mut runtime_caps = Vec::with_capacity(TNS_RCAP_MAX as usize);
    runtime_caps[TNS_RCAP_COMPAT as usize] = TNS_RCAP_COMPAT_81;
    runtime_caps[TNS_RCAP_TTC as usize] = TNS_RCAP_TTC_ZERO_COPY | TNS_RCAP_TTC_32K;
    return runtime_caps;
}

impl Default for Capabilities {
    fn default() -> Self {
        Self::new()
    }
}

impl Capabilities {
    fn new() -> Self {
        let ttc_field_version = TNS_CCAP_FIELD_VERSION_MAX;
        let compile_caps = init_compile_caps(ttc_field_version);
        let runtime_caps = init_runtime_caps();
        Capabilities {
            protocol_version: 0,
            ttc_field_version,
            charset_id: 0,
            ncharset_id: 0,
            compile_caps,
            runtime_caps,
            max_string_size: 4000,
            supports_fast_auth: false,
            supports_oob: false,
            supports_oob_check: false,
            supports_end_of_response: false,
            supports_pipelining: false,
            supports_request_boundaries: false,
            sdu: 8192,
        }
    }

    fn adjust_for_protocol(&mut self, protocol_version: u16, protocol_options: u16, flags: u32) {
        self.protocol_version = protocol_options;
        self.supports_oob = (protocol_options & TNS_GSO_CAN_RECV_ATTENTION) > 0;
        if (flags & TNS_ACCEPT_FLAG_FAST_AUTH) > 0 {
            self.supports_fast_auth = true;
        }
        if flags & TNS_ACCEPT_FLAG_CHECK_OOB > 0 {
            self.supports_oob_check = true;
        }
        if protocol_version >= TNS_VERSION_MIN_END_OF_RESPONSE {
            if flags & TNS_ACCEPT_FLAG_HAS_END_OF_RESPONSE > 0 {
                self.compile_caps[TNS_CCAP_TTC4 as usize] |= TNS_CCAP_END_OF_RESPONSE;
                self.supports_end_of_response = true;
                self.supports_pipelining = true;
            }
        }
    }

    fn adjust_for_server_compile_caps(&mut self, server_compile_caps: &[u8]) {
        if server_compile_caps[TNS_CCAP_FIELD_VERSION as usize] < self.ttc_field_version {
            self.ttc_field_version = server_compile_caps[TNS_CCAP_FIELD_VERSION as usize];
            self.compile_caps[TNS_CCAP_FIELD_VERSION as usize] = self.ttc_field_version;
        }
        if server_compile_caps[TNS_CCAP_TTC4 as usize] & TNS_CCAP_EXPLICIT_BOUNDARY > 0 {
            self.supports_request_boundaries = true;
        }
    }

    fn adjust_for_server_runtime_caps(&mut self, server_compile_caps: &[u8]) {
        if server_compile_caps[TNS_RCAP_TTC as usize] & TNS_RCAP_TTC_32K > 0 {
            self.max_string_size = 32767;
        } else {
            self.max_string_size = 4000; // set default to 4000, this branch don't need
        }
        if (server_compile_caps[TNS_RCAP_TTC as usize] & TNS_RCAP_TTC_SESSION_STATE_OPS) <= 0 {
            self.supports_request_boundaries = false; // origin value might be true?
        }
    }

    fn check_ncharset_id(&self) -> Result<bool, Error> {
        if self.ncharset_id != TNS_CHARSET_UTF16 {
            return Err(Error::Configuration("".into()));
        }
        return Ok(true);
    }
}
