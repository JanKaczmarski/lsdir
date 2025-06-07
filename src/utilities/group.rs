use chrono::{DateTime, Datelike, Local, Timelike};
use std::{collections::HashMap, time::SystemTime};

use crate::file::File;

/// Represents different size magnitudes for file size formatting and grouping.
///
/// This enum defines the available size units that can be used when converting
/// file sizes from bytes to human-readable formats or when grouping files by size.
///
/// # Variants
/// - `Bytes`: Raw byte count
/// - `Kilobytes`: Size in kilobytes (1024 bytes)
/// - `Megabytes`: Size in megabytes (1024^2 bytes)
/// - `Gigabytes`: Size in gigabytes (1024^3 bytes)
/// - `Terabytes`: Size in terabytes (1024^4 bytes)
#[derive(Debug, Clone)]
pub enum SizeMagnitude {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
    Terabytes,
}

impl SizeMagnitude {
    /// Converts a size in bytes to a human-readable string format.
    ///
    /// This method takes a file size in bytes and converts it to the appropriate
    /// unit specified by the `SizeMagnitude` variant, formatting it with appropriate
    /// decimal places and unit suffixes.
    ///
    /// # Arguments
    ///
    /// * `size` - The size in bytes to convert
    ///
    /// # Returns
    ///
    /// A formatted string representing the size with the appropriate unit suffix.
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

/// Configuration for time-based grouping of files.
///
/// This struct defines which components of a timestamp should be considered
/// when grouping files by time. Each boolean field determines whether that
/// time component should be included in the grouping key.
///
/// # Fields
/// - `year`: Include the year in the grouping (4-digit format)
/// - `month`: Include the month in the grouping (2-digit format)
/// - `day`: Include the day in the grouping (2-digit format)
/// - `hour`: Include the hour in the grouping (2-digit format, 24-hour)
/// - `minute`: Include the minute in the grouping (2-digit format)
/// - `second`: Include the second in the grouping (2-digit format)
///
/// When a component is set to `false`, it will be represented as "*" in the
/// formatted time string, effectively ignoring that component for grouping purposes.
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
    /// Formats a `SystemTime` according to the grouping configuration.
    ///
    /// This method converts a `SystemTime` to a formatted string that includes
    /// only the time components specified in the `TimeGrouping` configuration.
    /// Components not included in the grouping are represented with "*".
    ///
    /// The format follows the pattern: "DD.MM.YYYY HH:MM:SS" where each component
    /// is either a zero-padded number or "*" if that component is excluded.
    ///
    /// # Arguments
    ///
    /// * `time` - The `SystemTime` to format
    ///
    /// # Returns
    ///
    /// A formatted string representing the time according to the grouping configuration.
    pub fn format(&self, time: SystemTime) -> String {
        let datetime: DateTime<Local> = time.into();
        String::from(format!(
            "{}.{}.{} {}:{}:{}",
            if self.day {
                format!("{:02}", datetime.day())
            } else {
                String::from("*")
            },
            if self.month {
                format!("{:02}", datetime.month())
            } else {
                String::from("*")
            },
            if self.year {
                format!("{:04}", datetime.year())
            } else {
                String::from("*")
            },
            if self.hour {
                format!("{:02}", datetime.hour())
            } else {
                String::from("*")
            },
            if self.minute {
                format!("{:02}", datetime.minute())
            } else {
                String::from("*")
            },
            if self.second {
                format!("{:02}", datetime.second())
            } else {
                String::from("*")
            }
        ))
    }
}

/// Defines the different ways files can be grouped together.
///
/// This enum specifies the various criteria that can be used to group files
/// when organizing or displaying them. Each variant represents a different
/// grouping strategy with its own parameters and behavior.
///
/// # Variants
/// - `Extension`: Group files by their file extension
/// - `Size(SizeMagnitude)`: Group files by size, converted to the specified magnitude
/// - `Modified(TimeGrouping)`: Group files by modification time using the specified time components
/// - `Accessed(TimeGrouping)`: Group files by access time using the specified time components
/// - `Created(TimeGrouping)`: Group files by creation time using the specified time components
/// - `FileType`: Group files by their type (file, directory, etc.)
#[derive(Debug, Clone)]
pub enum GroupingOperator {
    Extension,
    Size(SizeMagnitude),
    Modified(TimeGrouping),
    Accessed(TimeGrouping),
    Created(TimeGrouping),
    FileType,
}

/// Groups a collection of files according to the specified grouping operator.
///
/// This function takes a slice of files and groups them based on the provided
/// `GroupingOperator`. Files with the same grouping key (as determined by the
/// operator) will be placed in the same group.
///
/// # Arguments
///
/// * `files` - A slice of files to be grouped
/// * `operator` - The grouping criteria to use for organizing the files
///
/// # Returns
///
/// A vector of vectors, where each inner vector contains files that belong
/// to the same group. The order of groups is not guaranteed.
pub fn group(files: &[File], operator: GroupingOperator) -> Vec<Vec<&File>> {
    let mut groups: HashMap<String, Vec<&File>> = HashMap::new();

    for file in files {
        let group_key = match &operator {
            GroupingOperator::Extension => file.extension.clone(),
            GroupingOperator::Size(magnitude) => magnitude.convert(file.size),
            GroupingOperator::Modified(time_grouping) => time_grouping.format(file.modified),
            GroupingOperator::Accessed(time_grouping) => time_grouping.format(file.accessed),
            GroupingOperator::Created(time_grouping) => time_grouping.format(file.created),
            GroupingOperator::FileType => file.file_type.clone(),
        };

        groups.entry(group_key).or_insert_with(Vec::new).push(file);
    }

    groups.into_iter().map(|(_, group)| group).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

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
        let extensions: Vec<String> = groups
            .iter()
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
