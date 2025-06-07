use regex::Regex;
use std::time::SystemTime;

use crate::file::File;

/// Defines comparison operators for filtering operations.
///
/// This enum provides different comparison operations that can be used
/// when filtering files based on numeric or time-based criteria such as
/// file size, modification time, access time, or creation time.
///
/// # Variants
/// - `EqualTo`: Exact equality comparison (==)
/// - `GreaterThan`: Greater than comparison (>)
/// - `GreaterThanOrEqualTo`: Greater than or equal comparison (>=)
/// - `LessThan`: Less than comparison (<)
/// - `LessThanOrEqualTo`: Less than or equal comparison (<=)
#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    EqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
}

impl ComparisonOperator {
    /// Compares two values using the specified comparison operator.
    ///
    /// This method performs a comparison between two values of the same type
    /// using the comparison operation defined by the enum variant. The values
    /// must implement both `PartialEq` and `PartialOrd` traits.
    ///
    /// # Arguments
    ///
    /// * `a` - The first value to compare
    /// * `b` - The second value to compare
    ///
    /// # Returns
    ///
    /// `true` if the comparison condition is met, `false` otherwise.
    pub fn compare<T: PartialEq + PartialOrd>(&self, a: T, b: T) -> bool {
        match self {
            ComparisonOperator::EqualTo => a == b,
            ComparisonOperator::GreaterThan => a > b,
            ComparisonOperator::GreaterThanOrEqualTo => a >= b,
            ComparisonOperator::LessThan => a < b,
            ComparisonOperator::LessThanOrEqualTo => a <= b,
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
/// - `Size(u64, ComparisonOperator)`: Filter by file size with comparison operator
/// - `Modified(SystemTime, ComparisonOperator)`: Filter by modification time with comparison
/// - `Accessed(SystemTime, ComparisonOperator)`: Filter by access time with comparison
/// - `Created(SystemTime, ComparisonOperator)`: Filter by creation time with comparison
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
    Size(u64, ComparisonOperator),
    Modified(SystemTime, ComparisonOperator),
    Accessed(SystemTime, ComparisonOperator),
    Created(SystemTime, ComparisonOperator),
    FileType(String),
}

/// Filters a collection of files based on the specified predicate.
///
/// This function takes a slice of file references and applies the given predicate
/// to each file, returning only those files that satisfy the filtering condition.
/// The function preserves the lifetime of the input references in the output.
///
/// # Arguments
///
/// * `paths` - A slice of references to `File` objects to be filtered
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
pub fn filter<'a>(paths: &[&'a File], predicate: Predicate) -> Vec<&'a File> {
    paths
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
                Predicate::Size(size, comparison_operator) => {
                    comparison_operator.compare(entry.size, *size)
                }
                Predicate::Modified(time, comparison_operator) => {
                    comparison_operator.compare(entry.modified, *time)
                }
                Predicate::Accessed(time, comparison_operator) => {
                    comparison_operator.compare(entry.accessed, *time)
                }
                Predicate::Created(time, comparison_operator) => {
                    comparison_operator.compare(entry.created, *time)
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
    use std::time::{Duration, UNIX_EPOCH};

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
            modified: UNIX_EPOCH + Duration::from_secs(modified),
            accessed: UNIX_EPOCH + Duration::from_secs(accessed),
            created: UNIX_EPOCH + Duration::from_secs(created),
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
        let result = filter(&files, Predicate::Size(15, ComparisonOperator::GreaterThan));
        assert_eq!(result, vec![&file2]);
    }

    #[test]
    fn test_modified_predicate() {
        let file1 = mock_file("a.txt", "txt", 10, 10, 0, 0, "File");
        let file2 = mock_file("b.txt", "txt", 20, 20, 0, 0, "File");
        let files = vec![&file1, &file2];
        let result = filter(
            &files,
            Predicate::Modified(
                UNIX_EPOCH + Duration::from_secs(15),
                ComparisonOperator::LessThan,
            ),
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
            Predicate::Accessed(
                UNIX_EPOCH + Duration::from_secs(20),
                ComparisonOperator::EqualTo,
            ),
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
            Predicate::Created(
                UNIX_EPOCH + Duration::from_secs(10),
                ComparisonOperator::GreaterThanOrEqualTo,
            ),
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
