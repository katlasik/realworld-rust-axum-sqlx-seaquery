use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

const MIN_PASSWORD_LENGTH: usize = 8;
const MAX_PASSWORD_LENGTH: usize = 128;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Password(String);

impl Password {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Password {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("Password cannot be empty".to_string());
        }

        if value.len() < MIN_PASSWORD_LENGTH {
            return Err(format!(
                "Password must be at least {} characters long",
                MIN_PASSWORD_LENGTH
            ));
        }

        if value.len() > MAX_PASSWORD_LENGTH {
            return Err(format!(
                "Password cannot be longer than {} characters",
                MAX_PASSWORD_LENGTH
            ));
        }

        Ok(Password(value))
    }
}

impl TryFrom<&str> for Password {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl From<Password> for String {
    fn from(password: Password) -> String {
        password.0
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "********")
    }
}

impl Deref for Password {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
