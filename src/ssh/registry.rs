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
        let v: SSHRegistry = from_reader(f)?;
        Ok(v)
    }

    pub fn add(&mut self, server: &Server) -> Result<(), Box<dyn std::error::Error>> {
        let f = File::create(SSH_REGISTRY_PATH)?;

        self.servers.push(server.clone());

        to_writer_pretty(f, &self)?;

        Ok(())
    }
    pub fn remove(&mut self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let f = File::create(SSH_REGISTRY_PATH)?;

        let initial_len = self.servers.len();
        self.servers = self
            .servers
            .clone()
            .into_iter()
            .filter(|s| s.id != id)
            .collect();

        if self.servers.len() == initial_len {
            return Err(format!("Server with ID {} not found", id).into());
        }

        to_writer_pretty(f, &self)?;

        Ok(())
    }

    pub fn servers(&self) -> &[Server] {
        &self.servers
    }
}
