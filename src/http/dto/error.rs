use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub errors: ErrorBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorBody {
    pub body: Vec<String>,
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        Self {
            errors: ErrorBody {
                body: vec![message],
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationErrorResponse {
    pub errors: HashMap<String, Vec<String>>,
}
