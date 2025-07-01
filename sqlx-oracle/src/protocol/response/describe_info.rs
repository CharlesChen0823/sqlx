use crate::error::Error;
use crate::io::OraBufExt;
use crate::protocol::{parse_column, ColumnDefinition};
use crate::OracleConnection;
use crate::{constants::*, protocol::response::Response};

use sqlx_core::bytes::{Buf, Bytes};

pub struct Metadata {}

impl Response for Metadata {
    fn decode_body_with(mut buf: Bytes, _conn: &mut OracleConnection) -> Result<Self, Error> {
        let ora_type_num = buf.get_u8();
        let _ = buf.get_u8(); // flags
        let precision = buf.get_i8();
        let scale = buf.get_i8();
        let buffer_size = buf.get_u32();
        let _ = buf.get_u32(); // max number of array elements
        let _ = buf.get_u64(); // cont flags
        let oid = buf.read_bytes();
        let _ = buf.get_u16(); //version
        let _ = buf.get_u16(); // character set id
        let csfrm = buf.get_u8(); // character set form
                                  // todo _from_ora_type_and_csfrm
        let mut max_size = buf.get_u32();
        if ora_type_num == ORA_TYPE_NUM_RAW {
            max_size = buffer_size;
        }
        //todo!() caps.ttc_field_version >= TNS_CCAP_FIELD_VERSION_12_2
        // todo!() let _ = buf.get_u32(); // oaccolid
        let nulls_allowed = buf.get_u8();
        let _ = buf.get_u8(); // v7 length of name
                              // let metadata.name = buf.read_str_with_length(); // todo!()
                              // let schema = buf.read_str_with_length();
                              // let name = buf.read_str_with_length();
        let _ = buf.get_u16(); // column position
        let uds_flags = buf.get_u32();
        let is_json = uds_flags & TNS_UDS_FLAGS_IS_JSON;
        let is_oson = uds_flags & TNS_UDS_FLAGS_IS_OSON;
        // todo!() _caps.ttc_field_version >= TNS_CCAP_FIELD_VERSION_23_1
        // let metadata.domain_schema = buf.read_str_with_length();
        // let metadata.domain_name = buf.read_str_with_length();
        // todo!() _caps.ttc_field_version >= TNS_CCAP_FIELD_VERSION_23_1_EXT_3
        // let num_annotations = buf.get_u32();
        // if num_annotations > 0 {
        //     let _ = buf.get_u8();
        //     let mut annotations = HashMap::default();
        //     let num_annotations = buf.get_u32();
        //     let _ = buf.get_u8();
        //     for _ in 0..num_annotations {
        //         let key = buf.read_str_with_length();
        //         let value = buf.read_str_with_length();
        //         // todo!() value is null
        //         annotations.insert(key, value);
        //         let _ = buf.get_u32(); // flags
        //     }
        //     let _ = buf.get_u32(); // flags
        // }
        // todo!() _caps.ttc_field_version >= TNS_CCAP_FIELD_VERSION_23_4
        // let vector_dimensions = buf.get_u32();
        // let vector_format = buf.get_u8();
        // let vector_flags = buf.get_u8();
        if ora_type_num == ORA_TYPE_NUM_OBJECT {
            // todo type_cache is None
        }

        Ok(Metadata {})
    }
}

pub struct DescribeInfo {
    columns: Vec<ColumnDefinition>,
}

impl Response for DescribeInfo {
    fn decode_body_with(mut buf: Bytes, conn: &mut OracleConnection) -> Result<Self, Error> {
        buf.skip_raw_bytes_chunked(); //todo!() this should move out?

        let _ = buf.get_u32(); // max row size
        let _num_columns = buf.get_u32();
        // todo!()
        //
        if _num_columns > 0 {
            let _ = buf.get_u8();
        }

        let mut columns: Vec<ColumnDefinition> = Vec::with_capacity(_num_columns as usize);
        for i in 0.._num_columns {
            let column = parse_column(&mut buf, &conn.inner.caps)?;
            columns.push(column);
        }

        let num_bytes = buf.get_u32();
        if num_bytes > 0 {
            buf.skip_raw_bytes_chunked(); // current date
        }
        let _ = buf.get_u32(); // dcbflag
        let _ = buf.get_u32(); // dcbmdbz
        let _ = buf.get_u32(); // dcbmnpr
        let _ = buf.get_u32(); // dcbmxpr
        let num_bytes = buf.get_u32();
        if num_bytes > 0 {
            buf.skip_raw_bytes_chunked(); // dcbqcky
        }

        Ok(DescribeInfo { columns })
    }
}
