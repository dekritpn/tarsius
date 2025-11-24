#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;
use tauri::State;
use tarsius_core::{ScratchManager, ProjectManager, ScratchRepository, ProjectRepository};
use tarsius_storage::{FilesystemScratchRepository, FilesystemProjectRepository, Workspace};

struct AppState {
    scratch_manager: Arc<ScratchManager>,
    project_manager: Arc<ProjectManager>,
}

#[derive(serde::Deserialize)]
struct CreateScratchRequest {
    title: String,
    content: String,
    tags: Vec<String>,
    source: Option<String>,
}

#[derive(serde::Deserialize)]
struct UpdateScratchRequest {
    id: String,
    title: Option<String>,
    content: Option<String>,
    tags: Option<Vec<String>>,
    source: Option<Option<String>>,
}

#[derive(serde::Deserialize)]
struct CreateProjectRequest {
    title: String,
    template_id: String,
    output_dir: String,
}

fn main() {
    let workspace_path = std::env::current_dir().unwrap().join("workspace");
    let workspace = Arc::new(Workspace::new(workspace_path));
    workspace.ensure_dirs().unwrap();

    let scratch_repo: Box<dyn ScratchRepository> = Box::new(FilesystemScratchRepository::new(workspace.clone()));
    let project_repo: Box<dyn ProjectRepository> = Box::new(FilesystemProjectRepository::new(workspace.clone()));

    let scratch_manager = Arc::new(ScratchManager::new(scratch_repo));
    let project_manager = Arc::new(ProjectManager::new(project_repo));

    let state = AppState {
        scratch_manager,
        project_manager,
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            create_scratch,
            update_scratch,
            load_scratch,
            list_scratches,
            delete_scratch,
            create_project,
            load_project,
            save_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn create_scratch(state: State<AppState>, request: CreateScratchRequest) -> std::result::Result<tarsius_core::ScratchDto, String> {
    let scratch = state.scratch_manager.create(request.title, request.content, request.tags, request.source)
        .map_err(|e| format!("Failed to create scratch: {}", e))?;
    Ok(scratch.into())
}

#[tauri::command]
fn update_scratch(state: State<AppState>, request: UpdateScratchRequest) -> std::result::Result<tarsius_core::ScratchDto, String> {
    let scratch = state.scratch_manager.update(request.id, request.title, request.content, request.tags, request.source)
        .map_err(|e| format!("Failed to update scratch: {}", e))?;
    Ok(scratch.into())
}

#[tauri::command]
fn load_scratch(state: State<AppState>, id: String) -> std::result::Result<tarsius_core::ScratchDto, String> {
    let scratch = state.scratch_manager.load(&id)
        .map_err(|e| format!("Failed to load scratch: {}", e))?;
    Ok(scratch.into())
}

#[tauri::command]
fn list_scratches(state: State<AppState>) -> std::result::Result<Vec<tarsius_core::ScratchDto>, String> {
    let scratches = state.scratch_manager.list()
        .map_err(|e| format!("Failed to list scratches: {}", e))?;
    Ok(scratches.into_iter().map(Into::into).collect())
}

#[tauri::command]
fn delete_scratch(state: State<AppState>, id: String) -> std::result::Result<(), String> {
    state.scratch_manager.delete(&id)
        .map_err(|e| format!("Failed to delete scratch: {}", e))
}

#[tauri::command]
fn create_project(state: State<AppState>, request: CreateProjectRequest) -> std::result::Result<tarsius_core::ProjectDto, String> {
    let project = state.project_manager.create(request.title, request.template_id, request.output_dir)
        .map_err(|e| format!("Failed to create project: {}", e))?;
    Ok(project.into())
}

#[tauri::command]
fn load_project(state: State<AppState>, id: String) -> std::result::Result<tarsius_core::ProjectDto, String> {
    let project = state.project_manager.load(&id)
        .map_err(|e| format!("Failed to load project: {}", e))?;
    Ok(project.into())
}

#[tauri::command]
fn save_project(state: State<AppState>, project_dto: tarsius_core::ProjectDto) -> std::result::Result<(), String> {
    let project = project_dto.try_into().map_err(|e| format!("Invalid project data: {}", e))?;
    state.project_manager.save(&project)
        .map_err(|e| format!("Failed to save project: {}", e))
}
