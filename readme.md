# OPC UA Client with Discovery and Subscription Support

This project implements an OPC UA client in Rust that includes functionality for discovering servers on a discovery server and subscribing to monitored items. It demonstrates how to interact with OPC UA servers using the [opcua](https://crates.io/crates/opcua) crate.

---

## Features

### 1. Server Discovery
- **Discovery Mechanism**: Connects to an OPC UA discovery server and retrieves a list of available servers.
- **Server Information**: Displays application names and discovery URLs for all discovered servers.

### 2. Data Subscription
- **Subscription Management**: Creates a subscription to monitored items on the selected server.
- **Data Change Notifications**: Prints value changes from the server in real-time for monitored items.

---

## Installation

### Prerequisites
1. **Rust and Cargo**: Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. **Tokio Runtime**: This program uses `tokio` for asynchronous operations.
3. **OPC UA Environment**: You need an OPC UA server or discovery server running to connect.

### Clone the Repository
```bash
git clone https://github.com/your-repo/opcua-client.git
cd opcua-client
