use std::time::SystemTime;
use regex::Regex;

use crate::file_type::FileType;

#[derive(Debug, Clone)]
pub enum ComparisonOperator{
    EqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
}

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

pub fn filter<'a>(
    paths: &[&'a FileType],
    predicate: Predicate
) -> Vec<&'a FileType> {
    paths
        .iter()
        .filter(|entry_ref| {
            let entry: &FileType = *entry_ref;
            match &predicate {
                Predicate::Name(name) => {
                    if let Ok(regex) = Regex::new(name) {
                        regex.is_match(&entry.name)
                    } else {
                        entry.name == *name
                    }
                }
                Predicate::Extension(extension) =>
                    entry.extension == *extension,
                Predicate::Size(size, comparison_operator) =>
                    match comparison_operator {
                        ComparisonOperator::EqualTo => entry.size == *size,
                        ComparisonOperator::GreaterThan => entry.size > *size,
                        ComparisonOperator::GreaterThanOrEqualTo => entry.size >= *size,
                        ComparisonOperator::LessThan => entry.size < *size,
                        ComparisonOperator::LessThanOrEqualTo => entry.size <= *size,
                    }
                Predicate::Modified(time, comparison_operator) =>
                    match comparison_operator {
                        ComparisonOperator::EqualTo => entry.modified == *time,
                        ComparisonOperator::GreaterThan => entry.modified > *time,
                        ComparisonOperator::GreaterThanOrEqualTo => entry.modified >= *time,
                        ComparisonOperator::LessThan => entry.modified < *time,
                        ComparisonOperator::LessThanOrEqualTo => entry.modified <= *time,
                    }
                Predicate::Accessed(time, comparison_operator) =>
                    match comparison_operator {
                        ComparisonOperator::EqualTo => entry.accessed == *time,
                        ComparisonOperator::GreaterThan => entry.accessed > *time,
                        ComparisonOperator::GreaterThanOrEqualTo => entry.accessed >= *time,
                        ComparisonOperator::LessThan => entry.accessed < *time,
                        ComparisonOperator::LessThanOrEqualTo => entry.accessed <= *time,
                    }
                Predicate::Created(time, comparison_operator) =>
                    match comparison_operator {
                        ComparisonOperator::EqualTo => entry.created == *time,
                        ComparisonOperator::GreaterThan => entry.created > *time,
                        ComparisonOperator::GreaterThanOrEqualTo => entry.created >= *time,
                        ComparisonOperator::LessThan => entry.created < *time,
                        ComparisonOperator::LessThanOrEqualTo => entry.created <= *time,
                    }
                Predicate::FileType(file_type) =>
                    entry.file_type == *file_type,
            }
        })
        .copied()
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_type::FileType;
    use std::time::{UNIX_EPOCH, Duration};

    fn mock_file(
        name: &str,
        extension: &str,
        size: u64,
        modified: u64,
        accessed: u64,
        created: u64,
        file_type: &str,
    ) -> FileType {
        FileType {
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
            Predicate::Modified(UNIX_EPOCH + Duration::from_secs(15), ComparisonOperator::LessThan),
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
            Predicate::Accessed(UNIX_EPOCH + Duration::from_secs(20), ComparisonOperator::EqualTo),
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
            Predicate::Created(UNIX_EPOCH + Duration::from_secs(10), ComparisonOperator::GreaterThanOrEqualTo),
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