use std::fmt;

#[derive(Debug)]
pub enum CredentialError {
    DatabaseNotFound,
    CredentialNotStored,
    CredentialNotFound(String),
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
    UserCancelled,
}

impl fmt::Display for CredentialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CredentialError::DatabaseNotFound => {
                write!(
                    f,
                    "Database file not found. Use 'add' command to create your first entry."
                )
            }
            CredentialError::CredentialNotStored => {
                write!(f, "No Credentials Stored yet.")
            }
            CredentialError::CredentialNotFound(service) => {
                write!(f, "No credential found for '{}'", service)
            }
            CredentialError::IoError(err) => {
                write!(f, "File operation failed: {}", err)
            }
            CredentialError::SerializationError(err) => {
                write!(f, "Data serialization failed: {}", err)
            }
            CredentialError::UserCancelled => {
                write!(f, "Operation cancelled by user")
            }
        }
    }
}

impl std::error::Error for CredentialError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CredentialError::IoError(err) => Some(err),
            CredentialError::SerializationError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for CredentialError {
    fn from(err: std::io::Error) -> Self {
        CredentialError::IoError(err)
    }
}

impl From<serde_json::Error> for CredentialError {
    fn from(err: serde_json::Error) -> Self {
        CredentialError::SerializationError(err)
    }
}

pub type CredentialResult<T> = Result<T, CredentialError>;

impl CredentialError {
    pub fn credential_not_found(service: &str) -> Self {
        CredentialError::CredentialNotFound(service.to_string())
    }

    pub fn database_not_found() -> Self {
        CredentialError::DatabaseNotFound
    }

    pub fn credentials_not_stored() -> Self {
        CredentialError::CredentialNotStored
    }

    pub fn user_cancelled() -> Self {
        CredentialError::UserCancelled
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            CredentialError::UserCancelled => 100, // Ctrl+C convention
            CredentialError::DatabaseNotFound => 1,
            CredentialError::CredentialNotFound(_) => 2,
            CredentialError::CredentialNotStored => 3,
            CredentialError::IoError(_) => 4,
            CredentialError::SerializationError(_) => 5,
        }
    }
}
