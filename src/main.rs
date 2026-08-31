use colored::*;
use std::env;
use std::process::{Command, Stdio};
// 💡 Add this line to bring the `t!` macro into scope
use rust_i18n::t;

// Initialize i18n localization.
// It looks for files in the "locales" directory and defaults to English ("en").
rust_i18n::i18n!("locales", fallback = "en");

/// Executes a system command and inherits stdout/stderr to show output in real-time.
fn run_command(cmd: &str, args: &[&str], envs: &[(&str, &str)]) -> Result<(), String> {
    let mut command = Command::new(cmd);
    command.args(args);

    // Set custom environment variables if provided (e.g., NEEDRESTART_MODE).
    for (key, val) in envs {
        command.env(key, val);
    }

    // Connect standard streams directly to the current terminal session.
    command.stdin(Stdio::inherit());
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());

    match command.status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(format!("Command returned error code: {}", status)),
        Err(e) => Err(format!("Command execution failed: {}", e)),
    }
}

/// Checks if a specific command-line tool is installed on the system.
fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn main() {
    // Detect system locale from the LANG environment variable.
    // Example: extracts "ko" from "ko_KR.UTF-8". Defaults to "en" if missing or unrecognized.
    let sys_lang = env::var("LANG").unwrap_or_else(|_| "en".to_string());
    if sys_lang.starts_with("ko") {
        rust_i18n::set_locale("ko");
    } else {
        rust_i18n::set_locale("en");
    }

    // Check command-line arguments for version flags.
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let arg = &args[1];
        // Support flexible version flag patterns: --version, -version, -V, version, -v, etc.
        if arg == "--version" || arg == "-version" || arg == "-V" || arg == "version" || arg == "-v"
        {
            // env!("CARGO_PKG_VERSION") fetches the version string defined in Cargo.toml at compile time.
            println!("upall version {}", env!("CARGO_PKG_VERSION").green().bold());
            return; // Terminate early after printing the version to bypass system updates.
        }
    }

    println!("{}", "==========================================".cyan());
    println!("    {}     ", t!("start_msg").bold().cyan());
    println!("{}", "==========================================".cyan());

    // 1. APT Package Manager Updates and Cleanup
    println!("\n{}", t!("apt_start").yellow());
    if let Err(e) = run_command("sudo", &["apt", "update"], &[]) {
        eprintln!("{} {}", "Error: apt update failed ->".red(), e);
    }

    // Inject NEEDRESTART_MODE=a to prevent the interactive purple prompt dialog.
    if let Err(e) = run_command(
        "sudo",
        &["apt", "upgrade", "-y"],
        &[("NEEDRESTART_MODE", "a")],
    ) {
        eprintln!("{} {}", "Error: apt upgrade failed ->".red(), e);
    }

    // Clean up unused packages and old Linux kernel revisions (--purge removes configuration remnants).
    println!("{}", t!("apt_clean"));
    let _ = run_command("sudo", &["apt", "autoremove", "--purge", "-y"], &[]);

    // Flush local repository package caches to reclaim additional drive space.
    println!("{}", t!("apt_cache"));
    let _ = run_command("sudo", &["apt", "clean"], &[]);

    // 2. MISE Development Tool Runtime Updates & Pruning
    println!("\n{}", t!("mise_start").yellow());
    if command_exists("mise") {
        println!("{}", t!("mise_self"));
        let _ = run_command("mise", &["self-update", "--yes"], &[]);

        println!("{}", t!("mise_upgrade"));
        let _ = run_command("mise", &["upgrade", "--yes"], &[]);

        // Erase old historical runtime versions that are no longer referenced in configuration blocks.
        println!("{}", t!("mise_prune"));
        let _ = run_command("mise", &["prune", "--yes"], &[]);
    } else {
        // Variable interpolation translates to proper localized text placeholders.
        println!("{}", t!("no_command", cmd = "mise").purple());
    }

    // 3. RUSTUP Toolchain and Compiler Architecture Updates
    println!("\n{}", t!("rust_start").yellow());
    if command_exists("rustup") {
        let _ = run_command("rustup", &["update"], &[]);
    } else {
        println!("{}", t!("no_command", cmd = "rustup").purple());
    }

    println!("\n{}", "==========================================".green());
    println!("     {}      ", t!("end_msg").bold().green());
    println!("{}", "==========================================".green());
}
