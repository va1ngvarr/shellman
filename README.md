# shellman

Easily manage and connect to SSH servers from a small, focused CLI.

**shellman** stores a small registry of server entries and provides simple subcommands to list, add, remove and connect (SSH or SFTP).

Currently a work-in-progress with core features implemented. Tested only on Linux.

**Features**

- List saved servers in a neat table
- Add and remove server entries
- Connect to a server by entry ID using SSH or SFTP
- Simple JSON-backed registry (`servers.json`)

**Repository**

- Main CLI entry: [src/main.rs](src/main.rs)
- Command handlers: [src/commands.rs](src/commands.rs)
- Registry and SSH helpers: [src/ssh/mod.rs](src/ssh/mod.rs)

**Prerequisites**

- Rust toolchain (stable) and Cargo
- A working SSH environment (ssh, sftp) on your machine

**Build & Install**

Build locally (debug):

```bash
cargo build
./target/debug/shellman <SUBCOMMAND>
# run directly
cargo run -- <SUBCOMMAND>
```

Build and run the release binary:

```bash
cargo build --release
./target/release/shellman <SUBCOMMAND>
```

Install into your Cargo bin (optional):

```bash
cargo install --path .
# then you can run
shellman <SUBCOMMAND>
```

**Usage**

The CLI provides four main subcommands. See `--help` for more details.

- List entries

```bash
shellman ls
```

- Add an entry

```bash
shellman add <user> <host> [--name "entry-name", --existing-ssh-key "/path/ to/key"]
# example
shellman add alice example.com --name "Alice's Server"
```

- Connect to an entry (SSH)

```bash
shellman use <id>
# example: connect to entry with id 1
shellman use 1
```

- Connect to an entry using SFTP

```bash
shellman use 1 --sftp
```

- Remove an entry

```bash
shellman rm <id>
# example
shellman rm 2
```

Notes: IDs start at 1 and are shown by `shellman ls`.

**Configuration & Data**

The registry is stored in `servers.json` in the project working directory. The registry entries include `id`, `name`, `user`, `host`, and an optional `existing_ssh_key` field used when connecting.

If an entry does not specify `existing_ssh_key`, the code currently falls back to `id_rsa_shellman.pub`.

**Implementation details & TODOs**

- Command definitions live in [src/main.rs](src/main.rs) and handlers in [src/commands.rs](src/commands.rs).
- `handle_add` currently records entries but has a `TODO` to generate an SSH keypair and copy it to the remote server.

**Testing / Development**

During development you can iterate quickly with:

```bash
cargo run -- ls
cargo run -- add alice 192.0.2.1 --name alice-test
cargo run -- use 1
```

**Contributing**

Contributions welcome — open an issue or PR. If you plan to add features, please document intended changes and update tests (if any).

**License**

No license specified in the repository. Add a `LICENSE` file if you intend to set one.
