# Building AI Agents with Nucleus

Nucleus Agent Framework allows you to build autonomous AI agents that can reason, plan, and interact with the world through tools. It implements a robust ReAct (Reasoning + Acting) loop and integrates seamlessly with the Model Context Protocol (MCP).

## Quick Start

The core abstraction is the `Agent`, which you configure using the `AgentBuilder`.

```rust
use nucleus_std::agent::Agent;
use nucleus_std::neural::Neural;

#[tokio::main]
async fn main() {
    // 1. Configure the LLM client
    let neural = Neural::new("your-api-key");
    
    // 2. Build the Agent
    let agent = Agent::builder(neural)
        .system("You are a specialized coding assistant")
        .max_steps(15) // Limit reasoning steps
        .build();

    // 3. Run the agent
    let response = agent.run("Refactor my login page").await.unwrap();
    println!("Agent Analysis: {}", response);
}
```

## Adding Tools

Agents become powerful when they can use tools. Nucleus supports standard MCP tools and local Rust functions.

```rust
use nucleus_std::mcp::Tool;
use serde_json::json;

// Define a tool
let weather_tool = Tool {
    name: "get_weather".into(),
    description: "Get weather for a city".into(),
    input_schema: json!({
        "type": "object",
        "properties": {
            "city": { "type": "string" }
        },
        "required": ["city"]
    })
};

// Register with handler
agent.register_tool(weather_tool, |args| async move {
    let city = args["city"].as_str().unwrap_or("Unknown");
    // Call external API here...
    Ok(format!("Weather in {} is Sunny", city))
}).await;
```

## Configuration

| Method | Description | Default |
|--------|-------------|---------|
| `system(prompt)` | Sets the system prompt / persona | "You are a helpful AI assistant." |
| `max_steps(n)` | Safety limit for reasoning loops | 10 |
| `temperature(f)` | Sets LLM creativity (0.0 - 1.0) | Model default |

## Observability

The Agent framework utilizes `tracing` for deep observability. Initialize a subscriber to see detailed logs of the agent's thought process.

```rust
tracing_subscriber::fmt::init();
```

Logs will show:
- Step count and loop progress
- Tool calls and arguments
- Tool execution results and errors
- LLM response sizes
