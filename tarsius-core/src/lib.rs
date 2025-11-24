use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

// Managers
pub struct ScratchManager;

impl ScratchManager {
    pub fn new() -> Self {
        Self
    }
}

pub struct ProjectManager;

impl ProjectManager {
    pub fn new() -> Self {
        Self
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