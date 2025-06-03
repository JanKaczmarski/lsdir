use std::time::SystemTime;
use std::fs::DirEntry;
use std::io::Result;

#[derive(Debug, Clone)]
pub struct FileType {
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub modified: SystemTime,
    pub accessed: SystemTime,
    pub created: SystemTime,
    pub file_type: String,
}

impl FileType {
    pub fn from_dir_entry(entry: &DirEntry) -> Result<Self> {
        let metadata = entry.metadata()?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let extension = entry.path().extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string();
        
        Ok(Self {
            name,
            extension,
            size: metadata.len(),
            modified: metadata.modified()?,
            accessed: metadata.accessed()?,
            created: metadata.created()?,
            file_type: if metadata.is_dir() { "Directory".to_string() } else { "File".to_string() },
        })
    }
}