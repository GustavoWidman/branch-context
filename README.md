# branch-context

This is a simple library that can help with branched contexts when interacting with AI completion models. It was developed for use on my [chatbot](https://github.com/GustavoWidman/chatbot) project, but can be used in other projects as well.

## Installation

To install the library, add the following line to your `Cargo.toml` file:

```toml
[dependencies.branch-context]
git = "https://github.com/GustavoWidman/branch-context"
branch = "main"
```

## Usage

The library exports two structs: `Message` and `Messages` (yes i know, very intuitive). The `Message` struct represents a single message response, and the `Messages` struct can represent all the possible responses to a single prompt. The objective is to have the user prompt the AI and have things such as message editing and regeneration be just a little bit easier. To make a full context tree in your project, you will have to create your own struct that will have a `Vec<Messages>` or something similar (in my case, i ended up using a `IndexMap<u64, Messages>` to map the discord message ID to the possible responses/edits that message could have).

If you'd like to check the full example of how I ended up using this library, you can check the [context](https://github.com/GustavoWidman/chatbot/tree/main/src/chat/context) module of my [chatbot](https://github.com/GustavoWidman/chatbot) project.

### Example

```rust
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub sent_at: DateTime<Utc>,
    pub name: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_call_id: Option<String>,
}

pub struct ChatContext {
    messages: Vec<Messages<ChatMessage>>,
}

... // other relevant code

/// Building the context should be as simple as
impl ChatContext {
    ... // other methods

    pub fn build_context(&mut self) -> Vec<ChatMessage> {
        ... // perform other actions like checking if the messages vector is empty

        let ctx = self
            .messages
            .iter()
            .map(|messages| messages.selected().clone())
            .collect::<Vec<_>>();

        ... // perform other actions like prepending the context with a system prompt

        ctx
    }

    ... // other methods
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
