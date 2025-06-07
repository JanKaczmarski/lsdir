use crate::utilities::{AggrFunc, Comparison, Field};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Directory path to analyze (defaults to current directory)
    #[arg(value_name = "PATH")]
    pub path: Option<String>,

    /// GROUP BY clause - field to group files by
    #[arg(short, long, value_name = "FIELD")]
    pub group_by: Option<Field>,

    /// WHERE clause - filter condition in format: field,operator,value
    /// Examples: size,gt,123 or name,eq,test_*
    #[arg(short, long, value_name = "CONDITION")]
    pub r#where: Option<String>,

    /// Aggregating function to use
    #[arg(short, long, value_name = "FUNCTION")]
    pub function: Option<AggrFunc>,

    /// Parameters for the aggregating function
    /// For SUM/AVG: field name (e.g., 'size')
    /// For COUNT: no parameters needed
    #[arg(short, long, num_args = 0.., value_delimiter = ',')]
    pub params: Vec<String>,
}

/// Represents a parsed WHERE condition
#[derive(Debug, Clone)]
pub struct WhereCondition {
    pub field: Field,
    pub operator: Comparison,
    pub value: String,
}

impl WhereCondition {
    /// Parse a WHERE condition from string format: field,operator,value
    pub fn parse(condition: &str) -> Result<Self, String> {
        let parts: Vec<&str> = condition.splitn(3, ',').collect();
        if parts.len() != 3 {
            return Err(format!(
                "Invalid WHERE condition format. Expected: field,operator,value, got: {}",
                condition
            ));
        }

        let field = parts[0]
            .parse::<Field>()
            .map_err(|_| format!("Invalid field: {}", parts[0]))?;

        let operator = parts[1]
            .parse::<Comparison>()
            .map_err(|_| format!("Invalid operator: {}", parts[1]))?;

        let value = parts[2].to_string();

        Ok(WhereCondition {
            field,
            operator,
            value,
        })
    }
}
