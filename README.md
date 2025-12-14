# Tarsius

Tarsius is a cross-platform desktop application for note-taking and LaTeX document compilation. It allows users to create and manage "Scratches" (notes) and assemble them into structured "Projects" for generating LaTeX documents with live PDF preview.

## Features

- **Scratches**: Create and edit notes with a modern, card-based interface
- **Projects**: Organize notes into hierarchical outlines and compile to LaTeX
- **Modern UI**: Dark theme with glassmorphism, smooth animations, and professional typography
- **LaTeX Integration**: Automatic compilation using Tectonic for portable PDF generation *(coming in Phase 7)*
- **Cross-Platform**: Runs on Windows, macOS, and Linux
- **Live Preview**: Real-time PDF updates as you edit *(coming in Phase 8)*

## Current Status

The project is currently in **Phase 6** of development:

- ✅ **Phase 0-6**: Core functionality complete (Scratches, Projects, UI Framework)
- 🚧 **Phase 7**: LaTeX Pipeline (in progress)
- 🚧 **Phase 8**: Live PDF Preview (planned)
- 🚧 **Phase 9**: Global Search & Indexing (planned)
- 🚧 **Phase 10**: Optimization & Stability (planned)

See [TODO.md](TODO.md) for detailed development roadmap.

## Architecture

### Backend (Rust)

- **`tarsius-core`**: Domain logic for Scratches, Projects, and LaTeX processing
- **`tarsius-storage`**: Filesystem-based persistence layer
- **`tarsius-tauri`**: Tauri application with web frontend integration

### Frontend (Svelte)

- Modern web UI with dark theme and glassmorphism effects
- Monaco Editor integration for rich text editing
- Drag-and-drop functionality for organizing content
- Responsive, card-based layouts

### Key Design Principles

- Clean separation of concerns between layers
- Comprehensive data models with serde serialization
- DTO pattern for frontend-backend communication
- Atomic file writes for data integrity
- Modular, testable architecture

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
# Run in development mode
cargo tauri dev

# Build frontend only
cd tarsius-tauri/frontend
npm install
npm run build
```

## Contributing

Contributions are welcome! Please see [TODO.md](TODO.md) for current development phases and [Requirements-Technical.md](Requirements-Technical.md) for technical specifications.

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.