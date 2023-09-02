# jsonify_torrent

## Setup

```bash

# Empty the Target Folder

$ cargo clean 

# Compile the App

$ cargo build

# Test the App

$ ./target/debug/jsonify_torrent <torrent file>

# Example:

$ ./target/debug/jsonify_torrent src/debian.iso.torrent

 # Compile for Release

$ cargo build --release

# Add to Path

$ cp target/release/jsonify_torrent /usr/local/bin

# Run App Anywhere

$ jsonify_torrent debian.iso.torrent

```