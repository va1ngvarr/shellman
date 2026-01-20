use comfy_table::{Table, presets::UTF8_FULL};

use crate::ssh::registry::SSHRegistry;
use crate::ssh::utils::{SSHProtocol, connect_secure_server, copy_ssh_key, remove_ssh_key};

pub fn handle_use(
    registry: &SSHRegistry,
    id: u32,
    sftp: bool,
    private_key_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let server = registry
        .servers()
        .iter()
        .find(|s| s.id == id)
        .ok_or(format!("server with id {} not found", id))?;

    connect_secure_server(
        &server.host,
        &server.user,
        if sftp {
            SSHProtocol::SFTP
        } else {
            SSHProtocol::SSH
        },
        server
            .existing_ssh_key
            .as_deref()
            .unwrap_or(private_key_path),
    );

    Ok(())
}

pub fn handle_ls(registry: &SSHRegistry) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["ID", "Name", "User", "Host"]);
    for e in registry.servers() {
        table.add_row(vec![
            e.id.to_string(),
            e.name.clone(),
            e.user.clone(),
            e.host.clone(),
        ]);
    }
    println!("{table}");
}

pub fn handle_add(
    registry: &mut SSHRegistry,
    user: String,
    host: String,
    mut name: String,
    existing_ssh_key: Option<String>,
    public_key_path: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if name.is_empty() {
        name = format!("{}@{}", user, host);
    }
    let max_id = registry.servers().iter().map(|s| s.id).max().unwrap_or(0);
    let id = max_id + 1;

    let new_server = crate::ssh::registry::Server {
        id: id,
        name: name,
        user: user,
        host: host,
        existing_ssh_key: existing_ssh_key,
    };

    if new_server.existing_ssh_key.is_none() {
        copy_ssh_key(&new_server.host, &new_server.user, public_key_path.unwrap());
    }

    registry.add(&new_server)?;

    println!("Added entry with id {}", id);
    Ok(())
}

pub fn handle_rm(registry: &mut SSHRegistry, id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let server = registry.remove(id)?;

    if server.existing_ssh_key.is_none() {
        remove_ssh_key(&server.host);
    }

    println!("Removed entry with id {}", id);
    Ok(())
}
