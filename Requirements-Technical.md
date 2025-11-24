# **Tarsius — Updated Technical Requirements (Rust + Tauri Architecture)**

Tarsius will be built as a cross-platform desktop application using a Rust backend and a Tauri-based web frontend. This updated specification ensures that all developers operate with the same technical understanding so the project evolves coherently and efficiently.

## **1. Overall Application Architecture**

The application consists of three primary layers: **Core Logic (Rust crates)**, **Storage Layer (Rust)**, and the **Tauri Frontend (Web UI + Rust Commands)**.
The Core layer holds all domain logic including Scratch, Project, Outline, Template handling, and LaTeX rendering.
The Storage layer handles filesystem persistence and indexing.
The Frontend manages all user interaction, calls into Rust via `tauri::command`, and displays data.

The application must run smoothly on Windows, macOS, and Linux without requiring external dependencies beyond the Tauri runtime and the embedded LaTeX engine (Tectonic).

## **2. Core Technologies**

The backend must be implemented entirely in Rust (stable toolchain). The user interface is built with Tauri, with the frontend implemented using a modern web framework (Svelte, React, or Vue). For text editing, Tarsius must use a high-quality code editor component such as Monaco Editor or CodeMirror 6 to support syntax highlighting, linting, multi-cursor editing, drag-and-drop, and large document handling.

LaTeX compilation must use Tectonic because it is portable, deterministic, and requires no external TeX environment.

## **3. Project Structure and Workspace**

The repository must use a Cargo workspace containing three main crates:
`tarsius-core` for domain logic,
`tarsius-storage` for persistence,
and `tarsius-tauri` for the UI application.

Each crate must have clearly defined boundaries:

* **core** may not depend on UI code,
* **storage** must not contain presentation logic,
* **tauri** should not contain domain logic beyond command wrappers.

The frontend lives inside `tauri-app/src` and remains isolated from Rust code.

## **4. Data Models and Representation**

All models—Scratch, Project, OutlineNode, Template—must be defined as Rust structs with full `serde` serialization support. Data exchanged between backend and frontend must use stable JSON structures. Internal models not intended for UI exposure should be mapped to separate DTOs (Data Transfer Objects).

OutlineNode must support arbitrarily nested hierarchical structures. ScratchLink must store integration mode (Include vs Link) and insertion flags (body, footnote, reference, appendix).

## **5. Frontend–Backend Communication**

All communication between frontend and backend must occur through `#[tauri::command]`. Commands must be deterministic, accepting JSON input and returning JSON output. Any operation that modifies the filesystem or project state must occur on the Rust side. The frontend’s responsibility is strictly presentation and interactivity.

Frontend state (Scratches view, Project view, LaTeX view) must be managed within the UI layer rather than the backend.

## **6. Data Storage and Workspace Handling**

Data must be stored on the local filesystem following the previously defined structure: each Scratch is a standalone `.json` file, and each Project has its own directory with a consistent layout.
The Storage layer must provide traits for `ScratchRepository`, `ProjectRepository`, and `TemplateRepository`, with filesystem-based implementations. In later phases, SQLite (FTS5) may be introduced to support indexing and global search.

Autosave must use atomic writes to prevent corruption.

## **7. LaTeX Compilation and PDF Preview**

The LaTeX pipeline consists of two Rust modules: `LatexSourceBuilder` (constructs `.tex` source from template + outline + scratch data) and `LatexCompiler` (runs Tectonic asynchronously).
Generated PDFs must be stored inside the Project directory and displayed in an embedded PDF viewer within the frontend.

Live preview must use debounce logic on the frontend: after the user stops typing for a short interval, the backend receives updates, stores changes, and triggers a recompilation.

## **8. Frontend UI Requirements**

The UI must offer three main panels—Scratches View, Project View, and LaTeX View—with support for panel switching and split views. The Project editor must use Monaco/CodeMirror and handle large documents smoothly.
The UI must support drag-and-drop of Scratches into OutlineNodes.
UI state must be managed using an appropriate state store (Svelte Store, React Zustand, etc.).
All LaTeX compilation errors must be displayed in a dedicated Log panel, ideally with clickable references that highlight related content.

## **9. Performance and Optimization**

The application must efficiently handle:

* 5000+ Scratches
* 200+ OutlineNodes
* LaTeX documents up to ~200 pages

Search operations must feel near real-time.
Heavy operations—LaTeX parsing, scratch indexing, document merging—must run asynchronously in the backend.
Frontend lists (Scratch list, Outline list) must use virtualization for smooth rendering.

## **10. Security, Validation, and Isolation**

The application must not create network connections without explicit user action.
All file operations must remain inside the workspace or user-selected directories.
Path traversal, command injection, and arbitrary file access must be explicitly prevented.
Rust commands must validate all input before writing to disk.

## **11. Extensibility and Future Development**

The design must be modular to allow theming, future plugins, new template packs, and integration layers.
The Core logic must remain fully independent of Tauri, enabling reuse in CLI tools or servers.
The LaTeX templating system must support future expansion (e.g., online template galleries).
The previously defined UML and phase mapping must guide long-term architecture and delivery.

