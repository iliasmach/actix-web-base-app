
use diesel::result::Error as DieselError;
use std::fmt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InfraError {
    pub error_status_code: u16,
    pub error_message: String,
}

impl InfraError {
    pub fn new(error_status_code: u16, error_message: String) -> InfraError {
        InfraError {
            error_status_code,
            error_message,
        }
    }
}

impl fmt::Display for InfraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}

impl From<DieselError> for InfraError {
    fn from(error: DieselError) -> InfraError {
        match error {
            DieselError::DatabaseError(_, err) => InfraError::new(409, err.message().to_string()),
            DieselError::NotFound => {
                InfraError::new(404, "The record not found".to_string())
            }
            err => InfraError::new(500, format!("Unknown Diesel error: {}", err)),
        }
    }
}