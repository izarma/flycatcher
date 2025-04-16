# Flycatcher Chat Client 🐈

[![Built with egui](https://img.shields.io/badge/built%20with-egui-%23FFA726)](https://www.egui.rs/)
[![Async Runtime](https://img.shields.io/badge/async%20runtime-Tokio-%23ECD53F)](https://tokio.rs)
[![Rust](https://img.shields.io/badge/rust-1.72%2B-orange)](https://www.rust-lang.org)

**Work in Progress**  
A native GUI chat client for connecting to async-chat-server, built with Rust and egui.


## Features (Current Development)
- 🖥️ Native GUI interface using egui/eframe
- 🔌 Basic server connection to async-chat-server
- 📨 Simple message sending/receiving
- 🧵 Async I/O using Tokio runtime
- � JSON message serialization

## Planned Features
- Message history display
- Multiple server connections
- User authentication support
- Chat room management
- Message formatting options
- Connection status indicators
- Notification system
- Client configuration persistence

## Installation

### Prerequisites
- Rust 1.72+
- Cargo package manager
- System dependencies for native GUI:
  ```bash
  # Ubuntu/Debian
  sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

  # macOS (via Homebrew)
  brew install openssl cmake

  # Windows: VS Build Tools
  ```

```bash
git clone https://github.com/yourusername/flycatcher.git
cd flycatcher

# Run in debug mode
cargo run

# Build release version
cargo build --release
```

## Usage
1. Start your [flycatcher-chat-server](https://github.com/izarma/flycatcher-chat-server)
2. Launch Flycatcher:
   ```bash
   cargo run -- --server localhost:8080
   ```
3. Interact with the GUI:
   - Connect/Disconnect using control buttons
   - Type messages in the input field
   - View incoming messages in the chat history panel

## Key Dependencies
| Crate | Purpose |
|-------|---------|
| `egui` | Immediate mode GUI |
| `eframe` | Native window framework |
| `tokio` | Async runtime |
| `serde` | Message serialization |
| `futures` | Async abstractions |

## Implementation Overview
1. **GUI Architecture**:
   - Uses egui's immediate mode rendering
   - Separate async thread for network operations
   - Channel-based communication between GUI and network layers

2. **Network Layer**:
   ```rust
   async fn connect_to_server(addr: &str) -> Result<Connection> {
       let socket = TcpStream::connect(addr).await?;
       let (reader, writer) = socket.into_split();
       Ok(Connection { reader, writer })
   }
   ```
3. **Message Protocol**:
   ```json
   {
     "user": "Alice",
     "content": "Hello world!",
     "timestamp": 1690834567
   }
   ```

## Development Roadmap
1. Implement basic message exchange ✔️
2. Add connection management UI
3. Introduce message formatting
4. Implement user authentication flow
5. Add configuration persistence

## Contributing
This project is in active development. Contributions welcome!
1. Check open issues for current priorities
2. Follow egui's design patterns
3. Maintain async/await best practices
4. Write integration tests for network operations

## License
[MIT License](LICENSE)  
