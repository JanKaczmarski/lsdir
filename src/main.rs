mod cli;
mod file;
mod utilities;

use std::str::FromStr;
use std::collections::HashMap;
use std::fs;
use clap::Parser;
use cli::Cli;


use file::File;
use utilities::filter::{filter, Predicate};
use utilities::group::{group, GroupingOperator};
use utilities::aggregate::{AggregateFunction};

use crate::utilities::aggregate::{count, sum, max, min, avg};

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    // Get directory path (default to current directory)
    let dir_path = args.path.as_deref().unwrap_or(".");

    // Read files from directory
    let files = read_directory(dir_path)?;

    let file_refs: Vec<&File> = files.iter().collect();

    // Apply WHERE filter if specified
    let filtered_files = if let Some(where_clause) = &args.r#where {
        match Predicate::from_str(where_clause) {
            Ok(condition) => filter(&file_refs, condition),
            Err(e) => {
                eprintln!("Error parsing WHERE condition: {}", e);
                return Ok(());
            }
        }
    } else {
        file_refs
    };
    let grouped_files = if let Some(group_field) = &args.group_by {
        match GroupingOperator::from_str(group_field) {
            Ok(operator) => group(&filtered_files, operator),
            Err(e) => {
                eprintln!("Error parsing GROUP BY field: {}", e);
                return Ok(());
            }
        }
    } else {
        let mut map = HashMap::new();
        map.insert(dir_path.to_string(), filtered_files);
        map
    };

    if let Some(aggregate) = &args.aggregate {
        match AggregateFunction::from_str(aggregate) {
            Ok(aggregate_function) => {
               display_aggregated(&grouped_files, aggregate_function); 
            }
            Err(e) => {
                eprintln!("Error parsing aggregate function: {}", e);
            }
        }
    } else {
        display(&grouped_files);    
    }


    Ok(())
}

fn read_directory(path: &str) -> std::io::Result<Vec<File>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        match File::from_dir_entry(&entry) {
            Ok(file) => files.push(file),
            Err(e) => eprintln!("Warning: Could not read file {:?}: {}", entry.path(), e),
        }
    }
    Ok(files)
}

fn display(files: &HashMap<String, Vec<&File>>) {
    println!("   {:<19} | {:<19} | {:<19} | {:<10} | {:>10} | {:<30}",
        "Modified", "Accessed", "Created", "Type", "Size (bytes)", "Name"
    );
    for (key, group) in files {
        println!("{}", key);
        for file in group {
            println!("   {}", file);
        }
    }
    
}

fn display_aggregated(files: &HashMap<String, Vec<&File>>, aggregate_function: AggregateFunction) {
    match aggregate_function {
        AggregateFunction::Count => {
            let aggregated = count(files);
            for (key, count) in aggregated {
                println!("Group: {}, Count: {}\n", key, count);
            }
        }
        AggregateFunction::Sum(field) => {
            let aggregated = sum(files, field.clone());
            for (key, sum) in aggregated {
                println!("{}, Sum of {}: {}\n", key, field, sum);
            }
        }
        AggregateFunction::Max(field) => {
            println!("{:<19} | {:<19} | {:<19} | {:<10} | {:>10} | {:<30}",
                "Modified", "Accessed", "Created", "Type", "Size (bytes)", "Name"
            );
            let aggregated = max(files, field.clone());
            for (key, max_value) in aggregated {
                println!("{}, Max of {}:\n{}\n", key, field, max_value);
            }
        }
        AggregateFunction::Min(field) => {
            println!("{:<19} | {:<19} | {:<19} | {:<10} | {:>10} | {:<30}",
                "Modified", "Accessed", "Created", "Type", "Size (bytes)", "Name"
            );
            let aggregated = min(files, field.clone());
            for (key, min_value) in aggregated {
                println!("{}, Min of {}:\n{}\n", key, field, min_value);
            }
        }
        AggregateFunction::Avg(field) => {
            let aggregated = avg(files, field.clone());
            for (key, avg_value) in aggregated {
                println!("{}, Avg of {}: {}\n", key, field, avg_value);
            }
        }
    }
}