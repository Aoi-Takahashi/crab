use crate::models::CredentialDatabase;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn get_database_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("Home directory not found")?;
    Ok(home_dir.join(".crab-shell").join("credentials.json"))
}

pub fn save_database(database: &CredentialDatabase) -> Result<()> {
    let path = get_database_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    let json_data =
        serde_json::to_string_pretty(database).context("Failed to serialize database")?;

    fs::write(&path, json_data)
        .with_context(|| format!("Failed to write database file: {}", path.display()))?;

    Ok(())
}

pub fn load_database() -> Result<CredentialDatabase> {
    let path = get_database_path()?;

    if !path.exists() {
        return Ok(CredentialDatabase::new());
    }

    let json_data = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read database file: {}", path.display()))?;

    let database: CredentialDatabase =
        serde_json::from_str(&json_data).context("Failed to parse database file")?;

    Ok(database)
}

pub fn database_exists() -> bool {
    if let Ok(path) = get_database_path() {
        path.exists()
    } else {
        false
    }
}

pub fn delete_database() -> Result<()> {
    let path = get_database_path()?;

    if path.exists() {
        fs::remove_file(&path)
            .with_context(|| format!("Failed to delete database file: {}", path.display()))?;
        println!("✅ Database file deleted: {}", path.display());
    } else {
        println!("ℹ️  Database file not found");
    }

    Ok(())
}

pub fn get_database_info() -> Result<std::fs::Metadata> {
    let path = get_database_path()?;
    let metadata = fs::metadata(&path)
        .with_context(|| format!("Failed to get file metadata: {}", path.display()))?;
    Ok(metadata)
}

pub fn backup_database() -> Result<()> {
    let path = get_database_path()?;

    if !path.exists() {
        anyhow::bail!("Database file not found for backup");
    }

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .context("Failed to get system time")?
        .as_secs();

    let backup_filename = format!("credentials_{}.json.bak", timestamp);
    let backup_path = path.with_file_name(backup_filename);

    fs::copy(&path, &backup_path).with_context(|| {
        format!(
            "Failed to create backup: {} -> {}",
            path.display(),
            backup_path.display()
        )
    })?;

    println!("✅ Database backup created: {}", backup_path.display());
    Ok(())
}
