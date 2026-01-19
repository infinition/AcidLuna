# AcidLuna

AcidLuna is a Windows-based project built with Rust. It leverages the `windows` crate to interact with Win32 APIs for keyboard, mouse, and window management. Created to solve the lack of drag and drop support and right click support using LunaDisplay from iPad to windows.

## Features

- Native Windows API integration.
- Built with Rust for performance and safety.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Windows OS (for Win32 API support)

## Building

To build the project in release mode:

```bash
cargo build --release
```

The executable will be located in `target/release/AcidLuna.exe`.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
