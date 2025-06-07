use clap::Parser;
mod cli;
mod file;
mod utilities;

use cli::{Cli, WhereCondition};
use file::File;
use std::{collections::HashMap, fs};
use utilities::aggregate::{ArithmeticAggregator, ComparingAggregator, average, max, min, sum};
use utilities::filter::{filter, Predicate};
use utilities::group::{group, GroupingOperator, SizeMagnitude, TimeGrouping};
use utilities::{AggrFunc, Comparison};

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    // Validate grouping field early - utilities only support certain fields for grouping
    if let Some(group_field) = &args.group_by {
        match group_field.to_lowercase().as_str() {
            "name" => {
                eprintln!("Error: Grouping by 'name' is not supported by the utilities.");
                eprintln!(
                    "Supported grouping fields: extension, file_type, size, modified, accessed, created"
                );
                return Ok(());
            }
            "extension" | "file_type" | "size" | "modified" | "accessed" | "created" => {} // Valid
            _ => {
                eprintln!(
                    "Error: Invalid grouping field '{}'. Supported fields: extension, file_type, size, modified, accessed, created",
                    group_field
                );
                return Ok(());
            }
        }
    }

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

        if grouped.is_empty() {
            return Ok(());
        }

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

// Chose only those `files` that match `condition`
fn filter_files(files: &[File], condition: &WhereCondition) -> Vec<File> {
    // casting to appropriate datatype
    let predicate = match condition.field.to_lowercase().as_str() {
        "name" => Predicate::Name(condition.value.clone()),
        "extension" => Predicate::Extension(condition.value.clone()),
        "file_type" => Predicate::FileType(condition.value.clone()),
        "size" => {
            if let Ok(size_value) = condition.value.parse::<u64>() {
                Predicate::Size(size_value, condition.operator.clone())
            } else {
                eprintln!("Warning: Invalid size value '{}'", condition.value);
                return files.to_vec();
            }
        }
        "modified" | "accessed" | "created" => {
            eprintln!("Warning: Time-based filtering requires time parsing (not implemented yet)");
            return files.to_vec();
        }
        _ => {
            eprintln!("Warning: Unknown field '{}'", condition.field);
            return files.to_vec();
        }
    };
    let file_refs: Vec<&File> = files.iter().collect();
    let filtered_refs = filter(&file_refs, predicate);
    filtered_refs.into_iter().cloned().collect()
}

// given some `files` and `group_field` return a HashMap that will have `identifier` and this identifier
// would be the id that identifies elements in that key-value pair.
// When grouping by file extension keys would be like: "txt", "py", "rs". And the values would be text files for .txt
// python files for .py and rust files for .rs
fn group_files(files: &[File], group_field: &str) -> HashMap<String, Vec<File>> {
    let grouping_operator = match group_field.to_lowercase().as_str() {
        "extension" => GroupingOperator::Extension,
        "file_type" => GroupingOperator::FileType,
        "size" => GroupingOperator::Size(SizeMagnitude::Bytes),
        "modified" => GroupingOperator::Modified(TimeGrouping {
            year: true,
            month: true,
            day: false,
            hour: false,
            minute: false,
            second: false,
        }),
        "accessed" => GroupingOperator::Accessed(TimeGrouping {
            year: true,
            month: true,
            day: false,
            hour: false,
            minute: false,
            second: false,
        }),
        "created" => GroupingOperator::Created(TimeGrouping {
            year: true,
            month: true,
            day: false,
            hour: false,
            minute: false,
            second: false,
        }),
        _ => {
            eprintln!("Warning: Unknown grouping field '{}'", group_field);
            return HashMap::new();
        }
    };
    let grouped_refs = group(files, grouping_operator);
    let mut result = HashMap::new();
    for group in grouped_refs {
        if let Some(file) = group.first() {
            // put file to its group, ex. if grouping by extension, this would dump all .txt files into one key,value and .py to other key,value
            let key = match group_field.to_lowercase().as_str() {
                "extension" => file.extension.clone(),
                "file_type" => file.file_type.clone(),
                "size" => SizeMagnitude::Bytes.convert(file.size),
                "modified" => TimeGrouping {
                    year: true,
                    month: true,
                    day: false,
                    hour: false,
                    minute: false,
                    second: false,
                }.format(file.modified),
                "accessed" => TimeGrouping {
                    year: true,
                    month: true,
                    day: false,
                    hour: false,
                    minute: false,
                    second: false,
                }.format(file.accessed),
                "created" => TimeGrouping {
                    year: true,
                    month: true,
                    day: false,
                    hour: false,
                    minute: false,
                    second: false,
                }.format(file.created),
                _ => "unknown".to_string(),
            };
            let owned_files: Vec<File> = group.into_iter().cloned().collect();
            result.insert(key, owned_files);
        }
    }
    result
}

fn apply_aggregation(
    grouped_files: &HashMap<String, Vec<File>>,
    function: &AggrFunc,
    params: &[String],
) {
    for (group_key, files) in grouped_files {
        println!("Group: {}", group_key);
        match function {
            AggrFunc::Count => {
                println!("  Count: {}", files.len());
            }
            AggrFunc::Sum => {
                if let Some(param) = params.first() {
                    if param == "size" {
                        println!("  Sum (size): {:.2}", sum(files, ArithmeticAggregator::Size));
                    } else {
                        eprintln!("Error: Unsupported parameter '{}' for SUM. Only 'size' is supported.", param);
                        return;
                    }
                } else {
                    eprintln!("Error: SUM requires a parameter (e.g., 'size').");
                    return;
                }
            }
            AggrFunc::Avg => {
                if let Some(param) = params.first() {
                    if param == "size" {
                        println!("  Average (size): {:.2}", average(files, ArithmeticAggregator::Size).unwrap_or(0.0));
                    } else {
                        eprintln!("Error: Unsupported parameter '{}' for AVG. Only 'size' is supported.", param);
                        return;
                    }
                } else {
                    eprintln!("Error: AVG requires a parameter (e.g., 'size').");
                    return;
                }
            }
            AggrFunc::Min => {
                if let Some(param) = params.first() {
                    if param == "size" {
                        if let Some(f) = min(files, ComparingAggregator::Size) {
                            println!("  Min Size: {}", f.size);
                        }
                    } else {
                        eprintln!("Error: Unsupported parameter '{}' for MIN. Only 'size' is supported.", param);
                        return;
                    }
                } else {
                    eprintln!("Error: MIN requires a parameter (e.g., 'size').");
                    return;
                }
            }
            AggrFunc::Max => {
                if let Some(param) = params.first() {
                    if param == "size" {
                        if let Some(f) = max(files, ComparingAggregator::Size) {
                            println!("  Max Size: {}", f.size);
                        }
                    } else {
                        eprintln!("Error: Unsupported parameter '{}' for MAX. Only 'size' is supported.", param);
                        return;
                    }
                } else {
                    eprintln!("Error: MAX requires a parameter (e.g., 'size').");
                    return;
                }
            }
        }
        println!();
    }
}

fn apply_single_aggregation(files: &[File], function: &AggrFunc, params: &[String]) {
    match function {
        AggrFunc::Count => {
            println!("Count: {}", files.len());
        }
        AggrFunc::Sum => {
            if let Some(param) = params.first() {
                if param == "size" {
                    println!("Sum (size): {:.2}", sum(files, ArithmeticAggregator::Size));
                } else {
                    eprintln!("Error: Unsupported parameter '{}' for SUM. Only 'size' is supported.", param);
                    return;
                }
            } else {
                eprintln!("Error: SUM requires a parameter (e.g., 'size').");
                return;
            }
        }
        AggrFunc::Avg => {
            if let Some(param) = params.first() {
                if param == "size" {
                    println!("Average (size): {:.2}", average(files, ArithmeticAggregator::Size).unwrap_or(0.0));
                } else {
                    eprintln!("Error: Unsupported parameter '{}' for AVG. Only 'size' is supported.", param);
                    return;
                }
            } else {
                eprintln!("Error: AVG requires a parameter (e.g., 'size').");
                return;
            }
        }
        AggrFunc::Min => {
            if let Some(param) = params.first() {
                if param == "size" {
                    if let Some(f) = min(files, ComparingAggregator::Size) {
                        println!("Min Size: {}", f.size);
                    }
                } else {
                    eprintln!("Error: Unsupported parameter '{}' for MIN. Only 'size' is supported.", param);
                    return;
                }
            } else {
                eprintln!("Error: MIN requires a parameter (e.g., 'size').");
                return;
            }
        }
        AggrFunc::Max => {
            if let Some(param) = params.first() {
                if param == "size" {
                    if let Some(f) = max(files, ComparingAggregator::Size) {
                        println!("Max Size: {}", f.size);
                    }
                } else {
                    eprintln!("Error: Unsupported parameter '{}' for MAX. Only 'size' is supported.", param);
                    return;
                }
            } else {
                eprintln!("Error: MAX requires a parameter (e.g., 'size').");
                return;
            }
        }
    }
}

fn display_grouped_files(grouped_files: &HashMap<String, Vec<File>>) {
    for (group_key, files) in grouped_files {
        println!("Group: {} ({} files)", group_key, files.len());
        for file in files {
            println!("  {} ({} bytes)", file.name, file.size);
        }
        println!();
    }
}

fn display_files(files: &[File]) {
    for file in files {
        println!("{} ({} bytes)", file.name, file.size);
    }
}
