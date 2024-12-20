use chrono::Utc;
use serde::Serialize;
use serde_json::json;
use std::fmt::Debug;
use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;

static mut CONSOLE: Option<WebConsole> = None;
const DEFAULT_URI: &str = "ws://localhost:9090";

pub struct WebConsole {
    uri: String,
    is_enable: bool,
    web_socket: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    current_group: Vec<String>,
    client_info: ClientInfo,
}

impl WebConsole {
    fn get_uri(uri: Option<&str>) -> String {
        let uri = match uri {
            None => return DEFAULT_URI.to_string(),
            Some(u) => u.trim(),
        };

        let mut uri_new = String::from(uri);

        // Add ws:// prefix if not present
        if !uri_new.starts_with("ws://") && !uri_new.starts_with("wss://") {
            uri_new = format!("ws://{}", uri_new);
        }

        let uri_parts: Vec<&str> = uri_new.split(':').collect();

        match uri_parts.len() {
            // If already has port number
            3 => uri_new,

            // If needs port number
            2 => {
                let ip_parts: Vec<&str> = uri_parts[1].split('.').collect();

                if ip_parts.len() == 4 || uri_parts[1] == "localhost" {
                    format!("{}:9090", uri_new)
                } else {
                    uri_new
                }
            }

            // Other cases
            _ => uri_new,
        }
    }

    fn new() -> Self {
        WebConsole {
            uri: Self::get_uri(None),
            is_enable: true,
            web_socket: None,
            current_group: Vec::new(),
            client_info: ClientInfo::new(),
        }
    }

    fn get_instance() -> &'static mut WebConsole {
        unsafe {
            if CONSOLE.is_none() {
                CONSOLE = Some(WebConsole::new());
            }
            CONSOLE.as_mut().unwrap()
        }
    }

    fn send_log<T: Debug + Serialize>(&mut self, log_type: &str, args: &[T]) {
        if !self.is_enable {
            return;
        }

        if self.web_socket.is_none() {
            self.connect_web_socket();
        }

        if let Some(ws) = &mut self.web_socket {
            let payload = json!({
                "timestamp": Utc::now().timestamp(),
                "logType": log_type,
                "language": "rust",
                "secure": false,
                "payload": {
                    "data": json!({
                        "clientInfo": self.client_info,
                        "data": args,
                    }).to_string(),
                },
            });
            ws.send(Message::Text(payload.to_string()))
                .unwrap_or_else(|_| {
                    eprintln!("Failed to send log message");
                });
        }
    }

    fn connect_web_socket(&mut self) {
        match connect(Url::parse(&self.uri).unwrap()) {
            Ok((ws, _)) => self.web_socket = Some(ws),
            Err(e) => eprintln!("Failed to connect to WebSocket server: {}", e),
        }
    }
}

pub struct NConsole;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LogArg {
    String(String),
    Number(f64),
    Bool(bool),
    Object(serde_json::Value),
}

impl From<&str> for LogArg {
    fn from(s: &str) -> Self {
        LogArg::String(s.to_string())
    }
}

impl From<serde_json::Value> for LogArg {
    fn from(v: serde_json::Value) -> Self {
        LogArg::Object(v)
    }
}

impl NConsole {
    pub fn set_uri(uri: &str) {
        let console = WebConsole::get_instance();
        console.uri = WebConsole::get_uri(Some(uri));
        console.web_socket = None; // Reset connection
    }

    pub fn is_enable(enable: bool) {
        let console = WebConsole::get_instance();
        console.is_enable = enable;
    }

    pub fn log<T: Debug + Serialize>(args: &[T]) {
        let console = WebConsole::get_instance();
        console.send_log("log", args);
    }

    pub fn info<T: Debug + Serialize>(args: &[T]) {
        let console = WebConsole::get_instance();
        console.send_log("info", args);
    }

    pub fn warn<T: Debug + Serialize>(args: &[T]) {
        let console = WebConsole::get_instance();
        console.send_log("warn", args);
    }

    pub fn error<T: Debug + Serialize>(args: &[T]) {
        let console = WebConsole::get_instance();
        console.send_log("error", args);
    }

    pub fn group(label: &str) {
        let console = WebConsole::get_instance();
        console.current_group.push(label.to_string());
        console.send_log("group", &[label]);
    }

    pub fn group_collapsed(label: &str) {
        let console = WebConsole::get_instance();
        console.current_group.push(label.to_string());
        console.send_log("groupCollapsed", &[label]);
    }

    pub fn group_end() {
        let console = WebConsole::get_instance();
        if console.current_group.pop().is_some() {
            console.send_log("groupEnd", &[""]);
        }
    }
}

#[derive(Debug, Serialize)]
struct ClientInfo {
    id: String,
    name: String,
    platform: String,
    version: String,
    os: String,
    os_version: String,
    language: String,
    time_zone: String,
    user_agent: String,
}

impl ClientInfo {
    fn new() -> Self {
        ClientInfo {
            id: format!(
                "Rust/{} ({})",
                rustc_version::version().unwrap(),
                std::env::consts::OS
            ),
            name: "Rust Client".to_string(),
            platform: "rust".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            os: std::env::consts::OS.to_string(),
            os_version: os_info::get().version().to_string(),
            language: std::env::var("LANG").unwrap_or_else(|_| "en-US".to_string()),
            time_zone: chrono::Local::now().offset().to_string(),
            user_agent: format!(
                "Rust/{} ({})",
                rustc_version::version().unwrap(),
                std::env::consts::OS
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_logging() {
        NConsole::set_uri("10.10.30.40");
        NConsole::is_enable(true);

        // Test basic logging
        NConsole::log(&["Hello, World!"]);
        NConsole::info(&["Server started"]);
        NConsole::warn(&["Memory usage high"]);
        NConsole::error(&["Connection failed"]);

        // Test group logging
        NConsole::group("Test Group");
        NConsole::log(&["Inside group"]);
        NConsole::group_end();

        // Test collapsed group
        NConsole::group_collapsed("Collapsed Group");
        NConsole::log(&[
            "%cInside collapsed group",
            "color: green; font-size: 20px; font-weight: bold",
            &json!({"name": "name", "age": 18}).to_string(),
        ]);
        NConsole::group_end();

        // wait 1 seconds
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    #[test]
    fn test_uri_formatting() {
        // Test default URI
        assert_eq!(WebConsole::get_uri(None), "ws://localhost:9090");

        // Test with IP address without port
        assert_eq!(
            WebConsole::get_uri(Some("192.168.1.1")),
            "ws://192.168.1.1:9090"
        );

        // Test with localhost without port
        assert_eq!(WebConsole::get_uri(Some("localhost")), "ws://localhost");

        // Test with full URI
        assert_eq!(
            WebConsole::get_uri(Some("ws://example.com:8080")),
            "ws://example.com:8080"
        );

        // Test with custom domain without port
        assert_eq!(WebConsole::get_uri(Some("example.com")), "ws://example.com");

        // Test with spaces
        assert_eq!(WebConsole::get_uri(Some(" localhost ")), "ws://localhost");
    }
}
