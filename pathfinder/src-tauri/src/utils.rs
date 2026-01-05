use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Toast {
    pub alert_type: String,
    pub text: String,
}

impl Toast {
    pub fn alert_type_to_string(v: u8) -> Result<String, ()> {
        match v {
            0 => Ok("none".to_string()),
            1 => Ok("success".to_string()),
            2 => Ok("info".to_string()),
            3 => Ok("warning".to_string()),
            4 => Ok("danger".to_string()),
            _ => Err(()),
        }
    }
}
