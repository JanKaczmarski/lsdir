pub mod aggregate;
pub mod filter;
pub mod group;

// Re-export common types for CLI usage
pub use aggregate::{ArithmeticAggregator, ComparingAggregator};
pub use filter::{Predicate};
pub use group::{GroupingOperator, SizeMagnitude, TimeGrouping};

// Create unified enums for CLI
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
            _ => false, // Only makes sense for string-specific ops
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
