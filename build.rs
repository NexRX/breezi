use std::process::Command;

const FRONTEND_BUILD_DIR: &str = "dist";

fn main() {
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
        panic!("Frontend build command failed with status: {}", status);
    }
}

fn js_package_manager() -> &'static str {
    let manager = ["pnpm", "bun", "deno", "yarn", "npm"]
        .iter()
        .find_map(|manager| {
            Command::new(manager)
                .arg("--version")
                .output()
                .map(|_| manager)
                .ok()
        })
        .expect("no JavaScript package manager installation found");
    println!("cargo:warning=Using {} as package manager", manager);
    manager
}
