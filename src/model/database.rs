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

    pub fn remove_entry(&mut self, service: &str) {
        self.entries.retain(|entry| entry.service != service);
    }

    pub fn find_entry(&self, service: &str) -> Option<&CredentialEntry> {
        self.entries.iter().find(|entry| entry.service == service)
    }

    pub fn edit_entry(&mut self, service: &str) -> Option<&mut CredentialEntry> {
        self.entries
            .iter_mut()
            .find(|entry| entry.service == service)
    }

    pub fn list_entries(&self) -> Vec<&CredentialEntry> {
        self.entries.iter().collect()
    }

    // NOTE: For Debug
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entry(service: &str) -> CredentialEntry {
        CredentialEntry::new(
            service.to_string(),
            "account".to_string(),
            "secret".to_string(),
        )
    }

    #[test]
    fn add_entry_success() {
        let mut database = CredentialDatabase::new();
        let service = "github";
        let entry = sample_entry(&service);

        database.add_entry(entry);

        let result = database.find_entry(&service).expect("entry should exist");
        assert_eq!(result.service, "github");
        assert_eq!(result.account, "account");
        assert_eq!(result.secret, "secret");
    }

    #[test]
    fn remove_entry_by_exist_service() {
        let mut database = CredentialDatabase::new();
        let service = "github";
        database.add_entry(sample_entry(&service));

        database.remove_entry(&service);

        assert!(database.find_entry("github").is_none());
    }

    #[test]
    fn remove_entry_by_missing_service() {
        let mut database = CredentialDatabase::new();
        let service = "github";
        database.add_entry(sample_entry(&service));
        database.remove_entry("not-there-service");

        assert_eq!(database.len(), 1);
    }

    #[test]
    fn edit_entry() {
        let mut database = CredentialDatabase::new();
        let service = "github";
        database.add_entry(sample_entry(&service));

        let entry = database.edit_entry(&service).expect("entry should exist");
        entry.update_account("update-account".to_string());
        entry.update_secret("update-secret".to_string());
        let result = database.find_entry(&service).expect("entry should exist");

        assert_eq!(result.service, "github");
        assert_eq!(result.account, "update-account");
        assert_eq!(result.secret, "update-secret");
    }

    #[test]
    fn list_services() {
        let mut database = CredentialDatabase::new();
        database.add_entry(sample_entry("service-a"));
        database.add_entry(sample_entry("service-b"));

        let entries = database.list_entries();
        let services: Vec<&str> = entries.iter().map(|entry| entry.service.as_str()).collect();

        assert_eq!(services, vec!["service-a", "service-b"]);
    }
}
