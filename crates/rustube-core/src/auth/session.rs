use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub is_signed_in: bool,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            access_token: None,
            refresh_token: None,
            is_signed_in: false,
        }
    }
}
