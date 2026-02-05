# Clipper

A metamodern macOS menu bar app that displays statistics of your clipboard content, copies them to the clipboard, and acts as a separator between other menu bar items thanks to its lack of an icon.

## Installation

Download the latest `Clipper.dmg` from [Releases](https://github.com/dimitrygrebenyuk/clipper/releases) and drag Clipper.app to your Applications folder.

### Building from Source

```bash
# Clone the repository
git clone https://github.com/dimitrygrebenyuk/clipper.git
cd clipper

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build for production
cargo tauri build
```

## Requirements

- macOS 10.15 or later
- For development: Rust 1.70+