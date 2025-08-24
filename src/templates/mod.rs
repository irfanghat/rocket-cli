pub mod common;
pub mod minimal;
pub mod mongo_db;
pub mod postgres;

use std::path::PathBuf;

pub fn load_template_files(template: &str) -> Option<Vec<(PathBuf, &'static str)>> {
    match template {
        "minimal" => Some(minimal::manifest::load_template()),
        "mongodb" => Some(mongo_db::manifest::load_template()),
        "postgres" => Some(postgres::manifest::load_template()),
        _ => None,
    }
}
