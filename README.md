# Rust Port Scanner

A simple TCP port scanner written in Rust for learning cybersecurity and Rust fundamentals.

## Features

- ✅ Scans a range of TCP ports
- ✅ Checks whether a port accepts a TCP connection
- ✅ Displays OPEN or CLOSED status
- ✅ Command-line argument support (target, port range)
- ✅ Connection timeout (1 second) for faster scanning
- ✅ Displays summary of open ports
- ✅ Works with any target (localhost, remote hosts, etc.)

## Installation

Requires Rust 2024 edition

```bash
cargo build --release
```

## Usage

### Default (localhost, ports 8000-8010)
```bash
cargo run
```

### Specify target
```bash
cargo run -- 192.168.1.1
```

### Specify target and port range
```bash
cargo run -- 192.168.1.1 80 443
```

### Full example
```bash
cargo run -- 127.0.0.1 8000 8010
```

## Example Output

```text
=== Rust Port Scanner ===

Target: 127.0.0.1
Port Range: 8000 - 8010
Scanning...

[+] 127.0.0.1:8000 OPEN
[-] 127.0.0.1:8001 CLOSED
[-] 127.0.0.1:8002 CLOSED
[+] 127.0.0.1:8003 OPEN
...

=== Scan Complete ===
Open ports: 8000, 8003
```

## Command-line Arguments

```
USAGE: rust-portscan [TARGET] [START_PORT] [END_PORT]

ARGUMENTS:
  TARGET      - IP address or hostname (default: 127.0.0.1)
  START_PORT  - Starting port number (default: 8000)
  END_PORT    - Ending port number (default: 8010)
```

## Notes

- **Timeout:** Each connection attempt has a 1-second timeout
- **Localhost:** For scanning localhost, use `127.0.0.1`
- **Remote hosts:** Can scan any IP address (with permission)
- **Ethical use:** Only scan networks you own or have permission to test

## Future Improvements

- [ ] Parallel scanning with `tokio`
- [ ] Service name detection
- [ ] Custom timeout configuration
- [ ] Output to file
- [ ] Progress bar
