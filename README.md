# Miyabi - AI Agent Development System

🚀 **Miyabi** is an AI-powered development automation system built in Rust.

## Features

- 🤖 **Multiple AI Agents** - Specialized agents for different tasks
- ⚡ **Fast & Reliable** - Built with Rust for performance
- 🔧 **Extensible** - Easy to add new agents
- 📦 **Modular** - Clean workspace architecture

## Project Structure

```
Miyabitest02/
├── crates/
│   ├── miyabi-core/     # Core library with shared abstractions
│   ├── miyabi-agents/   # Agent implementations
│   └── miyabi-cli/      # Command line interface
├── Cargo.toml           # Workspace configuration
└── README.md
```

## Agents

| Agent | Description |
|-------|-------------|
| `coordinator` | Orchestrates multiple agents |
| `codegen` | Generates code from specifications |
| `review` | Reviews code and provides feedback |

## Quick Start

```bash
# Build the project
cargo build

# Run an agent
cargo run --bin miyabi -- run --agent codegen --task "Create hello world"

# List available agents
cargo run --bin miyabi -- list

# Check project status
cargo run --bin miyabi -- status
```

## Development

```bash
# Run tests
cargo test

# Check linting
cargo clippy

# Format code
cargo fmt
```

## License

MIT License - See LICENSE for details.
