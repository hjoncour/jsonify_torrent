# torrentify

## Setup

```bash

# Empty the Target Folder

$ cargo clean 

# Compile the App

$ cargo build

# Test the App

$ ./target/debug/torrentify <torrent file>

# Example:

$ ./target/debug/torrentify src/debian.iso.torrent

 # Compile for Release

$ cargo build --release

# Add to Path

$ cp target/release/torrentify /usr/local/bin

# Run App Anywhere

$ torrentify debian.iso.torrent

```