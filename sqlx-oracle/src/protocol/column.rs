use crate::{
    constants::{
        CS_FORM_IMPLICIT, ORA_TYPE_NUM_RAW, TNS_CCAP_FIELD_VERSION_12_2,
        TNS_CCAP_FIELD_VERSION_23_1, TNS_CCAP_FIELD_VERSION_23_1_EXT_3,
        TNS_CCAP_FIELD_VERSION_23_4, TNS_UDS_FLAGS_IS_JSON, TNS_UDS_FLAGS_IS_OSON,
    },
    io::OraBufExt,
    protocol::{dbtype::DbType, Capabilities},
};
use sqlx_core::{
    bytes::{Buf, Bytes},
    HashMap,
};

pub struct BaseDbObjectType {
    pub schema: String,
    pub name: String,
    pub package_name: String,
    pub attrs: Vec<String>,
    pub is_collection: bool,
    pub attrs_by_name: HashMap<String, String>,
}

pub struct ColumnDefinition {
    pub name: String,
    pub dbtype: DbType,
    pub object_type: BaseDbObjectType,
    pub precision: i8,
    pub scale: i8,
    pub max_size: u32,
    pub buffer_size: u32,
    pub nulls_allowed: bool,
    pub is_json: bool,
    pub is_oson: bool,
    pub domain_schema: String,
    pub domain_name: String,
    pub annotations: HashMap<String, String>,
    pub vector_dimensions: u32,
    pub vector_format: bool,
    pub vector_flags: bool,
    // _arrow_type: ArrowType, // todo!()
    _py_type_num: u8,
}

pub fn parse_column(
    buf: &mut Bytes,
    caps: &Capabilities,
) -> Result<ColumnDefinition, sqlx_core::Error> {
    let ora_type_num = buf.get_u8();
    let _ = buf.get_u8(); // flags
    let precision = buf.get_i8();
    let scale = buf.get_i8();
    let buffer_size = buf.get_u32();
    let _ = buf.get_u32(); // max number of array elements
    let _ = buf.get_u64(); // cont flags
    let oid = buf.read_bytes();
    let _ = buf.get_u16(); // version
    let _ = buf.get_u16(); // character set id
    let csfrm = buf.get_u8(); // character set form
                              // let dbtype = DbType::from_ora_type_num(ora_type_num); // todo!()
    let mut max_size = buf.get_u32();
    if ora_type_num == ORA_TYPE_NUM_RAW {
        max_size = dbtype.buffer_size;
    }
    if caps.ttc_field_version >= TNS_CCAP_FIELD_VERSION_12_2 {
        let _ = buf.get_u32(); // oaccolid
    }
    let nulls_allowed = buf.get_u8() != 0;
    let _ = buf.get_u8(); // v7 length of name
    let name = buf.read_str(CS_FORM_IMPLICIT).unwrap_or_default(); //
    let schema = buf.read_str(CS_FORM_IMPLICIT).unwrap_or_default(); //
    let _ = buf.get_u16(); // column position
    let uds_flags = buf.get_u32();
    let is_json = uds_flags & TNS_UDS_FLAGS_IS_JSON != 0;
    let is_oson = uds_flags & TNS_UDS_FLAGS_IS_OSON != 0;
    let mut domain_name = String::new();
    let mut domain_schema = String::new();
    let mut annotation = String::new();
    if caps.ttc_field_version >= TNS_CCAP_FIELD_VERSION_23_1 {
        domain_schema = buf.read_str(CS_FORM_IMPLICIT).unwrap_or_default();
        domain_name = buf.read_str(CS_FORM_IMPLICIT).unwrap_or_default();
    }
    let mut annotations = HashMap::default();
    if caps.ttc_field_version >= TNS_CCAP_FIELD_VERSION_23_1_EXT_3 {
        let num_annotations = buf.get_u32();
        if num_annotations > 0 {
            let _ = buf.get_u8();
            let num_annotations = buf.get_u32();
            let _ = buf.get_u8();
            for i in 0..num_annotations {
                let key = buf.read_str(CS_FORM_IMPLICIT).unwrap_or_default();
                let value = buf.read_str(CS_FORM_IMPLICIT).unwrap_or_default();
                annotations.insert(key, value);
                let _ = buf.get_u32(); // flags
            }
            let _ = buf.get_u32(); //flags
        }
    }

    let mut vector_dimensions = 0;
    let mut vector_format = false;
    let mut vector_flags = false;
    if caps.ttc_field_version >= TNS_CCAP_FIELD_VERSION_23_4 {
        vector_dimensions = buf.get_u32();
        vector_format = buf.get_u8() != 0;
        vector_flags = buf.get_u8() != 0;
    }
    Ok(ColumnDefinition {
        name,
        dbtype,
        object_type,
        precision,
        scale,
        max_size,
        buffer_size,
        nulls_allowed,
        is_json,
        is_oson,
        domain_schema,
        domain_name,
        annotations,
        vector_dimensions,
        vector_format,
        vector_flags,
        _py_type_num: 0,
    })
}
