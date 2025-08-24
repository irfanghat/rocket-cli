#[cfg(test)]
mod tests {
    use rocket_cli::templates::common;
    use rocket_cli::templates::minimal::{files, manifest::load_template};
    use std::collections::{HashMap, HashSet};
    use std::path::PathBuf;

    #[test]
    fn test_load_template_contains_expected_files() {
        let files = load_template();
        let paths: HashSet<_> = files.iter().map(|(p, _)| p.clone()).collect();

        let expected = vec![
            PathBuf::from("Cargo.toml"),
            PathBuf::from("src/main.rs"),
            PathBuf::from("src/routes/mod.rs"),
            PathBuf::from("src/fairings/mod.rs"),
            PathBuf::from("Rocket.toml"),
            PathBuf::from(".gitignore"),
            PathBuf::from(".env"),
        ];

        for path in expected {
            assert!(
                paths.contains(&path),
                "Expected file {:?} not found in template",
                path
            );
        }
    }

    #[test]
    fn test_load_template_file_contents_match() {
        let files = load_template();
        let lookup: HashMap<_, _> = files.into_iter().collect();

        assert_eq!(lookup[&PathBuf::from("Cargo.toml")], files::CARGO_TOML);
        assert_eq!(lookup[&PathBuf::from("src/main.rs")], files::MAIN_RS);
        assert_eq!(
            lookup[&PathBuf::from("src/routes/mod.rs")],
            files::ROUTES_MOD
        );
        assert_eq!(
            lookup[&PathBuf::from("src/fairings/mod.rs")],
            common::files::CORS
        );
        assert_eq!(
            lookup[&PathBuf::from("Rocket.toml")],
            common::files::ROCKET_CONFIG
        );
        assert_eq!(
            lookup[&PathBuf::from(".gitignore")],
            common::files::GITIGNORE
        );
        assert_eq!(lookup[&PathBuf::from(".env")], common::files::ENV);
    }

    #[test]
    fn test_load_template_no_duplicate_paths() {
        let files = load_template();
        let mut seen = HashSet::new();

        for (path, _) in files {
            assert!(
                seen.insert(path.clone()),
                "Duplicate path detected: {:?}",
                path
            );
        }
    }
}
