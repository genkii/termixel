# Termixel

> **This project was not written by a clanker**

Termixel is a minimal and sort of fast terminal sprite renderer. It uses Unicode characters to render pixel art directly in the terminal.
Termixel is heavily optimized for size and currently comes in at around **200 KB**. It currently only supports `.png` files and will probably never support any other formats.
If you want support for more file formats, feel free to fork the repo and implement it yourself.

> Termixel is designed for rendering pixel art and may not be suitable for bigger images.

## Installation

### Linux

If you want to quickly install Termixel, you can use our installation script:

```bash
curl -fsSL https://raw.githubusercontent.com/genkii/termixel/main/build.sh | bash
```

The script downloads the latest Termixel binary and installs it to `~/.local/bin`.

After the installation, make sure `~/.local/bin` is in your `PATH`. If it isn't, add this to your shell configuration:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

You can also build Termixel yourself from source if you prefer.

### From Source

Clone the repository and build it using Cargo:

```bash
git clone https://github.com/genkii/termixel.git
cd termixel
cargo build --release
```

If you want a smaller binary you can also use the `build.sh` script to build the project.

The compiled binary will be located in:

```text
target/release/termixel
```


## Setup

Termixel looks for `.png` files in its config directory.

### Linux

```text
~/.config/termixel/
```

### Windows

```text
%APPDATA%\termixel\
```

Just put your `.png` files in the folder. For example:

```text
termixel/
├── creeper.png
├── sword.png
└── tree.png
```

## CLI

### `termixel list`

Lists all available sprites.

### `termixel sprite <name>`

Renders a sprite in the terminal.


The name is the filename without `.png`.

## Supported Formats

Currently, only `.png` is supported.
There probably won't be support for other formats in the future. If you need another format, feel free to fork the repo and add it yourself.

## Limitations

Termixel is made for small pixel-art sprites. Bigger images can get difficult to read and take up a lot of space in the terminal.
The main goal of the project is to keep it small, simple, and portable.

## Contributing

If you want to improve Termixel or add something to it, feel free to open an issue or pull request.
