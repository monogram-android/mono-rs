#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitConnectionInfo {
    pub device_model: String,
    pub system_version: String,
    pub app_version: String,
    pub system_lang_code: String,
    pub lang_pack: String,
    pub lang_code: String,
}

impl Default for InitConnectionInfo {
    fn default() -> Self {
        Self {
            device_model: "Android".into(),
            system_version: "14".into(),
            app_version: env!("CARGO_PKG_VERSION").into(),
            system_lang_code: "en".into(),
            lang_pack: "android".into(),
            lang_code: "en".into(),
        }
    }
}

impl InitConnectionInfo {
    pub fn sanitized(self) -> Self {
        self.sanitized_with(&Self::default())
    }

    pub fn sanitized_with(self, fallback: &Self) -> Self {
        Self {
            device_model: nonempty(self.device_model, &fallback.device_model),
            system_version: nonempty(self.system_version, &fallback.system_version),
            app_version: nonempty(self.app_version, &fallback.app_version),
            system_lang_code: nonempty(self.system_lang_code, &fallback.system_lang_code),
            lang_pack: nonempty(self.lang_pack, &fallback.lang_pack),
            lang_code: nonempty(self.lang_code, &fallback.lang_code),
        }
    }
}

fn nonempty(value: String, fallback: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        fallback.to_string()
    } else {
        trimmed.to_string()
    }
}
