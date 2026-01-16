# Neural AI/LLM Guide

Nucleus Neural provides built-in AI/LLM integration with OpenAI-compatible APIs.

## Quick Start

```rust
use nucleus_std::neural::{Neural, ChatMessage};

// Create client
let ai = Neural::new("sk-your-api-key")
    .with_model("gpt-4o")
    .with_temperature(0.7);

// Simple question
let answer = ai.ask("What is Rust?").await?;

// Chat with context
let response = ai.chat(vec![
    ChatMessage::system("You are a helpful assistant."),
    ChatMessage::user("Explain async/await in Rust."),
]).await?;
```

## Configuration

### Models

```rust
// GPT-4 (default)
let ai = Neural::new("sk-xxx");

// GPT-3.5 Turbo (faster, cheaper)
let ai = Neural::new("sk-xxx").with_model("gpt-3.5-turbo");

// GPT-4 Turbo
let ai = Neural::new("sk-xxx").with_model("gpt-4-turbo");

// Claude (via compatible API)
let ai = Neural::new("sk-xxx")
    .with_model("claude-3-opus")
    .with_base_url("https://api.anthropic.com/v1");
```

### Temperature

```rust
// More creative (higher temperature)
let ai = Neural::new("sk-xxx").with_temperature(1.5);

// More deterministic (lower temperature)
let ai = Neural::new("sk-xxx").with_temperature(0.2);

// Default (model-specific)
let ai = Neural::new("sk-xxx"); // No temperature set
```

### Max Tokens

```rust
// Limit response length
let ai = Neural::new("sk-xxx").with_max_tokens(500);
```

### Local Models (Ollama)

```rust
let ai = Neural::new("ollama")  // Any non-empty key
    .with_base_url("http://localhost:11434/v1")
    .with_model("llama2");

let response = ai.ask("Hello!").await?;
```

## Chat Conversations

### With System Prompt

```rust
let response = ai.chat_with_system(
    "You are a senior Rust developer. Be concise.",
    "How do I handle errors in async functions?"
).await?;
```

### Multi-turn Conversations

```rust
let mut history = vec![
    ChatMessage::system("You are a cooking assistant."),
];

// First turn
history.push(ChatMessage::user("How do I make pasta?"));
let response = ai.chat(history.clone()).await?;
history.push(ChatMessage::assistant(&response));

// Second turn
history.push(ChatMessage::user("What sauce goes well with it?"));
let response = ai.chat(history).await?;
```

## Message Types

```rust
// System message (sets behavior)
ChatMessage::system("You are helpful.")

// User message (input)
ChatMessage::user("Hello!")

// Assistant message (AI response, for history)
ChatMessage::assistant("Hi there!")
```

## Error Handling

```rust
use nucleus_std::neural::NeuralError;

match ai.ask("Hello").await {
    Ok(response) => println!("{}", response),
    Err(NeuralError::Network(e)) => eprintln!("Network error: {}", e),
    Err(NeuralError::Api(e)) => eprintln!("API error: {}", e),
    Err(NeuralError::Parse(e)) => eprintln!("Parse error: {}", e),
    Err(NeuralError::NoResponse) => eprintln!("No response from model"),
    Err(NeuralError::InvalidApiKey) => eprintln!("Invalid API key"),
}
```

## Practical Examples

### Code Generation

```rust
let ai = Neural::new("sk-xxx").with_model("gpt-4o");

let code = ai.chat_with_system(
    "You are a Rust developer. Return only code, no explanations.",
    "Write a function to calculate fibonacci numbers"
).await?;
```

### Text Summarization

```rust
let summary = ai.chat_with_system(
    "Summarize the following text in 3 bullet points.",
    &long_text
).await?;
```

### Content Moderation

```rust
let is_safe = ai.chat_with_system(
    "Respond only with 'safe' or 'unsafe'. Is this content appropriate?",
    &user_content
).await?;

if is_safe.trim().to_lowercase() == "safe" {
    // Allow content
}
```

### Translation

```rust
let spanish = ai.chat_with_system(
    "Translate to Spanish. Return only the translation.",
    "Hello, how are you?"
).await?;
```

## Best Practices

1. **Store API keys securely** - Use environment variables
2. **Set appropriate temperatures** - Lower for factual, higher for creative
3. **Use system prompts** - Define clear behavior expectations
4. **Handle errors gracefully** - Network can fail
5. **Consider rate limits** - Implement backoff for production
6. **Cache responses** - Use Redis cache for identical queries
