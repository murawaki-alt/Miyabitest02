//! Configuration management for Miyabi

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Project root directory
    pub project_root: PathBuf,
    /// GitHub repository (owner/repo format)
    pub github_repo: Option<String>,
    /// API configurations
    pub api: ApiConfig,
    /// Agent-specific configurations
    pub agents: AgentConfigs,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiConfig {
    pub anthropic_api_key: Option<String>,
    pub github_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentConfigs {
    pub codegen: CodegenConfig,
    pub review: ReviewConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodegenConfig {
    pub max_file_size: Option<usize>,
    pub allowed_languages: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewConfig {
    pub auto_approve: bool,
    pub required_checks: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project_root: PathBuf::from("."),
            github_repo: None,
            api: ApiConfig::default(),
            agents: AgentConfigs::default(),
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        // Try to load from miyabi.toml or use defaults
        let config_path = PathBuf::from("miyabi.toml");
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }
}
