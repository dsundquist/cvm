use clap::{Parser, CommandFactory};
use std::process::Command;
mod commands;
mod systemd;
mod github;

use commands::{Cli, Commands, VersionAction, ServiceAction};
use systemd::SystemdManager;
use github::GitHubClient;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Version { action }) => {
            handle_version(action).await;
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

async fn handle_version(action: &Option<VersionAction>) {
    match action {
        Some(VersionAction::Current) => {
            get_cloudflared_version();
        }
        Some(VersionAction::List) => {
            get_github_releases().await;
        }
        Some(VersionAction::Install { version }) => {

            // TODO: 
            // Right now this just prints out all the download URLs for the specified version
            // What we want to do is grab the appropriate one, and then put it where it needs to go
            // which is described in the Unit file, but its typically /usr/bin/cloudflared
            let urls = get_download_urls_for_version(version).await;
            if urls.is_empty() {
                eprintln!("No download URLs found for version: {}", version);
            } else {
                println!("Download URLs for {}:", version);
                for url in urls {
                    println!("  {}", url);
                }
            }
        }
        None => {
            // Default behavior for `cvm version` - show current version
            get_cloudflared_version();
        }
    }
}

async fn get_github_releases() {
    println!("Fetching cloudflared releases from GitHub...");
    const NUM_TO_SHOW: usize = 20;

    let github_client = GitHubClient::new();
    
    match github_client.get_releases("cloudflare", "cloudflared").await {
        Ok(releases) => {
            println!("Available cloudflared releases:");
            for release in releases.iter().take(NUM_TO_SHOW) {
                let mut browser_download_url = String::new();

                for asset in &release.assets {
                    if asset.name.ends_with("linux-amd64") {
                        browser_download_url = asset.browser_download_url.clone();
                    }
                }

                println!(" * {} - {}", 
                    release.tag_name,
                    browser_download_url
                );
            }

            if releases.len() > NUM_TO_SHOW {
                println!("  ... and {} more releases", releases.len() - NUM_TO_SHOW);
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch releases from GitHub: {}", e);
        }
    }
}

async fn get_download_urls_for_version(version: &str) -> Vec<String> {
    let github_client = GitHubClient::new();
    
    match github_client.get_release_by_tag("cloudflare", "cloudflared", version).await {
        Ok(release) => {
            release.assets.iter()
                .map(|asset| asset.browser_download_url.clone())
                .collect()
        }
        Err(_) => {
            Vec::new()
        }
    }
}

fn get_cloudflared_version() {
    println!("Getting cloudflared version...");
    
    let output = Command::new("cloudflared")
        .arg("-V")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout);
                println!("Cloudflared version: {}", version.trim());
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error getting version: {}", error.trim());
            }
        }
        Err(e) => {
            eprintln!("Failed to execute cloudflared command: {}", e);
            eprintln!("Make sure cloudflared is installed and in your PATH");
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