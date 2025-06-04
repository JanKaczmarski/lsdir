use std::{collections::HashMap, time::SystemTime};
use chrono::{DateTime, Local, Datelike, Timelike};

use crate::file::File;

#[derive(Debug, Clone)]
pub enum SizeMagnitude {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
    Terabytes,
}

impl SizeMagnitude {
    pub fn convert(&self, size: u64) -> String {
        match self {
            SizeMagnitude::Bytes => format!("{} B", size),
            SizeMagnitude::Kilobytes => format!("{:.2} KB", size / 1024),
            SizeMagnitude::Megabytes => format!("{:.2} MB", size / (1024 * 1024)),
            SizeMagnitude::Gigabytes => format!("{:.2} GB", size / (1024 * 1024 * 1024)),
            SizeMagnitude::Terabytes => format!("{:.2} TB", size / (1024 * 1024 * 1024 * 1024)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimeGrouping {
    pub year: bool,
    pub month: bool,
    pub day: bool,
    pub hour: bool,
    pub minute: bool,
    pub second: bool,
}

impl TimeGrouping {
    pub fn format(&self, time: SystemTime) -> String {
        let datetime: DateTime<Local> = time.into();
        String::from(format!(
            "{}.{}.{} {}:{}:{}",
            if self.day { format!("{:02}", datetime.day()) } else { String::from("*") },
            if self.month { format!("{:02}", datetime.month()) } else { String::from("*") },
            if self.year { format!("{:04}", datetime.year()) } else { String::from("*") },
            if self.hour { format!("{:02}", datetime.hour()) } else { String::from("*") },
            if self.minute { format!("{:02}", datetime.minute()) } else { String::from("*") },
            if self.second { format!("{:02}", datetime.second()) } else { String::from("*") }
        ))
    }
}


#[derive(Debug, Clone)]
pub enum GroupingOperator {
    Extension,
    Size(SizeMagnitude),
    Modified(TimeGrouping),
    Accessed(TimeGrouping),
    Created(TimeGrouping),
    FileType,
}

pub fn group(
    files: &[File],
    operator: GroupingOperator,
) -> Vec<Vec<&File>> {
    let mut groups: HashMap<String, Vec<&File>> = HashMap::new();

    for file in files {
        let group_key = match &operator {
            GroupingOperator::Extension => file.extension.clone(),
            GroupingOperator::Size(magnitude) => {
                magnitude.convert(file.size)
            }
            GroupingOperator::Modified(time_grouping) => {
                time_grouping.format(file.modified)
            }
            GroupingOperator::Accessed(time_grouping) => {
                time_grouping.format(file.accessed)
            }
            GroupingOperator::Created(time_grouping) => {
                time_grouping.format(file.created)
            }
            GroupingOperator::FileType => file.file_type.clone(),
        };

        groups.entry(group_key)
            .or_insert_with(Vec::new)
            .push(file);
    }

    groups.into_iter()
        .map(|(_, group)| group)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, Duration};

    #[derive(Debug, Clone)]
    pub struct MockFile {
        pub extension: String,
        pub size: u64,
        pub modified: SystemTime,
        pub accessed: SystemTime,
        pub created: SystemTime,
        pub file_type: String,
    }

    // Implement conversion from MockFile to File if needed, or use File directly if possible.
    impl From<&MockFile> for File {
        fn from(m: &MockFile) -> Self {
            File {
                name: "mock_file".to_string(),
                extension: m.extension.clone(),
                size: m.size,
                modified: m.modified,
                accessed: m.accessed,
                created: m.created,
                file_type: m.file_type.clone(),
            }
        }
    }

    fn sample_files() -> Vec<File> {
        let now = SystemTime::now();
        let earlier = now - Duration::from_secs(3600);

        vec![
            File {
                name: "file1.txt".to_string(),
                extension: "txt".to_string(),
                size: 1000,
                modified: now,
                accessed: now,
                created: now,
                file_type: "file".to_string(),
            },
            File {
                name: "file2.rs".to_string(),
                extension: "rs".to_string(),
                size: 2048,
                modified: earlier,
                accessed: earlier,
                created: earlier,
                file_type: "file".to_string(),
            },
            File {
                name: "file3.txt".to_string(),
                extension: "txt".to_string(),
                size: 4096,
                modified: now,
                accessed: now,
                created: now,
                file_type: "file".to_string(),
            },
        ]
    }

    #[test]
    fn test_group_by_extension() {
        let files = sample_files();
        let groups = group(&files, GroupingOperator::Extension);
        // Should be 2 groups: "txt" and "rs"
        assert_eq!(groups.len(), 2);
        let extensions: Vec<String> = groups.iter()
            .flat_map(|g| g.iter().map(|f| f.extension.clone()))
            .collect();
        assert!(extensions.contains(&"txt".to_string()));
        assert!(extensions.contains(&"rs".to_string()));
    }

    #[test]
    fn test_group_by_size() {
        let files = sample_files();
        let groups = group(&files, GroupingOperator::Size(SizeMagnitude::Kilobytes));
        // Should be 3 groups, as all sizes are different in KB
        assert_eq!(groups.len(), 3);
    }

    #[test]
    fn test_group_by_file_type() {
        let files = sample_files();
        let groups = group(&files, GroupingOperator::FileType);
        // All are "file"
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].len(), 3);
    }

    #[test]
    fn test_group_by_modified_time_day() {
        let files = sample_files();
        let grouping = TimeGrouping {
            year: true,
            month: true,
            day: true,
            hour: false,
            minute: false,
            second: false,
        };
        let groups = group(&files, GroupingOperator::Modified(grouping));
        // Should be 1 or 2 groups depending on the day difference
        assert!(groups.len() >= 1);
    }
}