# RConsole

RConsole is a logging library for Rust, supports sending logs to WebSocket server and supports multiple log types.

![Demo NConsole](https://github.com/nghinv-software/nconsole-flutter/blob/main/assets/demo_nconsole.png)

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

    RConsole::log(&["Hello, World!"]);
        RConsole::info(&["Server started"]);
        RConsole::warn(&["Memory usage high"]);
        RConsole::error(&["Connection failed"]);

        RConsole::group("Test Group");
        RConsole::log(&["Inside group"]);
        RConsole::group_end();

        RConsole::group_collapsed("Collapsed Group");
        RConsole::log(&[
            "%cInside collapsed group",
            "color: green; font-size: 20px; font-weight: bold",
            &json!({"name": "name", "age": 18}).to_string(),
        ]);
        RConsole::group_end();
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
