pub mod mongo_db;
pub mod common;
pub mod minimal;

use std::path::PathBuf;

pub fn get_template_files(template: &str) -> Option<Vec<(PathBuf, &'static str)>> {
    match template {
        "minimal" => Some(minimal::manifest::get_files()),
        "mongodb" => Some(mongo_db::manifest::get_files()),
        _ => None,
    }
}
