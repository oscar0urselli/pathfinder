use std::{fmt, io, fs, path::Path};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Toast {
    pub r#type: ToastType,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub enum ToastType {
    Danger,
    Warning,
    Info,
    Success,
    None
}

impl fmt::Display for ToastType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Danger => "Danger",
            Self::Warning => "Warning",
            Self::Info => "Info",
            Self::Success => "Success",
            Self::None => "None"
        })?;
        Ok(())
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

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}