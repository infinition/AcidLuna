<img width="677" height="369" alt="AcidLuna" src="https://github.com/user-attachments/assets/26994d6e-a938-48e9-9eaf-c5db98f7ee7d" />

# AcidLuna

[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE) ![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white) ![Windows](https://img.shields.io/badge/Windows-0078D4?style=flat&logo=windows&logoColor=white) [![Release](https://img.shields.io/github/v/release/infinition/AcidLuna?style=flat)](https://github.com/infinition/AcidLuna/releases) [![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-ffdd00?style=flat&logo=buy-me-a-coffee&logoColor=black)](https://buymeacoffee.com/infinition)

A lightweight Windows tray utility written in Rust that adds drag-and-drop and right-click support when using an iPad as a secondary display via LunaDisplay.

LunaDisplay mirrors a Windows desktop to an iPad but does not expose native mouse button events through the touch interface. AcidLuna bridges that gap by intercepting keyboard inputs and translating them into the right mouse actions.

---

## How it works

AcidLuna registers a low-level keyboard hook via Win32 and runs silently in the system tray:

- **Left Ctrl held down** simulates holding the left mouse button, enabling drag-and-drop through the iPad touch interface or Apple Pencil.
- **Globe key** (mapped to Caps Lock by Windows) triggers a right-click at the current cursor position. The Caps Lock toggle is suppressed so it does not interfere with normal typing.

---

## Features

- No visible window. Lives entirely in the system tray (crescent moon icon).
- Right-click the tray icon to exit.
- Native Win32 hooks for near-instant response latency.
- Custom icon embedded in both the executable and the tray.

---

## Building

Requires Rust (latest stable) on Windows.

```bash
cargo build --release
```

The binary is at `target/release/AcidLuna.exe`.

---

## Star History

<a href="https://www.star-history.com/?repos=infinition%2FAcidLuna&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=infinition/AcidLuna&type=date&theme=dark&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=infinition/AcidLuna&type=date&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=infinition/AcidLuna&type=date&legend=top-left" />
 </picture>
</a>

---

## License

MIT. See [LICENSE](LICENSE).
