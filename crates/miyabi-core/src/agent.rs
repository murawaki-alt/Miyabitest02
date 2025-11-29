//! Agent trait and core abstractions

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result type for agent operations
pub type AgentResult<T> = anyhow::Result<T>;

/// Context passed to agents during execution
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentContext {
    /// Task description
    pub task: Option<String>,
    /// GitHub issue number if applicable
    pub issue_number: Option<u64>,
    /// Additional context data
    pub data: HashMap<String, serde_json::Value>,
}

impl AgentContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_task(mut self, task: impl Into<String>) -> Self {
        self.task = Some(task.into());
        self
    }

    pub fn with_issue(mut self, issue_number: u64) -> Self {
        self.issue_number = Some(issue_number);
        self
    }

    pub fn with_data(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.data.insert(key.into(), value);
        self
    }
}

/// Output from agent execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOutput {
    pub success: bool,
    pub message: String,
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub name: String,
    pub content: String,
    pub artifact_type: ArtifactType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactType {
    Code,
    Document,
    Report,
    Other(String),
}

/// Core trait that all Miyabi agents must implement
#[async_trait]
pub trait Agent: Send + Sync {
    /// Returns the name of the agent
    fn name(&self) -> &str;

    /// Returns a description of the agent's capabilities
    fn description(&self) -> &str;

    /// Execute the agent's main logic
    async fn execute(&self, context: AgentContext) -> AgentResult<AgentOutput>;

    /// Check if the agent can handle a given task
    fn can_handle(&self, task: &str) -> bool {
        let _ = task;
        true
    }
}
