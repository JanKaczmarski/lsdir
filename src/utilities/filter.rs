use regex::Regex;
use crate::file::File;

use chrono::{DateTime, Local, NaiveDateTime, NaiveTime, TimeZone};
use clap::ValueEnum;
use std::str::FromStr;


#[derive(Debug, Clone, ValueEnum)]
pub enum Comparison {
    /// Not equal to
    Ne,
    /// Equal to
    Eq,
    /// Greater than
    Gt,
    /// Greater than or equal
    Ge,
    /// Less than
    Lt,
    /// Less than or equal
    Le,
}

impl FromStr for Comparison {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "eq" | "equal" | "equals" => Ok(Comparison::Eq),
            "ne" | "not_equal" | "neq" => Ok(Comparison::Ne),
            "gt" | "greater" | "greater_than" => Ok(Comparison::Gt),
            "ge" | "gte" | "greater_equal" => Ok(Comparison::Ge),
            "lt" | "less" | "less_than" => Ok(Comparison::Lt),
            "le" | "lte" | "less_equal" => Ok(Comparison::Le),
            _ => Err(format!("Invalid comparison operator: {}", s)),
        }
    }
}

impl Comparison {
    /// Compares two values using the specified comparison operator.
    /// This method performs a comparison between two values of the same type
    /// using the comparison operation defined by the enum variant. The values
    /// must implement both `PartialEq` and `PartialOrd` traits.
    pub fn compare<T: PartialEq + PartialOrd>(&self, a: T, b: T) -> bool {
        match self {
            Comparison::Ne => a != b,
            Comparison::Eq => a == b,
            Comparison::Gt => a > b,
            Comparison::Ge => a >= b,
            Comparison::Lt => a < b,
            Comparison::Le => a <= b,
        }
    }
}

/// Defines various filtering predicates for files.
///
/// This enum represents different criteria that can be used to filter files.
/// Each variant specifies a particular file attribute and the condition that
/// must be met for a file to pass the filter.
///
/// # Variants
/// - `Name(String)`: Filter by file name using exact match or regex pattern
/// - `Extension(String)`: Filter by file extension (exact match)
/// - `Size(u64, Comparison)`: Filter by file size with comparison operator
/// - `Modified(DateTime<Local>, Comparison)`: Filter by modification time with comparison
/// - `Accessed(DateTime<Local>, Comparison)`: Filter by access time with comparison
/// - `Created(DateTime<Local>, Comparison)`: Filter by creation time with comparison
/// - `FileType(String)`: Filter by file type (e.g., "File", "Directory")
///
/// # Name Filtering Behavior
///
/// The `Name` predicate first attempts to interpret the string as a regular expression.
/// If the regex compilation succeeds, it uses pattern matching. If regex compilation
/// fails, it falls back to exact string matching.
#[derive(Debug, Clone)]
pub enum Predicate {
    Name(String),
    Extension(String),
    Size(u64, Comparison),
    Modified(DateTime<Local>, Comparison),
    Accessed(DateTime<Local>, Comparison),
    Created(DateTime<Local>, Comparison),
    FileType(String),
}

impl FromStr for Predicate {
    type Err = String;
    fn from_str(s: &str) -> Result<Predicate, Self::Err> {
        let mut parts: Vec<String> = s.splitn(3, ',').map(|part| part.trim().to_lowercase()).collect();
        
        if parts.len() == 2 {
            parts.insert(1, "eq".to_string());
        }

        if parts.len() != 3 {
            return Err(format!(
                "Invalid predicate format. Expected: field,operator,value, got: {}",
                s
            ));
        }

        let operator = parts[1]
            .parse::<Comparison>()
            .map_err(|_| format!("Invalid operator: {}", parts[1]))?;

        let parse_datetime = |date_str: &str| {
            if let Ok(naive_dt) = NaiveDateTime::parse_from_str(date_str, "%d.%m.%Y %H:%M") {
                return Ok(Local.from_local_datetime(&naive_dt)
                    .single()
                    .ok_or_else(|| "Ambiguous or invalid local datetime".to_string())?);
            }
            // Try time only, use today's date
            if let Ok(naive_time) = NaiveTime::parse_from_str(date_str, "%H:%M") {
                let today = Local::now().date_naive();
                let naive_dt = NaiveDateTime::new(today, naive_time);
                return Ok(Local.from_local_datetime(&naive_dt)
                    .single()
                    .ok_or_else(|| "Ambiguous or invalid local datetime".to_string())?);
            }
            Err(format!("Invalid date/time format: {}", s))
        };


        match (parts[0].as_str(), operator, parts[2].as_str()) {
            ("name" | "n", Comparison::Eq, name) => Ok(Predicate::Name(name.to_string())),
            ("extension" | "ext" | "e", Comparison::Eq, ext) => Ok(Predicate::Extension(ext.to_string())),
            ("size" | "s", operator, size_str) => {
                let size = size_str.parse::<u64>().map_err(|_| format!("Invalid size value: {}", size_str))?;
                Ok(Predicate::Size(size, operator))
            }
            ("modified" | "mod" | "m", operator, time_str) => {
                Ok(Predicate::Modified(parse_datetime(time_str)?, operator))
            }
            ("accessed" | "acc" | "a", operator, time_str) => {
                Ok(Predicate::Accessed(parse_datetime(time_str)?, operator))
            }
            ("created" | "cre" | "c", operator, time_str) => {
                Ok(Predicate::Created(parse_datetime(time_str)?, operator))
            }
            ("filetype" | "file_type" | "type" | "f" | "t", Comparison::Eq, file_type) => Ok(Predicate::FileType(file_type.to_string())),
            _ => Err(format!("Invalid predicate: {}", s)),
        }


    }
}

/// Filters a collection of files based on the specified predicate.
///
/// This function takes a slice of file references and applies the given predicate
/// to each file, returning only those files that satisfy the filtering condition.
/// The function preserves the lifetime of the input references in the output.
///
/// # Arguments
///
/// * `files` - A slice of references to `File` objects to be filtered
/// * `predicate` - The filtering criterion to apply to each file
///
/// # Returns
///
/// A vector containing references to the files that satisfy the predicate condition.
/// The returned references have the same lifetime as the input references.
///
/// # Name Filtering Details
///
/// When using `Predicate::Name`, the function first attempts to compile the provided
/// string as a regular expression. If successful, it uses regex matching against
/// the file name. If regex compilation fails (due to invalid regex syntax), it
/// falls back to exact string comparison.
pub fn filter<'a>(files: &[&'a File], predicate: Predicate) -> Vec<&'a File> {
    files
        .iter()
        .filter(|entry_ref| {
            let entry: &File = *entry_ref;
            match &predicate {
                Predicate::Name(name) => {
                    if let Ok(regex) = Regex::new(name) {
                        regex.is_match(&entry.name)
                    } else {
                        entry.name == *name
                    }
                }
                Predicate::Extension(extension) => entry.extension == *extension,
                Predicate::Size(size, comparison) => {
                    comparison.compare(entry.size, *size)
                }
                Predicate::Modified(time, comparison) => {
                    comparison.compare(entry.modified, *time)
                }
                Predicate::Accessed(time, comparison) => {
                    comparison.compare(entry.accessed, *time)
                }
                Predicate::Created(time, comparison) => {
                    comparison.compare(entry.created, *time)
                }
                Predicate::FileType(file_type) => entry.file_type == *file_type,
            }
        })
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::File;
    use chrono::{DateTime, Local, TimeZone};

    fn dt(secs: u64) -> DateTime<Local> {
        // Helper to convert UNIX timestamp to DateTime<Local>
        Local.timestamp_opt(secs as i64, 0).unwrap()
    }

    fn mock_file(
        name: &str,
        extension: &str,
        size: u64,
        modified: u64,
        accessed: u64,
        created: u64,
        file_type: &str,
    ) -> File {
        File {
            name: name.to_string(),
            extension: extension.to_string(),
            size,
            modified: dt(modified),
            accessed: dt(accessed),
            created: dt(created),
            file_type: file_type.to_string(),
        }
    }

    #[test]
    fn test_name_predicate_exact_match() {
        let file = mock_file("report.txt", "txt", 100, 0, 0, 0, "File");
        let files = vec![&file];
        let result = filter(&files, Predicate::Name("report.txt".to_string()));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_extension_predicate() {
        let file1 = mock_file("a.txt", "txt", 10, 0, 0, 0, "File");
        let file2 = mock_file("b.md", "md", 20, 0, 0, 0, "File");
        let files = vec![&file1, &file2];
        let result = filter(&files, Predicate::Extension("txt".to_string()));
        assert_eq!(result, vec![&file1]);
    }

    #[test]
    fn test_size_predicate() {
        let file1 = mock_file("a.txt", "txt", 10, 0, 0, 0, "File");
        let file2 = mock_file("b.txt", "txt", 20, 0, 0, 0, "File");
        let files = vec![&file1, &file2];
        let result = filter(&files, Predicate::Size(15, Comparison::Gt));
        assert_eq!(result, vec![&file2]);
    }

    #[test]
    fn test_modified_predicate() {
        let file1 = mock_file("a.txt", "txt", 10, 10, 0, 0, "File");
        let file2 = mock_file("b.txt", "txt", 20, 20, 0, 0, "File");
        let files = vec![&file1, &file2];
        let result = filter(
            &files,
            Predicate::Modified(dt(15), Comparison::Lt),
        );
        assert_eq!(result, vec![&file1]);
    }

    #[test]
    fn test_accessed_predicate() {
        let file1 = mock_file("a.txt", "txt", 10, 0, 10, 0, "File");
        let file2 = mock_file("b.txt", "txt", 20, 0, 20, 0, "File");
        let files = vec![&file1, &file2];
        let result = filter(
            &files,
            Predicate::Accessed(dt(20), Comparison::Eq),
        );
        assert_eq!(result, vec![&file2]);
    }

    #[test]
    fn test_created_predicate() {
        let file1 = mock_file("a.txt", "txt", 10, 0, 0, 10, "File");
        let file2 = mock_file("b.txt", "txt", 20, 0, 0, 20, "File");
        let files = vec![&file1, &file2];
        let result = filter(
            &files,
            Predicate::Created(dt(10), Comparison::Ge),
        );
        assert_eq!(result, vec![&file1, &file2]);
    }

    #[test]
    fn test_filetype_predicate() {
        let file1 = mock_file("a", "txt", 10, 0, 0, 0, "File");
        let file2 = mock_file("b", "", 0, 0, 0, 0, "Directory");
        let files = vec![&file1, &file2];
        let result = filter(&files, Predicate::FileType("Directory".to_string()));
        assert_eq!(result, vec![&file2]);
    }

    #[test]
    fn test_name_predicate_regex_match() {
        let file = mock_file("report.txt", "txt", 100, 0, 0, 0, "File");
        let files = vec![&file];
        let result = filter(&files, Predicate::Name(r"re.*\.txt".to_string()));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_name_predicate_regex_no_match() {
        let file = mock_file("summary.txt", "txt", 100, 0, 0, 0, "File");
        let files = vec![&file];
        let result = filter(&files, Predicate::Name(r"re.*\.txt".to_string()));
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_name_predicate_invalid_regex_fallback_to_string() {
        let file = mock_file("re[port.txt", "txt", 100, 0, 0, 0, "File");
        let files = vec![&file];
        let result = filter(&files, Predicate::Name("re[port.txt".to_string()));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_name_predicate_invalid_regex_no_match() {
        let file = mock_file("report.txt", "txt", 100, 0, 0, 0, "File");
        let files = vec![&file];
        let result = filter(&files, Predicate::Name("re[port.txt".to_string()));
        assert_eq!(result.len(), 0);
    }
}