# RConsole

RConsole is a logging library for Rust, supports sending logs to WebSocket server and supports multiple log types.

## Installation

Add dependency to `Cargo.toml`:

```toml
[dependencies]
rconsole = "1.0.0"
```

App desktop download [NConsole](https://drive.google.com/drive/folders/1P4cqXhalzsiPtrVAKWvoD9tK_pt9ZpzJ?usp=share_link)

## Usage

Add `use rconsole::RConsole;` to the file you want to use.

```rust
use rconsole::RConsole;
use serde_json::json;

fn main() {
    // Set URI WebSocket server
    RConsole::set_uri("ws://localhost:9090");
    // Enable or disable logging
    RConsole::is_enable(true);

    RConsole::log("Hello, Rustaceans!");
    RConsole::error("Hello, Rustaceans!");
    RConsole::warn("Hello, Rustaceans!");
    RConsole::info("Hello, Rustaceans!");

    RConsole::group("Group 1");
    RConsole::log("Hello, Rustaceans!", "log", &json!({"name": "John", "age": 30}).to_string());
    RConsole::group_end();

    RConsole::group_collapse("Group 2");
    RConsole::log("Hello, Rustaceans!", "log", &json!({"name": "John", "age": 30}).to_string());
    RConsole::group_end();

    RConsole::log("%cLog with color", "color: red", &json!({"name": "John", "age": 30}).to_string()); 

    // Log with multiple data
    RConsole::log("Log with multiple data", "log", &json!({"name": "John", "age": 30, "address": "123 Main St"}).to_string(), &json!({"name": "Jane", "age": 25, "address": "456 Main St"}).to_string());  
}
```

## Log types

- `log`: Normal log
- `error`: Error log
- `warn`: Warning log
- `info`: Info log
- `group`: Log by group
- `groupCollapsed`: Log by group and collapse
- `group_end`: End group

# Author

NghiNV

# License

MIT
