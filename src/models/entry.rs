use serde::{Deserialize,Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialEntry {
    pub service: String,
    pub account: String,
    pub secret: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl CredentialEntry{
    pub fn new(service: String, account: String, secret: String) -> Self {
       let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        CredentialEntry {
            service,
            account,
            secret,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_service(&mut self, new_service: String) {
        self.service = new_service;
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn update_account(&mut self, new_account: String) {
        self.account = new_account;
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn update_secret(&mut self, new_secret: String) {
        self.secret = new_secret;
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    
}