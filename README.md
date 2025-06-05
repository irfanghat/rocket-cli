# 🚀 rocket-cli

A command-line interface (CLI) for developing, building, and running [Rocket](https://rocket.rs) web applications in Rust.  
Scaffold production-ready APIs in seconds with idiomatic folder structure and database-ready templates.

---

## ✨ Features

- 🔧 `rocket new` to scaffold new Rocket projects
- 📦 Multiple templates: `minimal` (default), `mongodb`, `postgres`, `mysql`, `mssql`, `sqlite`, and more via `rbatis`
- 📂 Standard project layout (routes, db, middleware, etc.)
- 💡 Intuitive command design inspired by the `dotnet` CLI
- 🛠️ Git integration (`--git`)
- ✅ Cross-platform and built in pure Rust

---

## 📦 Installation

### 🚀 Option 1: Install via Cargo

```bash
cargo install --path .
```

### 🧱 Option 2: Clone & Build Locally

```bash
git clone https://github.com/irfanghat/rocket-cli
cd rocket-cli
cargo build --release
```

### 📥 Option 3: Download Precompiled Binary

Download the latest binary from [GitHub Releases](https://github.com/irfanghat/rocket-cli/releases):

1. Go to the [Releases page](https://github.com/irfanghat/rocket-cli/releases)
2. Download the binary for your platform:

   * `rocket-cli-x86_64-unknown-linux-gnu`
   * `rocket-cli-x86_64-pc-windows-msvc.exe` (Coming soon/Available via WSL)
   * `rocket-cli-aarch64-apple-darwin` (macOS ARM - Coming soon)
3. Make it executable (Linux/macOS):

```bash
chmod +x rocket-cli-*
mv rocket-cli-* /usr/local/bin/rocket-cli
```

4. Verify installation:

```bash
rocket-cli --version
```

---

## 🚀 Usage

```bash
rocket-cli <command> [options]
```

### 🔨 Create a new project

```bash
rocket-cli new my-api
```

Options:

```bash
--template <name>   Choose a template: minimal, mongodb, postgres, mysql, mssql, rbatis
--git               Initialize a Git repository
```

Example:

```bash
rocket-cli new my-api --template postgres --git
```

### ▶️ Run your project

```bash
rocket-cli run
```

---

## 📁 Project Layout (Standardized)

Every template follows a production-grade folder structure:

```
my-api/
├── Cargo.toml
├── src/
│   ├── main.rs         # App entrypoint
│   ├── routes/         # Route handlers
│   ├── db/             # DB config & connections
│   ├── repositories/   # Data access layer
│   ├── middleware/     # Middleware & guards
│   └── fairings/       # Fairings & launch hooks
```

---

## 🧪 Templates

Supported templates:

* `minimal` — Basic Rocket app (default)
* `mongodb` — Rocket with `mongodb`
* `postgres` — Rocket with `PostgreSQL` (WIP)
* `mysql` — Rocket with `MySQL` (WIP)
* `mssql` — Rocket with `SQL Server` (WIP)
* `sqlite` — Rocket with `SQLite` (WIP)

---

## 📘 Documentation

* 🌐 [Rocket.rs Docs](https://rocket.rs)
* 💻 [GitHub Repo](https://github.com/irfanghat/rocket-cli)

---

## 🛠️ Contributing

Contributions, templates, and suggestions are welcome!

```bash
git clone https://github.com/irfanghat/rocket-cli
cd rocket-cli
cargo run -- --help
```

---

## 🧾 License

This project is licensed under the [MIT License](LICENSE).

---

Built with ❤️ and Rust — from the community, for the community.