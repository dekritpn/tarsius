use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use tarsius_core::*;

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
        let json =
            serde_json::to_string_pretty(scratch).map_err(|e| CoreError::Storage(e.to_string()))?;
        write_atomic(&path, json)?;
        Ok(())
    }

    fn load(&self, id: &str) -> Result<Scratch> {
        let path = self.scratch_path(id);
        if !path.exists() {
            return Err(CoreError::NotFound(format!("Scratch {}", id)));
        }
        let content = fs::read_to_string(path).map_err(|e| CoreError::Storage(e.to_string()))?;
        let scratch: Scratch =
            serde_json::from_str(&content).map_err(|e| CoreError::Storage(e.to_string()))?;
        Ok(scratch)
    }

    fn list(&self) -> Result<Vec<Scratch>> {
        let mut scratches = Vec::new();
        for entry in fs::read_dir(self.workspace.scratches_dir())
            .map_err(|e| CoreError::Storage(e.to_string()))?
        {
            let entry = entry.map_err(|e| CoreError::Storage(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content =
                    fs::read_to_string(&path).map_err(|e| CoreError::Storage(e.to_string()))?;
                let scratch: Scratch = serde_json::from_str(&content)
                    .map_err(|e| CoreError::Storage(e.to_string()))?;
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
        let json =
            serde_json::to_string_pretty(project).map_err(|e| CoreError::Storage(e.to_string()))?;
        write_atomic(&path, json)?;
        Ok(())
    }

    fn load(&self, id: &str) -> Result<Project> {
        let path = self.project_path(id);
        if !path.exists() {
            return Err(CoreError::NotFound(format!("Project {}", id)));
        }
        let content = fs::read_to_string(path).map_err(|e| CoreError::Storage(e.to_string()))?;
        let project: Project =
            serde_json::from_str(&content).map_err(|e| CoreError::Storage(e.to_string()))?;
        Ok(project)
    }

    fn list(&self) -> Result<Vec<Project>> {
        let mut projects = Vec::new();
        for entry in fs::read_dir(self.workspace.projects_dir())
            .map_err(|e| CoreError::Storage(e.to_string()))?
        {
            let entry = entry.map_err(|e| CoreError::Storage(e.to_string()))?;
            if entry
                .file_type()
                .map_err(|e| CoreError::Storage(e.to_string()))?
                .is_dir()
            {
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
        let json = serde_json::to_string_pretty(template)
            .map_err(|e| CoreError::Storage(e.to_string()))?;
        write_atomic(&path, json)?;
        Ok(())
    }

    fn load(&self, id: &str) -> Result<Template> {
        let path = self.template_path(id);
        if !path.exists() {
            return Err(CoreError::NotFound(format!("Template {}", id)));
        }
        let content = fs::read_to_string(path).map_err(|e| CoreError::Storage(e.to_string()))?;
        let template: Template =
            serde_json::from_str(&content).map_err(|e| CoreError::Storage(e.to_string()))?;
        Ok(template)
    }

    fn list(&self) -> Result<Vec<Template>> {
        let mut templates = Vec::new();
        for entry in fs::read_dir(self.workspace.templates_dir())
            .map_err(|e| CoreError::Storage(e.to_string()))?
        {
            let entry = entry.map_err(|e| CoreError::Storage(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content =
                    fs::read_to_string(&path).map_err(|e| CoreError::Storage(e.to_string()))?;
                let template: Template = serde_json::from_str(&content)
                    .map_err(|e| CoreError::Storage(e.to_string()))?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tempfile::TempDir;
    use tarsius_core::{ScratchRepository, ProjectRepository, Scratch, Project, OutlineNode, ProjectSettings};

    #[test]
    fn test_filesystem_scratch_repository() {
        let temp_dir = TempDir::new().unwrap();
        let workspace = Workspace::new(temp_dir.path());
        workspace.ensure_dirs().unwrap();

        let repo = FilesystemScratchRepository::new(Arc::new(workspace));

        // Create a test scratch
        let scratch = Scratch {
            id: "test-scratch".to_string(),
            title: "Test Scratch".to_string(),
            content: "Test content".to_string(),
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
            tags: vec!["tag1".to_string()],
            source: Some("source".to_string()),
        };

        // Save
        repo.save(&scratch).unwrap();

        // Load
        let loaded = repo.load("test-scratch").unwrap();
        assert_eq!(loaded.id, scratch.id);
        assert_eq!(loaded.title, scratch.title);
        assert_eq!(loaded.content, scratch.content);

        // List
        let scratches = repo.list().unwrap();
        assert_eq!(scratches.len(), 1);
        assert_eq!(scratches[0].id, scratch.id);

        // Delete
        repo.delete("test-scratch").unwrap();
        let scratches = repo.list().unwrap();
        assert_eq!(scratches.len(), 0);

        // Try to load deleted
        assert!(repo.load("test-scratch").is_err());
    }

    #[test]
    fn test_filesystem_project_repository() {
        let temp_dir = TempDir::new().unwrap();
        let workspace = Workspace::new(temp_dir.path());
        workspace.ensure_dirs().unwrap();

        let repo = FilesystemProjectRepository::new(Arc::new(workspace));

        // Create a test project
        let project = Project {
            id: "test-project".to_string(),
            title: "Test Project".to_string(),
            outline: OutlineNode {
                id: "root".to_string(),
                title: "Root".to_string(),
                content: None,
                children: vec![],
                scratches: vec![],
            },
            settings: ProjectSettings {
                template_id: "template1".to_string(),
                output_dir: "/tmp".to_string(),
            },
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
        };

        // Save
        repo.save(&project).unwrap();

        // Load
        let loaded = repo.load("test-project").unwrap();
        assert_eq!(loaded.id, project.id);
        assert_eq!(loaded.title, project.title);

        // List
        let projects = repo.list().unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].id, project.id);

        // Delete
        repo.delete("test-project").unwrap();
        let projects = repo.list().unwrap();
        assert_eq!(projects.len(), 0);

        // Try to load deleted
        assert!(repo.load("test-project").is_err());
    }

    #[test]
    fn test_workspace_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let workspace = Workspace::new(temp_dir.path());

        // Initially dirs don't exist
        assert!(!workspace.scratches_dir().exists());
        assert!(!workspace.projects_dir().exists());
        assert!(!workspace.templates_dir().exists());

        // Ensure dirs
        workspace.ensure_dirs().unwrap();

        // Now they exist
        assert!(workspace.scratches_dir().exists());
        assert!(workspace.projects_dir().exists());
        assert!(workspace.templates_dir().exists());
    }
}
