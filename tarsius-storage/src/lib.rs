use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tarsius_core::*;
use serde_json;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Entity not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, StorageError>;

pub trait ScratchRepository {
    fn save(&self, scratch: &Scratch) -> Result<()>;
    fn load(&self, id: &str) -> Result<Scratch>;
    fn list(&self) -> Result<Vec<Scratch>>;
    fn delete(&self, id: &str) -> Result<()>;
}

pub trait ProjectRepository {
    fn save(&self, project: &Project) -> Result<()>;
    fn load(&self, id: &str) -> Result<Project>;
    fn list(&self) -> Result<Vec<Project>>;
    fn delete(&self, id: &str) -> Result<()>;
}

pub trait TemplateRepository {
    fn save(&self, template: &Template) -> Result<()>;
    fn load(&self, id: &str) -> Result<Template>;
    fn list(&self) -> Result<Vec<Template>>;
    fn delete(&self, id: &str) -> Result<()>;
}

pub struct Workspace {
    base_path: PathBuf,
}

impl Workspace {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    pub fn scratches_dir(&self) -> PathBuf {
        self.base_path.join("scratches")
    }

    pub fn projects_dir(&self) -> PathBuf {
        self.base_path.join("projects")
    }

    pub fn templates_dir(&self) -> PathBuf {
        self.base_path.join("templates")
    }

    pub fn ensure_dirs(&self) -> Result<()> {
        fs::create_dir_all(self.scratches_dir())?;
        fs::create_dir_all(self.projects_dir())?;
        fs::create_dir_all(self.templates_dir())?;
        Ok(())
    }
}

pub struct FilesystemScratchRepository<'a> {
    workspace: &'a Workspace,
}

impl<'a> FilesystemScratchRepository<'a> {
    pub fn new(workspace: &'a Workspace) -> Self {
        Self { workspace }
    }

    fn scratch_path(&self, id: &str) -> PathBuf {
        self.workspace.scratches_dir().join(format!("{}.json", id))
    }
}

impl<'a> ScratchRepository for FilesystemScratchRepository<'a> {
    fn save(&self, scratch: &Scratch) -> Result<()> {
        let path = self.scratch_path(&scratch.id);
        let json = serde_json::to_string_pretty(scratch)?;
        write_atomic(&path, json)?;
        Ok(())
    }

    fn load(&self, id: &str) -> Result<Scratch> {
        let path = self.scratch_path(id);
        if !path.exists() {
            return Err(StorageError::NotFound(format!("Scratch {}", id)));
        }
        let content = fs::read_to_string(path)?;
        let scratch: Scratch = serde_json::from_str(&content)?;
        Ok(scratch)
    }

    fn list(&self) -> Result<Vec<Scratch>> {
        let mut scratches = Vec::new();
        for entry in fs::read_dir(self.workspace.scratches_dir())? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                let scratch: Scratch = serde_json::from_str(&content)?;
                scratches.push(scratch);
            }
        }
        Ok(scratches)
    }

    fn delete(&self, id: &str) -> Result<()> {
        let path = self.scratch_path(id);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}

pub struct FilesystemProjectRepository<'a> {
    workspace: &'a Workspace,
}

impl<'a> FilesystemProjectRepository<'a> {
    pub fn new(workspace: &'a Workspace) -> Self {
        Self { workspace }
    }

    fn project_dir(&self, id: &str) -> PathBuf {
        self.workspace.projects_dir().join(id)
    }

    fn project_path(&self, id: &str) -> PathBuf {
        self.project_dir(id).join("project.json")
    }
}

impl<'a> ProjectRepository for FilesystemProjectRepository<'a> {
    fn save(&self, project: &Project) -> Result<()> {
        let dir = self.project_dir(&project.id);
        fs::create_dir_all(&dir)?;
        let path = self.project_path(&project.id);
        let json = serde_json::to_string_pretty(project)?;
        write_atomic(&path, json)?;
        Ok(())
    }

    fn load(&self, id: &str) -> Result<Project> {
        let path = self.project_path(id);
        if !path.exists() {
            return Err(StorageError::NotFound(format!("Project {}", id)));
        }
        let content = fs::read_to_string(path)?;
        let project: Project = serde_json::from_str(&content)?;
        Ok(project)
    }

    fn list(&self) -> Result<Vec<Project>> {
        let mut projects = Vec::new();
        for entry in fs::read_dir(self.workspace.projects_dir())? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let dir_name = entry.file_name().to_string_lossy().to_string();
                if let Ok(project) = self.load(&dir_name) {
                    projects.push(project);
                }
            }
        }
        Ok(projects)
    }

    fn delete(&self, id: &str) -> Result<()> {
        let dir = self.project_dir(id);
        if dir.exists() {
            fs::remove_dir_all(dir)?;
        }
        Ok(())
    }
}

pub struct FilesystemTemplateRepository<'a> {
    workspace: &'a Workspace,
}

impl<'a> FilesystemTemplateRepository<'a> {
    pub fn new(workspace: &'a Workspace) -> Self {
        Self { workspace }
    }

    fn template_path(&self, id: &str) -> PathBuf {
        self.workspace.templates_dir().join(format!("{}.json", id))
    }
}

impl<'a> TemplateRepository for FilesystemTemplateRepository<'a> {
    fn save(&self, template: &Template) -> Result<()> {
        let path = self.template_path(&template.id);
        let json = serde_json::to_string_pretty(template)?;
        write_atomic(&path, json)?;
        Ok(())
    }

    fn load(&self, id: &str) -> Result<Template> {
        let path = self.template_path(id);
        if !path.exists() {
            return Err(StorageError::NotFound(format!("Template {}", id)));
        }
        let content = fs::read_to_string(path)?;
        let template: Template = serde_json::from_str(&content)?;
        Ok(template)
    }

    fn list(&self) -> Result<Vec<Template>> {
        let mut templates = Vec::new();
        for entry in fs::read_dir(self.workspace.templates_dir())? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                let template: Template = serde_json::from_str(&content)?;
                templates.push(template);
            }
        }
        Ok(templates)
    }

    fn delete(&self, id: &str) -> Result<()> {
        let path = self.template_path(id);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}

fn write_atomic<P: AsRef<Path>>(path: P, content: String) -> Result<()> {
    let path = path.as_ref();
    let temp_path = path.with_extension("tmp");
    fs::write(&temp_path, content)?;
    fs::rename(temp_path, path)?;
    Ok(())
}