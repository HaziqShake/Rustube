#[derive(Debug, Clone)]
pub struct DeviceFlowResponse {
    pub user_code: String,
    pub verification_url: String,
    pub device_code: String,
}

pub struct GoogleAuth;

impl GoogleAuth {
    pub fn new() -> Self {
        Self
    }

    pub async fn start_device_flow(&self) -> anyhow::Result<DeviceFlowResponse> {
        Ok(DeviceFlowResponse {
            user_code: "ABCD-EFGH".to_string(),
            verification_url: "https://www.google.com/device".to_string(),
            device_code: "dev_12345".to_string(),
        })
    }
}

impl Default for GoogleAuth {
    fn default() -> Self {
        Self::new()
    }
}
