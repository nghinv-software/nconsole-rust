# NConsole

NConsole là một thư viện logging cho Rust, cho phép gửi log tới WebSocket server và hỗ trợ nhiều kiểu log khác nhau.

## Cài đặt

Thêm dependency vào file `Cargo.toml`:

```toml
[dependencies]
nconsole = "1.0.0"
```

## Sử dụng

Thêm `use nconsole::NConsole;` vào file cần sử dụng.

```rust
use nconsole::NConsole;
use serde_json::json;

fn main() {
    // Set URI WebSocket server
    NConsole::set_uri("ws://localhost:9090");
    // Bật hoặc tắt logging
    NConsole::is_enable(true);

    NConsole::log("Hello, Rustaceans!");
    NConsole::error("Hello, Rustaceans!");
    NConsole::warn("Hello, Rustaceans!");
    NConsole::info("Hello, Rustaceans!");

    NConsole::group("Group 1");
    NConsole::log("Hello, Rustaceans!", "log", &json!({"name": "John", "age": 30}).to_string());
    NConsole::group_end();

    NConsole::group_collapse("Group 2");
    NConsole::log("Hello, Rustaceans!", "log", &json!({"name": "John", "age": 30}).to_string());
    NConsole::group_end();

    NConsole::log("%cLog with color", "color: red", &json!({"name": "John", "age": 30}).to_string()); 

    // Log with multiple data
    NConsole::log("Log with multiple data", "log", &json!({"name": "John", "age": 30, "address": "123 Main St"}).to_string(), &json!({"name": "Jane", "age": 25, "address": "456 Main St"}).to_string());  
}
```

## Các kiểu log

- `log`: Log thông thường
- `error`: Log lỗi
- `warn`: Log cảnh báo
- `info`: Log thông tin
- `group`: Log theo nhóm
- `groupCollapsed`: Log theo nhóm và collapse

# Author

NghiNV

# License

MIT
