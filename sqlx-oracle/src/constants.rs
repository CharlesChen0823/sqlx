/// packet types
pub(crate) const TNS_PACKET_TYPE_CONNECT: u8 = 1;
pub(crate) const TNS_PACKET_TYPE_ACCEPT: u8 = 2;
pub(crate) const TNS_PACKET_TYPE_REFUSE: u8 = 4;
pub(crate) const TNS_PACKET_TYPE_DATA: u8 = 6;
pub(crate) const TNS_PACKET_TYPE_RESEND: u8 = 11;
pub(crate) const TNS_PACKET_TYPE_MARKER: u8 = 12;
pub(crate) const TNS_PACKET_TYPE_CONTROL: u8 = 14;
pub(crate) const TNS_PACKET_TYPE_REDIRECT: u8 = 5;

/// packet flags
pub(crate) const TNS_PACKET_FLAG_REDIRECT: u8 = 0x04;
pub(crate) const TNS_PACKET_FLAG_TLS_RENEG: u8 = 0x08;

/// data flags
pub(crate) const TNS_DATA_FLAGS_BEGIN_PIPELINE: u16 = 0x1000;
pub(crate) const TNS_DATA_FLAGS_END_OF_REQUEST: u16 = 0x800;
pub(crate) const TNS_DATA_FLAGS_END_OF_RESPONSE: u16 = 0x2000;
pub(crate) const TNS_DATA_FLAGS_EOF: u16 = 0x0040;

/// marker types
pub(crate) const TNS_MARKER_TYPE_BREAK: u8 = 1;
pub(crate) const TNS_MARKER_TYPE_RESET: u8 = 2;
pub(crate) const TNS_MARKER_TYPE_INTERRUPT: u8 = 3;

/// AQ delivery modes
pub(crate) const TNS_AQ_MSG_BUFFERED: u8 = 2;
pub(crate) const TNS_AQ_MSG_PERSISTENT: u8 = 1;
pub(crate) const TNS_AQ_MSG_PERSISTENT_OR_BUFFERED: u8 = 3;

/// AQ dequeue modes
pub(crate) const TNS_AQ_DEQ_BROWSE: u8 = 1;
pub(crate) const TNS_AQ_DEQ_LOCKED: u8 = 2;
pub(crate) const TNS_AQ_DEQ_REMOVE: u8 = 3;
pub(crate) const TNS_AQ_DEQ_REMOVE_NODATA: u8 = 4;

/// AQ dequeue navigation modes
pub(crate) const TNS_AQ_DEQ_FIRST_MSG: u8 = 1;
pub(crate) const TNS_AQ_DEQ_NEXT_MSG: u8 = 3;
pub(crate) const TNS_AQ_DEQ_NEXT_TRANSACTION: u8 = 2;

/// AQ dequeue visibility modes
pub(crate) const TNS_AQ_DEQ_IMMEDIATE: u8 = 1;
pub(crate) const TNS_AQ_DEQ_ON_COMMIT: u8 = 2;

/// AQ dequeue wait modes
pub(crate) const TNS_AQ_DEQ_NO_WAIT: u32 = 0;
pub(crate) const TNS_AQ_DEQ_WAIT_FOREVER: u32 = u32::MAX - 1;

/// AQ enqueue visibility modes
pub(crate) const TNS_AQ_ENQ_IMMEDIATE: u8 = 1;
pub(crate) const TNS_AQ_ENQ_ON_COMMIT: u8 = 2;

/// AQ message states
pub(crate) const TNS_AQ_MSG_EXPIRED: u8 = 3;
pub(crate) const TNS_AQ_MSG_PROCESSED: u8 = 2;
pub(crate) const TNS_AQ_MSG_READY: u8 = 0;
pub(crate) const TNS_AQ_MSG_WAITING: u8 = 1;

/// AQ other constants
pub(crate) const TNS_AQ_MSG_NO_DELAY: i32 = 0;
pub(crate) const TNS_AQ_MSG_NO_EXPIRATION: i32 = -1;
pub(crate) const TNS_AQ_ARRAY_ENQ: i32 = 0x01;
pub(crate) const TNS_AQ_ARRAY_DEQ: i32 = 0x02;
pub(crate) const TNS_AQ_ARRAY_FLAGS_RETURN_MESSAGE_ID: i32 = 0x01;
pub(crate) const TNS_TTC_ENQ_STREAMING_ENABLED: i32 = 0x00000001;
pub(crate) const TNS_TTC_ENQ_STREAMING_DISABLED: i32 = 0x00000000;

/// AQ flags
pub(crate) const TNS_KPD_AQ_BUFMSG: u8 = 0x02;
pub(crate) const TNS_KPD_AQ_EITHER: u8 = 0x10;

/// errors
pub(crate) const TNS_ERR_INCONSISTENT_DATA_TYPES: u16 = 932;
pub(crate) const TNS_ERR_VAR_NOT_IN_SELECT_LIST: u16 = 1007;
pub(crate) const TNS_ERR_INBAND_MESSAGE: u16 = 12573;
pub(crate) const TNS_ERR_INVALID_SERVICE_NAME: u16 = 12514;
pub(crate) const TNS_ERR_INVALID_SID: u16 = 12505;
pub(crate) const TNS_ERR_NO_DATA_FOUND: u16 = 1403;
pub(crate) const TNS_ERR_SESSION_SHUTDOWN: u16 = 12572;
pub(crate) const TNS_ERR_ARRAY_DML_ERRORS: u16 = 24381;
pub(crate) const TNS_ERR_EXCEEDED_IDLE_TIME: u16 = 2396;
pub(crate) const TNS_ERR_NO_MESSAGES_FOUND: u16 = 25228;

/// message types
pub(crate) const TNS_MSG_TYPE_PROTOCOL: u8 = 1;
pub(crate) const TNS_MSG_TYPE_DATA_TYPES: u8 = 2;
pub(crate) const TNS_MSG_TYPE_FUNCTION: u8 = 3;
pub(crate) const TNS_MSG_TYPE_ERROR: u8 = 4;
pub(crate) const TNS_MSG_TYPE_ROW_HEADER: u8 = 6;
pub(crate) const TNS_MSG_TYPE_ROW_DATA: u8 = 7;
pub(crate) const TNS_MSG_TYPE_PARAMETER: u8 = 8;
pub(crate) const TNS_MSG_TYPE_STATUS: u8 = 9;
pub(crate) const TNS_MSG_TYPE_IO_VECTOR: u8 = 11;
pub(crate) const TNS_MSG_TYPE_LOB_DATA: u8 = 14;
pub(crate) const TNS_MSG_TYPE_WARNING: u8 = 15;
pub(crate) const TNS_MSG_TYPE_DESCRIBE_INFO: u8 = 16;
pub(crate) const TNS_MSG_TYPE_PIGGYBACK: u8 = 17;
pub(crate) const TNS_MSG_TYPE_FLUSH_OUT_BINDS: u8 = 19;
pub(crate) const TNS_MSG_TYPE_BIT_VECTOR: u8 = 21;
pub(crate) const TNS_MSG_TYPE_SERVER_SIDE_PIGGYBACK: u8 = 23;
pub(crate) const TNS_MSG_TYPE_ONEWAY_FN: u8 = 26;
pub(crate) const TNS_MSG_TYPE_IMPLICIT_RESULTSET: u8 = 27;
pub(crate) const TNS_MSG_TYPE_RENEGOTIATE: u8 = 28;
pub(crate) const TNS_MSG_TYPE_END_OF_RESPONSE: u8 = 29;
pub(crate) const TNS_MSG_TYPE_TOKEN: u8 = 33;
pub(crate) const TNS_MSG_TYPE_FAST_AUTH: u8 = 34;

/// parameter keyword numbers
pub(crate) const TNS_KEYWORD_NUM_CURRENT_SCHEMA: u8 = 168;
pub(crate) const TNS_KEYWORD_NUM_EDITION: u8 = 172;

/// bind flags
pub(crate) const TNS_BIND_USE_INDICATORS: u16 = 0x0001;
pub(crate) const TNS_BIND_ARRAY: u16 = 0x0040;

/// bind directions
pub(crate) const TNS_BIND_DIR_OUTPUT: u8 = 16;
pub(crate) const TNS_BIND_DIR_INPUT: u8 = 32;
pub(crate) const TNS_BIND_DIR_INPUT_OUTPUT: u8 = 48;

/// database object image flags
pub(crate) const TNS_OBJ_IS_VERSION_81: u8 = 0x80;
pub(crate) const TNS_OBJ_IS_DEGENERATE: u8 = 0x10;
pub(crate) const TNS_OBJ_IS_COLLECTION: u8 = 0x08;
pub(crate) const TNS_OBJ_NO_PREFIX_SEG: u8 = 0x04;
pub(crate) const TNS_OBJ_IMAGE_VERSION: u8 = 1;

/// database object flags
pub(crate) const TNS_OBJ_MAX_SHORT_LENGTH: u8 = 245;
pub(crate) const TNS_OBJ_ATOMIC_NULL: u8 = 253;
pub(crate) const TNS_OBJ_NON_NULL_OID: u8 = 0x02;
pub(crate) const TNS_OBJ_HAS_EXTENT_OID: u8 = 0x08;
pub(crate) const TNS_OBJ_TOP_LEVEL: u8 = 0x01;
pub(crate) const TNS_OBJ_HAS_INDEXES: u8 = 0x10;

/// database object collection types
pub(crate) const TNS_OBJ_PLSQL_INDEX_TABLE: u8 = 1;
pub(crate) const TNS_OBJ_NESTED_TABLE: u8 = 2;
pub(crate) const TNS_OBJ_VARRAY: u8 = 3;

/// database object TDS type codes
pub(crate) const TNS_OBJ_TDS_TYPE_CHAR: u8 = 1;
pub(crate) const TNS_OBJ_TDS_TYPE_DATE: u8 = 2;
pub(crate) const TNS_OBJ_TDS_TYPE_FLOAT: u8 = 5;
pub(crate) const TNS_OBJ_TDS_TYPE_NUMBER: u8 = 6;
pub(crate) const TNS_OBJ_TDS_TYPE_VARCHAR: u8 = 7;
pub(crate) const TNS_OBJ_TDS_TYPE_BOOLEAN: u8 = 8;
pub(crate) const TNS_OBJ_TDS_TYPE_RAW: u8 = 19;
pub(crate) const TNS_OBJ_TDS_TYPE_TIMESTAMP: u8 = 21;
pub(crate) const TNS_OBJ_TDS_TYPE_TIMESTAMP_TZ: u8 = 23;
pub(crate) const TNS_OBJ_TDS_TYPE_OBJ: u8 = 27;
pub(crate) const TNS_OBJ_TDS_TYPE_COLL: u8 = 28;
pub(crate) const TNS_OBJ_TDS_TYPE_CLOB: u8 = 29;
pub(crate) const TNS_OBJ_TDS_TYPE_BLOB: u8 = 30;
pub(crate) const TNS_OBJ_TDS_TYPE_TIMESTAMP_LTZ: u8 = 33;
pub(crate) const TNS_OBJ_TDS_TYPE_BINARY_FLOAT: u8 = 37;
pub(crate) const TNS_OBJ_TDS_TYPE_START_EMBED_ADT: u8 = 39;
pub(crate) const TNS_OBJ_TDS_TYPE_END_EMBED_ADT: u8 = 40;
pub(crate) const TNS_OBJ_TDS_TYPE_SUBTYPE_MARKER: u8 = 43;
pub(crate) const TNS_OBJ_TDS_TYPE_EMBED_ADT_INFO: u8 = 44;
pub(crate) const TNS_OBJ_TDS_TYPE_BINARY_DOUBLE: u8 = 45;

/// xml type constants
pub(crate) const TNS_XML_TYPE_LOB: u32 = 0x0001;
pub(crate) const TNS_XML_TYPE_STRING: u32 = 0x0004;
pub(crate) const TNS_XML_TYPE_FLAG_SKIP_NEXT_4: u32 = 0x100000;

/// execute options
pub(crate) const TNS_EXEC_OPTION_PARSE: u32 = 0x01;
pub(crate) const TNS_EXEC_OPTION_BIND: u32 = 0x08;
pub(crate) const TNS_EXEC_OPTION_DEFINE: u32 = 0x10;
pub(crate) const TNS_EXEC_OPTION_EXECUTE: u32 = 0x20;
pub(crate) const TNS_EXEC_OPTION_FETCH: u32 = 0x40;
pub(crate) const TNS_EXEC_OPTION_COMMIT: u32 = 0x100;
pub(crate) const TNS_EXEC_OPTION_COMMIT_REEXECUTE: u32 = 0x1;
pub(crate) const TNS_EXEC_OPTION_PLSQL_BIND: u32 = 0x400;
pub(crate) const TNS_EXEC_OPTION_NOT_PLSQL: u32 = 0x8000;
pub(crate) const TNS_EXEC_OPTION_DESCRIBE: u32 = 0x20000;
pub(crate) const TNS_EXEC_OPTION_NO_COMPRESSED_FETCH: u32 = 0x40000;
pub(crate) const TNS_EXEC_OPTION_BATCH_ERRORS: u32 = 0x80000;

/// execute flags
pub(crate) const TNS_EXEC_FLAGS_DML_ROWCOUNTS: u16 = 0x4000;
pub(crate) const TNS_EXEC_FLAGS_IMPLICIT_RESULTSET: u16 = 0x8000;
pub(crate) const TNS_EXEC_FLAGS_SCROLLABLE: u16 = 0x02;

/// fetch orientations
pub(crate) const TNS_FETCH_ORIENTATION_ABSOLUTE: u8 = 0x20;
pub(crate) const TNS_FETCH_ORIENTATION_CURRENT: u8 = 0x01;
pub(crate) const TNS_FETCH_ORIENTATION_FIRST: u8 = 0x04;
pub(crate) const TNS_FETCH_ORIENTATION_LAST: u8 = 0x08;
pub(crate) const TNS_FETCH_ORIENTATION_NEXT: u8 = 0x02;
pub(crate) const TNS_FETCH_ORIENTATION_PRIOR: u8 = 0x10;
pub(crate) const TNS_FETCH_ORIENTATION_RELATIVE: u8 = 0x40;

/// server side piggyback op codes
pub(crate) const TNS_SERVER_PIGGYBACK_QUERY_CACHE_INVALIDATION: u8 = 1;
pub(crate) const TNS_SERVER_PIGGYBACK_OS_PID_MTS: u8 = 2;
pub(crate) const TNS_SERVER_PIGGYBACK_TRACE_EVENT: u8 = 3;
pub(crate) const TNS_SERVER_PIGGYBACK_SESS_RET: u8 = 4;
pub(crate) const TNS_SERVER_PIGGYBACK_SYNC: u8 = 5;
pub(crate) const TNS_SERVER_PIGGYBACK_LTXID: u8 = 7;
pub(crate) const TNS_SERVER_PIGGYBACK_AC_REPLAY_CONTEXT: u8 = 8;
pub(crate) const TNS_SERVER_PIGGYBACK_EXT_SYNC: u8 = 9;
pub(crate) const TNS_SERVER_PIGGYBACK_SESS_SIGNATURE: u8 = 10;

/// session return constants
pub(crate) const TNS_SESSGET_SESSION_CHANGED: u8 = 4;

/// LOB operations
pub(crate) const TNS_LOB_OP_GET_LENGTH: u32 = 0x0001;
pub(crate) const TNS_LOB_OP_READ: u32 = 0x0002;
pub(crate) const TNS_LOB_OP_TRIM: u32 = 0x0020;
pub(crate) const TNS_LOB_OP_WRITE: u32 = 0x0040;
pub(crate) const TNS_LOB_OP_GET_CHUNK_SIZE: u32 = 0x4000;
pub(crate) const TNS_LOB_OP_CREATE_TEMP: u32 = 0x0110;
pub(crate) const TNS_LOB_OP_FREE_TEMP: u32 = 0x0111;
pub(crate) const TNS_LOB_OP_OPEN: u32 = 0x8000;
pub(crate) const TNS_LOB_OP_CLOSE: u32 = 0x10000;
pub(crate) const TNS_LOB_OP_IS_OPEN: u32 = 0x11000;
pub(crate) const TNS_LOB_OP_ARRAY: u32 = 0x80000;
pub(crate) const TNS_LOB_OP_FILE_EXISTS: u32 = 0x0800;
pub(crate) const TNS_LOB_OP_FILE_OPEN: u32 = 0x0100;
pub(crate) const TNS_LOB_OP_FILE_CLOSE: u32 = 0x0200;
pub(crate) const TNS_LOB_OP_FILE_ISOPEN: u32 = 0x0400;

/// LOB locator constants
pub(crate) const TNS_LOB_LOC_OFFSET_FLAG_1: u8 = 4;
pub(crate) const TNS_LOB_LOC_OFFSET_FLAG_3: u8 = 6;
pub(crate) const TNS_LOB_LOC_OFFSET_FLAG_4: u8 = 7;
pub(crate) const TNS_LOB_QLOCATOR_VERSION: u8 = 4;
pub(crate) const TNS_LOB_LOC_FIXED_OFFSET: u8 = 16;

/// LOB locator flags (byte 1)
pub(crate) const TNS_LOB_LOC_FLAGS_BLOB: u8 = 0x01;
pub(crate) const TNS_LOB_LOC_FLAGS_VALUE_BASED: u8 = 0x20;
pub(crate) const TNS_LOB_LOC_FLAGS_ABSTRACT: u8 = 0x40;

/// LOB locator flags (byte 2)
pub(crate) const TNS_LOB_LOC_FLAGS_INIT: u8 = 0x08;

/// LOB locator flags (byte 4)
pub(crate) const TNS_LOB_LOC_FLAGS_TEMP: u8 = 0x01;
pub(crate) const TNS_LOB_LOC_FLAGS_VAR_LENGTH_CHARSET: u8 = 0x80;

/// other LOB constants
pub(crate) const TNS_LOB_OPEN_READ_WRITE: u32 = 2;
pub(crate) const TNS_LOB_OPEN_READ_ONLY: u32 = 11;
pub(crate) const TNS_LOB_PREFETCH_FLAG: u32 = 0x2000000;

/// end-to-end metrics
pub(crate) const TNS_END_TO_END_ACTION: u32 = 0x0010;
pub(crate) const TNS_END_TO_END_CLIENT_IDENTIFIER: u32 = 0x0001;
pub(crate) const TNS_END_TO_END_CLIENT_INFO: u32 = 0x0100;
pub(crate) const TNS_END_TO_END_DBOP: u32 = 0x0200;
pub(crate) const TNS_END_TO_END_MODULE: u32 = 0x0008;

/// versions
pub(crate) const TNS_VERSION_DESIRED: u16 = 319;
pub(crate) const TNS_VERSION_MINIMUM: u16 = 300;
pub(crate) const TNS_VERSION_MIN_ACCEPTED: u16 = 315;
pub(crate) const TNS_VERSION_MIN_LARGE_SDU: u16 = 315;
pub(crate) const TNS_VERSION_MIN_OOB_CHECK: u16 = 318;
pub(crate) const TNS_VERSION_MIN_END_OF_RESPONSE: u16 = 319;

/// control packet types
pub(crate) const TNS_CONTROL_TYPE_INBAND_NOTIFICATION: u16 = 8;
pub(crate) const TNS_CONTROL_TYPE_RESET_OOB: u16 = 9;

/// connect flags
pub(crate) const TNS_GSO_DONT_CARE: u16 = 0x0001;
pub(crate) const TNS_GSO_CAN_RECV_ATTENTION: u16 = 0x0400;
pub(crate) const TNS_NSI_NA_REQUIRED: u16 = 0x10;
pub(crate) const TNS_NSI_DISABLE_NA: u16 = 0x04;
pub(crate) const TNS_NSI_SUPPORT_SECURITY_RENEG: u16 = 0x80;

/// other connection constants
pub(crate) const TNS_PROTOCOL_CHARACTERISTICS: u16 = 0x4f98;
pub(crate) const TNS_CHECK_OOB: u16 = 0x01;

/// TTC functions
pub(crate) const TNS_FUNC_AUTH_PHASE_ONE: u8 = 118;
pub(crate) const TNS_FUNC_AUTH_PHASE_TWO: u8 = 115;
pub(crate) const TNS_FUNC_CLOSE_CURSORS: u8 = 105;
pub(crate) const TNS_FUNC_COMMIT: u8 = 14;
pub(crate) const TNS_FUNC_EXECUTE: u8 = 94;
pub(crate) const TNS_FUNC_FETCH: u8 = 5;
pub(crate) const TNS_FUNC_LOB_OP: u8 = 96;
pub(crate) const TNS_FUNC_AQ_ENQ: u8 = 121;
pub(crate) const TNS_FUNC_AQ_DEQ: u8 = 122;
pub(crate) const TNS_FUNC_ARRAY_AQ: u8 = 145;
pub(crate) const TNS_FUNC_LOGOFF: u8 = 9;
pub(crate) const TNS_FUNC_PING: u8 = 147;
pub(crate) const TNS_FUNC_PIPELINE_BEGIN: u8 = 199;
pub(crate) const TNS_FUNC_PIPELINE_END: u8 = 200;
pub(crate) const TNS_FUNC_ROLLBACK: u8 = 15;
pub(crate) const TNS_FUNC_SET_END_TO_END_ATTR: u8 = 135;
pub(crate) const TNS_FUNC_REEXECUTE: u8 = 4;
pub(crate) const TNS_FUNC_REEXECUTE_AND_FETCH: u8 = 78;
pub(crate) const TNS_FUNC_SESSION_GET: u8 = 162;
pub(crate) const TNS_FUNC_SESSION_RELEASE: u8 = 163;
pub(crate) const TNS_FUNC_SESSION_STATE: u8 = 176;
pub(crate) const TNS_FUNC_SET_SCHEMA: u8 = 152;
pub(crate) const TNS_FUNC_TPC_TXN_SWITCH: u8 = 103;
pub(crate) const TNS_FUNC_TPC_TXN_CHANGE_STATE: u8 = 104;

/// TTC authentication modes
pub(crate) const TNS_AUTH_MODE_LOGON: u32 = 0x00000001;
pub(crate) const TNS_AUTH_MODE_CHANGE_PASSWORD: u32 = 0x00000002;
pub(crate) const TNS_AUTH_MODE_SYSDBA: u32 = 0x00000020;
pub(crate) const TNS_AUTH_MODE_SYSOPER: u32 = 0x00000040;
pub(crate) const TNS_AUTH_MODE_WITH_PASSWORD: u32 = 0x00000100;
pub(crate) const TNS_AUTH_MODE_SYSASM: u32 = 0x00400000;
pub(crate) const TNS_AUTH_MODE_SYSBKP: u32 = 0x01000000;
pub(crate) const TNS_AUTH_MODE_SYSDGD: u32 = 0x02000000;
pub(crate) const TNS_AUTH_MODE_SYSKMT: u32 = 0x04000000;
pub(crate) const TNS_AUTH_MODE_SYSRAC: u32 = 0x08000000;
pub(crate) const TNS_AUTH_MODE_IAM_TOKEN: u32 = 0x20000000;

/// character sets and encodings
pub(crate) const TNS_CHARSET_UTF8: u16 = 873;
pub(crate) const TNS_CHARSET_UTF16: u16 = 2000;
pub(crate) const TNS_ENCODING_MULTI_BYTE: u16 = 0x01;
pub(crate) const TNS_ENCODING_CONV_LENGTH: u16 = 0x02;

/// compile time capability indices
pub(crate) const TNS_CCAP_SQL_VERSION: u8 = 0;
pub(crate) const TNS_CCAP_LOGON_TYPES: u8 = 4;
pub(crate) const TNS_CCAP_FEATURE_BACKPORT: u8 = 5;
pub(crate) const TNS_CCAP_FIELD_VERSION: u8 = 7;
pub(crate) const TNS_CCAP_SERVER_DEFINE_CONV: u8 = 8;
pub(crate) const TNS_CCAP_DEQUEUE_WITH_SELECTOR: u8 = 9;
pub(crate) const TNS_CCAP_TTC1: u8 = 15;
pub(crate) const TNS_CCAP_OCI1: u8 = 16;
pub(crate) const TNS_CCAP_TDS_VERSION: u8 = 17;
pub(crate) const TNS_CCAP_RPC_VERSION: u8 = 18;
pub(crate) const TNS_CCAP_RPC_SIG: u8 = 19;
pub(crate) const TNS_CCAP_DBF_VERSION: u8 = 21;
pub(crate) const TNS_CCAP_LOB: u8 = 23;
pub(crate) const TNS_CCAP_TTC2: u8 = 26;
pub(crate) const TNS_CCAP_UB2_DTY: u8 = 27;
pub(crate) const TNS_CCAP_OCI2: u8 = 31;
pub(crate) const TNS_CCAP_CLIENT_FN: u8 = 34;
pub(crate) const TNS_CCAP_TTC3: u8 = 37;
pub(crate) const TNS_CCAP_SESS_SIGNATURE_VERSION: u8 = 39;
pub(crate) const TNS_CCAP_TTC4: u8 = 40;
pub(crate) const TNS_CCAP_LOB2: u8 = 42;
pub(crate) const TNS_CCAP_TTC5: u8 = 44;
pub(crate) const TNS_CCAP_VECTOR_FEATURES: u8 = 52;
pub(crate) const TNS_CCAP_MAX: u8 = 53;

/// compile time capability values
pub(crate) const TNS_CCAP_SQL_VERSION_MAX: u8 = 6;
pub(crate) const TNS_CCAP_FIELD_VERSION_11_2: u8 = 6;
pub(crate) const TNS_CCAP_FIELD_VERSION_12_1: u8 = 7;
pub(crate) const TNS_CCAP_FIELD_VERSION_12_2: u8 = 8;
pub(crate) const TNS_CCAP_FIELD_VERSION_12_2_EXT1: u8 = 9;
pub(crate) const TNS_CCAP_FIELD_VERSION_18_1: u8 = 10;
pub(crate) const TNS_CCAP_FIELD_VERSION_18_1_EXT_1: u8 = 11;
pub(crate) const TNS_CCAP_FIELD_VERSION_19_1: u8 = 12;
pub(crate) const TNS_CCAP_FIELD_VERSION_19_1_EXT_1: u8 = 13;
pub(crate) const TNS_CCAP_FIELD_VERSION_20_1: u8 = 14;
pub(crate) const TNS_CCAP_FIELD_VERSION_20_1_EXT_1: u8 = 15;
pub(crate) const TNS_CCAP_FIELD_VERSION_21_1: u8 = 16;
pub(crate) const TNS_CCAP_FIELD_VERSION_23_1: u8 = 17;
pub(crate) const TNS_CCAP_FIELD_VERSION_23_1_EXT_1: u8 = 18;
pub(crate) const TNS_CCAP_FIELD_VERSION_23_1_EXT_2: u8 = 19;
pub(crate) const TNS_CCAP_FIELD_VERSION_23_1_EXT_3: u8 = 20;
pub(crate) const TNS_CCAP_FIELD_VERSION_23_1_EXT_4: u8 = 21;
pub(crate) const TNS_CCAP_FIELD_VERSION_23_1_EXT_5: u8 = 22;
pub(crate) const TNS_CCAP_FIELD_VERSION_23_3_EXT_6: u8 = 23;
pub(crate) const TNS_CCAP_FIELD_VERSION_23_4: u8 = 24;
pub(crate) const TNS_CCAP_FIELD_VERSION_MAX: u8 = 24;
pub(crate) const TNS_CCAP_O5LOGON: u8 = 8;
pub(crate) const TNS_CCAP_O5LOGON_NP: u8 = 2;
pub(crate) const TNS_CCAP_O7LOGON: u8 = 32;
pub(crate) const TNS_CCAP_O8LOGON_LONG_IDENTIFIER: u8 = 64;
pub(crate) const TNS_CCAP_O9LOGON_LONG_PASSWORD: u8 = 0x80;
pub(crate) const TNS_CCAP_CTB_IMPLICIT_POOL: u8 = 0x08;
pub(crate) const TNS_CCAP_END_OF_CALL_STATUS: u8 = 0x01;
pub(crate) const TNS_CCAP_IND_RCD: u8 = 0x08;
pub(crate) const TNS_CCAP_FAST_BVEC: u8 = 0x20;
pub(crate) const TNS_CCAP_FAST_SESSION_PROPAGATE: u8 = 0x10;
pub(crate) const TNS_CCAP_APP_CTX_PIGGYBACK: u8 = 0x80;
pub(crate) const TNS_CCAP_TDS_VERSION_MAX: u8 = 3;
pub(crate) const TNS_CCAP_RPC_VERSION_MAX: u8 = 7;
pub(crate) const TNS_CCAP_RPC_SIG_VALUE: u8 = 3;
pub(crate) const TNS_CCAP_DBF_VERSION_MAX: u8 = 1;
pub(crate) const TNS_CCAP_LTXID: u8 = 0x08;
pub(crate) const TNS_CCAP_IMPLICIT_RESULTS: u8 = 0x10;
pub(crate) const TNS_CCAP_BIG_CHUNK_CLR: u8 = 0x20;
pub(crate) const TNS_CCAP_KEEP_OUT_ORDER: u8 = 0x80;
pub(crate) const TNS_CCAP_LOB_UB8_SIZE: u8 = 0x01;
pub(crate) const TNS_CCAP_LOB_ENCS: u8 = 0x02;
pub(crate) const TNS_CCAP_LOB_PREFETCH_DATA: u8 = 0x04;
pub(crate) const TNS_CCAP_LOB_TEMP_SIZE: u8 = 0x08;
pub(crate) const TNS_CCAP_LOB_PREFETCH_LENGTH: u8 = 0x40;
pub(crate) const TNS_CCAP_LOB_12C: u8 = 0x80;
pub(crate) const TNS_CCAP_LOB2_QUASI: u8 = 0x01;
pub(crate) const TNS_CCAP_LOB2_2GB_PREFETCH: u8 = 0x04;
pub(crate) const TNS_CCAP_DRCP: u8 = 0x10;
pub(crate) const TNS_CCAP_ZLNP: u8 = 0x04;
pub(crate) const TNS_CCAP_INBAND_NOTIFICATION: u8 = 0x04;
pub(crate) const TNS_CCAP_EXPLICIT_BOUNDARY: u8 = 0x40;
pub(crate) const TNS_CCAP_END_OF_RESPONSE: u8 = 0x20;
pub(crate) const TNS_CCAP_CLIENT_FN_MAX: u8 = 12;
pub(crate) const TNS_CCAP_VECTOR_SUPPORT: u8 = 0x08;
pub(crate) const TNS_CCAP_TOKEN_SUPPORTED: u8 = 0x02;
pub(crate) const TNS_CCAP_PIPELINING_SUPPORT: u8 = 0x04;
pub(crate) const TNS_CCAP_PIPELINING_BREAK: u8 = 0x10;
pub(crate) const TNS_CCAP_VECTOR_FEATURE_BINARY: u8 = 0x01;
pub(crate) const TNS_CCAP_VECTOR_FEATURE_SPARSE: u8 = 0x02;

/// runtime capability indices
pub(crate) const TNS_RCAP_COMPAT: u8 = 0;
pub(crate) const TNS_RCAP_TTC: u8 = 6;
pub(crate) const TNS_RCAP_MAX: u8 = 11;

/// runtime capability values
pub(crate) const TNS_RCAP_COMPAT_81: u8 = 2;
pub(crate) const TNS_RCAP_TTC_ZERO_COPY: u8 = 0x01;
pub(crate) const TNS_RCAP_TTC_32K: u8 = 0x04;
pub(crate) const TNS_RCAP_TTC_SESSION_STATE_OPS: u8 = 0x10;

/// verifier types
pub(crate) const TNS_VERIFIER_TYPE_11G_1: u16 = 0xb152;
pub(crate) const TNS_VERIFIER_TYPE_11G_2: u16 = 0x1b25;
pub(crate) const TNS_VERIFIER_TYPE_12C: u16 = 0x4815;

/// UDS flags
pub(crate) const TNS_UDS_FLAGS_IS_JSON: u32 = 0x00000100;
pub(crate) const TNS_UDS_FLAGS_IS_OSON: u32 = 0x00000800;

/// end of call status flags
pub(crate) const TNS_EOCS_FLAGS_TXN_IN_PROGRESS: u32 = 0x00000002;
pub(crate) const TNS_EOCS_FLAGS_SESS_RELEASE: u32 = 0x00008000;

/// accept flags
pub(crate) const TNS_ACCEPT_FLAG_CHECK_OOB: u32 = 0x00000001;
pub(crate) const TNS_ACCEPT_FLAG_FAST_AUTH: u32 = 0x10000000;
pub(crate) const TNS_ACCEPT_FLAG_HAS_END_OF_RESPONSE: u32 = 0x02000000;

/// transaction switching op codes
pub(crate) const TNS_TPC_TXN_START: u8 = 0x01;
pub(crate) const TNS_TPC_TXN_DETACH: u8 = 0x02;

/// transaction change state op codes
pub(crate) const TNS_TPC_TXN_COMMIT: u8 = 0x01;
pub(crate) const TNS_TPC_TXN_ABORT: u8 = 0x02;
pub(crate) const TNS_TPC_TXN_PREPARE: u8 = 0x03;
pub(crate) const TNS_TPC_TXN_FORGET: u8 = 0x04;

/// transaction states
pub(crate) const TNS_TPC_TXN_STATE_PREPARE: u8 = 0;
pub(crate) const TNS_TPC_TXN_STATE_REQUIRES_COMMIT: u8 = 1;
pub(crate) const TNS_TPC_TXN_STATE_COMMITTED: u8 = 2;
pub(crate) const TNS_TPC_TXN_STATE_ABORTED: u8 = 3;
pub(crate) const TNS_TPC_TXN_STATE_READ_ONLY: u8 = 4;
pub(crate) const TNS_TPC_TXN_STATE_FORGOTTEN: u8 = 5;

/// pipeline modes
pub(crate) const TNS_PIPELINE_MODE_CONTINUE_ON_ERROR: u8 = 1;
pub(crate) const TNS_PIPELINE_MODE_ABORT_ON_ERROR: u8 = 2;

/// AQ extension keywords
pub(crate) const TNS_AQ_EXT_KEYWORD_AGENT_NAME: u8 = 64;
pub(crate) const TNS_AQ_EXT_KEYWORD_AGENT_ADDRESS: u8 = 65;
pub(crate) const TNS_AQ_EXT_KEYWORD_AGENT_PROTOCOL: u8 = 66;
pub(crate) const TNS_AQ_EXT_KEYWORD_ORIGINAL_MSGID: u8 = 69;

/// session state flags
pub(crate) const TNS_SESSION_STATE_REQUEST_BEGIN: u8 = 0x04;
pub(crate) const TNS_SESSION_STATE_REQUEST_END: u8 = 0x08;
pub(crate) const TNS_SESSION_STATE_EXPLICIT_BOUNDARY: u8 = 0x40;

/// other constants
pub(crate) const TNS_ESCAPE_CHAR: u32 = 253;
pub(crate) const TNS_MAX_ROWID_LENGTH: u32 = 18;
pub(crate) const TNS_DURATION_SESSION: u32 = 10;
pub(crate) const TNS_MAX_LONG_LENGTH: u32 = 0x7fffffff;
pub(crate) const TNS_MAX_CONNECT_DATA: u32 = 230;
pub(crate) const TNS_MAX_UROWID_LENGTH: u32 = 5267;
pub(crate) const TNS_SERVER_CONVERTS_CHARS: u32 = 0x01;
pub(crate) const TNS_JSON_MAX_LENGTH: u32 = 32 * 1024 * 1024;
pub(crate) const TNS_VECTOR_MAX_LENGTH: u32 = 1 * 1024 * 1024;
pub(crate) const TNS_AQ_MESSAGE_ID_LENGTH: u32 = 16;
pub(crate) const TNS_AQ_MESSAGE_VERSION: u32 = 1;

/// base 64 encoding alphabet
pub(crate) const TNS_BASE64_ALPHABET: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub(crate) const TNS_BASE64_ALPHABET_ARRAY: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // todo!()
pub(crate) const TNS_EXTENT_OID: &[u8] = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1];

/// drcp release mode
pub(crate) const DRCP_DEAUTHENTICATE: u32 = 0x00000002;

pub(crate) const PY_TYPE_NUM_ARRAY: u8 = 13;
pub(crate) const PY_TYPE_NUM_BOOL: u8 = 4;
pub(crate) const PY_TYPE_NUM_BYTES: u8 = 10;
pub(crate) const PY_TYPE_NUM_DATETIME: u8 = 7;
pub(crate) const PY_TYPE_NUM_DECIMAL: u8 = 14;
pub(crate) const PY_TYPE_NUM_FLOAT: u8 = 2;
pub(crate) const PY_TYPE_NUM_INT: u8 = 3;
pub(crate) const PY_TYPE_NUM_OBJECT: u8 = 9;
pub(crate) const PY_TYPE_NUM_ORACLE_CURSOR: u8 = 6;
pub(crate) const PY_TYPE_NUM_ORACLE_INTERVAL_YM: u8 = 12;
pub(crate) const PY_TYPE_NUM_ORACLE_LOB: u8 = 1;
pub(crate) const PY_TYPE_NUM_ORACLE_OBJECT: u8 = 11;
pub(crate) const PY_TYPE_NUM_STR: u8 = 5;
pub(crate) const PY_TYPE_NUM_TIMEDELTA: u8 = 8;

pub(crate) const DB_TYPE_NUM_MIN: u32 = 2000;
pub(crate) const DB_TYPE_NUM_MAX: u32 = 2034;
pub(crate) const DB_TYPE_NUM_BFILE: u32 = 2020;
pub(crate) const DB_TYPE_NUM_BINARY_DOUBLE: u32 = 2008;
pub(crate) const DB_TYPE_NUM_BINARY_FLOAT: u32 = 2007;
pub(crate) const DB_TYPE_NUM_BINARY_INTEGER: u32 = 2009;
pub(crate) const DB_TYPE_NUM_BLOB: u32 = 2019;
pub(crate) const DB_TYPE_NUM_BOOLEAN: u32 = 2022;
pub(crate) const DB_TYPE_NUM_CHAR: u32 = 2003;
pub(crate) const DB_TYPE_NUM_CLOB: u32 = 2017;
pub(crate) const DB_TYPE_NUM_CURSOR: u32 = 2021;
pub(crate) const DB_TYPE_NUM_DATE: u32 = 2011;
pub(crate) const DB_TYPE_NUM_INTERVAL_DS: u32 = 2015;
pub(crate) const DB_TYPE_NUM_INTERVAL_YM: u32 = 2016;
pub(crate) const DB_TYPE_NUM_JSON: u32 = 2027;
pub(crate) const DB_TYPE_NUM_LONG_NVARCHAR: u32 = 2031;
pub(crate) const DB_TYPE_NUM_LONG_RAW: u32 = 2025;
pub(crate) const DB_TYPE_NUM_LONG_VARCHAR: u32 = 2024;
pub(crate) const DB_TYPE_NUM_NCHAR: u32 = 2004;
pub(crate) const DB_TYPE_NUM_NCLOB: u32 = 2018;
pub(crate) const DB_TYPE_NUM_NUMBER: u32 = 2010;
pub(crate) const DB_TYPE_NUM_NVARCHAR: u32 = 2002;
pub(crate) const DB_TYPE_NUM_OBJECT: u32 = 2023;
pub(crate) const DB_TYPE_NUM_RAW: u32 = 2006;
pub(crate) const DB_TYPE_NUM_ROWID: u32 = 2005;
pub(crate) const DB_TYPE_NUM_TIMESTAMP: u32 = 2012;
pub(crate) const DB_TYPE_NUM_TIMESTAMP_LTZ: u32 = 2014;
pub(crate) const DB_TYPE_NUM_TIMESTAMP_TZ: u32 = 2013;
pub(crate) const DB_TYPE_NUM_UNKNOWN: u32 = 0;
pub(crate) const DB_TYPE_NUM_UROWID: u32 = 2030;
pub(crate) const DB_TYPE_NUM_VARCHAR: u32 = 2001;
pub(crate) const DB_TYPE_NUM_VECTOR: u32 = 2033;
pub(crate) const DB_TYPE_NUM_XMLTYPE: u32 = 2032;

pub(crate) const NATIVE_TYPE_NUM_BOOLEAN: u32 = 3011;
pub(crate) const NATIVE_TYPE_NUM_BYTES: u32 = 3004;
pub(crate) const NATIVE_TYPE_NUM_DOUBLE: u32 = 3003;
pub(crate) const NATIVE_TYPE_NUM_FLOAT: u32 = 3002;
pub(crate) const NATIVE_TYPE_NUM_INTERVAL_DS: u32 = 3006;
pub(crate) const NATIVE_TYPE_NUM_INTERVAL_YM: u32 = 3007;
pub(crate) const NATIVE_TYPE_NUM_INT64: u32 = 3000;
pub(crate) const NATIVE_TYPE_NUM_JSON: u32 = 3013;
pub(crate) const NATIVE_TYPE_NUM_LOB: u32 = 3008;
pub(crate) const NATIVE_TYPE_NUM_OBJECT: u32 = 3009;
pub(crate) const NATIVE_TYPE_NUM_ROWID: u32 = 3012;
pub(crate) const NATIVE_TYPE_NUM_STMT: u32 = 3010;
pub(crate) const NATIVE_TYPE_NUM_TIMESTAMP: u32 = 3005;
pub(crate) const NATIVE_TYPE_NUM_VECTOR: u32 = 3017;

pub(crate) const ORA_TYPE_NUM_BFILE: u8 = 114;
pub(crate) const ORA_TYPE_NUM_BINARY_DOUBLE: u8 = 101;
pub(crate) const ORA_TYPE_NUM_BINARY_FLOAT: u8 = 100;
pub(crate) const ORA_TYPE_NUM_BINARY_INTEGER: u8 = 3;
pub(crate) const ORA_TYPE_NUM_BLOB: u8 = 113;
pub(crate) const ORA_TYPE_NUM_BOOLEAN: u8 = 252;
pub(crate) const ORA_TYPE_NUM_CHAR: u8 = 96;
pub(crate) const ORA_TYPE_NUM_CLOB: u8 = 112;
pub(crate) const ORA_TYPE_NUM_CURSOR: u8 = 102;
pub(crate) const ORA_TYPE_NUM_DATE: u8 = 12;
pub(crate) const ORA_TYPE_NUM_INTERVAL_DS: u8 = 183;
pub(crate) const ORA_TYPE_NUM_INTERVAL_YM: u8 = 182;
pub(crate) const ORA_TYPE_NUM_JSON: u8 = 119;
pub(crate) const ORA_TYPE_NUM_LONG: u8 = 8;
pub(crate) const ORA_TYPE_NUM_LONG_RAW: u8 = 24;
pub(crate) const ORA_TYPE_NUM_NUMBER: u8 = 2;
pub(crate) const ORA_TYPE_NUM_OBJECT: u8 = 109;
pub(crate) const ORA_TYPE_NUM_RAW: u8 = 23;
pub(crate) const ORA_TYPE_NUM_ROWID: u8 = 11;
pub(crate) const ORA_TYPE_NUM_TIMESTAMP: u8 = 180;
pub(crate) const ORA_TYPE_NUM_TIMESTAMP_LTZ: u8 = 231;
pub(crate) const ORA_TYPE_NUM_TIMESTAMP_TZ: u8 = 181;
pub(crate) const ORA_TYPE_NUM_UROWID: u8 = 208;
pub(crate) const ORA_TYPE_NUM_VARCHAR: u8 = 1;
pub(crate) const ORA_TYPE_NUM_VECTOR: u8 = 127;

pub(crate) const CS_FORM_IMPLICIT: u8 = 1;
pub(crate) const CS_FORM_NCHAR: u8 = 2;

pub(crate) const TNS_LONG_LENGTH_INDICATOR: u8 = 254;
pub(crate) const TNS_NULL_LENGTH_INDICATOR: u8 = 255;

pub(crate) const AUTH_MODE_DEFAULT: u32 = 0;
pub(crate) const AUTH_MODE_PRELIM: u32 = 0x00000008;
pub(crate) const AUTH_MODE_SYSASM: u32 = 0x00008000;
pub(crate) const AUTH_MODE_SYSBKP: u32 = 0x00020000;
pub(crate) const AUTH_MODE_SYSDBA: u32 = 0x00000002;
pub(crate) const AUTH_MODE_SYSDGD: u32 = 0x00040000;
pub(crate) const AUTH_MODE_SYSKMT: u32 = 0x00080000;
pub(crate) const AUTH_MODE_SYSOPER: u32 = 0x00000004;
pub(crate) const AUTH_MODE_SYSRAC: u32 = 0x00100000;

pub(crate) const PIPELINE_OP_TYPE_CALL_FUNC: u8 = 1;
pub(crate) const PIPELINE_OP_TYPE_CALL_PROC: u8 = 2;
pub(crate) const PIPELINE_OP_TYPE_COMMIT: u8 = 3;
pub(crate) const PIPELINE_OP_TYPE_EXECUTE: u8 = 4;
pub(crate) const PIPELINE_OP_TYPE_EXECUTE_MANY: u8 = 5;
pub(crate) const PIPELINE_OP_TYPE_FETCH_ALL: u8 = 6;
pub(crate) const PIPELINE_OP_TYPE_FETCH_MANY: u8 = 7;
pub(crate) const PIPELINE_OP_TYPE_FETCH_ONE: u8 = 8;

pub(crate) const POOL_GETMODE_WAIT: u8 = 0;
pub(crate) const POOL_GETMODE_NOWAIT: u8 = 1;
pub(crate) const POOL_GETMODE_FORCEGET: u8 = 2;
pub(crate) const POOL_GETMODE_TIMEDWAIT: u8 = 3;

pub(crate) const PURITY_DEFAULT: u8 = 0;
pub(crate) const PURITY_NEW: u8 = 1;
pub(crate) const PURITY_SELF: u8 = 2;

pub(crate) const VECTOR_FORMAT_BINARY: u8 = 5;
pub(crate) const VECTOR_FORMAT_FLOAT32: u8 = 2;
pub(crate) const VECTOR_FORMAT_FLOAT64: u8 = 3;
pub(crate) const VECTOR_FORMAT_INT8: u8 = 4;
