use std::process::Command;

pub enum SSHProtocol {
    SSH,
    SFTP,
}

pub fn generate_ssh_keypair() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("ssh-keygen")
        .arg("-t")
        .arg("rsa")
        .arg("-b")
        .arg("2048")
        .arg("-f")
        .arg("./id_rsa_shellman")
        .arg("-N")
        .arg("")
        .output()
        .expect("failed to execute ssh-keygen");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

pub fn copy_ssh_key(host: &str, user: &str, public_key_path: &str) {
    let output = Command::new("ssh-copy-id")
        .arg("-i")
        .arg(public_key_path)
        .arg(format!("{}@{}", user, host))
        .status()
        .expect("failed to execute ssh-copy-id");
    println!("ssh-copy-id exited with: {}", output);
}

pub fn remove_ssh_key(host: &str) {
    let output = Command::new("ssh-keygen")
        .arg("-R")
        .arg(host)
        .status()
        .expect("failed to execute ssh-copy-id");
    println!("ssh-keygen exited with: {}", output);
}

pub fn connect_secure_server(
    host: &str,
    user: &str,
    protocol: SSHProtocol,
    private_key_path: &str,
) {
    match protocol {
        SSHProtocol::SSH => {
            Command::new("ssh")
                .arg("-t")
                .arg("-i")
                .arg(private_key_path)
                .arg(format!("{}@{}", user, host))
                .status()
                .expect("failed to execute ssh");
        }
        SSHProtocol::SFTP => {
            Command::new("sftp")
                .arg("-i")
                .arg(private_key_path)
                .arg(format!("{}@{}", user, host))
                .status()
                .expect("failed to execute sftp");
        }
    }
}
