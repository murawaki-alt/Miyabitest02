//! Miyabi Core - Foundation library for the Miyabi AI Agent System
//!
//! This crate provides the core abstractions and utilities used by all Miyabi agents.

pub mod agent;
pub mod error;
pub mod config;

pub use agent::{Agent, AgentContext, AgentResult};
pub use error::MiyabiError;
pub use config::Config;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::agent::{Agent, AgentContext, AgentResult};
    pub use crate::error::MiyabiError;
    pub use crate::config::Config;
    pub use anyhow::Result;
}
