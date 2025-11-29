//! Miyabi CLI - Command line interface for the Miyabi AI Agent System

use clap::{Parser, Subcommand};
use miyabi_agents::AgentRegistry;
use miyabi_core::AgentContext;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "miyabi")]
#[command(author = "Miyabi Team")]
#[command(version = "0.1.0")]
#[command(about = "AI-powered development automation", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute an agent
    Run {
        /// Agent name to execute
        #[arg(short, long)]
        agent: String,

        /// Task description
        #[arg(short, long)]
        task: Option<String>,

        /// GitHub issue number
        #[arg(short, long)]
        issue: Option<u64>,
    },

    /// List available agents
    List,

    /// Show project status
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("miyabi=info".parse()?))
        .init();

    let cli = Cli::parse();
    let registry = AgentRegistry::new();

    match cli.command {
        Commands::Run { agent, task, issue } => {
            info!("Running agent: {}", agent);
            
            if let Some(agent_instance) = registry.get(&agent) {
                let mut context = AgentContext::new();
                if let Some(t) = task {
                    context = context.with_task(t);
                }
                if let Some(i) = issue {
                    context = context.with_issue(i);
                }

                let result = agent_instance.execute(context).await?;
                println!("✅ {}", result.message);
                
                for artifact in result.artifacts {
                    println!("📄 Generated: {}", artifact.name);
                }
            } else {
                eprintln!("❌ Agent '{}' not found", agent);
                eprintln!("Available agents: {:?}", registry.list());
            }
        }

        Commands::List => {
            println!("🤖 Available Miyabi Agents:");
            println!("{}", "─".repeat(40));
            for name in registry.list() {
                if let Some(agent) = registry.get(name) {
                    println!("  • {} - {}", name, agent.description());
                }
            }
        }

        Commands::Status => {
            println!("📊 Miyabi Project Status");
            println!("{}", "─".repeat(40));
            println!("  Version: 0.1.0");
            println!("  Agents: {}", registry.list().len());
            println!("  Status: Ready");
        }
    }

    Ok(())
}
