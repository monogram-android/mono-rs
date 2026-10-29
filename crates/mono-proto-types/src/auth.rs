#[derive(Debug, Clone)]
pub struct AuthCodeSent {
    pub phone: String,
    pub phone_code_hash: String,
    pub code_type: String,
    pub code_length: i32,
}

#[derive(Debug, Clone)]
pub struct AuthSignedIn {
    pub user_id: i64,
    pub dc_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceSessionDto {
    pub hash: i64,
    pub current: bool,
    pub password_pending: bool,
    pub device_model: String,
    pub platform: String,
    pub system_version: String,
    pub app_name: String,
    pub app_version: String,
    pub date_active: i32,
    pub ip: String,
    pub country: String,
    pub region: String,
}

#[derive(Clone)]
pub enum QrLoginDto {
    Token { token: Vec<u8>, expires: i32 },
    Authorized { session: AuthSignedIn },
}

#[derive(Clone)]
pub struct PasskeyCredentialDto {
    pub id: String,
    pub raw_id: String,
    pub client_data: String,
    pub authenticator_data: Vec<u8>,
    pub signature: Vec<u8>,
    pub user_handle: String,
    pub attestation_data: Vec<u8>,
}

#[derive(Clone)]
pub struct PasskeyLoginOptionsDto {
    pub options: String,
    pub dc_id: i32,
    pub auth_key_id: i64,
}

#[derive(Clone)]
pub struct PasskeyDto {
    pub id: String,
    pub name: String,
    pub date: i32,
    pub software_emoji_id: Option<i64>,
    pub last_usage_date: Option<i32>,
}
