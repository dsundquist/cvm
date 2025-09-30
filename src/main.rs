use clap::{Parser, CommandFactory};
use std::process::Command;
mod commands;

use commands::{Cli, Commands, VersionAction, ServiceAction};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Version { action }) => {
            handle_version(action);
        }
        Some(Commands::Service { action }) => {
            handle_service(action);
        }
        None => {
            // Print help when no command is provided
            let mut cmd = Cli::command();
            cmd.print_help().unwrap();
            println!(); // Add a newline after help
        }
    }
}

fn handle_version(action: &Option<VersionAction>) {
    match action {
        Some(VersionAction::Current) => {
            println!("Current version: 0.1.0");
        }
        Some(VersionAction::List) => {
            println!("Available versions:");
            println!(" - 0.1.0");
            println!(" - 0.2.0");
        }
        Some(VersionAction::Install { version }) => {
            println!("Installing version: {}", version);
        }
        None => {
            // Default behavior for `cvm version` - show current version
            println!("Current version: 0.1.0");
        }
    }
}

fn handle_service(action: &Option<ServiceAction>) {
    match action {
        Some(ServiceAction::Start) => {
            let output = sudo_systemctl_x_cloudflared("start".into());
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Some(ServiceAction::Stop) => {
            let output = sudo_systemctl_x_cloudflared("stop".into());
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Some(ServiceAction::Status) => {
            let output = systemctl_status_cloudflared();
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        None => {
            // Default behavior for `cvm service` - show status
            let output = systemctl_status_cloudflared();
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}

fn sudo_systemctl_x_cloudflared(command: String) -> std::process::Output {
    println!("Executing sudo systemctl {} cloudflared", command);
    let output = Command::new("sudo")
        .args(&["systemctl", &command, "cloudflared"])
        .output();

    output.unwrap()
}

fn systemctl_status_cloudflared() -> std::process::Output {
    println!("Checking cloudflared service status...");
    let output = Command::new("systemctl")
        .args(&["status", "cloudflared"])
        .output();

    output.unwrap()
}