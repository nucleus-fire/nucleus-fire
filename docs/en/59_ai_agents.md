# Building AI Agents with Nucleus

Nucleus provides a professional-grade **AI Agent Framework** that empowers you to build autonomous systems capable of reasoning, planning, and interacting with the world.

Unlike simple chatbots, Nucleus Agents operate on a **ReAct (Reasoning + Acting)** loop, allowing them to:
1.  **Analyze** a complex user request.
2.  **Plan** a sequence of actions.
3.  **Execute** tools (function calling) to gather information or perform tasks.
4.  **Observe** the results.
5.  **Iterate** until the task is complete.

---

## 🚀 Quick Start: The "Research Agent"

Let's build a capable agent that can research topics and save its findings to a file. This demonstrates how to combine **LLM reasoning** with **native tools**.

### 1. Define Your Tools

First, we define the tools our agent can use. Nucleus supports standard Model Context Protocol (MCP) tools.

```rust
use nucleus_std::mcp::Tool;
use serde_json::{json, Value};

// Tool: Search the "web" (mocked for this example)
let search_tool = Tool {
    name: "search_web".into(),
    description: "Search the web for information about a topic".into(),
    input_schema: json!({
        "type": "object",
        "properties": {
            "query": { "type": "string", "description": "The search query" }
        },
        "required": ["query"]
    })
};

// Tool: Save report to file
let save_tool = Tool {
    name: "save_report".into(),
    description: "Save the final report to a markdown file".into(),
    input_schema: json!({
        "type": "object",
        "properties": {
            "filename": { "type": "string" },
            "content": { "type": "string" }
        },
        "required": ["filename", "content"]
    })
};
```

### 2. Build the Agent

Configuration is handled via the `AgentBuilder`, which offers a fluent API for defining behavior.

```rust
use nucleus_std::agent::Agent;
use nucleus_std::neural::Neural;

#[tokio::main]
async fn main() {
    // Initialize the Neural Client (OpenAI compatible)
    let neural = Neural::new(std::env::var("OPENAI_API_KEY").unwrap_or_default());

    let agent = Agent::builder(neural)
        // Set a persona that encourages thoroughness
        .system("You are a senior technical researcher. Always verify facts before saving a report.")
        // register_tool takes: 1. Tool definition, 2. Async closure handler
        .with_tool(search_tool, |args| async move {
            let query = args["query"].as_str().unwrap_or("");
            // In a real app, call a search API here
            println!("🔍 Searching for: {}", query);
            Ok(format!("Results for '{}': Nucleus is a high-performance Rust framework...", query))
        })
        .with_tool(save_tool, |args| async move {
            let filename = args["filename"].as_str().unwrap();
            let content = args["content"].as_str().unwrap();
            println!("💾 Saving report to {}", filename);
            // In a real app, use tokio::fs::write
            Ok(format!("Successfully saved {} bytes to {}", content.len(), filename))
        })
        // Limit steps to prevent infinite loops during testing
        .max_steps(10) 
        .build();

    // Run the agent
    println!("🤖 Agent starting...");
    let result = agent.run("Research Nucleus Framework and save a brief report to nucleus_report.md").await;

    match result {
        Ok(response) => println!("✅ Mission Complete:\n{}", response),
        Err(e) => eprintln!("❌ Agent failed: {}", e),
    }
}
```

---

## 🧠 Core Architecture

Understanding the internal loop helps you build better agents.

### The ReAct Loop
When you call `agent.run()`, Nucleus enters a `while` loop:

1.  **Thought**: The LLM analyzes the conversation history and the user's prompt.
2.  **Decision**: It decides if it needs more information (Tool Call) or if it has the answer (Final Response).
3.  **Action**: If a tool call is generated (e.g., `Action: search_web(...)`), the Agent pauses the LLM, executes the Rust closure associated with that tool, and robustly handles any errors.
4.  **Observation**: The tool's output is fed back into the conversation history as an `Observation`.
5.  **Repeat**: The LLM sees the new observation and continues reasoning.

### Error Handling & Reliability
Nucleus Agent Framework is designed for production:
*   **Task Limits**: `max_steps(n)` prevents runaway costs if an agent gets stuck in a loop.
*   **Typed Errors**: `ImportError` explicitly categorizes failures (Neural API error, Tool execution error, Configuration error).
*   **Observability**: Fully instrumented with `tracing`.

---

## 🛠️ Configuration Reference

The `AgentBuilder` provides fine-grained control over the agent's lifecycle.

| Method | Description | Default |
|--------|-------------|---------|
| `system(prompt)` | Sets the system prompt. Use this to define the Agent's persona, constraints, and format requirements. | "You are a helpful AI assistant." |
| `max_steps(n)` | The maximum number of ReAct iterations allowed before aborting. | 10 |
| `temperature(f)` | Controls the randomness of the LLM (0.0 = deterministic, 1.0 = creative). | Model Default |
| `with_tool(tool, handler)` | Registers a tool and its execution logic simultaneously. | - |

---

## 💡 Best Practices

### 1. Specialized Personas
Don't use a generic "You are an AI". Be specific:
> "You are a database migration specialist. You only suggest safe, reversible SQL changes. You always check the current schema before proposing alterations."

### 2. Robust Tool Outputs
Return rich, descriptive strings from your tools. If a tool fails, return a string describing *why* it failed so the Agent can self-correct.
*   **Bad**: `Ok("Error".to_string())`
*   **Good**: `Ok("Error: File not found. usage: read_file(path: string)".to_string())`

### 3. Observability
Always enable tracing in production to debug agent "thoughts":
```rust
tracing_subscriber::fmt::init();
```
This reveals the entire decision-making process, including tool inputs/outputs and reasoning steps.
