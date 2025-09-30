use clap::{Parser, Subcommand};

/// CVM - A Rust CLI application
#[derive(Parser)]
#[command(name = "cvm")]
#[command(about = "CVM - Cloudflared Version Manager", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage versions
    Version {
        #[command(subcommand)]
        action: Option<VersionAction>,
    },
    /// Manage Cloudflare Tunnel service
    Service {
        #[command(subcommand)]
        action: Option<ServiceAction>,
    },
}

#[derive(Subcommand)]
pub enum VersionAction {
    /// Show current version
    Current,
    /// List all available versions
    List,
    /// Install a specific version
    Install {
        /// Version to install
        version: String,
    },
}

#[derive(Subcommand)]
pub enum ServiceAction {
    /// Start cloudflared service
    Start,
    /// Stop cloudflared service
    Stop,
    /// Check cloudflared service status
    Status,
}
