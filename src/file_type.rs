use std::time::SystemTime;

pub struct FileType {
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub modified: SystemTime,
    pub changed: SystemTime,
    pub accessed: SystemTime,
    pub created: SystemTime,
    pub file_type: String,
}