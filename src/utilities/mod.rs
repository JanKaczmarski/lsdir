pub mod aggregate;
pub mod filter;
pub mod group;

// Re-export common types for CLI usage
pub use aggregate::{ArithmeticAggregator, ComparingAggregator};
pub use filter::{ComparisonOperator, Predicate};
pub use group::{GroupingOperator, SizeMagnitude, TimeGrouping};

// Create unified enums for CLI
use clap::ValueEnum;
use std::str::FromStr;

#[derive(Debug, Clone, ValueEnum)]
pub enum Field {
    Name,
    Extension,
    Size,
    FileType,
    Modified,
    Accessed,
    Created,
}

impl FromStr for Field {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "name" => Ok(Field::Name),
            "extension" => Ok(Field::Extension),
            "size" => Ok(Field::Size),
            "file_type" | "filetype" => Ok(Field::FileType),
            "modified" => Ok(Field::Modified),
            "accessed" => Ok(Field::Accessed),
            "created" => Ok(Field::Created),
            _ => Err(format!("Invalid field: {}", s)),
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Comparison {
    /// Equal to
    Eq,
    /// Not equal to
    Ne,
    /// Greater than
    Gt,
    /// Greater than or equal
    Ge,
    /// Less than
    Lt,
    /// Less than or equal
    Le,
    /// Contains (for string fields)
    Contains,
    /// Starts with (for string fields)
    StartsWith,
    /// Ends with (for string fields)
    EndsWith,
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
            "contains" => Ok(Comparison::Contains),
            "starts_with" | "startswith" => Ok(Comparison::StartsWith),
            "ends_with" | "endswith" => Ok(Comparison::EndsWith),
            _ => Err(format!("Invalid comparison operator: {}", s)),
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AggrFunc {
    Count,
    Sum,
    Avg,
    Max,
    Min,
}
