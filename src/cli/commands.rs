use crate::models::CredentialEntry;
use crate::storage::{database_exists, load_database, save_database};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::{Confirm, Input, Password};

#[derive(Parser)]
#[command(name = "crab")]
#[command(about = "A secure credential manager for storing sensitive information")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        service: Option<String>,
        #[arg(short, long)]
        account: Option<String>,
    },
    Get {
        service: String,
    },
    List,
    Edit {
        service: String,
    },
    Remove {
        service: String,
    },
    Info,
    Backup,
}

impl Commands {
    pub fn execute(self) -> Result<()> {
        match self {
            Commands::Add { service, account } => add_credential(service, account),
            Commands::Get { service } => get_credential(&service),
            Commands::List => list_credentials(),
            Commands::Edit { service } => edit_credential(&service),
            Commands::Remove { service } => remove_credential(&service),
            Commands::Info => show_database_info(),
            Commands::Backup => backup_database(),
        }
    }
}

fn add_credential(service: Option<String>, account: Option<String>) -> Result<()> {
    let mut database = load_database().context("Failed to load database")?;

    let service_name = match service {
        Some(s) => s,
        None => Input::new()
            .with_prompt("Service name")
            .interact_text()
            .context("Failed to get service name")?,
    };

    if database.find_entry(&service_name).is_some() {
        println!("⚠️  Service '{}' already exists!", service_name);
        let overwrite = Confirm::new()
            .with_prompt("Do you want to overwrite it?")
            .interact()
            .context("Failed to get confirmation")?;

        if !overwrite {
            println!("Operation cancelled.");
            return Ok(());
        }
        database.remove_entry(&service_name);
    }

    let account_name = match account {
        Some(a) => a,
        None => Input::new()
            .with_prompt("Account/Username")
            .interact_text()
            .context("Failed to get account name")?,
    };

    let secret = Password::new()
        .with_prompt("Password/Secret")
        .with_confirmation("Confirm password", "Passwords don't match")
        .interact()
        .context("Failed to get password")?;

    let entry = CredentialEntry::new(service_name.clone(), account_name, secret);
    database.add_entry(entry);

    save_database(&database).context("Failed to save database")?;

    println!("✅ Credential for '{}' added successfully!", service_name);
    Ok(())
}

fn get_credential(service: &str) -> Result<()> {
    let database = load_database().context("Failed to load database")?;

    match database.find_entry(service) {
        Some(entry) => {
            println!("📋 Credential found:");
            println!("  Service: {}", entry.service);
            println!("  Account: {}", entry.account);

            let show_secret = Confirm::new()
                .with_prompt("Show password/secret?")
                .interact()
                .context("Failed to get confirmation")?;

            if show_secret {
                println!("  Secret: {}", entry.secret);
            }

            let created = std::time::UNIX_EPOCH + std::time::Duration::from_secs(entry.created_at);
            let updated = std::time::UNIX_EPOCH + std::time::Duration::from_secs(entry.updated_at);
            println!("  Created: {:?}", created);
            println!("  Updated: {:?}", updated);
        }
        None => {
            println!("❌ No credential found for service '{}'", service);
        }
    }
    Ok(())
}

fn list_credentials() -> Result<()> {
    let database = load_database().context("Failed to load database")?;

    let services = database.list_services();

    if services.is_empty() {
        println!("📭 No credentials stored yet.");
    } else {
        println!("📋 Stored credentials ({} entries):", services.len());
        for (i, service) in services.iter().enumerate() {
            println!("  {}. {}", i + 1, service);
        }
    }
    Ok(())
}

fn edit_credential(service: &str) -> Result<()> {
    let mut database = load_database().context("Failed to load database")?;

    match database.edit_entry(service) {
        Some(entry) => {
            println!("📝 Editing credential for '{}'", service);
            println!("Current values:");
            println!("  Service: {}", entry.service);
            println!("  Account: {}", entry.account);

            let new_service: String = Input::new()
                .with_prompt("New service name")
                .default(entry.service.clone())
                .interact_text()
                .context("Failed to get new service name")?;

            let new_account: String = Input::new()
                .with_prompt("New account/username")
                .default(entry.account.clone())
                .interact_text()
                .context("Failed to get new account name")?;

            let change_password = Confirm::new()
                .with_prompt("Change password/secret?")
                .interact()
                .context("Failed to get confirmation")?;

            if new_service != entry.service {
                entry.update_service(new_service);
            }
            if new_account != entry.account {
                entry.update_account(new_account);
            }

            if change_password {
                let new_secret = Password::new()
                    .with_prompt("New password/secret")
                    .with_confirmation("Confirm password", "Passwords don't match")
                    .interact()
                    .context("Failed to get new password")?;
                entry.update_secret(new_secret);
            }

            save_database(&database).context("Failed to save database")?;

            println!("✅ Credential updated successfully!");
        }
        None => {
            println!("❌ No credential found for service '{}'", service);
        }
    }
    Ok(())
}

fn remove_credential(service: &str) -> Result<()> {
    let mut database = load_database().context("Failed to load database")?;

    if database.find_entry(service).is_none() {
        println!("❌ No credential found for service '{}'", service);
        return Ok(());
    }

    let confirm = Confirm::new()
        .with_prompt(&format!("Are you sure you want to remove '{}'?", service))
        .interact()
        .context("Failed to get confirmation")?;

    if confirm {
        if database.remove_entry(service) {
            save_database(&database).context("Failed to save database")?;
            println!("✅ Credential for '{}' removed successfully!", service);
        } else {
            println!("❌ Failed to remove credential for '{}'", service);
        }
    } else {
        println!("Operation cancelled.");
    }
    Ok(())
}

fn show_database_info() -> Result<()> {
    if !database_exists() {
        println!("📭 No database file exists yet.");
        return Ok(());
    }

    let database = load_database().context("Failed to load database")?;

    println!("📊 Database Information:");
    println!("  Version: {}", database.version);
    println!("  Entries: {}", database.len());

    match crate::storage::file::get_database_info() {
        Ok(metadata) => {
            println!("  File size: {} bytes", metadata.len());
            if let Ok(modified) = metadata.modified() {
                println!("  Last modified: {:?}", modified);
            }
        }
        Err(e) => {
            println!("  Failed to get file info: {}", e);
        }
    }

    if let Ok(path) = crate::storage::file::get_database_path() {
        println!("  Location: {}", path.display());
    }

    Ok(())
}

fn backup_database() -> Result<()> {
    crate::storage::file::backup_database().context("Failed to create backup")?;
    Ok(())
}
