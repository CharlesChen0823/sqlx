use hmac::Hmac;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1v15::SigningKey;
use rsa::RsaPrivateKey;
use sqlx_core::bytes::{BufMut, Bytes};
use sqlx_core::connection::Connection;
use sqlx_core::HashMap;

use crate::constants::*;
use crate::error::Error;
use crate::io::OraBufMutExt;
use crate::options::{AccessToken, ConnectionClass, DefaultOsParams};
use crate::{OracleConnectOptions, OracleConnection};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use cbc::cipher::{block_padding::NoPadding, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use rand::RngCore;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

/// Error type for key derivation operations
#[derive(Debug)]
pub enum KeyDerivationError {
    /// Invalid number of iterations (must be > 0)
    InvalidIterations,
    /// Invalid output length
    InvalidLength,
}

#[derive(Debug)]
pub enum SignatureError {
    KeyParseError(String),
    SigningError(String),
    EncodingError(String),
}

type Aes192CbcEnc = cbc::Encryptor<aes::Aes192>;
type Aes192CbcDec = cbc::Decryptor<aes::Aes192>;
type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

use crate::message::{FrontendMessage, FrontendMessageFormat};

// AuthMessage for change password

impl std::fmt::Display for SignatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SignatureError::KeyParseError(e) => write!(f, "Failed to parse private key: {}", e),
            SignatureError::SigningError(e) => write!(f, "Failed to create signature: {}", e),
            SignatureError::EncodingError(e) => write!(f, "Failed to encode signature: {}", e),
        }
    }
}

fn get_signature(private_key_str: &str, text: &str) -> Result<String, SignatureError> {
    todo!()
    // let private_key = RsaPrivateKey::from_pkcs1_pem(private_key_str)
    //     .map_err(|e| SignatureError::KeyParseError(e.to_string()))?;

    // // Create a signing key
    // let signing_key = SigningKey::<Sha256>::new(private_key);

    // // Sign the text
    // let signature = signing_key
    //     .try_sign_with_rng(&mut rand::thread_rng(), text.as_bytes())
    //     .map_err(|e| SignatureError::SigningError(e.to_string()))?;

    // // Encode the signature in base64
    // Ok(BASE64.encode(signature.as_bytes()))
}

pub struct AuthMessageOne<'a> {
    pub username: &'a str,
    pub auth_mode: u32,
    pub default_os_params: &'a DefaultOsParams,
}

struct AuthMessageModifyPassword {
    password: Bytes,
    newpassword: Bytes,
    debug_jdwp: Option<String>,
    proxy_user: Option<String>,
    token: Option<String>,
    private_key: Option<String>,
    service_name: String,
    purity: u8,
    username: Bytes,
    session_data: HashMap<String, Vec<u8>>,
    auth_mode: u32,
    verifier_type: u16,
    edition: String,
    connect_string: String,
}

pub struct AuthMessageTwo<'a, 'b> {
    pub options: &'a OracleConnectOptions,
    session_data: &'b HashMap<String, String>,
    verifier_type: u16,
}

fn write_key_value(buf: &mut Vec<u8>, key: &str, value: &str, flags: u32) {
    let key_len = key.as_bytes().len();
    let value_len = value.as_bytes().len();
    buf.write_ub4(key_len as u32);
    buf.write_bytes(key.as_bytes());
    buf.write_ub4(value_len as u32);
    if value_len > 0 {
        buf.write_bytes(value.as_bytes());
    }
    buf.write_ub4(flags);
}

fn write_piggybacks(buf: &mut Vec<u8>, conn: &mut OracleConnection) {
    if conn.inner.pipeline_mode != 0 {
        write_begin_pipeline_piggyback(buf, conn);
        conn.inner.pipeline_mode = 0;
    }
    if conn.current_schema_modified {
        write_current_schema_piggyback(buf, conn);
    }

    if !conn.inner.cache_statement.is_empty()
        && conn.cached_statements_size() > 0
        && !conn.inner.drcp_establish_session
    {
        write_close_cursors_piggyback(buf, conn);
    }

    if conn.inner.action_modified
        || conn.inner.client_identifier_modified
        || conn.inner.client_info_modified
        || conn.inner.dbop_modified
        || conn.inner.module_modified
    {
        write_end_to_end_piggyback(buf, conn);
    }

    if conn.inner.temp_lobs_total_size > 0 {
        write_close_temp_lobs_piggyback(buf, conn);
    }
    if conn.inner.session_state_desired != 0 {
        write_session_state_piggyback(buf, conn);
    }
}

fn write_session_state_piggyback(buf: &mut Vec<u8>, conn: &mut OracleConnection) {
    let state = conn.inner.session_state_desired;
    write_piggyback_code(buf, TNS_FUNC_SESSION_STATE, conn);
    buf.write_ub8(state | (TNS_SESSION_STATE_EXPLICIT_BOUNDARY as u64));
    conn.inner.session_state_desired = 0;
}

fn write_close_temp_lobs_piggyback(buf: &mut Vec<u8>, conn: &mut OracleConnection) {
    write_piggyback_code(buf, TNS_FUNC_LOB_OP, conn);
    let op_code = TNS_LOB_OP_FREE_TEMP | TNS_LOB_OP_ARRAY;

    // temp lob data
    buf.put_u8(1);
    buf.write_ub4(conn.inner.temp_lobs_total_size as u32);
    buf.put_u8(0);
    buf.write_ub4(0);
    buf.write_ub4(0);
    buf.write_ub4(0);
    buf.put_u8(0);
    buf.put_u8(0);
    buf.put_u8(0);
    buf.write_ub4(op_code);
    buf.put_u8(0);
    buf.write_ub4(0);
    buf.write_ub8(0);
    buf.write_ub8(0);
    buf.put_u8(0);
    // array lob fields
    buf.put_u8(0);
    buf.write_ub4(0);
    buf.put_u8(0);
    buf.write_ub4(0);
    buf.put_u8(0);
    buf.write_ub4(0);
    for lob in conn.inner.temp_lobs_to_close.iter() {
        buf.write_bytes(&lob.as_bytes());
    }

    // reset values
    conn.inner.temp_lobs_to_close = Vec::new();
    conn.inner.temp_lobs_total_size = 0;
}

fn write_end_to_end_piggyback(buf: &mut Vec<u8>, conn: &mut OracleConnection) {
    let mut flags: u32 = 0;
    // determine which flags to send
    if conn.inner.action_modified {
        flags |= TNS_END_TO_END_ACTION;
    }
    if conn.inner.client_identifier_modified {
        flags |= TNS_END_TO_END_CLIENT_IDENTIFIER;
    }
    if conn.inner.client_info_modified {
        flags |= TNS_END_TO_END_CLIENT_INFO;
    }
    if conn.inner.module_modified {
        flags |= TNS_END_TO_END_MODULE;
    }
    if conn.inner.dbop_modified {
        flags |= TNS_END_TO_END_DBOP;
    }
    // write initial packet data
    write_piggyback_code(buf, TNS_FUNC_SET_END_TO_END_ATTR, conn);
    buf.put_u8(0);
    buf.put_u8(0);
    buf.write_ub4(flags);

    // write client identifier data
    if conn.inner.client_identifier_modified {
        buf.put_u8(1);
        if conn.inner.client_identifier.is_none() {
            buf.write_ub4(0);
        } else {
            buf.write_ub4(
                conn.inner
                    .client_identifier
                    .as_ref()
                    .unwrap()
                    .as_bytes()
                    .len() as u32,
            );
        }
    } else {
        buf.put_u8(0);
        buf.write_ub4(0);
    }

    // write client info data
    if conn.inner.client_info_modified {
        buf.put_u8(1);
        if conn.inner.client_info.is_none() {
            buf.write_ub4(0);
        } else {
            buf.write_ub4(conn.inner.client_info.as_ref().unwrap().as_bytes().len() as u32);
        }
    } else {
        buf.put_u8(0);
        buf.write_ub4(0);
    }

    // write module data
    if conn.inner.module_modified {
        buf.put_u8(1);
        if conn.inner.module.is_none() {
            buf.write_ub4(0);
        } else {
            buf.write_ub4(conn.inner.module.as_ref().unwrap().as_bytes().len() as u32);
        }
    } else {
        buf.put_u8(0);
        buf.write_ub4(0);
    }

    if conn.inner.action_modified {
        buf.put_u8(1);
        if conn.inner.action.is_none() {
            buf.write_ub4(0);
        } else {
            buf.write_ub4(conn.inner.action.as_ref().unwrap().as_bytes().len() as u32);
        }
    } else {
        buf.put_u8(0);
        buf.write_ub4(0);
    }

    // write unsupported bits
    buf.put_u8(0);
    buf.write_ub4(0);
    buf.put_u8(0);
    buf.write_ub4(0);

    // write client info header info
    if conn.inner.client_info_modified {
        buf.put_u8(1);
        if conn.inner.client_info.is_none() {
            buf.write_ub4(0);
        } else {
            buf.write_ub4(conn.inner.client_info.as_ref().unwrap().as_bytes().len() as u32);
        }
    } else {
        buf.put_u8(0);
        buf.write_ub4(0);
    }

    // write more unsupported bits
    buf.put_u8(0);
    buf.write_ub4(0);
    buf.put_u8(0);
    buf.write_ub4(0);

    // write dbop header info
    if conn.inner.dbop_modified {
        buf.put_u8(1);
        if conn.inner.dbop.is_none() {
            buf.write_ub4(0);
        } else {
            buf.write_ub4(conn.inner.dbop.as_ref().unwrap().as_bytes().len() as u32);
        }
    } else {
        buf.put_u8(0);
        buf.write_ub4(0);
    }

    // write strings
    if conn.inner.client_identifier_modified && conn.inner.client_identifier.is_some() {
        buf.write_bytes(conn.inner.client_identifier.as_ref().unwrap().as_bytes());
    }
    if conn.inner.module_modified && conn.inner.module.is_some() {
        buf.write_bytes(conn.inner.module.as_ref().unwrap().as_bytes());
    }

    if conn.inner.action_modified && conn.inner.action.is_some() {
        buf.write_bytes(conn.inner.action.as_ref().unwrap().as_bytes());
    }

    if conn.inner.client_info_modified && conn.inner.client_info.is_some() {
        buf.write_bytes(conn.inner.client_info.as_ref().unwrap().as_bytes());
    }

    if conn.inner.dbop_modified && conn.inner.dbop.is_some() {
        buf.write_bytes(conn.inner.dbop.as_ref().unwrap().as_bytes());
    }

    // reset flags and values
    conn.inner.action_modified = false;
    conn.inner.dbop_modified = false;
    conn.inner.client_identifier_modified = false;
    conn.inner.action = None;
    conn.inner.dbop = None;
    conn.inner.client_identifier = None;
    conn.inner.module_modified = false;
    conn.inner.module = None;
    conn.inner.client_info = None;
    conn.inner.client_info_modified = false;
}

fn write_close_cursors_piggyback(buf: &mut Vec<u8>, conn: &mut OracleConnection) {
    write_piggyback_code(buf, TNS_FUNC_CLOSE_CURSORS, conn);
    buf.put_u8(1);
    write_cursors_to_close(buf, conn);
}

fn write_cursors_to_close(buf: &mut Vec<u8>, conn: &mut OracleConnection) {
    // todo!() might not same with original
    let len = conn.inner.cursors_to_close.len();
    buf.write_ub4(len as u32);
    for cursor_id in conn.inner.cursors_to_close.iter() {
        buf.write_ub4(*cursor_id);
    }
    conn.inner.cursors_to_close.clear();
}

fn write_current_schema_piggyback(buf: &mut Vec<u8>, conn: &mut OracleConnection) {
    write_piggyback_code(buf, TNS_FUNC_SET_SCHEMA, conn);
    buf.put_u8(1);
    buf.write_ub4(conn.inner.current_schema.as_bytes().len() as u32);
    buf.write_bytes(conn.inner.current_schema.as_bytes());
}

fn write_begin_pipeline_piggyback(buf: &mut Vec<u8>, conn: &mut OracleConnection) {
    conn.inner.data_flags |= TNS_DATA_FLAGS_BEGIN_PIPELINE;
    write_piggyback_code(buf, TNS_FUNC_PIPELINE_BEGIN, conn);
    buf.write_ub2(0);
    buf.put_u8(0);
    buf.put_u8(conn.inner.pipeline_mode as u8);
}

fn write_piggyback_code(buf: &mut Vec<u8>, code: u8, conn: &mut OracleConnection) {
    buf.put_u8(TNS_MSG_TYPE_PIGGYBACK);
    buf.put_u8(code);
    buf.put_u8(conn.get_seq_num());
    if conn.inner.caps.ttc_field_version >= TNS_CCAP_FIELD_VERSION_23_1_EXT_1 {
        buf.write_ub8(conn.inner.token_num);
    }
}

fn set_auth_mode(auth_mode: u32, change_password: bool, is_private_key: bool) -> u32 {
    let mut auth_mode = auth_mode;
    if change_password {
        auth_mode = TNS_AUTH_MODE_LOGON;
    }

    if auth_mode & AUTH_MODE_SYSDBA != 0 {
        auth_mode |= TNS_AUTH_MODE_SYSDBA;
    }
    if auth_mode & AUTH_MODE_SYSOPER != 0 {
        auth_mode |= TNS_AUTH_MODE_SYSOPER;
    }

    if auth_mode & AUTH_MODE_SYSASM != 0 {
        auth_mode |= TNS_AUTH_MODE_SYSASM;
    }

    if auth_mode & AUTH_MODE_SYSBKP != 0 {
        auth_mode |= TNS_AUTH_MODE_SYSBKP;
    }

    if auth_mode & AUTH_MODE_SYSDGD != 0 {
        auth_mode |= TNS_AUTH_MODE_SYSDGD;
    }

    if auth_mode & AUTH_MODE_SYSKMT != 0 {
        auth_mode |= TNS_AUTH_MODE_SYSKMT;
    }

    if auth_mode & AUTH_MODE_SYSRAC != 0 {
        auth_mode |= TNS_AUTH_MODE_SYSRAC;
    }

    if is_private_key {
        auth_mode |= TNS_AUTH_MODE_IAM_TOKEN;
    }

    auth_mode
}

impl FrontendMessage for AuthMessageOne<'_> {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::AuthPhaseOne;
    fn encode_body_with(
        &self,
        buf: &mut Vec<u8>,
        conn: &mut OracleConnection,
    ) -> Result<(), Error> {
        let num_pairs = 5;
        let auth_mode = set_auth_mode(conn.inner.auth_mode, false, false);
        // write basic data to packet, same _write_function_code
        write_piggybacks(buf, conn);
        buf.put_u8(TNS_MSG_TYPE_FUNCTION);
        buf.put_u8(Self::FORMAT as u8); // function_code
        buf.put_u8(conn.get_seq_num());
        if conn.inner.caps.ttc_field_version >= TNS_CCAP_FIELD_VERSION_23_1_EXT_1 {
            buf.write_ub8(conn.inner.token_num);
        }
        if self.username.len() > 0 {
            buf.put_u8(1);
        }
        buf.write_ub4(self.username.as_bytes().len() as u32);
        buf.write_ub4(auth_mode);
        buf.put_u8(1);
        buf.write_ub4(num_pairs);
        buf.put_u8(1);
        buf.put_u8(1);
        if self.username.len() > 0 {
            buf.write_bytes(self.username.as_bytes());
        }
        write_key_value(buf, "AUTH_TERMINAL", &self.default_os_params.terminal, 0);
        write_key_value(buf, "AUTH_PROGRAM_NM", &self.default_os_params.program, 0);
        write_key_value(buf, "AUTH_MACHINE", &self.default_os_params.machine, 0);
        write_key_value(
            buf,
            "AUTH_PID",
            self.default_os_params.pid.to_string().as_str(),
            0,
        );
        write_key_value(buf, "AUTH_SID", &self.default_os_params.osuser, 0);
        Ok(())
    }
}

fn get_derived_key(
    key: &[u8],
    salt: &[u8],
    length: usize,
    iterations: u32,
) -> Result<Vec<u8>, KeyDerivationError> {
    // Create output buffer
    let mut derived_key = vec![0u8; length];

    pbkdf2::pbkdf2::<Hmac<Sha256>>(key, salt, iterations, &mut derived_key);

    Ok(derived_key)
}

fn generate_verifier(auth_message_two: &AuthMessageTwo<'_, '_>, _buf: &[u8]) {
    let mut password_key_keep = None;
    let verifier_data: String = auth_message_two
        .session_data
        .get("AUTH_VFR_DATA")
        .unwrap()
        .clone();
    let (key_len, password_hash) = if auth_message_two.verifier_type == TNS_VERIFIER_TYPE_12C {
        let key_len = 32;
        let iterations = auth_message_two
            .session_data
            .get("AUTH_PBKDF2_VGEN_COUNT")
            .unwrap();
        let iterations = iterations.parse::<u32>().unwrap();
        let mut salt = verifier_data.as_bytes().to_vec();
        salt.extend(b"AUTH_PBKDF2_SPEEDY_KEY");
        let password_key = get_derived_key(
            &auth_message_two.options.password.as_bytes(),
            &salt,
            64,
            iterations,
        )
        .unwrap();
        password_key_keep = Some(password_key.clone());

        let mut hasher = sha2::Sha512::new();
        hasher.update(&password_key);
        hasher.update(verifier_data.as_bytes());
        let result = hasher.finalize();
        let password_hash = result[..32].to_vec();
        (key_len, password_hash)
    } else {
        let key_len = 24;
        let mut hasher = Sha1::new();
        hasher.update(auth_message_two.options.password.as_bytes());
        hasher.update(verifier_data.as_bytes());
        let result = hasher.finalize();
        let mut password_hash = result.to_vec();
        password_hash.extend(b"\x00\x00\x00\x00");
        (key_len, password_hash)
    };

    // decrypt first half of session key
    let encoded_server_key = auth_message_two.session_data.get("AUTH_SESSKEY").unwrap();
    let session_key_part_a = decrypt_cbc(&password_hash, encoded_server_key.as_bytes());

    // generate second half of session key
    let length = session_key_part_a.len();
    let mut session_key_part_b = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut session_key_part_b);
    let encoded_client_key = encrypt_cbc(&password_hash, &session_key_part_b);

    // create session key and combo key
    let combo_key = if session_key_part_a.len() == 48 {
        let _session_key = hex::encode_upper(&encoded_client_key)[..96].to_string();
        let mut b = [0u8; 24];
        for i in 16..40 {
            b[i - 16] = session_key_part_a[i] ^ session_key_part_b[i];
        }
        let (p1, p2) = b.split_at(16);
        let mut md5hash = md5::Md5::new();
        md5hash.update(p1);
        let p1_result = md5hash.finalize();
        let mut md5hash = md5::Md5::new();
        md5hash.update(p2);
        let p2_result = md5hash.finalize();
        let mut combo_key = p1_result.to_vec();
        combo_key.extend(p2_result);
        combo_key[..key_len].to_vec()
    } else {
        let _session_key = hex::encode_upper(&encoded_client_key)[..64].to_string();
        let salt = auth_message_two
            .session_data
            .get("AUTH_PBKDF2_CSK_SALT")
            .unwrap();
        let iterations = auth_message_two
            .session_data
            .get("AUTH_PBKDF2_SDER_COUNT")
            .unwrap();
        let iterations = iterations.parse::<u32>().unwrap();
        let (p1, _) = session_key_part_a.split_at(key_len);
        let (p2, _) = session_key_part_b.split_at(key_len);
        let mut temp_key = p1.to_vec();
        temp_key.extend(p2);
        let temp_key = temp_key.to_ascii_uppercase();
        get_derived_key(&temp_key, salt.as_bytes(), key_len, iterations).unwrap()
    };

    // retain session key for use by the change password API
    // conn.inner.combo_key = combo_key; todo!()

    // generate speedy key for 12c verifiers
    if auth_message_two.verifier_type == TNS_VERIFIER_TYPE_12C {
        let mut salt = vec![0u8; 16];
        rand::thread_rng().fill_bytes(&mut salt);
        let password_key = password_key_keep.unwrap();
        salt.extend(password_key);
        let speedy_key = encrypt_cbc(&combo_key, salt.as_slice());
        let (speedy_key, _) = speedy_key.split_at(80);
        let _speedy_key_hex = hex::encode_upper(speedy_key);
    }

    // encrypts the passsword
    let mut salt = vec![0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt.extend(auth_message_two.options.password.as_bytes());
    let encrypted_password = encrypt_cbc(&combo_key, salt.as_slice());
    let _encrypted_password_hex = hex::encode_upper(&encrypted_password);

    // check if debug_jdwp is set. if set, encode the data using the combo session key with zeros padding
    if let Some(ref _debug_jdwp) = auth_message_two.options.debug_jdwp {
        let encrypted_jwpd_data = encrypt_cbc(&combo_key, _debug_jdwp.as_bytes());
        let _encrypted_jwpd_data = hex::encode_upper(&encrypted_jwpd_data);
    }
}

fn encrypt_cbc(hasher: &[u8], plain_text: &[u8]) -> Vec<u8> {
    let iv = [0u8; 16]; // Zero IV as per Python implementation

    // Pad to block size (16 bytes for AES)
    let mut buffer = plain_text.to_vec();
    let block_size = 16;
    let padding_needed = block_size - (buffer.len() % block_size);
    if padding_needed != block_size {
        buffer.extend(vec![0u8; padding_needed]);
    }

    // Determine key size and use appropriate AES variant
    match hasher.len() {
        24 => {
            // AES-192
            let mut key = [0u8; 24];
            key.copy_from_slice(hasher);
            let cipher = Aes192CbcEnc::new(&key.into(), &iv.into());
            cipher
                .encrypt_padded_mut::<NoPadding>(&mut buffer, plain_text.len())
                .expect("Encryption failed");
            buffer
        }
        32 => {
            // AES-256
            let mut key = [0u8; 32];
            key.copy_from_slice(hasher);
            let cipher = Aes256CbcEnc::new(&key.into(), &iv.into());
            cipher
                .encrypt_padded_mut::<NoPadding>(&mut buffer, plain_text.len())
                .expect("Encryption failed");
            buffer
        }
        _ => panic!("Unsupported key size: {}", hasher.len()),
    }
}

fn decrypt_cbc(hasher: &[u8], as_str: &[u8]) -> Vec<u8> {
    let iv = [0u8; 16]; // Zero IV as per Python implementation

    // Convert hex string to bytes
    let encrypted_text = hex::decode(as_str).expect("Invalid hex string");
    let mut buffer = encrypted_text.clone();

    // Determine key size and use appropriate AES variant
    match hasher.len() {
        24 => {
            // AES-192
            let mut key = [0u8; 24];
            key.copy_from_slice(hasher);
            let cipher = Aes192CbcDec::new(&key.into(), &iv.into());
            let decrypted = cipher
                .decrypt_padded_mut::<NoPadding>(&mut buffer)
                .expect("Decryption failed");
            decrypted.to_vec()
        }
        32 => {
            // AES-256
            let mut key = [0u8; 32];
            key.copy_from_slice(hasher);
            let cipher = Aes256CbcDec::new(&key.into(), &iv.into());
            let decrypted = cipher
                .decrypt_padded_mut::<NoPadding>(&mut buffer)
                .expect("Decryption failed");
            decrypted.to_vec()
        }
        _ => panic!("Unsupported key size: {}", hasher.len()),
    }
}

fn get_alter_timezone_statement() -> String {
    match env::var_os("ORA_SDTZ") {
        Some(val) => {
            format!("ALTER SESSION SET TIME_ZONE = '{:?}'\x00", val)
        }
        None => {
            // Get local offset in seconds
            let local_offset = match SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64
                - SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64
            {
                offset => -offset, // Negate because we want local - UTC
            };

            let tz_hour = local_offset / 3600;
            let tz_minute = (local_offset - (tz_hour * 3600)) / 60;

            let (sign, abs_tz_hour) = if tz_hour < 0 {
                ("-", -tz_hour)
            } else {
                ("+", tz_hour)
            };

            let tz_repr = format!("{}{:02}:{:02}", sign, abs_tz_hour, tz_minute);
            format!("ALTER SESSION SET TIME_ZONE = '{}'\x00", tz_repr)
        }
    }
}

impl FrontendMessage for AuthMessageTwo<'_, '_> {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::AuthPhaseTwo;
    fn encode_body_with(
        &self,
        buf: &mut Vec<u8>,
        conn: &mut OracleConnection,
    ) -> Result<(), Error> {
        let mut num_pairs = 4;
        let mut auth_mode = set_auth_mode(conn.inner.auth_mode, false, false);
        // write basic data to packet, same _write_function_code
        write_piggybacks(buf, conn);
        buf.put_u8(TNS_MSG_TYPE_FUNCTION);
        buf.put_u8(Self::FORMAT as u8); // function_code
        buf.put_u8(conn.get_seq_num());
        if conn.inner.caps.ttc_field_version >= TNS_CCAP_FIELD_VERSION_23_1_EXT_1 {
            buf.write_ub8(conn.inner.token_num);
        }
        if self.options.username.len() > 0 {
            buf.put_u8(1);
        }
        buf.write_ub4(self.options.username.as_bytes().len() as u32);

        if self.options.access_token.is_some() {
            num_pairs += 1;
        } else {
            // normal user/password authentication
            num_pairs += 2;
            auth_mode |= TNS_AUTH_MODE_WITH_PASSWORD;
            if self.verifier_type == TNS_VERIFIER_TYPE_12C {
                num_pairs += 1;
            }
            if self.verifier_type != TNS_VERIFIER_TYPE_11G_1
                || self.verifier_type != TNS_VERIFIER_TYPE_11G_2
            {
                return Err(err_protocol!("Unexpectable"));
            }
            generate_verifier(self, buf);
        }

        if self.options.proxy_user.is_some() {
            num_pairs += 1;
        }
        if conn.inner.cclass.is_some() {
            num_pairs += 1;
        }

        if self.options.purity != 0 {
            num_pairs += 1;
        }

        if self.options.get_private_key().is_some() {
            num_pairs += 2;
        }

        if self.options.debug_jdwp.is_some() {
            num_pairs += 1;
        }

        num_pairs += 1; // for connect_string

        buf.write_ub4(auth_mode);
        buf.put_u8(1);
        buf.write_ub4(num_pairs as u32);
        buf.put_u8(1);
        buf.put_u8(1);
        if self.options.username.len() > 0 {
            buf.write_bytes(self.options.username.as_bytes());
        }

        // write key/value pairs
        if let Some(ref proxy_user) = self.options.proxy_user {
            write_key_value(buf, "PROXY_CLIENT_NAME", proxy_user, 0);
        }
        if let Some(AccessToken::OAuth(ref token)) = self.options.access_token {
            // todo!()
            write_key_value(buf, "AUTH_TOKEN", token.as_str(), 0);
        } else {
            let session_key = self.session_data.get("SESSION_KEY").unwrap();
            write_key_value(buf, "AUTH_SESSKEY", session_key, 1);
            if self.verifier_type == TNS_VERIFIER_TYPE_12C {
                let speedy_key = self.session_data.get("SPEEDY_KEY").unwrap();
                write_key_value(buf, "AUTH_PBKDF2_SPEEDY_KEY", speedy_key, 1);
            }
        }
        let encoded_password = &self.options.password; // todo!()
        write_key_value(buf, "AUTH_PASSWORD", encoded_password.as_str(), 0);

        write_key_value(buf, "SESSION_CLIENT_CHARSET", "873", 0);
        write_key_value(
            buf,
            "SESSION_CLIENT_DRIVER_NAME",
            self.options.os_params.driver_name.as_str(),
            0,
        );
        let full_version_num = 1000_000;
        write_key_value(
            buf,
            "SESSION_CLIENT_VERSION",
            full_version_num.to_string().as_str(),
            0,
        );
        write_key_value(
            buf,
            "AUTH_ALTER_SESSION",
            get_alter_timezone_statement().as_str(),
            1,
        );
        if let Some(ref cclass) = conn.inner.cclass {
            write_key_value(buf, "AUTH_KPPL_CONN_CCLASS", cclass, 0);
        }
        let purity = 0;
        if purity != 0 {
            write_key_value(buf, "AUTH_KPPL_PURITY", purity.to_string().as_str(), 1);
        }
        if let Some(ref private_key) = self.options.get_private_key() {
            // Use system time since UNIX_EPOCH to format date manually
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            // Simple RFC 2822 date format approximation
            let date_str = format!("timestamp: {}", now);
            let host_info = conn.get_host_info();
            let host_info = format!("{:?}:{:?}", host_info.0, host_info.1);
            let service_name = self
                .options
                .service_name
                .clone()
                .unwrap_or_default()
                .clone();
            let header = format!(
                "date: {}\n(request-target: {}\nhost: {}",
                date_str, service_name, host_info
            );
            let signature = get_signature(private_key, &header).unwrap();
            write_key_value(buf, "AUTH_HEADER", &header, 0);
            write_key_value(buf, "AUTH_SIGNATURE", signature.as_str(), 0);
        }

        if let Some(ref _debug_jdwp) = self.options.debug_jdwp {
            let encoded_debug_jdwp = "";
            write_key_value(buf, "AUTH_ORA_DEBUG_JDWP", encoded_debug_jdwp, 0);
        }
        if let Some(ConnectionClass::Edition(ref edition)) = self.options.conn_class {
            write_key_value(buf, "AUTH_ORA_EDITION", edition.as_str(), 0);
        }
        write_key_value(buf, "AUTH_CONNECT_STRING", self.options.connect_string(), 0);
        Ok(())
    }
}
