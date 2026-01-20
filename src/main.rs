use clap::{Parser, Subcommand};
use std::env;

mod commands;
mod ssh;
use crate::ssh::registry::SSHRegistry;
use crate::ssh::utils::generate_ssh_keypair;

const SSH_KEY_DIR: &str = ".ssh";
const SSH_KEY_NAME: &str = "id_rsa_shellman";

#[derive(Parser, Debug)]
#[command(name = "shellman", author, version, about = "Easily manage SSH connections", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Connect to an existing entry by id
    Use {
        /// ID (number starting from 1)
        #[arg(value_parser = parse_id)]
        id: u32,
        #[arg(long, default_value_t = false)]
        sftp: bool,
    },

    /// Add a new entry (example: name)
    Add {
        /// User for the new entry
        #[arg(value_parser = parse_user)]
        user: String,
        /// Host for the new entry
        #[arg(value_parser = parse_host)]
        host: String,
        /// Name for the new entry
        #[arg(long, default_value_t = String::new())]
        name: String,
        /// Existing SSH public key path
        #[arg(long)]
        existing_ssh_key: Option<String>,
    },

    /// Remove an entry by id
    Rm {
        /// ID to remove
        #[arg(value_parser = parse_id)]
        id: u32,
    },

    /// List entries
    Ls,
}

fn parse_user(s: &str) -> Result<String, String> {
    if s.is_empty() {
        Err("user cannot be empty".into())
    } else {
        Ok(s.to_string())
    }
}

// Domain validation
fn is_valid_domain(domain: &str) -> bool {
    let domain_regex =
        regex::Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$")
            .unwrap();
    domain_regex.is_match(domain)
}

fn parse_host(s: &str) -> Result<String, String> {
    // Should be a valid domain or IP address

    if s.is_empty() {
        Err("host cannot be empty".into())
    } else if s.parse::<std::net::IpAddr>().is_ok() || is_valid_domain(s) {
        Ok(s.to_string())
    } else {
        Err("host must be a valid domain or IP address".into())
    }
}

fn parse_id(s: &str) -> Result<u32, String> {
    match s.parse::<u32>() {
        Ok(n) if n >= 1 => Ok(n),
        Ok(_) => Err("id must be >= 1".into()),
        Err(e) => Err(format!("invalid id: {}", e)),
    }
}

fn ssh_key_path() -> String {
    format!(
        "{}/{}/{}",
        env::var("HOME").unwrap(),
        SSH_KEY_DIR,
        SSH_KEY_NAME
    )
}

fn ssh_pub_key_path() -> String {
    format!(
        "{}/{}/{}.pub",
        env::var("HOME").unwrap(),
        SSH_KEY_DIR,
        SSH_KEY_NAME
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut config = SSHRegistry::load()?;

    if !std::path::Path::new(&ssh_key_path()).exists() {
        println!(
            "Generating SSH key pair at ~/{}/{}...\n",
            SSH_KEY_DIR, SSH_KEY_NAME
        );
        generate_ssh_keypair(&ssh_key_path())?;
    }

    match cli.command {
        Commands::Use { id, sftp } => {
            println!("Connecting to id {}", id);
            commands::handle_use(&config, id, sftp, &ssh_key_path())?;
        }
        Commands::Add {
            name,
            user,
            host,
            existing_ssh_key,
        } => {
            commands::handle_add(
                &mut config,
                user,
                host,
                name,
                existing_ssh_key,
                Some(&ssh_pub_key_path()),
            )?;
        }
        Commands::Rm { id } => {
            commands::handle_rm(&mut config, id)?;
        }
        Commands::Ls {} => {
            commands::handle_ls(&config);
        }
    }

    Ok(())
}
