use std::path::PathBuf;

use crate::templates::{common, postgres::files};

pub fn load_template() -> Vec<(PathBuf, &'static str)> {
    vec![
        ("Cargo.toml".into(), files::CARGO_TOML),
        ("src/main.rs".into(), files::MAIN_RS),
        ("src/routes/mod.rs".into(), files::ROUTES_MOD),
        ("src/fairings/mod.rs".into(), common::files::CORS),
        ("src/guards/mod.rs".into(), common::files::AUTH_GUARD),
        ("src/catchers/mod.rs".into(), files::CATCHERS),
        ("src/options/mod.rs".into(), files::OPTIONS),
        ("src/repositories/mod.rs".into(), files::REPOSITORIES),
        ("src/db/mod.rs".into(), files::DB),
        ("src/models/mod.rs".into(), files::MODELS),
        ("src/auth/mod.rs".into(), common::files::BASIC_AUTH),
        ("src/middleware/mod.rs".into(), common::files::MIDDLEWARE),
        ("Rocket.toml".into(), common::files::ROCKET_CONFIG),
        (".gitignore".into(), common::files::GITIGNORE),
        (".env".into(), files::ENV_TEMPLATE),
        (
            "migrations/001_create_users_table.sql".into(),
            files::MIGRATIONS,
        ),
    ]
}
