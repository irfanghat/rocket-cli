use crate::commands::NewArgs;
use crate::templates::load_template_files;
use colored::*;
use handlebars::Handlebars;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn handle(args: NewArgs) {
    if args.list {
        println!("\nAvailable Rocket templates:\n");
        println!("  minimal     → Basic Rocket project with a single route [default]");
        println!("  mongodb     → Rocket + MongoDB integration");
        println!("  postgres    → Rocket + PostgreSQL");
        println!("  mysql       → Rocket + MySQL (WIP)");
        println!("  mssql       → Rocket + SQL Server (WIP)");
        println!("  sqlite      → Rocket + SQLite (WIP)");
        println!("\nExample: rocket new my-app --template postgres --git");
        return;
    }

    if let Some(name) = args.name {
        execute(name, args.git, args.template);
    } else {
        eprintln!(
            "{}",
            "Project name is required. Use `rocket new <name>` or `rocket new --list`.".yellow()
        );
        std::process::exit(1);
    }
}

pub fn execute(name: String, git: bool, template: String) {
    eprintln!(
        "Creating Rocket project -> {} using template '{}'",
        name, template
    );

    let project_dir = Path::new(&name);

    if project_dir.exists() {
        eprintln!(
            "{}",
            format!("Project directory '{}' already exists.", name).yellow()
        );
        std::process::exit(1);
    }

    let template_files = load_template_files(&template).unwrap_or_else(|| {
        eprintln!("{}", format!("Rocket Template '{}' not found...", template).red());
        std::process::exit(1);
    });

    let handlebars = Handlebars::new();
    let mut ctx = HashMap::new();
    ctx.insert("project_name", name.as_str());

    for (relative_path, content) in template_files {
        let rendered = handlebars.render_template(content, &ctx).unwrap();
        let full_path = project_dir.join(relative_path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create directories...");
        }

        fs::write(full_path, rendered).expect("Failed to write file...");
    }

    if git {
        std::process::Command::new("git")
            .arg("init")
            .arg(&name)
            .status()
            .expect("Failed to initialize git...");
        println!("Git initialized.");
    }

    println!(
        "{}",
        format!(
            "Project '{}' created successfully using '{}' template!",
            name, template
        )
        .green()
    );
}
