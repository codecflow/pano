# Pano

A lightweight utility for displaying web content in a transparent, always-on-top window.

## Features

- Transparent, always-on-top window
- Display web content from any URL
- Control via command-line arguments or Unix Domain Socket
- Cross-platform support (Windows, macOS, Linux)

## Installation

```bash
# Clone the repository
git clone [repository-url]
cd pano

# Build the project
cargo build --release

# The binary will be available at target/release/pano
```

## Usage

### Command-line Options

```bash
# Basic usage
pano --url https://example.com

# Specify size and position
pano --url https://example.com --width 800 --height 600 -x 100 -y 100

# Enable GPU acceleration
pano --url https://example.com --gpu

# Custom socket path
pano --url https://example.com --socket /path/to/socket
```

### Remote Control

You can control a running instance by sending commands to the Unix Domain Socket:

```bash
# Update URL
echo "url https://new-example.com" > /tmp/pano

# Resize window
echo "resize 1024 768" > /tmp/pano

# Move window
echo "move 200 300" > /tmp/pano
```

## Use Cases

- **Streaming Overlays**: Create transparent overlays for streaming content, such as viewer counts, notifications, or custom browser widgets
- Desktop widgets and information displays
- Floating dashboards and monitoring tools
- Web-based HUDs (Heads-Up Displays) for various applications
