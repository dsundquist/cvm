use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
    pub content_type: String,
}

#[derive(Deserialize, Debug)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: String,
    pub published_at: String,
    pub prerelease: bool,
    pub draft: bool,
    pub assets: Vec<GitHubAsset>,
}

pub struct GitHubClient {
    client: reqwest::Client,
}

impl GitHubClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_releases(&self, owner: &str, repo: &str) -> Result<Vec<GitHubRelease>, Box<dyn std::error::Error>> {
        let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "cvm-cli")
            .send()
            .await?;

        if response.status().is_success() {
            let releases: Vec<GitHubRelease> = response.json().await?;
            Ok(releases)
        } else {
            Err(format!("GitHub API error: {}", response.status()).into())
        }
    }

    pub async fn get_release_by_tag(&self, owner: &str, repo: &str, tag: &str) -> Result<GitHubRelease, Box<dyn std::error::Error>> {
        let url = format!("https://api.github.com/repos/{}/{}/releases/tags/{}", owner, repo, tag);
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "cvm-cli")
            .send()
            .await?;

        if response.status().is_success() {
            let release: GitHubRelease = response.json().await?;
            Ok(release)
        } else {
            Err(format!("GitHub API error: {}", response.status()).into())
        }
    }

    pub async fn get_latest_release(&self, owner: &str, repo: &str) -> Result<GitHubRelease, Box<dyn std::error::Error>> {
        let url = format!("https://api.github.com/repos/{}/{}/releases/latest", owner, repo);
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "cvm-cli")
            .send()
            .await?;

        if response.status().is_success() {
            let release: GitHubRelease = response.json().await?;
            Ok(release)
        } else {
            Err(format!("GitHub API error: {}", response.status()).into())
        }
    }
}