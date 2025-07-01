#[derive(Debug, Clone)]
pub struct OraError {
    pub message: String,
    pub code: u16,
    pub offset: u32,
    pub isrecoverable: bool,
    pub iswarning: bool,
}

impl OraError {
    pub fn default() -> Self {
        OraError {
            message: String::new(),
            code: 0,
            offset: 0,
            isrecoverable: false,
            iswarning: false,
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_error_code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }

    pub fn warning(mut self) -> Self {
        self.iswarning = true;
        self
    }
}

impl std::fmt::Display for OraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Oracle Error: {} (Code: {}, Offset: {})",
            self.message, self.code, self.offset
        )
    }
}

/// error numbers that result in InterfaceError
pub const ERR_MISSING_ERROR: u16 = 1000;
pub const ERR_NOT_CONNECTED: u16 = 1001;
pub const ERR_POOL_NOT_OPEN: u16 = 1002;
pub const ERR_NOT_A_QUERY: u16 = 1003;
pub const ERR_NO_STATEMENT_EXECUTED: u16 = 1004;
pub const ERR_POOL_HAS_BUSY_CONNECTIONS: u16 = 1005;
pub const ERR_CURSOR_NOT_OPEN: u16 = 1006;

/// error numbers that result in ProgrammingError
pub const ERR_MESSAGE_HAS_NO_PAYLOAD: u16 = 2000;
pub const ERR_NO_STATEMENT: u16 = 2001;
pub const ERR_NO_STATEMENT_PREPARED: u16 = 2002;
pub const ERR_WRONG_EXECUTE_PARAMETERS_TYPE: u16 = 2003;
pub const ERR_WRONG_EXECUTEMANY_PARAMETERS_TYPE: u16 = 2004;
pub const ERR_ARGS_AND_KEYWORD_ARGS: u16 = 2005;
pub const ERR_MIXED_POSITIONAL_AND_NAMED_BINDS: u16 = 2006;
pub const ERR_EXPECTING_TYPE: u16 = 2007;
pub const ERR_WRONG_OBJECT_TYPE: u16 = 2008;
pub const ERR_WRONG_SCROLL_MODE: u16 = 2009;
pub const ERR_MIXED_ELEMENT_TYPES: u16 = 2010;
pub const ERR_WRONG_ARRAY_DEFINITION: u16 = 2011;
pub const ERR_ARGS_MUST_BE_LIST_OR_TUPLE: u16 = 2012;
pub const ERR_KEYWORD_ARGS_MUST_BE_DICT: u16 = 2013;
pub const ERR_DUPLICATED_PARAMETER: u16 = 2014;
pub const ERR_EXPECTING_VAR: u16 = 2015;
pub const ERR_INCORRECT_VAR_ARRAYSIZE: u16 = 2016;
pub const ERR_LIBRARY_ALREADY_INITIALIZED: u16 = 2017;
pub const ERR_WALLET_FILE_MISSING: u16 = 2018;
pub const ERR_THIN_CONNECTION_ALREADY_CREATED: u16 = 2019;
pub const ERR_INVALID_MAKEDSN_ARG: u16 = 2020;
pub const ERR_INIT_ORACLE_CLIENT_NOT_CALLED: u16 = 2021;
pub const ERR_INVALID_OCI_ATTR_TYPE: u16 = 2022;
pub const ERR_INVALID_CONN_CLASS: u16 = 2023;
pub const ERR_INVALID_CONNECT_PARAMS: u16 = 2025;
pub const ERR_INVALID_POOL_CLASS: u16 = 2026;
pub const ERR_INVALID_POOL_PARAMS: u16 = 2027;
pub const ERR_EXPECTING_LIST_FOR_ARRAY_VAR: u16 = 2028;
pub const ERR_HTTPS_PROXY_REQUIRES_TCPS: u16 = 2029;
pub const ERR_INVALID_LOB_OFFSET: u16 = 2030;
pub const ERR_INVALID_ACCESS_TOKEN_PARAM: u16 = 2031;
pub const ERR_INVALID_ACCESS_TOKEN_RETURNED: u16 = 2032;
pub const ERR_EXPIRED_ACCESS_TOKEN: u16 = 2033;
pub const ERR_ACCESS_TOKEN_REQUIRES_TCPS: u16 = 2034;
pub const ERR_INVALID_OBJECT_TYPE_NAME: u16 = 2035;
pub const ERR_OBJECT_IS_NOT_A_COLLECTION: u16 = 2036;
pub const ERR_MISSING_TYPE_NAME_FOR_OBJECT_VAR: u16 = 2037;
pub const ERR_INVALID_COLL_INDEX_GET: u16 = 2038;
pub const ERR_INVALID_COLL_INDEX_SET: u16 = 2039;
pub const ERR_EXECUTE_MODE_ONLY_FOR_DML: u16 = 2040;
pub const ERR_MISSING_ENDING_SINGLE_QUOTE: u16 = 2041;
pub const ERR_MISSING_ENDING_DOUBLE_QUOTE: u16 = 2042;
pub const ERR_DBOBJECT_ATTR_MAX_SIZE_VIOLATED: u16 = 2043;
pub const ERR_DBOBJECT_ELEMENT_MAX_SIZE_VIOLATED: u16 = 2044;
pub const ERR_INVALID_ARRAYSIZE: u16 = 2045;
pub const ERR_CURSOR_HAS_BEEN_CLOSED: u16 = 2046;
pub const ERR_INVALID_LOB_AMOUNT: u16 = 2047;
pub const ERR_DML_RETURNING_DUP_BINDS: u16 = 2048;
pub const ERR_MISSING_ADDRESS: u16 = 2049;
pub const ERR_INVALID_TPC_BEGIN_FLAGS: u16 = 2050;
pub const ERR_INVALID_TPC_END_FLAGS: u16 = 2051;
pub const ERR_MISMATCHED_TOKEN: u16 = 2052;
pub const ERR_THICK_MODE_ENABLED: u16 = 2053;
pub const ERR_NAMED_POOL_MISSING: u16 = 2054;
pub const ERR_NAMED_POOL_EXISTS: u16 = 2055;
pub const ERR_PROTOCOL_HANDLER_FAILED: u16 = 2056;
pub const ERR_PASSWORD_TYPE_HANDLER_FAILED: u16 = 2057;
pub const ERR_PLAINTEXT_PASSWORD_IN_CONFIG: u16 = 2058;
pub const ERR_MISSING_CONNECT_DESCRIPTOR: u16 = 2059;
pub const ERR_ARROW_C_API_ERROR: u16 = 2060;
pub const ERR_PARAMS_HOOK_HANDLER_FAILED: u16 = 2061;
pub const ERR_PAYLOAD_CANNOT_BE_ENQUEUED: u16 = 2062;
pub const ERR_SCROLL_OUT_OF_RESULT_SET: u16 = 2063;

/// error numbers that result in NotSupportedError
pub const ERR_TIME_NOT_SUPPORTED: u16 = 3000;
pub const ERR_FEATURE_NOT_SUPPORTED: u16 = 3001;
pub const ERR_PYTHON_VALUE_NOT_SUPPORTED: u16 = 3002;
pub const ERR_PYTHON_TYPE_NOT_SUPPORTED: u16 = 3003;
pub const ERR_UNSUPPORTED_TYPE_SET: u16 = 3004;
pub const ERR_ARRAYS_OF_ARRAYS: u16 = 3005;
pub const ERR_ORACLE_TYPE_NOT_SUPPORTED: u16 = 3006;
pub const ERR_DB_TYPE_NOT_SUPPORTED: u16 = 3007;
pub const ERR_UNSUPPORTED_INBAND_NOTIFICATION: u16 = 3008;
pub const ERR_SELF_BIND_NOT_SUPPORTED: u16 = 3009;
pub const ERR_SERVER_VERSION_NOT_SUPPORTED: u16 = 3010;
pub const ERR_NCHAR_CS_NOT_SUPPORTED: u16 = 3012;
pub const ERR_UNSUPPORTED_PYTHON_TYPE_FOR_DB_TYPE: u16 = 3013;
pub const ERR_LOB_OF_WRONG_TYPE: u16 = 3014;
pub const ERR_UNSUPPORTED_VERIFIER_TYPE: u16 = 3015;
pub const ERR_NO_CRYPTOGRAPHY_PACKAGE: u16 = 3016;
pub const ERR_ORACLE_TYPE_NAME_NOT_SUPPORTED: u16 = 3017;
pub const ERR_TDS_TYPE_NOT_SUPPORTED: u16 = 3018;
pub const ERR_OSON_NODE_TYPE_NOT_SUPPORTED: u16 = 3019;
pub const ERR_OSON_FIELD_NAME_LIMITATION: u16 = 3020;
pub const ERR_OSON_VERSION_NOT_SUPPORTED: u16 = 3021;
pub const ERR_NAMED_TIMEZONE_NOT_SUPPORTED: u16 = 3022;
pub const ERR_VECTOR_VERSION_NOT_SUPPORTED: u16 = 3023;
pub const ERR_VECTOR_FORMAT_NOT_SUPPORTED: u16 = 3024;
pub const ERR_OPERATION_NOT_SUPPORTED_ON_BFILE: u16 = 3025;
pub const ERR_OPERATION_ONLY_SUPPORTED_ON_BFILE: u16 = 3026;
pub const ERR_CURSOR_DIFF_CONNECTION: u16 = 3027;
pub const ERR_UNSUPPORTED_PIPELINE_OPERATION: u16 = 3028;
pub const ERR_INVALID_NETWORK_NAME: u16 = 3029;
pub const ERR_ARROW_UNSUPPORTED_DATA_TYPE: u16 = 3030;

/// error numbers that result in DatabaseError
pub const ERR_TNS_ENTRY_NOT_FOUND: u16 = 4000;
pub const ERR_NO_CREDENTIALS: u16 = 4001;
pub const ERR_COLUMN_TRUNCATED: u16 = 4002;
pub const ERR_ORACLE_NUMBER_NO_REPR: u16 = 4003;
pub const ERR_INVALID_NUMBER: u16 = 4004;
pub const ERR_POOL_NO_CONNECTION_AVAILABLE: u16 = 4005;
pub const ERR_ARRAY_DML_ROW_COUNTS_NOT_ENABLED: u16 = 4006;
pub const ERR_INCONSISTENT_DATATYPES: u16 = 4007;
pub const ERR_INVALID_BIND_NAME: u16 = 4008;
pub const ERR_WRONG_NUMBER_OF_POSITIONAL_BINDS: u16 = 4009;
pub const ERR_MISSING_BIND_VALUE: u16 = 4010;
pub const ERR_CONNECTION_CLOSED: u16 = 4011;
pub const ERR_NUMBER_WITH_INVALID_EXPONENT: u16 = 4012;
pub const ERR_NUMBER_STRING_OF_ZERO_LENGTH: u16 = 4013;
pub const ERR_NUMBER_STRING_TOO_LONG: u16 = 4014;
pub const ERR_NUMBER_WITH_EMPTY_EXPONENT: u16 = 4015;
pub const ERR_CONTENT_INVALID_AFTER_NUMBER: u16 = 4016;
pub const ERR_INVALID_CONNECT_DESCRIPTOR: u16 = 4017;
pub const ERR_CANNOT_PARSE_CONNECT_STRING: u16 = 4018;
pub const ERR_INVALID_REDIRECT_DATA: u16 = 4019;
pub const ERR_INVALID_PROTOCOL: u16 = 4021;
pub const ERR_INVALID_ENUM_VALUE: u16 = 4022;
pub const ERR_CALL_TIMEOUT_EXCEEDED: u16 = 4024;
pub const ERR_INVALID_REF_CURSOR: u16 = 4025;
pub const ERR_MISSING_FILE: u16 = 4026;
pub const ERR_NO_CONFIG_DIR: u16 = 4027;
pub const ERR_INVALID_SERVER_TYPE: u16 = 4028;
pub const ERR_TOO_MANY_BATCH_ERRORS: u16 = 4029;
pub const ERR_IFILE_CYCLE_DETECTED: u16 = 4030;
pub const ERR_INVALID_VECTOR: u16 = 4031;
pub const ERR_INVALID_SSL_VERSION: u16 = 4032;
pub const ERR_EXCEEDED_IDLE_TIME: u16 = 4033;
pub const ERR_INVALID_PASSWORD_TYPE: u16 = 4034;

/// error numbers that result in InternalError
pub const ERR_MESSAGE_TYPE_UNKNOWN: u16 = 5000;
pub const ERR_BUFFER_LENGTH_INSUFFICIENT: u16 = 5001;
pub const ERR_INTEGER_TOO_LARGE: u16 = 5002;
pub const ERR_UNEXPECTED_NEGATIVE_INTEGER: u16 = 5003;
pub const ERR_UNEXPECTED_DATA: u16 = 5004;
pub const ERR_UNEXPECTED_REFUSE: u16 = 5005;
pub const ERR_UNEXPECTED_END_OF_DATA: u16 = 5006;
pub const ERR_UNEXPECTED_XML_TYPE: u16 = 5007;
pub const ERR_UNKNOWN_SERVER_PIGGYBACK: u16 = 5009;
pub const ERR_UNKNOWN_TRANSACTION_STATE: u16 = 5010;
pub const ERR_UNEXPECTED_PIPELINE_FAILURE: u16 = 5011;
pub const ERR_NOT_IMPLEMENTED: u16 = 5012;

/// error numbers that result in OperationalError
pub const ERR_LISTENER_REFUSED_CONNECTION: u16 = 6000;
pub const ERR_INVALID_SERVICE_NAME: u16 = 6001;
pub const ERR_INVALID_SERVER_CERT_DN: u16 = 6002;
pub const ERR_INVALID_SID: u16 = 6003;
pub const ERR_PROXY_FAILURE: u16 = 6004;
pub const ERR_CONNECTION_FAILED: u16 = 6005;
pub const ERR_INVALID_SERVER_NAME: u16 = 6006;

/// error numbers that result in Warning
pub const WRN_COMPILATION_ERROR: u16 = 7000;
