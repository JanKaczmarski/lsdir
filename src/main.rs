use clap::Parser;
mod cli;
mod file;
mod utilities;

use cli::{Cli, WhereCondition};
use file::File;
use std::{collections::HashMap, fs};
use utilities::aggregate::{ArithmeticAggregator, ComparingAggregator, average, max, min, sum};
use utilities::{AggrFunc, Comparison, Field};

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    // Get directory path (default to current directory)
    let dir_path = args.path.as_deref().unwrap_or(".");

    // Read files from directory
    let files = read_directory(dir_path)?;

    // Apply WHERE filter if specified
    let filtered_files = if let Some(where_clause) = &args.r#where {
        match WhereCondition::parse(where_clause) {
            Ok(condition) => filter_files(&files, &condition),
            Err(e) => {
                eprintln!("Error parsing WHERE condition: {}", e);
                return Ok(());
            }
        }
    } else {
        files
    };

    // Apply grouping and aggregation
    if let Some(group_field) = &args.group_by {
        let grouped = group_files(&filtered_files, group_field);

        if let Some(function) = &args.function {
            apply_aggregation(&grouped, function, &args.params);
        } else {
            display_grouped_files(&grouped);
        }
    } else if let Some(function) = &args.function {
        // Apply aggregation without grouping
        apply_single_aggregation(&filtered_files, function, &args.params);
    } else {
        // Just list files
        display_files(&filtered_files);
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

fn filter_files(files: &[File], condition: &WhereCondition) -> Vec<File> {
    files
        .iter()
        .filter(|file| matches_condition(file, condition))
        .cloned()
        .collect()
}

fn matches_condition(file: &File, condition: &WhereCondition) -> bool {
    let field_value = match condition.field {
        Field::Name => &file.name,
        Field::Extension => &file.extension,
        Field::FileType => &file.file_type,
        Field::Size => return compare_numeric(file.size, &condition.operator, &condition.value),
        Field::Modified | Field::Accessed | Field::Created => {
            // For now, skip time comparisons (would need more complex parsing)
            return true;
        }
    };

    compare_string(field_value, &condition.operator, &condition.value)
}

fn compare_string(field_value: &str, operator: &Comparison, target_value: &str) -> bool {
    match operator {
        Comparison::Eq => field_value == target_value || matches_pattern(field_value, target_value),
        Comparison::Ne => {
            field_value != target_value && !matches_pattern(field_value, target_value)
        }
        Comparison::Contains => field_value.contains(target_value),
        Comparison::StartsWith => field_value.starts_with(target_value),
        Comparison::EndsWith => field_value.ends_with(target_value),
        _ => false, // Other operators don't make sense for strings
    }
}

fn compare_numeric(field_value: u64, operator: &Comparison, target_str: &str) -> bool {
    if let Ok(target_value) = target_str.parse::<u64>() {
        match operator {
            Comparison::Eq => field_value == target_value,
            Comparison::Ne => field_value != target_value,
            Comparison::Gt => field_value > target_value,
            Comparison::Ge => field_value >= target_value,
            Comparison::Lt => field_value < target_value,
            Comparison::Le => field_value <= target_value,
            _ => false,
        }
    } else {
        false
    }
}

fn matches_pattern(text: &str, pattern: &str) -> bool {
    // Simple wildcard matching for patterns like "test_*"
    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            let prefix = parts[0];
            let suffix = parts[1];
            return text.starts_with(prefix) && text.ends_with(suffix);
        }
    }
    false
}

fn group_files(files: &[File], group_field: &Field) -> HashMap<String, Vec<File>> {
    let mut groups: HashMap<String, Vec<File>> = HashMap::new();

    for file in files {
        let group_key = match group_field {
            Field::Name => file.name.clone(),
            Field::Extension => file.extension.clone(),
            Field::FileType => file.file_type.clone(),
            Field::Size => format!("{}", file.size),
            Field::Modified => format!("{:?}", file.modified),
            Field::Accessed => format!("{:?}", file.accessed),
            Field::Created => format!("{:?}", file.created),
        };

        groups.entry(group_key).or_default().push(file.clone());
    }

    groups
}

fn apply_aggregation(grouped: &HashMap<String, Vec<File>>, function: &AggrFunc, params: &[String]) {
    println!("Aggregation Results:");
    println!("-------------------");

    for (group_name, files) in grouped {
        let result = match function {
            AggrFunc::Count => files.len() as f64,
            AggrFunc::Sum | AggrFunc::Avg => {
                if params.is_empty() {
                    eprintln!(
                        "Error: {} function requires a parameter (field name)",
                        format!("{:?}", function)
                    );
                    continue;
                }
                match params[0].to_lowercase().as_str() {
                    "size" => {
                        if matches!(function, AggrFunc::Avg) {
                            average(files, ArithmeticAggregator::Size).unwrap_or(0.0)
                        } else {
                            sum(files, ArithmeticAggregator::Size) as f64
                        }
                    }
                    _ => {
                        eprintln!(
                            "Error: Unsupported field '{}' for arithmetic operations",
                            params[0]
                        );
                        continue;
                    }
                }
            }
            AggrFunc::Max => {
                if params.is_empty() {
                    eprintln!("Error: MAX function requires a parameter (field name)");
                    continue;
                }
                match params[0].to_lowercase().as_str() {
                    "size" => max(files, ComparingAggregator::Size)
                        .map(|f| f.size as f64)
                        .unwrap_or(0.0),
                    _ => {
                        eprintln!("Error: Unsupported field '{}' for MAX operation", params[0]);
                        continue;
                    }
                }
            }
            AggrFunc::Min => {
                if params.is_empty() {
                    eprintln!("Error: MIN function requires a parameter (field name)");
                    continue;
                }
                match params[0].to_lowercase().as_str() {
                    "size" => min(files, ComparingAggregator::Size)
                        .map(|f| f.size as f64)
                        .unwrap_or(0.0),
                    _ => {
                        eprintln!("Error: Unsupported field '{}' for MIN operation", params[0]);
                        continue;
                    }
                }
            }
        };

        println!("{}: {}", group_name, result);
    }
}

fn apply_single_aggregation(files: &[File], function: &AggrFunc, params: &[String]) {
    let result = match function {
        AggrFunc::Count => files.len() as f64,
        AggrFunc::Sum | AggrFunc::Avg => {
            if params.is_empty() {
                eprintln!(
                    "Error: {} function requires a parameter (field name)",
                    format!("{:?}", function)
                );
                return;
            }
            match params[0].to_lowercase().as_str() {
                "size" => {
                    if matches!(function, AggrFunc::Avg) {
                        average(files, ArithmeticAggregator::Size).unwrap_or(0.0)
                    } else {
                        sum(files, ArithmeticAggregator::Size) as f64
                    }
                }
                _ => {
                    eprintln!(
                        "Error: Unsupported field '{}' for arithmetic operations",
                        params[0]
                    );
                    return;
                }
            }
        }
        AggrFunc::Max => {
            if params.is_empty() {
                eprintln!("Error: MAX function requires a parameter (field name)");
                return;
            }
            match params[0].to_lowercase().as_str() {
                "size" => max(files, ComparingAggregator::Size)
                    .map(|f| f.size as f64)
                    .unwrap_or(0.0),
                _ => {
                    eprintln!("Error: Unsupported field '{}' for MAX operation", params[0]);
                    return;
                }
            }
        }
        AggrFunc::Min => {
            if params.is_empty() {
                eprintln!("Error: MIN function requires a parameter (field name)");
                return;
            }
            match params[0].to_lowercase().as_str() {
                "size" => min(files, ComparingAggregator::Size)
                    .map(|f| f.size as f64)
                    .unwrap_or(0.0),
                _ => {
                    eprintln!("Error: Unsupported field '{}' for MIN operation", params[0]);
                    return;
                }
            }
        }
    };

    println!("{:?}: {}", function, result);
}

fn display_grouped_files(grouped: &HashMap<String, Vec<File>>) {
    for (group_name, files) in grouped {
        println!("\n{} ({} files):", group_name, files.len());
        println!("{}", "-".repeat(40));
        for file in files {
            println!("  {} ({} bytes)", file.name, file.size);
        }
    }
}

fn display_files(files: &[File]) {
    println!("Files ({} total):", files.len());
    println!("{}", "-".repeat(40));
    for file in files {
        println!(
            "{:<30} {:<10} {:>10} bytes",
            file.name, file.file_type, file.size
        );
    }
}
