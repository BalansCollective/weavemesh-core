//! Core Editor Integration for WeaveMesh Core
//!
//! This module provides foundational collaborative editing capabilities
//! that can be extended by context-specific plugins.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::attribution::Attribution;
use crate::group_communication::{GroupCommunication, GroupId, Message, MessageId};
use crate::sacred_alliance::{SacredAllianceProvider, ChannelConfig};

/// Core programming language types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CoreLanguage {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Markdown,
    Json,
    Yaml,
    Toml,
    Unknown,
}

/// Core document representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDocument {
    /// Document path
    pub path: String,
    /// Document content
    pub content: String,
    /// Content lines for efficient editing
    pub lines: Vec<String>,
    /// Attribution for each line
    pub line_attributions: Vec<Attribution>,
    /// Programming language
    pub language: CoreLanguage,
    /// Last modified timestamp
    pub last_modified: DateTime<Utc>,
    /// Document metadata
    pub metadata: CoreDocumentMetadata,
}

/// Core document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDocumentMetadata {
    /// Project identifier
    pub project_id: Option<Uuid>,
    /// Collaborative individuation patterns detected
    pub collaboration_patterns: Vec<CoreCollaborationPattern>,
    /// Innovation markers
    pub innovation_markers: Vec<String>,
    /// Collaboration quality score (0.0 to 1.0)
    pub collaboration_quality: f64,
    /// Sacred Alliance integration level (0.0 to 1.0)
    pub sacred_alliance_integration: f64,
}

impl Default for CoreDocumentMetadata {
    fn default() -> Self {
        Self {
            project_id: None,
            collaboration_patterns: Vec::new(),
            innovation_markers: Vec::new(),
            collaboration_quality: 0.5,
            sacred_alliance_integration: 0.5,
        }
    }
}

/// Core collaboration patterns in code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCollaborationPattern {
    /// Pattern type
    pub pattern_type: CorePatternType,
    /// Line range where pattern appears
    pub line_range: (usize, usize),
    /// Confidence in pattern recognition (0.0 to 1.0)
    pub confidence: f64,
    /// Description of the pattern
    pub description: String,
    /// Collaborative individuation score (0.0 to 1.0)
    pub individuation_score: f64,
}

/// Core pattern types for collaborative individuation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CorePatternType {
    /// Individual contribution pattern
    IndividualContribution,
    /// Collective synergy pattern
    CollectiveSynergy,
    /// Innovation emergence pattern
    InnovationEmergence,
    /// Sacred Alliance pattern
    SacredAlliance,
    /// Knowledge sharing pattern
    KnowledgeSharing,
    /// Conflict resolution pattern
    ConflictResolution,
}

/// Core cursor position for collaborative editing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCursor {
    /// User identifier
    pub user_id: String,
    /// Current line (0-indexed)
    pub line: usize,
    /// Current column (0-indexed)
    pub column: usize,
    /// Selection range if any
    pub selection: Option<CoreSelection>,
    /// Cursor color
    pub color: CoreCursorColor,
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
}

/// Core selection range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSelection {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

/// Core cursor colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreCursorColor {
    /// Human user cursor
    Human { r: u8, g: u8, b: u8 },
    /// AI assistant cursor
    AI { r: u8, g: u8, b: u8 },
    /// Sacred Alliance cursor
    SacredAlliance { r: u8, g: u8, b: u8 },
}

/// Core content change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreContentChange {
    /// Document path
    pub document_path: String,
    /// Change type
    pub change_type: CoreChangeType,
    /// Position of change
    pub position: CorePosition,
    /// Content being changed
    pub content: String,
    /// Attribution for this change
    pub attribution: Attribution,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Change identifier
    pub change_id: Uuid,
}

/// Core change types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreChangeType {
    /// Insert text
    Insert,
    /// Delete text
    Delete,
    /// Replace text
    Replace,
    /// Insert line
    InsertLine,
    /// Delete line
    DeleteLine,
}

/// Core position in document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePosition {
    pub line: usize,
    pub column: usize,
}

/// Core editor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEditorConfig {
    /// Enable collaborative individuation pattern detection
    pub pattern_detection_enabled: bool,
    /// Show collaborative cursors
    pub show_collaborative_cursors: bool,
    /// Enable real-time attribution tracking
    pub real_time_attribution: bool,
    /// Auto-save interval in seconds
    pub auto_save_interval: u32,
    /// Tab size
    pub tab_size: usize,
    /// Use spaces instead of tabs
    pub use_spaces: bool,
    /// Show line numbers
    pub show_line_numbers: bool,
    /// Enable Sacred Alliance integration
    pub sacred_alliance_enabled: bool,
}

impl Default for CoreEditorConfig {
    fn default() -> Self {
        Self {
            pattern_detection_enabled: true,
            show_collaborative_cursors: true,
            real_time_attribution: true,
            auto_save_interval: 30,
            tab_size: 4,
            use_spaces: true,
            show_line_numbers: true,
            sacred_alliance_enabled: true,
        }
    }
}

/// Core collaborative editing state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCollaborativeEditingState {
    /// Active collaboration sessions
    pub active_sessions: Vec<Uuid>,
    /// Pending changes to synchronize
    pub pending_changes: Vec<CoreContentChange>,
    /// Last synchronization timestamp
    pub last_sync: DateTime<Utc>,
    /// Conflict state
    pub conflict_state: CoreConflictState,
}

/// Core conflict state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreConflictState {
    /// No conflicts
    None,
    /// Conflicts detected
    Detected { conflicts: Vec<CoreConflict> },
    /// Conflicts being resolved
    Resolving { conflicts: Vec<CoreConflict> },
}

/// Core conflict representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConflict {
    /// Conflict identifier
    pub id: Uuid,
    /// Conflicting changes
    pub changes: Vec<CoreContentChange>,
    /// Suggested resolution
    pub suggested_resolution: Option<CoreConflictResolution>,
    /// Human perspective
    pub human_perspective: Option<String>,
    /// AI perspective
    pub ai_perspective: Option<String>,
}

/// Core conflict resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConflictResolution {
    /// Resolution strategy
    pub strategy: CoreResolutionStrategy,
    /// Resolved content
    pub resolved_content: String,
    /// Confidence in resolution (0.0 to 1.0)
    pub confidence: f64,
    /// Explanation
    pub explanation: String,
}

/// Core resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreResolutionStrategy {
    /// Accept human changes
    AcceptHuman,
    /// Accept AI changes
    AcceptAI,
    /// Merge both changes
    Merge,
    /// Create new collaborative solution
    CollaborativeSynthesis,
    /// Escalate to manual resolution
    ManualResolution,
}

/// Core editor engine
pub struct CoreEditorEngine {
    /// Open documents
    pub open_documents: HashMap<String, CoreDocument>,
    /// Active cursors
    pub active_cursors: HashMap<String, CoreCursor>,
    /// Editor configuration
    pub config: CoreEditorConfig,
    /// Collaborative editing state
    pub collaboration_state: CoreCollaborativeEditingState,
    /// Group communication interface
    pub group_communication: Option<Box<dyn GroupCommunication + Send + Sync>>,
    /// Sacred Alliance provider
    pub sacred_alliance: Option<Box<dyn SacredAllianceProvider + Send + Sync>>,
}

impl CoreEditorEngine {
    /// Create a new core editor engine
    pub fn new() -> Self {
        Self {
            open_documents: HashMap::new(),
            active_cursors: HashMap::new(),
            config: CoreEditorConfig::default(),
            collaboration_state: CoreCollaborativeEditingState {
                active_sessions: Vec::new(),
                pending_changes: Vec::new(),
                last_sync: Utc::now(),
                conflict_state: CoreConflictState::None,
            },
            group_communication: None,
            sacred_alliance: None,
        }
    }
    
    /// Set group communication provider
    pub fn set_group_communication(&mut self, provider: Box<dyn GroupCommunication + Send + Sync>) {
        self.group_communication = Some(provider);
    }
    
    /// Set Sacred Alliance provider
    pub fn set_sacred_alliance(&mut self, provider: Box<dyn SacredAllianceProvider + Send + Sync>) {
        self.sacred_alliance = Some(provider);
    }
    
    /// Open a document for editing
    pub async fn open_document(&mut self, path: &str) -> Result<&CoreDocument> {
        // Read file content
        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))?;
        
        // Detect language
        let language = Self::detect_language(path);
        
        // Split into lines
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        // Initialize line attributions
        let line_attributions: Vec<Attribution> = lines.iter().map(|_| {
            Attribution::new(
                Some("human".to_string()),
                None,
                crate::attribution::CollaborationType::HumanLed,
                1.0,
            )
        }).collect();
        
        let document = CoreDocument {
            path: path.to_string(),
            content,
            lines,
            line_attributions,
            language,
            last_modified: Utc::now(),
            metadata: CoreDocumentMetadata::default(),
        };
        
        // Create Sacred Alliance channel for document if enabled
        if self.config.sacred_alliance_enabled {
            if let Some(sacred_alliance) = &self.sacred_alliance {
                let channel_id = format!("document-{}", path.replace('/', "-"));
                let config = ChannelConfig::default();
                let _ = sacred_alliance.create_channel(channel_id, config);
            }
        }
        
        self.open_documents.insert(path.to_string(), document);
        Ok(self.open_documents.get(path).unwrap())
    }
    
    /// Apply a content change
    pub async fn apply_change(&mut self, change: CoreContentChange) -> Result<()> {
        // Check if document exists
        if !self.open_documents.contains_key(&change.document_path) {
            return Err(anyhow::anyhow!("Document not found: {}", change.document_path));
        }
        
        // Apply the change
        match change.change_type {
            CoreChangeType::Insert => self.apply_insert_change(&change).await?,
            CoreChangeType::Delete => self.apply_delete_change(&change).await?,
            CoreChangeType::Replace => self.apply_replace_change(&change).await?,
            CoreChangeType::InsertLine => self.apply_insert_line_change(&change).await?,
            CoreChangeType::DeleteLine => self.apply_delete_line_change(&change).await?,
        }
        
        // Update document metadata
        if let Some(document) = self.open_documents.get_mut(&change.document_path) {
            document.last_modified = Utc::now();
            document.content = document.lines.join("\n");
        }
        
        // Analyze collaboration patterns
        self.analyze_collaboration_patterns(&change.document_path).await?;
        
        // Add to pending changes for synchronization
        self.collaboration_state.pending_changes.push(change);
        
        Ok(())
    }
    
    /// Update cursor position
    pub async fn update_cursor(&mut self, user_id: &str, cursor: CoreCursor) -> Result<()> {
        self.active_cursors.insert(user_id.to_string(), cursor);
        Ok(())
    }
    
    /// Get document by path
    pub fn get_document(&self, path: &str) -> Option<&CoreDocument> {
        self.open_documents.get(path)
    }
    
    /// List open documents
    pub fn list_open_documents(&self) -> Vec<&CoreDocument> {
        self.open_documents.values().collect()
    }
    
    /// Get active cursors
    pub fn get_active_cursors(&self) -> &HashMap<String, CoreCursor> {
        &self.active_cursors
    }
    
    /// Detect conflict between changes
    pub async fn detect_conflict(
        &mut self,
        change1: &CoreContentChange,
        change2: &CoreContentChange,
    ) -> Result<Option<CoreConflict>> {
        // Simple conflict detection - same line, overlapping positions
        if change1.document_path == change2.document_path 
            && change1.position.line == change2.position.line {
            
            let conflict = CoreConflict {
                id: Uuid::new_v4(),
                changes: vec![change1.clone(), change2.clone()],
                suggested_resolution: Some(CoreConflictResolution {
                    strategy: CoreResolutionStrategy::CollaborativeSynthesis,
                    resolved_content: format!("{} {}", change1.content, change2.content),
                    confidence: 0.7,
                    explanation: "Merged both changes collaboratively".to_string(),
                }),
                human_perspective: None,
                ai_perspective: None,
            };
            
            Ok(Some(conflict))
        } else {
            Ok(None)
        }
    }
    
    /// Resolve a conflict
    pub async fn resolve_conflict(
        &mut self,
        conflict_id: Uuid,
        resolution: CoreConflictResolution,
    ) -> Result<()> {
        // Update conflict state
        if let CoreConflictState::Detected { conflicts } = &mut self.collaboration_state.conflict_state {
            conflicts.retain(|c| c.id != conflict_id);
            if conflicts.is_empty() {
                self.collaboration_state.conflict_state = CoreConflictState::None;
            }
        }
        
        // Apply resolution would go here
        let _ = resolution; // Use the resolution to avoid unused variable warning
        
        Ok(())
    }
    
    /// Synchronize pending changes
    pub async fn synchronize_changes(&mut self) -> Result<()> {
        if let Some(group_comm) = &self.group_communication {
            for change in &self.collaboration_state.pending_changes {
                let message = Message {
                    id: MessageId::new(),
                    content: serde_json::to_string(change)?,
                    sender: "editor".to_string(),
                    timestamp: Utc::now(),
                    metadata: HashMap::new(),
                    priority: crate::group_communication::MessagePriority::Normal,
                    requires_ack: false,
                };
                
                let group_id = GroupId::new(&format!("editor/{}", change.document_path));
                let _ = group_comm.talk(group_id, message).await;
            }
        }
        
        self.collaboration_state.pending_changes.clear();
        self.collaboration_state.last_sync = Utc::now();
        
        Ok(())
    }
    
    /// Detect language from file path
    fn detect_language(path: &str) -> CoreLanguage {
        match path.split('.').last() {
            Some("rs") => CoreLanguage::Rust,
            Some("py") => CoreLanguage::Python,
            Some("js") => CoreLanguage::JavaScript,
            Some("ts") => CoreLanguage::TypeScript,
            Some("md") => CoreLanguage::Markdown,
            Some("json") => CoreLanguage::Json,
            Some("yaml") | Some("yml") => CoreLanguage::Yaml,
            Some("toml") => CoreLanguage::Toml,
            _ => CoreLanguage::Unknown,
        }
    }
    
    /// Apply insert change
    async fn apply_insert_change(&mut self, change: &CoreContentChange) -> Result<()> {
        if let Some(document) = self.open_documents.get_mut(&change.document_path) {
            let line_idx = change.position.line;
            let col_idx = change.position.column;
            
            if line_idx < document.lines.len() {
                let line = &mut document.lines[line_idx];
                if col_idx <= line.len() {
                    line.insert_str(col_idx, &change.content);
                    document.line_attributions[line_idx] = change.attribution.clone();
                }
            }
        }
        Ok(())
    }
    
    /// Apply delete change
    async fn apply_delete_change(&mut self, change: &CoreContentChange) -> Result<()> {
        if let Some(document) = self.open_documents.get_mut(&change.document_path) {
            let line_idx = change.position.line;
            let col_idx = change.position.column;
            
            if line_idx < document.lines.len() {
                let line = &mut document.lines[line_idx];
                let delete_len = change.content.len();
                
                if col_idx < line.len() {
                    let end_idx = (col_idx + delete_len).min(line.len());
                    line.drain(col_idx..end_idx);
                    document.line_attributions[line_idx] = change.attribution.clone();
                }
            }
        }
        Ok(())
    }
    
    /// Apply replace change
    async fn apply_replace_change(&mut self, change: &CoreContentChange) -> Result<()> {
        self.apply_delete_change(change).await?;
        self.apply_insert_change(change).await?;
        Ok(())
    }
    
    /// Apply insert line change
    async fn apply_insert_line_change(&mut self, change: &CoreContentChange) -> Result<()> {
        if let Some(document) = self.open_documents.get_mut(&change.document_path) {
            let line_idx = change.position.line;
            
            if line_idx <= document.lines.len() {
                document.lines.insert(line_idx, change.content.clone());
                document.line_attributions.insert(line_idx, change.attribution.clone());
            }
        }
        Ok(())
    }
    
    /// Apply delete line change
    async fn apply_delete_line_change(&mut self, change: &CoreContentChange) -> Result<()> {
        if let Some(document) = self.open_documents.get_mut(&change.document_path) {
            let line_idx = change.position.line;
            
            if line_idx < document.lines.len() {
                document.lines.remove(line_idx);
                document.line_attributions.remove(line_idx);
            }
        }
        Ok(())
    }
    
    /// Analyze collaboration patterns in document
    async fn analyze_collaboration_patterns(&mut self, document_path: &str) -> Result<()> {
        let document_lines = if let Some(document) = self.open_documents.get(document_path) {
            document.lines.clone()
        } else {
            return Ok(());
        };
        
        let mut patterns = Vec::new();
        
        // Detect individual contribution patterns
        for (i, line) in document_lines.iter().enumerate() {
            if line.contains("fn ") || line.contains("class ") || line.contains("struct ") {
                patterns.push(CoreCollaborationPattern {
                    pattern_type: CorePatternType::IndividualContribution,
                    line_range: (i, i),
                    confidence: 0.8,
                    description: "Individual creative contribution".to_string(),
                    individuation_score: 0.7,
                });
            }
            
            // Detect knowledge sharing patterns
            if line.trim_start().starts_with("//") || line.trim_start().starts_with("///") {
                patterns.push(CoreCollaborationPattern {
                    pattern_type: CorePatternType::KnowledgeSharing,
                    line_range: (i, i),
                    confidence: 0.7,
                    description: "Knowledge sharing through documentation".to_string(),
                    individuation_score: 0.6,
                });
            }
            
            // Detect collective synergy patterns
            if line.contains("collaborate") || line.contains("together") || line.contains("shared") {
                patterns.push(CoreCollaborationPattern {
                    pattern_type: CorePatternType::CollectiveSynergy,
                    line_range: (i, i),
                    confidence: 0.6,
                    description: "Collective synergy indicator".to_string(),
                    individuation_score: 0.8,
                });
            }
            
            // Detect Sacred Alliance patterns
            if line.contains("sacred") || line.contains("alliance") || line.contains("ceremony") {
                patterns.push(CoreCollaborationPattern {
                    pattern_type: CorePatternType::SacredAlliance,
                    line_range: (i, i),
                    confidence: 0.9,
                    description: "Sacred Alliance integration".to_string(),
                    individuation_score: 0.9,
                });
            }
        }
        
        // Update document with patterns
        if let Some(document) = self.open_documents.get_mut(document_path) {
            document.metadata.collaboration_patterns = patterns;
            
            // Update collaboration quality based on patterns
            let total_score: f64 = document.metadata.collaboration_patterns
                .iter()
                .map(|p| p.individuation_score)
                .sum();
            let pattern_count = document.metadata.collaboration_patterns.len() as f64;
            
            if pattern_count > 0.0 {
                document.metadata.collaboration_quality = (total_score / pattern_count).min(1.0);
            }
        }
        
        Ok(())
    }
}

impl Default for CoreEditorEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[tokio::test]
    async fn test_core_editor_document_opening() {
        let mut editor = CoreEditorEngine::new();
        
        // Create a temporary file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "fn main() {{\n    println!(\"Hello, world!\");\n}}").unwrap();
        let temp_path = temp_file.path().to_str().unwrap();
        
        // Open the document
        let document = editor.open_document(temp_path).await.unwrap();
        
        assert_eq!(document.path, temp_path);
        assert_eq!(document.language, CoreLanguage::Unknown); // No .rs extension
        assert_eq!(document.lines.len(), 3);
        assert_eq!(document.line_attributions.len(), 3);
    }
    
    #[tokio::test]
    async fn test_core_editor_content_changes() {
        let mut editor = CoreEditorEngine::new();
        
        // Create a temporary file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "fn main() {{}}").unwrap();
        let temp_path = temp_file.path().to_str().unwrap();
        
        // Open the document
        editor.open_document(temp_path).await.unwrap();
        
        // Create a content change
        let change = CoreContentChange {
            document_path: temp_path.to_string(),
            change_type: CoreChangeType::Insert,
            position: CorePosition { line: 0, column: 11 },
            content: "\n    println!(\"Hello!\");".to_string(),
            attribution: Attribution::new(
                Some("human".to_string()),
                None,
                crate::attribution::CollaborationType::HumanLed,
                1.0,
            ),
            timestamp: Utc::now(),
            change_id: Uuid::new_v4(),
        };
        
        // Apply the change
        assert!(editor.apply_change(change).await.is_ok());
        
        // Check the document was updated
        let document = editor.get_document(temp_path).unwrap();
        assert!(document.lines[0].contains("println!"));
    }
    
    #[tokio::test]
    async fn test_core_editor_cursor_management() {
        let mut editor = CoreEditorEngine::new();
        
        let cursor = CoreCursor {
            user_id: "user1".to_string(),
            line: 5,
            column: 10,
            selection: None,
            color: CoreCursorColor::Human { r: 255, g: 0, b: 0 },
            last_update: Utc::now(),
        };
        
        assert!(editor.update_cursor("user1", cursor).await.is_ok());
        
        let active_cursors = editor.get_active_cursors();
        assert_eq!(active_cursors.len(), 1);
        assert_eq!(active_cursors.get("user1").unwrap().line, 5);
    }
    
    #[tokio::test]
    async fn test_core_editor_conflict_detection() {
        let mut editor = CoreEditorEngine::new();
        
        let change1 = CoreContentChange {
            document_path: "test.rs".to_string(),
            change_type: CoreChangeType::Insert,
            position: CorePosition { line: 0, column: 0 },
            content: "Hello".to_string(),
            attribution: Attribution::new(
                Some("human".to_string()),
                None,
                crate::attribution::CollaborationType::HumanLed,
                1.0,
            ),
            timestamp: Utc::now(),
            change_id: Uuid::new_v4(),
        };
        
        let change2 = CoreContentChange {
            document_path: "test.rs".to_string(),
            change_type: CoreChangeType::Insert,
            position: CorePosition { line: 0, column: 0 },
            content: "World".to_string(),
            attribution: Attribution::new(
                Some("ai".to_string()),
                None,
                crate::attribution::CollaborationType::AiLed,
                1.0,
            ),
            timestamp: Utc::now(),
            change_id: Uuid::new_v4(),
        };
        
        let conflict = editor.detect_conflict(&change1, &change2).await.unwrap();
        assert!(conflict.is_some());
        
        let conflict = conflict.unwrap();
        assert_eq!(conflict.changes.len(), 2);
        assert!(conflict.suggested_resolution.is_some());
    }
    
    #[test]
    fn test_language_detection() {
        assert_eq!(CoreEditorEngine::detect_language("main.rs"), CoreLanguage::Rust);
        assert_eq!(CoreEditorEngine::detect_language("script.py"), CoreLanguage::Python);
        assert_eq!(CoreEditorEngine::detect_language("app.js"), CoreLanguage::JavaScript);
        assert_eq!(CoreEditorEngine::detect_language("README.md"), CoreLanguage::Markdown);
        assert_eq!(CoreEditorEngine::detect_language("config.json"), CoreLanguage::Json);
        assert_eq!(CoreEditorEngine::detect_language("unknown"), CoreLanguage::Unknown);
    }
}
