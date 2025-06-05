use crate::commands::NewArgs;
use crate::templates::get_template_files;
use handlebars::Handlebars;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn handle(args: NewArgs) {
    if args.list {
        println!("Available Rocket templates:\n");
        println!("  minimal     → Basic Rocket project with a single route [default]");
        println!("  mongodb     → Rocket + MongoDB integration");
        println!("  postgres    → Rocket + PostgreSQL using rbatis");
        println!("  mysql       → Rocket + MySQL using rbatis");
        println!("  mssql       → Rocket + SQL Server using rbatis");
        println!("  sqlite       → Rocket + SQLite using rbatis");
        println!("\nExample: rocket new my-app --template postgres --git");
        return;
    }

    if let Some(name) = args.name {
        execute(name, args.git, args.template);
    } else {
        eprintln!("❌ Project name is required. Use `rocket new <name>` or `rocket new --list`.");
        std::process::exit(1);
    }
}

pub fn execute(name: String, git: bool, template: String) {
    println!(
        "Creating Rocket project -> {} using template '{}'",
        name, template
    );

    let project_dir = Path::new(&name);

    if project_dir.exists() {
        eprintln!("Project directory '{}' already exists", name);
        std::process::exit(1);
    }

    let template_files = get_template_files(&template).unwrap_or_else(|| {
        eprintln!("Template '{}' not found", template);
        std::process::exit(1);
    });

    let handlebars = Handlebars::new();
    let mut ctx = HashMap::new();
    ctx.insert("project_name", name.as_str());

    for (relative_path, content) in template_files {
        let rendered = handlebars.render_template(content, &ctx).unwrap();
        let full_path = project_dir.join(relative_path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create directories");
        }

        fs::write(full_path, rendered).expect("Failed to write file");
    }

    if git {
        std::process::Command::new("git")
            .arg("init")
            .arg(&name)
            .status()
            .expect("Failed to run git init");
        println!("Git initialized.");
    }

    println!(
        "Project '{}' created successfully using '{}' template!",
        name, template
    );
}
