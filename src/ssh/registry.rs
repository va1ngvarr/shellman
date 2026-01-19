use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use serde_json::to_writer_pretty;
use std::fs::File;

const SSH_REGISTRY_PATH: &str = "servers.json"; //"~/.config/shellman/servers.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub id: u32,
    pub name: String,
    pub user: String,
    pub host: String,
    pub existing_ssh_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SSHRegistry {
    servers: Vec<Server>,
}

impl SSHRegistry {
    pub fn load() -> Result<SSHRegistry, Box<dyn std::error::Error>> {
        let f = File::open(SSH_REGISTRY_PATH)?;
        let v: SSHRegistry = from_reader(f).unwrap_or(SSHRegistry { servers: vec![] });
        Ok(v)
    }

    pub fn add(&mut self, server: &Server) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(server) = self
            .servers
            .iter()
            .find(|s| s.host == server.host && s.user == server.user)
        {
            return Err(format!(
                "Server with host {} and user {} already exists",
                server.host, server.user
            )
            .into());
        }

        self.servers.push(server.clone());

        let f = File::create(SSH_REGISTRY_PATH)?;
        to_writer_pretty(f, &self)?;

        Ok(())
    }
    pub fn remove(&mut self, id: u32) -> Result<Server, Box<dyn std::error::Error>> {
        // Find the index of the server to remove
        if let Some(index) = self.servers.iter().position(|server| server.id == id) {
            // Remove the server and retrieve its host
            let removed_server = self.servers.swap_remove(index);

            // Write the updated servers list to the file
            let f = File::create(SSH_REGISTRY_PATH)?;
            to_writer_pretty(f, self)?;

            Ok(removed_server)
        } else {
            Err(format!("Server with ID {} not found", id).into())
        }
    }

    pub fn servers(&self) -> &Vec<Server> {
        &self.servers
    }
}
