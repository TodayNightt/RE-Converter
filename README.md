<div align="center">

<br>
<img src="assets/img.png" height="100" alt="RE-Converter Showcase">

# RE:Converter

<img src="assets/showcase.gif" height="500" alt="RE-Converter Showcase">

</div>

A modern, user-friendly video and audio converter built with Rust and FFmpeg. RE-Converter provides an intuitive
graphical interface for encoding multimedia files with customizable parameters.

## Features

- **Simple GUI Interface** - Built with iced-rs for a responsive, cross-platform experience
- **FFmpeg Integration** - Leverages the power and flexibility of FFmpeg for high-quality conversion
- **Customizable Parameters** - Easy-to-use controls for encoding settings
- **Real-time Progress** - Monitor conversion progress with live updates
- **Batch Processing** - Convert multiple files efficiently

## Prerequisites

### FFmpeg Installation

RE-Converter requires FFmpeg to be installed on your system:

#### Windows

```bash
# Using Chocolatey
choco install ffmpeg

# Using Scoop
scoop install ffmpeg

# Or download from https://ffmpeg.org/download.html
```

### Building

```bash

# Using Cargo (cli program)
cargo build --release --package vidoe_converter_cli

# Using Cargo (iced ui program)
cargo build --release --package video_converter_iced

```

### Embedding FFmpeg into the binary

1. Download the ffmpeg binary @[FFmpeg](https://ffmpeg.org/download.html)
2. Place the `ffmpeg` binary file in the binaries directory of the RE-Converter project
3. Build the project with the `--features embedded` flag to include FFmpeg in the binary

