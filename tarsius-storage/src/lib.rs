use std::fs;
use std::path::{Path, PathBuf};
use tarsius_core::*;
use serde_json;

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

use std::sync::Arc;

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
        fs::create_dir_all(self.scratches_dir()).map_err(|e| CoreError::Storage(e.to_string()))?;
        fs::create_dir_all(self.projects_dir()).map_err(|e| CoreError::Storage(e.to_string()))?;
        fs::create_dir_all(self.templates_dir()).map_err(|e| CoreError::Storage(e.to_string()))?;
        Ok(())
    }
}

pub struct FilesystemScratchRepository {
    workspace: Arc<Workspace>,
}

impl FilesystemScratchRepository {
    pub fn new(workspace: Arc<Workspace>) -> Self {
        Self { workspace }
    }

    fn scratch_path(&self, id: &str) -> PathBuf {
        self.workspace.scratches_dir().join(format!("{}.json", id))
    }
}

impl tarsius_core::ScratchRepository for FilesystemScratchRepository {
    fn save(&self, scratch: &Scratch) -> Result<()> {
        let path = self.scratch_path(&scratch.id);
        let json = serde_json::to_string_pretty(scratch).map_err(|e| CoreError::Storage(e.to_string()))?;
        write_atomic(&path, json)?;
        Ok(())
    }

    fn load(&self, id: &str) -> Result<Scratch> {
        let path = self.scratch_path(id);
        if !path.exists() {
            return Err(CoreError::NotFound(format!("Scratch {}", id)));
        }
        let content = fs::read_to_string(path).map_err(|e| CoreError::Storage(e.to_string()))?;
        let scratch: Scratch = serde_json::from_str(&content).map_err(|e| CoreError::Storage(e.to_string()))?;
        Ok(scratch)
    }

    fn list(&self) -> Result<Vec<Scratch>> {
        let mut scratches = Vec::new();
        for entry in fs::read_dir(self.workspace.scratches_dir()).map_err(|e| CoreError::Storage(e.to_string()))? {
            let entry = entry.map_err(|e| CoreError::Storage(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path).map_err(|e| CoreError::Storage(e.to_string()))?;
                let scratch: Scratch = serde_json::from_str(&content).map_err(|e| CoreError::Storage(e.to_string()))?;
                scratches.push(scratch);
            }
        }
        Ok(scratches)
    }

    fn delete(&self, id: &str) -> Result<()> {
        let path = self.scratch_path(id);
        if path.exists() {
            fs::remove_file(path).map_err(|e| CoreError::Storage(e.to_string()))?;
        }
        Ok(())
    }
}

pub struct FilesystemProjectRepository {
    workspace: Arc<Workspace>,
}

impl FilesystemProjectRepository {
    pub fn new(workspace: Arc<Workspace>) -> Self {
        Self { workspace }
    }

    fn project_dir(&self, id: &str) -> PathBuf {
        self.workspace.projects_dir().join(id)
    }

    fn project_path(&self, id: &str) -> PathBuf {
        self.project_dir(id).join("project.json")
    }
}

impl tarsius_core::ProjectRepository for FilesystemProjectRepository {
    fn save(&self, project: &Project) -> Result<()> {
        let dir = self.project_dir(&project.id);
        fs::create_dir_all(&dir).map_err(|e| CoreError::Storage(e.to_string()))?;
        let path = self.project_path(&project.id);
        let json = serde_json::to_string_pretty(project).map_err(|e| CoreError::Storage(e.to_string()))?;
        write_atomic(&path, json)?;
        Ok(())
    }

    fn load(&self, id: &str) -> Result<Project> {
        let path = self.project_path(id);
        if !path.exists() {
            return Err(CoreError::NotFound(format!("Project {}", id)));
        }
        let content = fs::read_to_string(path).map_err(|e| CoreError::Storage(e.to_string()))?;
        let project: Project = serde_json::from_str(&content).map_err(|e| CoreError::Storage(e.to_string()))?;
        Ok(project)
    }

    fn list(&self) -> Result<Vec<Project>> {
        let mut projects = Vec::new();
        for entry in fs::read_dir(self.workspace.projects_dir()).map_err(|e| CoreError::Storage(e.to_string()))? {
            let entry = entry.map_err(|e| CoreError::Storage(e.to_string()))?;
            if entry.file_type().map_err(|e| CoreError::Storage(e.to_string()))?.is_dir() {
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
            fs::remove_dir_all(dir).map_err(|e| CoreError::Storage(e.to_string()))?;
        }
        Ok(())
    }
}

pub struct FilesystemTemplateRepository {
    workspace: Arc<Workspace>,
}

impl FilesystemTemplateRepository {
    pub fn new(workspace: Arc<Workspace>) -> Self {
        Self { workspace }
    }

    fn template_path(&self, id: &str) -> PathBuf {
        self.workspace.templates_dir().join(format!("{}.json", id))
    }
}

impl tarsius_core::TemplateRepository for FilesystemTemplateRepository {
    fn save(&self, template: &Template) -> Result<()> {
        let path = self.template_path(&template.id);
        let json = serde_json::to_string_pretty(template).map_err(|e| CoreError::Storage(e.to_string()))?;
        write_atomic(&path, json)?;
        Ok(())
    }

    fn load(&self, id: &str) -> Result<Template> {
        let path = self.template_path(id);
        if !path.exists() {
            return Err(CoreError::NotFound(format!("Template {}", id)));
        }
        let content = fs::read_to_string(path).map_err(|e| CoreError::Storage(e.to_string()))?;
        let template: Template = serde_json::from_str(&content).map_err(|e| CoreError::Storage(e.to_string()))?;
        Ok(template)
    }

    fn list(&self) -> Result<Vec<Template>> {
        let mut templates = Vec::new();
        for entry in fs::read_dir(self.workspace.templates_dir()).map_err(|e| CoreError::Storage(e.to_string()))? {
            let entry = entry.map_err(|e| CoreError::Storage(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path).map_err(|e| CoreError::Storage(e.to_string()))?;
                let template: Template = serde_json::from_str(&content).map_err(|e| CoreError::Storage(e.to_string()))?;
                templates.push(template);
            }
        }
        Ok(templates)
    }

    fn delete(&self, id: &str) -> Result<()> {
        let path = self.template_path(id);
        if path.exists() {
            fs::remove_file(path).map_err(|e| CoreError::Storage(e.to_string()))?;
        }
        Ok(())
    }
}

fn write_atomic<P: AsRef<Path>>(path: P, content: String) -> Result<()> {
    let path = path.as_ref();
    let temp_path = path.with_extension("tmp");
    fs::write(&temp_path, content).map_err(|e| CoreError::Storage(e.to_string()))?;
    fs::rename(temp_path, path).map_err(|e| CoreError::Storage(e.to_string()))?;
    Ok(())
}