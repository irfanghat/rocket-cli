use std::path::PathBuf;

use crate::templates::{mongo_db::files, common};

pub fn get_files() -> Vec<(PathBuf, &'static str)> {
    vec![
        ("Cargo.toml".into(), files::CARGO_TOML),
        ("src/main.rs".into(), files::MAIN_RS),
        ("src/routes/mod.rs".into(), files::ROUTES_MOD),
        ("src/fairings/mod.rs".into(), common::files::CORS),
        ("src/guards/mod.rs".into(), common::files::AUTH_GUARD),
        (".gitignore".into(), common::files::GITIGNORE),
        // More files will be added here if needed e.g.
        // db/mod.rs, middleware/logger.rs, etc.
        // ...
    ]
}
