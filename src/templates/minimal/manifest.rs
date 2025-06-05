use std::path::PathBuf;

use crate::templates::{common, minimal::files};

pub fn get_files() -> Vec<(PathBuf, &'static str)> {
    vec![
        ("Cargo.toml".into(), files::CARGO_TOML),
        ("src/main.rs".into(), files::MAIN_RS),
        ("src/routes/mod.rs".into(), files::ROUTES_MOD),
        ("src/fairings/mod.rs".into(), common::files::CORS),
        ("Rocket.toml".into(), common::files::ROCKET_CONFIG),
        (".gitignore".into(), common::files::GITIGNORE),
        (".env".into(), common::files::ENV),
        // More files will be added here if needed e.g.
        // db/mod.rs, middleware/logger.rs, etc.
        // ...
    ]
}
