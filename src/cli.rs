use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Directory path to analyze (defaults to current directory)
    #[arg(value_name = "PATH")]
    pub path: Option<String>,

    /// GROUP BY clause - field to group files by (e.g., 'extension', 'size', etc.)
    #[arg(short, long, value_name = "FIELD")]
    pub group_by: Option<String>,

    /// WHERE clause - filter condition in format: field,operator,value
    /// Examples: size,gt,123 or name,eq,test_*
    #[arg(short, long, value_name = "CONDITION")]
    pub r#where: Option<String>,

    /// Aggregating function to use
    #[arg(short, long, value_name = "FUNCTION")]
    pub aggregate: Option<String>,
}