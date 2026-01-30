use std::fmt;

use duckdb::params;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub r#type: LogType,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub enum LogType {
    Error,
    Warn,
    Info,
    Debug,
    Trace
}

impl fmt::Display for LogType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Error => "Error",
            Self::Warn => "Warn",
            Self::Info => "Info",
            Self::Debug => "Debug",
            Self::Trace => "Trace"
        })?;
        Ok(())
    }
}

impl Log {
    pub fn new(r#type: LogType, message: String) -> Self {
        Self {
            r#type,
            message
        }
    }
    
    pub fn insert(&self, db: &duckdb::Connection) -> Result<usize, ()> {
        match db.execute("INSERT INTO logs (ts, type, message) VALUES (now(), ?, ?);", params![self.r#type.to_string(), self.message]) {
            Ok(v) => Ok(v),
            Err(_) => Err(())
        }
    }
}

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