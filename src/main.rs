mod cli;
mod error;
mod model;
mod storage;
mod util;

use clap::Parser;
use cli::commands::Cli;
use error::CredentialError;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = cli.command.execute() {
        match &e {
            CredentialError::UserCancelled => {
                println!("ℹ️  Operation cancelled.");
            }
            CredentialError::DatabaseNotFound => {
                eprintln!("❌  {}", e);
                eprintln!("💡 Try running 'crab add' to create your first credential.");
            }
            CredentialError::CredentialNotFound(service) => {
                eprintln!("❌ {}", e);
                eprintln!(
                    "💡 Try 'crab list' to see available services or 'crab add {}' to create it.",
                    service
                );
            }
            _ => {
                eprintln!("❌ Error: {}", e);
            }
        }

        std::process::exit(e.exit_code());
    }
}
