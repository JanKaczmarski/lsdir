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