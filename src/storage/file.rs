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
