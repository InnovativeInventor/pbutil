## pbutil
Cross-platform `pbcopy` and `pbpaste` in Rust.

Are you tired of not having a `pbcopy` and `pbpaste` equivalent on Linux, etc.?
This is intended to bring the same, nice interface of `pbcopy` and `pbpaste` on macOS to other platforms.

Executables provided in `/target`

## Usage
To copy "Test" into the clipboard:
```bash
echo "Test" | pbcopy
```

To print out the clipboard to stdout:
```bash
pbpaste
```
