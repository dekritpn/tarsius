use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::result;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scratch {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub outline: OutlineNode,
    pub settings: ProjectSettings,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineNode {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub children: Vec<OutlineNode>,
    pub scratches: Vec<ScratchLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchLink {
    pub scratch_id: String,
    pub mode: IntegrationMode,
    pub insertion: InsertionFlags,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationMode {
    Include,
    Link,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertionFlags {
    pub body: bool,
    pub footnote: bool,
    pub reference: bool,
    pub appendix: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub template_id: String,
    pub output_dir: String,
}

#[derive(Debug)]
pub enum CoreError {
    Storage(String),
    NotFound(String),
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CoreError::Storage(s) => write!(f, "Storage error: {}", s),
            CoreError::NotFound(s) => write!(f, "Not found: {}", s),
        }
    }
}

impl std::error::Error for CoreError {}

pub type Result<T> = result::Result<T, CoreError>;

pub trait ScratchRepository: Send + Sync {
    fn save(&self, scratch: &Scratch) -> Result<()>;
    fn load(&self, id: &str) -> Result<Scratch>;
    fn list(&self) -> Result<Vec<Scratch>>;
    fn delete(&self, id: &str) -> Result<()>;
}

pub trait ProjectRepository: Send + Sync {
    fn save(&self, project: &Project) -> Result<()>;
    fn load(&self, id: &str) -> Result<Project>;
    fn list(&self) -> Result<Vec<Project>>;
    fn delete(&self, id: &str) -> Result<()>;
}

pub trait TemplateRepository: Send + Sync {
    fn save(&self, template: &Template) -> Result<()>;
    fn load(&self, id: &str) -> Result<Template>;
    fn list(&self) -> Result<Vec<Template>>;
    fn delete(&self, id: &str) -> Result<()>;
}

// Managers
pub struct ScratchManager {
    repo: Box<dyn ScratchRepository>,
}

impl ScratchManager {
    pub fn new(repo: Box<dyn ScratchRepository>) -> Self {
        Self { repo }
    }

    pub fn create(&self, title: String, content: String, tags: Vec<String>, source: Option<String>) -> Result<Scratch> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let scratch = Scratch {
            id: id.clone(),
            title,
            content,
            created_at: now,
            modified_at: now,
            tags,
            source,
        };
        self.repo.save(&scratch)?;
        Ok(scratch)
    }

    pub fn update(&self, id: String, title: Option<String>, content: Option<String>, tags: Option<Vec<String>>, source: Option<Option<String>>) -> Result<Scratch> {
        let mut scratch = self.repo.load(&id)?;
        if let Some(t) = title {
            scratch.title = t;
        }
        if let Some(c) = content {
            scratch.content = c;
        }
        if let Some(ts) = tags {
            scratch.tags = ts;
        }
        if let Some(s) = source {
            scratch.source = s;
        }
        scratch.modified_at = Utc::now();
        self.repo.save(&scratch)?;
        Ok(scratch)
    }

    pub fn load(&self, id: &str) -> Result<Scratch> {
        self.repo.load(id)
    }

    pub fn list(&self) -> Result<Vec<Scratch>> {
        self.repo.list()
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.repo.delete(id)
    }
}

pub struct ProjectManager {
    repo: Box<dyn ProjectRepository>,
}

impl ProjectManager {
    pub fn new(repo: Box<dyn ProjectRepository>) -> Self {
        Self { repo }
    }

    pub fn create(&self, title: String, template_id: String, output_dir: String) -> Result<Project> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let outline = OutlineNode {
            id: Uuid::new_v4().to_string(),
            title: "Root".to_string(),
            content: None,
            children: vec![],
            scratches: vec![],
        };
        let settings = ProjectSettings {
            template_id,
            output_dir,
        };
        let project = Project {
            id: id.clone(),
            title,
            outline,
            settings,
            created_at: now,
            modified_at: now,
        };
        self.repo.save(&project)?;
        Ok(project)
    }

    pub fn load(&self, id: &str) -> Result<Project> {
        self.repo.load(id)
    }

    pub fn save(&self, project: &Project) -> Result<()> {
        self.repo.save(project)
    }

    pub fn list(&self) -> Result<Vec<Project>> {
        self.repo.list()
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.repo.delete(id)
    }
}

// DTOs for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchDto {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: String, // ISO string
    pub modified_at: String,
    pub tags: Vec<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDto {
    pub id: String,
    pub title: String,
    pub outline: OutlineNodeDto,
    pub settings: ProjectSettingsDto,
    pub created_at: String,
    pub modified_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineNodeDto {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub children: Vec<OutlineNodeDto>,
    pub scratches: Vec<ScratchLinkDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchLinkDto {
    pub scratch_id: String,
    pub mode: String, // "Include" or "Link"
    pub insertion: InsertionFlagsDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertionFlagsDto {
    pub body: bool,
    pub footnote: bool,
    pub reference: bool,
    pub appendix: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateDto {
    pub id: String,
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettingsDto {
    pub template_id: String,
    pub output_dir: String,
}

impl From<Scratch> for ScratchDto {
    fn from(s: Scratch) -> Self {
        Self {
            id: s.id,
            title: s.title,
            content: s.content,
            created_at: s.created_at.to_rfc3339(),
            modified_at: s.modified_at.to_rfc3339(),
            tags: s.tags,
            source: s.source,
        }
    }
}

impl From<Project> for ProjectDto {
    fn from(p: Project) -> Self {
        Self {
            id: p.id,
            title: p.title,
            outline: p.outline.into(),
            settings: p.settings.into(),
            created_at: p.created_at.to_rfc3339(),
            modified_at: p.modified_at.to_rfc3339(),
        }
    }
}

impl From<OutlineNode> for OutlineNodeDto {
    fn from(o: OutlineNode) -> Self {
        Self {
            id: o.id,
            title: o.title,
            content: o.content,
            children: o.children.into_iter().map(Into::into).collect(),
            scratches: o.scratches.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ScratchLink> for ScratchLinkDto {
    fn from(s: ScratchLink) -> Self {
        Self {
            scratch_id: s.scratch_id,
            mode: match s.mode {
                IntegrationMode::Include => "Include".to_string(),
                IntegrationMode::Link => "Link".to_string(),
            },
            insertion: s.insertion.into(),
        }
    }
}

impl From<InsertionFlags> for InsertionFlagsDto {
    fn from(i: InsertionFlags) -> Self {
        Self {
            body: i.body,
            footnote: i.footnote,
            reference: i.reference,
            appendix: i.appendix,
        }
    }
}

impl From<ProjectSettings> for ProjectSettingsDto {
    fn from(p: ProjectSettings) -> Self {
        Self {
            template_id: p.template_id,
            output_dir: p.output_dir,
        }
    }
}

impl From<ScratchLinkDto> for ScratchLink {
    fn from(dto: ScratchLinkDto) -> Self {
        Self {
            scratch_id: dto.scratch_id,
            mode: match dto.mode.as_str() {
                "Include" => IntegrationMode::Include,
                "Link" => IntegrationMode::Link,
                _ => IntegrationMode::Include,
            },
            insertion: dto.insertion.into(),
        }
    }
}

impl From<InsertionFlagsDto> for InsertionFlags {
    fn from(dto: InsertionFlagsDto) -> Self {
        Self {
            body: dto.body,
            footnote: dto.footnote,
            reference: dto.reference,
            appendix: dto.appendix,
        }
    }
}

impl From<ProjectSettingsDto> for ProjectSettings {
    fn from(dto: ProjectSettingsDto) -> Self {
        Self {
            template_id: dto.template_id,
            output_dir: dto.output_dir,
        }
    }
}

impl TryFrom<OutlineNodeDto> for OutlineNode {
    type Error = String;
    fn try_from(dto: OutlineNodeDto) -> std::result::Result<Self, Self::Error> {
        let children = dto.children.into_iter().map(TryInto::try_into).collect::<std::result::Result<Vec<_>, _>>()?;
        let scratches = dto.scratches.into_iter().map(Into::into).collect();
        Ok(OutlineNode {
            id: dto.id,
            title: dto.title,
            content: dto.content,
            children,
            scratches,
        })
    }
}

impl TryFrom<ProjectDto> for Project {
    type Error = String;
    fn try_from(dto: ProjectDto) -> std::result::Result<Self, Self::Error> {
        let created_at = chrono::DateTime::parse_from_rfc3339(&dto.created_at).map_err(|e| e.to_string())?.with_timezone(&Utc);
        let modified_at = chrono::DateTime::parse_from_rfc3339(&dto.modified_at).map_err(|e| e.to_string())?.with_timezone(&Utc);
        let outline = dto.outline.try_into()?;
        let settings = dto.settings.into();
        Ok(Project {
            id: dto.id,
            title: dto.title,
            outline,
            settings,
            created_at,
            modified_at,
        })
    }
}