use crate::constants::*;
pub struct DbType {
    pub num: u32,
    pub name: &'static str,
    pub default_size: u32,
    _native_num: u32,
    _buffer_size_factor: u32,
    _ora_name: &'static str,
    _ora_type_num: u8,
    _csfrm: u8,
    _default_py_type_num: u8,
}

impl DbType {
    pub const fn new(
        num: u32,
        name: &'static str,
        ora_name: &'static str,
        native_num: u32,
        ora_type_num: u8,
        default_py_type_num: u8,
        default_size: u32,
        csfrm: u8,
        buffer_size_factor: u32,
    ) -> Self {
        DbType {
            num,
            name,
            default_size,
            _native_num: native_num,
            _buffer_size_factor: buffer_size_factor,
            _ora_name: ora_name,
            _ora_type_num: ora_type_num,
            _csfrm: csfrm,
            _default_py_type_num: default_py_type_num,
        }
    }
}

/// database types
pub const DB_TYPE_BFILE: DbType = DbType::new(
    DB_TYPE_NUM_BFILE,
    &"DB_TYPE_BFILE",
    &"BFILE",
    NATIVE_TYPE_NUM_LOB,
    ORA_TYPE_NUM_BFILE,
    PY_TYPE_NUM_ORACLE_LOB,
    0,
    0,
    4000,
);

pub const DB_TYPE_BINARY_DOUBLE: DbType = DbType::new(
    DB_TYPE_NUM_BINARY_DOUBLE,
    "DB_TYPE_BINARY_DOUBLE",
    "BINARY_DOUBLE",
    NATIVE_TYPE_NUM_DOUBLE,
    ORA_TYPE_NUM_BINARY_DOUBLE,
    PY_TYPE_NUM_FLOAT,
    0,
    0,
    8,
);

pub const DB_TYPE_BINARY_FLOAT: DbType = DbType::new(
    DB_TYPE_NUM_BINARY_FLOAT,
    "DB_TYPE_BINARY_FLOAT",
    "BINARY_FLOAT",
    NATIVE_TYPE_NUM_FLOAT,
    ORA_TYPE_NUM_BINARY_FLOAT,
    PY_TYPE_NUM_FLOAT,
    0,
    0,
    4,
);

pub const DB_TYPE_BINARY_INTEGER: DbType = DbType::new(
    DB_TYPE_NUM_BINARY_INTEGER,
    "DB_TYPE_BINARY_INTEGER",
    "BINARY_INTEGER",
    NATIVE_TYPE_NUM_INT64,
    ORA_TYPE_NUM_BINARY_INTEGER,
    PY_TYPE_NUM_INT,
    0,
    0,
    22,
);

pub const DB_TYPE_BLOB: DbType = DbType::new(
    DB_TYPE_NUM_BLOB,
    "DB_TYPE_BLOB",
    "BLOB",
    NATIVE_TYPE_NUM_LOB,
    ORA_TYPE_NUM_BLOB,
    PY_TYPE_NUM_ORACLE_LOB,
    0,
    0,
    112,
);

pub const DB_TYPE_BOOLEAN: DbType = DbType::new(
    DB_TYPE_NUM_BOOLEAN,
    "DB_TYPE_BOOLEAN",
    "BOOLEAN",
    NATIVE_TYPE_NUM_BOOLEAN,
    ORA_TYPE_NUM_BOOLEAN,
    PY_TYPE_NUM_BOOL,
    0,
    0,
    4,
);

pub const DB_TYPE_CHAR: DbType = DbType::new(
    DB_TYPE_NUM_CHAR,
    "DB_TYPE_CHAR",
    "CHAR",
    NATIVE_TYPE_NUM_BYTES,
    ORA_TYPE_NUM_CHAR,
    PY_TYPE_NUM_STR,
    2000,
    CS_FORM_IMPLICIT,
    4,
);

pub const DB_TYPE_CLOB: DbType = DbType::new(
    DB_TYPE_NUM_CLOB,
    "DB_TYPE_CLOB",
    "CLOB",
    NATIVE_TYPE_NUM_LOB,
    ORA_TYPE_NUM_CLOB,
    PY_TYPE_NUM_ORACLE_LOB,
    0,
    CS_FORM_IMPLICIT,
    112,
);

pub const DB_TYPE_CURSOR: DbType = DbType::new(
    DB_TYPE_NUM_CURSOR,
    "DB_TYPE_CURSOR",
    "CURSOR",
    NATIVE_TYPE_NUM_STMT,
    ORA_TYPE_NUM_CURSOR,
    PY_TYPE_NUM_ORACLE_CURSOR,
    0,
    0,
    4,
);

pub const DB_TYPE_DATE: DbType = DbType::new(
    DB_TYPE_NUM_DATE,
    "DB_TYPE_DATE",
    "DATE",
    NATIVE_TYPE_NUM_TIMESTAMP,
    ORA_TYPE_NUM_DATE,
    PY_TYPE_NUM_DATETIME,
    0,
    0,
    7,
);

pub const DB_TYPE_INTERVAL_DS: DbType = DbType::new(
    DB_TYPE_NUM_INTERVAL_DS,
    "DB_TYPE_INTERVAL_DS",
    "INTERVAL DAY TO SECOND",
    NATIVE_TYPE_NUM_INTERVAL_DS,
    ORA_TYPE_NUM_INTERVAL_DS,
    PY_TYPE_NUM_TIMEDELTA,
    0,
    0,
    11,
);

pub const DB_TYPE_INTERVAL_YM: DbType = DbType::new(
    DB_TYPE_NUM_INTERVAL_YM,
    "DB_TYPE_INTERVAL_YM",
    "INTERVAL YEAR TO MONTH",
    NATIVE_TYPE_NUM_INTERVAL_YM,
    ORA_TYPE_NUM_INTERVAL_YM,
    PY_TYPE_NUM_ORACLE_INTERVAL_YM,
    0,
    0,
    5,
);

pub const DB_TYPE_JSON: DbType = DbType::new(
    DB_TYPE_NUM_JSON,
    "DB_TYPE_JSON",
    "JSON",
    NATIVE_TYPE_NUM_JSON,
    ORA_TYPE_NUM_JSON,
    PY_TYPE_NUM_OBJECT,
    0,
    0,
    0,
);

pub const DB_TYPE_LONG: DbType = DbType::new(
    DB_TYPE_NUM_LONG_VARCHAR,
    "DB_TYPE_LONG",
    "LONG",
    NATIVE_TYPE_NUM_BYTES,
    ORA_TYPE_NUM_LONG,
    PY_TYPE_NUM_STR,
    0,
    CS_FORM_IMPLICIT,
    2147483647,
);

pub const DB_TYPE_LONG_NVARCHAR: DbType = DbType::new(
    DB_TYPE_NUM_LONG_NVARCHAR,
    "DB_TYPE_LONG_NVARCHAR",
    "LONG NVARCHAR",
    NATIVE_TYPE_NUM_BYTES,
    ORA_TYPE_NUM_LONG,
    PY_TYPE_NUM_STR,
    0,
    CS_FORM_NCHAR,
    2147483647,
);

pub const DB_TYPE_LONG_RAW: DbType = DbType::new(
    DB_TYPE_NUM_LONG_RAW,
    "DB_TYPE_LONG_RAW",
    "LONG RAW",
    NATIVE_TYPE_NUM_BYTES,
    ORA_TYPE_NUM_LONG_RAW,
    PY_TYPE_NUM_BYTES,
    0,
    0,
    2147483647,
);

pub const DB_TYPE_NCHAR: DbType = DbType::new(
    DB_TYPE_NUM_NCHAR,
    "DB_TYPE_NCHAR",
    "NCHAR",
    NATIVE_TYPE_NUM_BYTES,
    ORA_TYPE_NUM_CHAR,
    PY_TYPE_NUM_STR,
    2000,
    CS_FORM_NCHAR,
    4,
);

pub const DB_TYPE_NCLOB: DbType = DbType::new(
    DB_TYPE_NUM_NCLOB,
    "DB_TYPE_NCLOB",
    "NCLOB",
    NATIVE_TYPE_NUM_LOB,
    ORA_TYPE_NUM_CLOB,
    PY_TYPE_NUM_ORACLE_LOB,
    0,
    CS_FORM_NCHAR,
    112,
);

pub const DB_TYPE_NUMBER: DbType = DbType::new(
    DB_TYPE_NUM_NUMBER,
    "DB_TYPE_NUMBER",
    "NUMBER",
    NATIVE_TYPE_NUM_BYTES,
    ORA_TYPE_NUM_NUMBER,
    PY_TYPE_NUM_FLOAT,
    0,
    0,
    22,
);

pub const DB_TYPE_NVARCHAR: DbType = DbType::new(
    DB_TYPE_NUM_NVARCHAR,
    "DB_TYPE_NVARCHAR",
    "NVARCHAR2",
    NATIVE_TYPE_NUM_BYTES,
    ORA_TYPE_NUM_VARCHAR,
    PY_TYPE_NUM_STR,
    4000,
    CS_FORM_NCHAR,
    4,
);

pub const DB_TYPE_OBJECT: DbType = DbType::new(
    DB_TYPE_NUM_OBJECT,
    "DB_TYPE_OBJECT",
    "OBJECT",
    NATIVE_TYPE_NUM_OBJECT,
    ORA_TYPE_NUM_OBJECT,
    PY_TYPE_NUM_ORACLE_OBJECT,
    0,
    0,
    0,
);

pub const DB_TYPE_RAW: DbType = DbType::new(
    DB_TYPE_NUM_RAW,
    "DB_TYPE_RAW",
    "RAW",
    NATIVE_TYPE_NUM_BYTES,
    ORA_TYPE_NUM_RAW,
    PY_TYPE_NUM_BYTES,
    4000,
    0,
    1,
);

pub const DB_TYPE_ROWID: DbType = DbType::new(
    DB_TYPE_NUM_ROWID,
    "DB_TYPE_ROWID",
    "ROWID",
    NATIVE_TYPE_NUM_ROWID,
    ORA_TYPE_NUM_ROWID,
    PY_TYPE_NUM_STR,
    0,
    0,
    18,
);

pub const DB_TYPE_TIMESTAMP: DbType = DbType::new(
    DB_TYPE_NUM_TIMESTAMP,
    "DB_TYPE_TIMESTAMP",
    "TIMESTAMP",
    NATIVE_TYPE_NUM_TIMESTAMP,
    ORA_TYPE_NUM_TIMESTAMP,
    PY_TYPE_NUM_DATETIME,
    0,
    0,
    11,
);

pub const DB_TYPE_TIMESTAMP_LTZ: DbType = DbType::new(
    DB_TYPE_NUM_TIMESTAMP_LTZ,
    "DB_TYPE_TIMESTAMP_LTZ",
    "TIMESTAMP WITH LOCAL TIME ZONE",
    NATIVE_TYPE_NUM_TIMESTAMP,
    ORA_TYPE_NUM_TIMESTAMP_LTZ,
    PY_TYPE_NUM_DATETIME,
    0,
    0,
    11,
);

pub const DB_TYPE_TIMESTAMP_TZ: DbType = DbType::new(
    DB_TYPE_NUM_TIMESTAMP_TZ,
    "DB_TYPE_TIMESTAMP_TZ",
    "TIMESTAMP WITH TIME ZONE",
    NATIVE_TYPE_NUM_TIMESTAMP,
    ORA_TYPE_NUM_TIMESTAMP_TZ,
    PY_TYPE_NUM_DATETIME,
    0,
    0,
    13,
);

pub const DB_TYPE_UNKNOWN: DbType = DbType::new(
    DB_TYPE_NUM_UNKNOWN,
    "DB_TYPE_UNKNOWN",
    "UNKNOWN",
    0,
    0,
    0,
    0,
    0,
    0,
);

pub const DB_TYPE_UROWID: DbType = DbType::new(
    DB_TYPE_NUM_UROWID,
    "DB_TYPE_UROWID",
    "UROWID",
    NATIVE_TYPE_NUM_BYTES,
    ORA_TYPE_NUM_UROWID,
    PY_TYPE_NUM_STR,
    0,
    0,
    0,
);

pub const DB_TYPE_VARCHAR: DbType = DbType::new(
    DB_TYPE_NUM_VARCHAR,
    "DB_TYPE_VARCHAR",
    "VARCHAR2",
    NATIVE_TYPE_NUM_BYTES,
    ORA_TYPE_NUM_VARCHAR,
    PY_TYPE_NUM_STR,
    4000,
    CS_FORM_IMPLICIT,
    4,
);

pub const DB_TYPE_VECTOR: DbType = DbType::new(
    DB_TYPE_NUM_VECTOR,
    "DB_TYPE_VECTOR",
    "VECTOR",
    NATIVE_TYPE_NUM_VECTOR,
    ORA_TYPE_NUM_VECTOR,
    PY_TYPE_NUM_ARRAY,
    0,
    0,
    0,
);

pub const DB_TYPE_XMLTYPE: DbType = DbType::new(
    DB_TYPE_NUM_XMLTYPE,
    "DB_TYPE_XMLTYPE",
    "XMLTYPE",
    NATIVE_TYPE_NUM_BYTES,
    ORA_TYPE_NUM_OBJECT,
    PY_TYPE_NUM_STR,
    0,
    CS_FORM_IMPLICIT,
    2147483647,
);
