# 🚀 rocket-cli

A fast, ergonomic command-line interface (CLI) for scaffolding and running [Rocket](https://rocket.rs) web applications in Rust.  
Spin up production-ready APIs in seconds with idiomatic project structure and database-backed templates.

## Features

- `rocket new` — scaffold a new Rocket project
- Built-in templates:  
  - `minimal` (default)  
  - `mongodb`, `postgres`, `mysql`, `mssql`, `sqlite` (via [rbatis](https://github.com/rbatis/rbatis))  
- Standard project layout (routes, db, repositories, middleware, fairings)  
- Intuitive UX inspired by the `dotnet` CLI  
- Optional Git initialization (`--git`)  
- Cross-platform, written entirely in Rust  

## Installation

### Install from source

```bash
cargo install --path .
```

### Clone & build

```bash
git clone https://github.com/irfanghat/rocket-cli
cd rocket-cli
cargo build --release
```

### Precompiled binaries

Grab the latest binaries from [GitHub Releases](https://github.com/irfanghat/rocket-cli/releases):

* `rocket-cli-x86_64-unknown-linux-gnu`
* `rocket-cli-x86_64-pc-windows-msvc.exe` (via WSL/Windows soon)
* `rocket-cli-aarch64-apple-darwin` (macOS ARM soon)

Install (Linux/macOS):

```bash
chmod +x rocket-cli-*
mv rocket-cli-* /usr/local/bin/rocket-cli
```

Verify:

```bash
rocket-cli --version
```

## Usage

```bash
rocket-cli <command> [options]
```

### Create a new project

```bash
rocket-cli new my-api
```

Options:

```bash
--template <name>   # minimal | mongodb | postgres | mysql | mssql | sqlite
--git               # initialize a Git repository
```

Example:

```bash
rocket-cli new my-api --template postgres --git
```

### Run the project

```bash
rocket-cli run
```

## Project Layout

All templates follow a production-ready structure:

```
my-api/
├── Cargo.toml
├── src/
│   ├── main.rs         # Application entrypoint
│   ├── routes/         # Route handlers
│   ├── db/             # Database config & connections
│   ├── repositories/   # Data access layer
│   ├── middleware/     # Middleware & guards
│   └── fairings/       # Fairings & launch hooks
```

## Templates

* `minimal` — base Rocket app (default)
* `mongodb` — Rocket + MongoDB
* `postgres` — Rocket + PostgreSQL (via rbatis)
* `mysql` — Rocket + MySQL (WIP)
* `mssql` — Rocket + SQL Server (WIP)
* `sqlite` — Rocket + SQLite (WIP)

## Resources

* [Rocket.rs Documentation](https://rocket.rs)
* [rocket-cli GitHub](https://github.com/irfanghat/rocket-cli)

## Contributing

Contributions and new templates are always welcome.

```bash
git clone https://github.com/irfanghat/rocket-cli
cd rocket-cli
cargo run -- --help
```

## License

Licensed under the [MIT License](LICENSE).

---

Built with ❤️ in Rust — for the community, by the community.