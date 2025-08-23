use std::{fs, path::Path, process::Command};

const FRONTEND_BUILD_DIR: &str = "dist";
const DB_PATH: &str = "data/breezi.db";

fn main() {
    create_database();
    migrate_database();

    let is_release = std::env::var("PROFILE").is_ok_and(|v| v == "release");
    let is_frontend_built = std::fs::exists(FRONTEND_BUILD_DIR).is_ok_and(|v| v);

    match (is_release, is_frontend_built) {
        (true, true) => {
            println!("cargo:warning=Release build is forcing frontend rebuild");
            std::fs::remove_dir_all(FRONTEND_BUILD_DIR).expect("deletion of frontend build failed");
        }
        (false, true) => {
            println!("cargo:warning=Debug build is re-using frontend build");
            return;
        }
        _ => println!("cargo:warning=Building frontend"),
    }

    // Run the build command
    let status = Command::new(js_package_manager())
        .arg("run")
        .arg("build")
        .status()
        .expect("frontend build failed with detected js package manager");

    if !status.success() {
        panic!("Frontend build command failed with status: {status}");
    }
}

fn js_package_manager() -> &'static str {
    let manager = ["pnpm", "bun", "deno", "yarn", "npm"]
        .iter()
        .find_map(|manager| Command::new(manager).arg("--version").output().map(|_| manager).ok())
        .expect("no JavaScript package manager installation found");
    println!("cargo:warning=Using {manager} as package manager");
    manager
}

fn create_database() {
    // Create parent directory if it doesn't exist
    if let Some(parent) = Path::new(DB_PATH).parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // Create empty SQLite database file if it doesn't exist
    if !Path::new(DB_PATH).exists() {
        // SQLite will create the file if you open a connection
        let _ = rusqlite::Connection::open(DB_PATH).unwrap();
        println!("cargo:warning=Debug Build creating {DB_PATH} ");
    } else {
        println!("cargo:warning=Debug Build found existing database");
    }

    // Tell Cargo to rerun build.rs if this script or migrations changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=migrations");
}

fn migrate_database() {
    // Ensure sqlx-cli is installed
    let status = Command::new("cargo")
        .args(["install", "sqlx-cli"])
        .status()
        .expect("failed to run cargo install sqlx-cli");
    if !status.success() {
        panic!("Failed to install sqlx-cli");
    }

    // Run sqlx migrations
    let status = Command::new("sqlx")
        .args(["migrate", "run", "--database-url", &format!("sqlite://{DB_PATH}")])
        .status()
        .expect("failed to run sqlx migrate run");
    if !status.success() {
        panic!("sqlx migrate run failed");
    }
}
