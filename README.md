# Tarsius

Tarsius is a cross-platform desktop application for note-taking and LaTeX document compilation. It allows users to create and manage "Scratches" (notes) and assemble them into structured "Projects" for generating LaTeX documents with live PDF preview.

## Features

- **Scratches**: Create and edit notes with rich text support.
- **Projects**: Organize notes into hierarchical outlines and compile to LaTeX.
- **LaTeX Integration**: Automatic compilation using Tectonic for portable PDF generation.
- **Cross-Platform**: Runs on Windows, macOS, and Linux.
- **Live Preview**: Real-time PDF updates as you edit.

## Architecture

- **Backend**: Rust with three crates:
  - `tarsius-core`: Domain logic for Scratches, Projects, and LaTeX processing.
  - `tarsius-storage`: Filesystem-based persistence.
  - `tarsius-tauri`: Tauri application with web frontend.
- **Frontend**: Web UI built with modern frameworks (Svelte/React/Vue) using Monaco/CodeMirror for editing.

## Installation

### Prerequisites

- Rust (stable toolchain)
- Node.js (for frontend development)
- Tauri CLI

### Build

```bash
git clone https://github.com/dekritpn/tarsius.git
cd tarsius
cargo tauri build
```

### Development

```bash
cargo tauri dev
```

## Contributing

Contributions are welcome! Please see the TODO.md for current development phases.

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.