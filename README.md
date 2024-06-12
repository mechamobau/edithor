# Edithor

**⚠️ Work in Progress: Edithor is currently under active development. Features and functionality are subject to change. Contributions and feedback are welcome!**

Edithor is a terminal-based text editor inspired by Vim, written in Rust. It aims to provide a familiar and efficient editing experience while leveraging the Language Server Protocol (LSP) for advanced language features, including syntax highlighting.

## Features

- **Vim-inspired Keybindings**: Enjoy the power of Vim's modal editing in a Rust-based terminal editor.
- **LSP Support**: Integrated LSP for real-time syntax highlighting and other language-specific features.
- **Performance**: Written in Rust for safety and performance.
- **Extensible**: Easily extendable with plugins to add new features and language supports.
- **Customizable**: Highly configurable to suit your personal workflow.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.

### Building from Source

1. Clone the repository:
    ```sh
    git clone https://github.com/mechamobau/edithor.git
    cd edithor
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Add the binary to your PATH:
    ```sh
    cp target/release/edithor /usr/local/bin/
    ```

## Usage

Launch Edithor from your terminal:
```sh
edithor <filename>
```

### Basic Commands

- `i` - Enter insert mode.
- `C-s` - Save the current file.
- `C-q` - Quit Edithor.

## Contributing

We welcome contributions from the community! Please read our [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on how to get involved.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE.md) file for details.
