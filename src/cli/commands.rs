use crate::error::{CredentialError, CredentialResult};
use crate::model::CredentialEntry;
use crate::storage::{
    backup_database, database_exists, delete_database, get_database_info, load_database,
    save_database,
};
use crate::util::format_timestamp_local;
use clap::{Parser, Subcommand};
use dialoguer::{Confirm, Input, Password};

#[derive(Parser)]
#[command(name = "crab")]
#[command(
    about = "A secure credential manager for storing sensitive information",
    version
)]
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
    Delete,
}

impl Commands {
    pub fn execute(self) -> CredentialResult<()> {
        match self {
            Commands::Add { service, account } => add_credential(service, account),
            Commands::Get { service } => get_credential(&service),
            Commands::List => list_credentials(),
            Commands::Edit { service } => edit_credential(&service),
            Commands::Remove { service } => remove_credential(&service),
            Commands::Info => show_credential(),
            Commands::Backup => backup_database(),
            Commands::Delete => delete_credential(),
        }
    }
}

fn add_credential(service: Option<String>, account: Option<String>) -> CredentialResult<()> {
    let mut database = load_database()?;

    let service_name = match service {
        Some(s) => s,
        None => Input::new()
            .with_prompt("Service name")
            .interact_text()
            .map_err(|_| CredentialError::user_cancelled())?,
    };

    if database.find_entry(&service_name).is_some() {
        println!("⚠️ Service '{service_name}' already exists!");
        let overwrite = Confirm::new()
            .with_prompt("Do you want to overwrite it?")
            .interact()
            .map_err(|_| CredentialError::user_cancelled())?;

        if !overwrite {
            println!("Operation cancelled.");
            return Ok(());
        }
        database.remove_entry(&service_name);
    }

    let account_name = match account {
        Some(a) => a,
        None => Input::new()
            .with_prompt("Please Enter Account Name")
            .interact_text()
            .map_err(|_| CredentialError::user_cancelled())?,
    };

    let secret = Password::new()
        .with_prompt("Please Enter Secret")
        .with_confirmation("Confirm Secret", "Secrets don't match")
        .interact()
        .map_err(|_| CredentialError::user_cancelled())?;

    let entry = CredentialEntry::new(service_name.clone(), account_name, secret);
    database.add_entry(entry);

    save_database(&database)?;

    println!("✅ Credential for '{service_name}' added successfully!");
    Ok(())
}

fn get_credential(service: &str) -> CredentialResult<()> {
    let database = load_database()?;

    match database.find_entry(service) {
        Some(entry) => {
            println!("📋 Credential found:");
            println!("  Service: {}", entry.service);
            println!("  Account: {}", entry.account);
            println!("  Secret: {}", entry.secret);
            println!("  Created: {}", format_timestamp_local(entry.created_at));
            println!("  Updated: {}", format_timestamp_local(entry.updated_at));
            Ok(())
        }
        None => Err(CredentialError::credential_not_found(service)),
    }
}

fn list_credentials() -> CredentialResult<()> {
    let database = load_database()?;

    let services = database.list_services();

    if services.is_empty() {
        Err(CredentialError::credentials_not_stored())?
    } else {
        println!("📋 Stored Credentials ({} entries):", services.len());
        for (i, service) in services.iter().enumerate() {
            println!("  {}. {service}", i + 1);
        }
    }
    Ok(())
}

fn edit_credential(service: &str) -> CredentialResult<()> {
    let mut database = load_database()?;

    match database.edit_entry(service) {
        Some(entry) => {
            println!("📝 Editing Credential for '{service}'");
            println!("Current values:");
            println!("  Service: {}", entry.service);
            println!("  Account: {}", entry.account);

            let new_service: String = Input::new()
                .with_prompt("New Service Name")
                .default(entry.service.clone())
                .interact_text()
                .map_err(|_| CredentialError::user_cancelled())?;

            let new_account: String = Input::new()
                .with_prompt("New Account")
                .default(entry.account.clone())
                .interact_text()
                .map_err(|_| CredentialError::user_cancelled())?;

            let change_secret = Confirm::new()
                .with_prompt("Change Secret?")
                .interact()
                .map_err(|_| CredentialError::user_cancelled())?;

            if new_service != entry.service {
                entry.update_service(new_service);
            }
            if new_account != entry.account {
                entry.update_account(new_account);
            }

            if change_secret {
                let new_secret = Password::new()
                    .with_prompt("New Secret")
                    .with_confirmation("Confirm Secret", "Secrets don't match")
                    .interact()
                    .map_err(|_| CredentialError::user_cancelled())?;
                entry.update_secret(new_secret);
            }

            save_database(&database)?;

            println!("✅ Credential Updated Successfully!");
            Ok(())
        }
        None => Err(CredentialError::credential_not_found(service)),
    }
}

fn remove_credential(service: &str) -> CredentialResult<()> {
    let mut database = load_database()?;

    if database.find_entry(service).is_none() {
        return Err(CredentialError::credential_not_found(service));
    }

    let confirm = Confirm::new()
        .with_prompt(format!("Are you sure you want to remove '{service}'?"))
        .interact()
        .map_err(|_| CredentialError::user_cancelled())?;

    if confirm && database.remove_entry(service) {
        save_database(&database)?;
        println!("✅ Credential for '{service}' removed successfully!");
    }
    Ok(())
}

fn show_credential() -> CredentialResult<()> {
    if !database_exists() {
        return Err(CredentialError::database_not_found());
    }

    let database = load_database()?;

    println!("📊 Database Information:");
    println!("  Version: {}", database.version);
    println!("  Entries: {}", database.len());

    match get_database_info() {
        Ok(metadata) => {
            println!("  File size: {} bytes", metadata.len());
            if let Ok(modified) = metadata.modified() {
                println!("  Last modified: {modified:?}");
            }
        }
        Err(e) => {
            println!("  Failed to get file info: {e}");
        }
    }

    if let Ok(path) = crate::storage::file::get_database_path() {
        println!("  Location: {}", path.display());
    }

    Ok(())
}

fn delete_credential() -> CredentialResult<()> {
    if !database_exists() {
        return Err(CredentialError::database_not_found());
    }

    let database = load_database()?;
    println!("⚠️  You are about to delete the entire database!");
    println!("📊 Current database contains {} entries", database.len());

    let confirm = Confirm::new()
        .with_prompt("Are you sure you want to delete the ENTIRE database? This cannot be undone!")
        .interact()
        .map_err(|_| CredentialError::user_cancelled())?;

    if confirm {
        let create_backup = Confirm::new()
            .with_prompt("Create a backup before deletion?")
            .default(true)
            .interact()
            .map_err(|_| CredentialError::user_cancelled())?;

        if create_backup {
            crate::storage::file::backup_database()?;
        }

        delete_database()?;

        println!("🗑️  Database deleted successfully!");
    }

    Ok(())
}
