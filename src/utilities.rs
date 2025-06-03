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
    Size(u64, ComparisonOperator),
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
                    entry.name == *name
                }
                Predicate::Size(size, comparison_operator) =>
                    match comparison_operator {
                        ComparisonOperator::EqualTo => entry.size == *size,
                        ComparisonOperator::GreaterThan => entry.size > *size,
                        ComparisonOperator::GreaterThanOrEqualTo => entry.size >= *size,
                        ComparisonOperator::LessThan => entry.size < *size,
                        ComparisonOperator::LessThanOrEqualTo => entry.size <= *size,
                    }
            }
        })
        .copied()
        .collect()
}