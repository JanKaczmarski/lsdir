use std::{fmt::Display, fs::DirEntry};
use std::io::Result;
use chrono::{DateTime, Local};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a file with its metadata.
///
/// # Fields
/// - `name`: The name of the file (excluding the path).
/// - `extension`: The file's extension (e.g., "txt", "rs").
/// - `size`: The size of the file in bytes.
/// - `modified`: The last modification time of the file.
/// - `accessed`: The last access time of the file.
/// - `created`: The creation time of the file.
/// - `file_type`: The type of the file (e.g., "file", "directory", "symlink").
pub struct File {
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub modified: DateTime<Local>,
    pub accessed: DateTime<Local>,
    pub created: DateTime<Local>,
    pub file_type: String,
}

/// Creates a `File` instance from a given directory entry (`DirEntry`).
///
/// This method extracts metadata from the provided `DirEntry`, including the file name,
/// extension, size, modification time, access time, creation time, and determines whether
/// the entry is a directory or a file. Returns a `Result` containing the constructed `File`
/// on success, or an error if any metadata extraction fails.
///
/// # Arguments
///
/// * `entry` - A reference to a `DirEntry` from which to construct the `File`.
///
/// # Errors
///
/// Returns an error if retrieving metadata or any of the time fields fails.
impl File {
    pub fn from_dir_entry(entry: &DirEntry) -> Result<Self> {
        let metadata = entry.metadata()?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let extension = entry
            .path()
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string();

        Ok(Self {
            name,
            extension,
            size: metadata.len(),
            modified: DateTime::<Local>::from(metadata.modified()?),
            accessed: DateTime::<Local>::from(metadata.accessed()?),
            created: DateTime::<Local>::from(metadata.created()?),
            file_type: if metadata.is_dir() {
                "Directory".to_string()
            } else {
                "File".to_string()
            },
        })
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<19} | {:<19} | {:<19} | {:<10} | {:>10} | {:<30}",
            self.modified.format("%Y-%m-%d %H:%M:%S"),
            self.accessed.format("%Y-%m-%d %H:%M:%S"),
            self.created.format("%Y-%m-%d %H:%M:%S"),
            self.file_type,
            self.size,
            self.name,
        )
    }
}