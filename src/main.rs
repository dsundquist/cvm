use clap::{Parser, CommandFactory};
mod commands;
mod systemd;

use commands::{Cli, Commands, VersionAction, ServiceAction};
use systemd::SystemdManager;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Version { action }) => {
            handle_version(action);
        }
        Some(Commands::Service { action }) => {
            handle_service(action).await;
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

async fn handle_service(action: &Option<ServiceAction>) {
    let systemd = match SystemdManager::new().await {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Failed to connect to systemd: {}", e);
            return;
        }
    };

    match action {
        Some(ServiceAction::Start) => {
            if let Err(e) = systemd.start_service("cloudflared").await {
                eprintln!("Failed to start service: {}", e);
            }
        }
        Some(ServiceAction::Stop) => {
            if let Err(e) = systemd.stop_service("cloudflared").await {
                eprintln!("Failed to stop service: {}", e);
            }
        }
        Some(ServiceAction::Status) => {
            match systemd.get_service_status("cloudflared").await {
                Ok(status) => println!("{}", status),
                Err(e) => eprintln!("Failed to get service status: {}", e),
            }
        }
        None => {
            // Default behavior for `cvm service` - show status
            match systemd.get_service_status("cloudflared").await {
                Ok(status) => println!("{}", status),
                Err(e) => eprintln!("Failed to get service status: {}", e),
            }
        }
    }
}