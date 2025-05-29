use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct Cli {
    // GROUP BY clause from SQL
    #[arg(short, long, value_name = "FIELD")]
    pub group_by: Option<Field>,

    // WHERE clause from SQL
    #[arg(short, long, value_name = "FIELD")]
    pub r#where: Option<Field>,

    // Aggregating fucntion to use
    #[arg(short, long, value_name = "AGGR_FUNCTION_NAME")]
    pub func: Option<AggrFuncs>,

    // Aggreating func parameters, eg. for SUM(1, 2, 3) param would be '1,2,3'
    #[arg(short, long, num_args = 1.., value_delimiter = ',')]
    pub params: Vec<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AggrFuncs {
    Count,
    Sum,
    Avg,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Field {
    Name,
    Extension,
    Size,
}
