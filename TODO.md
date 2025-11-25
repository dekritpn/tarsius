# **Tarsius – Developer TODO Roadmap (Rust + Tauri)**

## **Phase 0 — Project Initialization**

* [x] Set up Git repository and Cargo workspace

* [x] Create crates:
  
  * [x] `tarsius-core`
  * [x] `tarsius-storage`
  * [x] `tarsius-tauri`

* [x] Scaffold Tauri app (`cargo tauri init`)

* [x] Confirm builds on Linux (focusing on Linux platform for now)

* [x] Establish coding conventions, linting, formatting (rustfmt, clippy)

* [x] Deliverable: **Baseline project compiles on all platforms**

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

* [x] Implement filesystem-based repositories:
  
  * [x] `ScratchRepository`
  * [x] `ProjectRepository`
  * [x] `TemplateRepository`

* [x] Use atomic file writes for safety

* [x] Implement workspace directory structure:
  
  * [x] `/scratches/`
  * [x] `/projects/<project_id>/`

* [x] Add load/save functions for Scratch and Project

* [x] Deliverable: **Scratches and Projects persist correctly on disk**

---

## **Phase 3 — Tauri Backend Commands**

* [x] Create `#[tauri::command]` endpoints:
  
  * [x] `create_scratch`
  * [x] `update_scratch`
  * [x] `load_scratch`
  * [x] `list_scratches`
  * [x] `delete_scratch`
  * [x] `create_project`
  * [x] `load_project`
  * [x] `save_project`

* [x] Implement state management using `Arc<Mutex<...>>`

* [x] Ensure JSON-safe DTOs for all commands

* [x] Deliverable: **Frontend↔Backend communication working end-to-end**

---

## **Phase 4 — Basic UI Framework**

* [x] Implement frontend framework (Svelte/React/Vue)

* [x] Create initial UI layout:
  
  * [x] Sidebar navigation (Scratches / Projects / LaTeX)
  * [x] Simple editor page for Scratch testing

* [x] Implement API wrappers using `@tauri-apps/api`

* [x] Test command roundtrip (UI → Rust → UI)

* [x] Deliverable: **UI connected to Rust backend with basic forms**

---

## **Phase 5 — Scratches System (Full Implementation)**

* [x] Build Scratches List View with virtualized list

* [x] Build Scratch Editor using Monaco/CodeMirror

* [x] Add metadata panel (created_at, modified_at, tags, source)

* [x] Implement:
  
  * [x] Create new Scratch
  * [x] Edit Scratch content
  * [x] Delete Scratch
  * [x] Autosave
  * [x] Search by text and tags

* [x] Deliverable: **Fully functioning Scratch-based note system**

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
