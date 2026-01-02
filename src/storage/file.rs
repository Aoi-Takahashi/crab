use crate::error::{CredentialError, CredentialResult};
use crate::model::CredentialDatabase;
use std::fs;
use std::path::PathBuf;

pub fn get_database_path() -> CredentialResult<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        CredentialError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Home directory not found",
        ))
    })?;
    Ok(home_dir.join(".crab").join("credentials.json"))
}

pub fn save_database(database: &CredentialDatabase) -> CredentialResult<()> {
    let path = get_database_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json_data = serde_json::to_string_pretty(database)?;

    fs::write(&path, json_data)?;

    Ok(())
}

pub fn load_database() -> CredentialResult<CredentialDatabase> {
    let path = get_database_path()?;

    if !path.exists() {
        return Ok(CredentialDatabase::new());
    }

    let json_data = fs::read_to_string(&path)?;

    let database: CredentialDatabase = serde_json::from_str(&json_data)?;

    Ok(database)
}

pub fn database_exists() -> bool {
    if let Ok(path) = get_database_path() {
        path.exists()
    } else {
        false
    }
}

pub fn delete_database() -> CredentialResult<()> {
    let path = get_database_path()?;

    if path.exists() {
        fs::remove_file(&path)?;
        println!("✅ Database file deleted: {}", path.display());
    } else {
        return Err(CredentialError::database_not_found());
    }

    Ok(())
}

pub fn get_database_info() -> CredentialResult<std::fs::Metadata> {
    let path = get_database_path()?;
    let metadata = fs::metadata(&path)?;
    Ok(metadata)
}

pub fn backup_database() -> CredentialResult<()> {
    let path = get_database_path()?;

    if !path.exists() {
        return Err(CredentialError::database_not_found());
    }

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| {
            CredentialError::IoError(std::io::Error::other(format!(
                "Failed to get system time: {e}"
            )))
        })?
        .as_secs();

    let backup_filename = format!("credentials_{timestamp}.json.bak");
    let backup_path = path.with_file_name(backup_filename);

    fs::copy(&path, &backup_path)?;

    println!("✅ Database backup created: {}", backup_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::sync::{Mutex, OnceLock};

    fn home_env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .expect("lock poisoned")
    }

    struct HomeGuard {
        original_home: Option<String>,
        original_userprofile: Option<String>,
    }

    impl HomeGuard {
        fn new(temp_home: &Path) -> Self {
            let original_home = std::env::var("HOME").ok();
            let original_userprofile = std::env::var("USERPROFILE").ok();
            std::env::set_var("HOME", temp_home);
            std::env::set_var("USERPROFILE", temp_home);
            Self {
                original_home,
                original_userprofile,
            }
        }
    }

    impl Drop for HomeGuard {
        fn drop(&mut self) {
            match &self.original_home {
                Some(value) => std::env::set_var("HOME", value),
                None => std::env::remove_var("HOME"),
            }
            match &self.original_userprofile {
                Some(value) => std::env::set_var("USERPROFILE", value),
                None => std::env::remove_var("USERPROFILE"),
            }
        }
    }

    fn sample_database() -> CredentialDatabase {
        let mut database = CredentialDatabase::new();
        let entry = crate::model::CredentialEntry::new(
            "service".to_string(),
            "account".to_string(),
            "secret".to_string(),
        );
        database.add_entry(entry);
        database
    }

    #[test]
    fn save_and_load_database_round_trip() {
        let _lock = home_env_lock();
        let temp_dir = tempfile::tempdir().expect("tempdir");
        let _guard = HomeGuard::new(temp_dir.path());
        let database = sample_database();

        save_database(&database).expect("save should succeed");
        let loaded = load_database().expect("load should succeed");

        assert_eq!(loaded.len(), 1);
        let entry = loaded.find_entry("service").expect("entry should exist");
        assert_eq!(entry.account, "account");
    }

    #[test]
    fn database_exists_and_delete_behave_as_expected() {
        let _lock = home_env_lock();
        let temp_dir = tempfile::tempdir().expect("tempdir");
        let _guard = HomeGuard::new(temp_dir.path());

        assert!(!database_exists());

        let database = sample_database();
        save_database(&database).expect("save should succeed");
        assert!(database_exists());

        delete_database().expect("delete should succeed");
        assert!(!database_exists());
    }

    #[test]
    fn backup_database_creates_bak_file() {
        let _lock = home_env_lock();
        let temp_dir = tempfile::tempdir().expect("tempdir");
        let _guard = HomeGuard::new(temp_dir.path());

        let database = sample_database();
        save_database(&database).expect("save should succeed");
        backup_database().expect("backup should succeed");

        let database_path = get_database_path().expect("path should exist");
        let parent = database_path.parent().expect("parent dir");
        let backup_count = std::fs::read_dir(parent)
            .expect("read dir")
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "bak"))
            .count();

        assert_eq!(backup_count, 1);
    }

    #[test]
    fn backup_database_errors_when_missing() {
        let _lock = home_env_lock();
        let temp_dir = tempfile::tempdir().expect("tempdir");
        let _guard = HomeGuard::new(temp_dir.path());

        let result = backup_database();

        assert!(matches!(result, Err(CredentialError::DatabaseNotFound)));
    }
}
