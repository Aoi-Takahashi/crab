use crate::model::CredentialEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialDatabase {
    pub entries: Vec<CredentialEntry>,
    pub version: String,
}

impl CredentialDatabase {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            version: String::from("1.0"), // TODO: explicit versioning
        }
    }

    pub fn add_entry(&mut self, entry: CredentialEntry) {
        self.entries.push(entry);
    }

    pub fn remove_entry(&mut self, service: &str) -> bool {
        let initial_len = self.entries.len();
        self.entries.retain(|entry| entry.service != service);
        self.entries.len() < initial_len
    }

    pub fn find_entry(&self, service: &str) -> Option<&CredentialEntry> {
        self.entries.iter().find(|entry| entry.service == service)
    }

    pub fn edit_entry(&mut self, service: &str) -> Option<&mut CredentialEntry> {
        self.entries
            .iter_mut()
            .find(|entry| entry.service == service)
    }

    pub fn list_services(&self) -> Vec<&str> {
        self.entries
            .iter()
            .map(|entry| entry.service.as_str())
            .collect()
    }

    // NOTE: For Debug
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
