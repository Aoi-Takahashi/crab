use crate::models::{CredentialDatabase, CredentialEntry};
use crate::storage::{database_exists, load_database, save_database};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::{Confirm, Input, Password};
