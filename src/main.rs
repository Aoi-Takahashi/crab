mod cli;
mod models;
mod storage;

use clap::Parser;
use cli::commands::Cli;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = cli.command.execute() {
        eprintln!("❌ Error: {}", e);

        #[cfg(debug_assertions)]
        {
            eprintln!("Debug info: {:?}", e);
        }

        std::process::exit(1);
    }
}
