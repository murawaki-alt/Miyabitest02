//! Miyabi Agents - Collection of AI agents for automated development
//!
//! This crate contains all the specialized agents used in the Miyabi system.

pub mod codegen;
pub mod review;
pub mod coordinator;

pub use codegen::CodegenAgent;
pub use review::ReviewAgent;
pub use coordinator::CoordinatorAgent;

use miyabi_core::Agent;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry of all available agents
pub struct AgentRegistry {
    agents: HashMap<String, Arc<dyn Agent>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            agents: HashMap::new(),
        };
        
        // Register default agents
        registry.register(Arc::new(CodegenAgent::new()));
        registry.register(Arc::new(ReviewAgent::new()));
        registry.register(Arc::new(CoordinatorAgent::new()));
        
        registry
    }

    pub fn register(&mut self, agent: Arc<dyn Agent>) {
        self.agents.insert(agent.name().to_string(), agent);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Agent>> {
        self.agents.get(name).cloned()
    }

    pub fn list(&self) -> Vec<&str> {
        self.agents.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
