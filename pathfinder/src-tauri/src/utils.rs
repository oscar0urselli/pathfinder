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


#[derive(Serialize, Deserialize)]
pub struct Modal {
    pub title: String,
    pub r#type: ModalType,
    pub data: ModalData,
    pub plugin: Option<String>
}

#[derive(Serialize, Deserialize)]
pub enum ModalType {
    PluginForm
}

#[derive(Serialize, Deserialize)]
pub enum ModalData {
    PluginForm { config: Vec<Vec<PluginFormField>> }
}

#[derive(Serialize, Deserialize)]
pub struct PluginFormField {
    name: String,
    title: String,
    r#type: String,
    options: Option<Vec<String>>,
    min: Option<String>,
    max: Option<String>,
    step: Option<String>,
    regex: Option<String>,
    default: Option<String>
}