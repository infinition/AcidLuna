# AcidLuna

AcidLuna is a lightweight background utility for Windows, built in Rust, designed to enhance the experience of using **LunaDisplay** with an iPad. 

It specifically solves the lack of native drag-and-drop and right-click support when using an iPad as a secondary display for Windows.

## How it Works

AcidLuna runs silently in your system tray (look for the crescent moon icon) and intercepts specific keyboard inputs to simulate mouse actions:

- **Drag & Drop**: Hold the **Left CTRL** key to simulate holding down the Left Mouse Button. This allows you to perform drag-and-drop operations easily using the iPad's touch interface or Apple Pencil.
- **Right Click**: Press the **Globe key** (which Windows maps to **Caps Lock**) to trigger a Right Click at the current cursor position. AcidLuna intercepts this key to prevent toggling Caps Lock while providing the right-click functionality.

## Features

- **Stealth Mode**: Runs without a terminal window, living entirely in the system tray.
- **System Tray Menu**: Right-click the crescent moon icon to exit the application.
- **Low Latency**: Built with Rust and native Win32 hooks for near-instant response.
- **Custom Icon**: Integrated crescent moon icon for both the executable and the tray.

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
