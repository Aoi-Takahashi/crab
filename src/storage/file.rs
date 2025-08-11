use crate::models::CredentialDatabase;
use std::fs;
use std::path::PathBuf;

pub fn get_database_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Home directory not found")?;

    Ok(home_dir.join(".crab-shell").join("credentials.json"))
}

pub fn save_database(database: &CredentialDatabase) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_database_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json_data = serde_json::to_string_pretty(database)?;

    fs::write(&path, json_data)?;

    println!("✅ Database saved: {}", path.display());
    Ok(())
}

pub fn load_database() -> Result<CredentialDatabase, Box<dyn std::error::Error>> {
    let path = get_database_path()?;

    if !path.exists() {
        println!("ℹ️  Database file not found. Creating a new database.");
        return Ok(CredentialDatabase::new());
    }

    let json_data = fs::read_to_string(&path)?;

    let database: CredentialDatabase = serde_json::from_str(&json_data)?;

    println!("✅ Database loaded: {}", path.display());
    Ok(database)
}

pub fn database_exists() -> bool {
    if let Ok(path) = get_database_path() {
        path.exists()
    } else {
        false
    }
}

pub fn delete_database() -> Result<(), Box<dyn std::error::Error>> {
    let path = get_database_path()?;

    if path.exists() {
        fs::remove_file(&path)?;
        println!("✅ Database file deleted: {}", path.display());
    } else {
        println!("ℹ️  Database file not found");
    }

    Ok(())
}

pub fn get_database_info() -> Result<std::fs::Metadata, Box<dyn std::error::Error>> {
    let path = get_database_path()?;
    let metadata = fs::metadata(&path)?;
    Ok(metadata)
}

pub fn backup_database() -> Result<(), Box<dyn std::error::Error>> {
    let path = get_database_path()?;

    if !path.exists() {
        return Err("Database file not found for backup".into());
    }

    let backup_path = path.with_extension("json.bak");
    fs::copy(&path, &backup_path)?;

    println!("✅ Database backup created: {}", backup_path.display());
    Ok(())
}
