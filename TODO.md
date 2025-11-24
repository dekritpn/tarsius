# **Tarsius – Developer TODO Roadmap (Rust + Tauri)**

## **Phase 0 — Project Initialization**

* [ ] Set up Git repository and Cargo workspace
* [ ] Create crates:

  * [ ] `tarsius-core`
  * [ ] `tarsius-storage`
  * [ ] `tarsius-tauri`
* [ ] Scaffold Tauri app (`cargo tauri init`)
* [ ] Confirm builds on Linux (focusing on Linux platform for now)
* [ ] Establish coding conventions, linting, formatting (rustfmt, clippy)
* [ ] Deliverable: **Baseline project compiles on all platforms**

---

## **Phase 1 — Core Domain Models**

* [x] Implement data models in `tarsius-core`:

  * [x] `Scratch`
  * [x] `Project`
  * [x] `OutlineNode`
  * [x] `ScratchLink`
  * [x] `Template`
  * [x] `ProjectSettings`
* [x] Add `serde` serialization for all models
* [x] Implement initial `ScratchManager` and `ProjectManager`
* [x] Define DTOs for frontend exposure
* [x] Deliverable: **Core logic compiles with complete model definitions**

---

## **Phase 2 — Storage Layer**

* [ ] Implement filesystem-based repositories:

  * [ ] `ScratchRepository`
  * [ ] `ProjectRepository`
  * [ ] `TemplateRepository`
* [ ] Use atomic file writes for safety
* [ ] Implement workspace directory structure:

  * [ ] `/scratches/`
  * [ ] `/projects/<project_id>/`
* [ ] Add load/save functions for Scratch and Project
* [ ] Deliverable: **Scratches and Projects persist correctly on disk**

---

## **Phase 3 — Tauri Backend Commands**

* [ ] Create `#[tauri::command]` endpoints:

  * [ ] `create_scratch`
  * [ ] `update_scratch`
  * [ ] `load_scratch`
  * [ ] `list_scratches`
  * [ ] `delete_scratch`
  * [ ] `create_project`
  * [ ] `load_project`
  * [ ] `save_project`
* [ ] Implement state management using `Arc<Mutex<...>>`
* [ ] Ensure JSON-safe DTOs for all commands
* [ ] Deliverable: **Frontend↔Backend communication working end-to-end**

---

## **Phase 4 — Basic UI Framework**

* [ ] Implement frontend framework (Svelte/React/Vue)
* [ ] Create initial UI layout:

  * [ ] Sidebar navigation (Scratches / Projects / LaTeX)
  * [ ] Simple editor page for Scratch testing
* [ ] Implement API wrappers using `@tauri-apps/api`
* [ ] Test command roundtrip (UI → Rust → UI)
* [ ] Deliverable: **UI connected to Rust backend with basic forms**

---

## **Phase 5 — Scratches System (Full Implementation)**

* [ ] Build Scratches List View with virtualized list
* [ ] Build Scratch Editor using Monaco/CodeMirror
* [ ] Add metadata panel (created_at, modified_at, tags, source)
* [ ] Implement:

  * [ ] Create new Scratch
  * [ ] Edit Scratch content
  * [ ] Delete Scratch
  * [ ] Autosave
  * [ ] Search by text and tags
* [ ] Deliverable: **Fully functioning Scratch-based note system**

---

## **Phase 6 — Project System**

* [ ] Implement Project Outline View:

  * [ ] Sections
  * [ ] Subsections
  * [ ] Reordering
* [ ] Implement Document Editor (Monaco/CodeMirror)
* [ ] Implement Scratches Panel filtered by search
* [ ] Enable drag-and-drop from Scratches into Outline:

  * [ ] Include mode (copy body)
  * [ ] Store link metadata for later use
* [ ] Deliverable: **User can assemble documents from Scratches**

---

## **Phase 7 — LaTeX Pipeline**

* [ ] Implement `LatexSourceBuilder`:

  * [ ] Combine template preamble
  * [ ] Convert OutlineNode hierarchy to LaTeX
  * [ ] Insert included Scratch content
  * [ ] Insert linked Scratch content as:

    * [ ] Body
    * [ ] Footnote
    * [ ] Reference
    * [ ] Appendix
* [ ] Implement `LatexCompiler` using Tectonic:

  * [ ] Async compilation
  * [ ] Error capture and parsing
* [ ] Deliverable: **Export .tex and compile to PDF successfully**

---

## **Phase 8 — Live PDF Preview**

* [ ] Add PDF viewer panel in frontend
* [ ] Implement debounce logic in frontend
* [ ] Trigger rebuild on backend when document changes
* [ ] Update PDF preview automatically
* [ ] Display error logs with clickable references
* [ ] Deliverable: **Real-time LaTeX preview with auto-compile**

---

## **Phase 9 — Global Search & Indexing**

* [ ] Implement search indexing:

  * [ ] Optional SQLite FTS5
  * [ ] Or custom in-memory index
* [ ] Add unified "Search Everywhere" UI
* [ ] Search across:

  * [ ] Scratch body
  * [ ] Scratch tags
  * [ ] Project documents
  * [ ] OutlineNode titles
* [ ] Deliverable: **Fast, near real-time global search**

---

## **Phase 10 — Optimization & Stability**

* [ ] Performance tests:

  * [ ] 5000+ Scratches
  * [ ] 200+ OutlineNodes
  * [ ] Large LaTeX documents
* [ ] Smooth scrolling and rendering optimizations
* [ ] Async operations for all heavy tasks
* [ ] UI state persistence (layout, split views)
* [ ] Memory usage review and tuning
* [ ] Deliverable: **Stable, performant app ready for end-users**

---

## **Phase 11 — Future Extensions (Optional)**

* [ ] Template gallery support
* [ ] Plugin system
* [ ] Git-based version control
* [ ] Cloud sync
* [ ] Multi-window or workspace switching


