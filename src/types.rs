use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GenericResponse {
    pub status: u16,
    pub message: String,
}

impl GenericResponse {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            status: 200,
            message: message.into(),
        }
    }

    #[allow(dead_code)]
    pub fn error(status: u16, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}