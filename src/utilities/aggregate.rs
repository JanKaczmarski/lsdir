use crate::file::File;

use std::collections::HashMap; 
use std::fmt::Display; 
use std::str::FromStr;



/// Represents an aggregate function that can be applied to a collection of files.
#[derive(Debug, Clone)]
pub enum AggregateFunction {
    Count,
    Sum(ArithmeticAggregator),
    Avg(ArithmeticAggregator),
    Max(ComparingAggregator),
    Min(ComparingAggregator),
}

impl FromStr for AggregateFunction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ',').collect();
        match parts[0].to_lowercase().as_str() {
            "count" | "c" => Ok(AggregateFunction::Count),
            "sum" | "s" => {
                Ok(AggregateFunction::Sum(ArithmeticAggregator::Size))
            }
            "average" | "avg" | "a" => {
                Ok(AggregateFunction::Avg(ArithmeticAggregator::Size))
            }
            "max" => {
                if parts.len() < 2 {
                    return Err("Missing argument for max".to_string());
                }
                let aggregator = ComparingAggregator::from_str(parts[1])?;
                Ok(AggregateFunction::Max(aggregator))
            }
            "min" => {
                if parts.len() < 2 {
                    return Err("Missing argument for min".to_string());
                }
                let aggregator = ComparingAggregator::from_str(parts[1])?;
                Ok(AggregateFunction::Min(aggregator))
            }
            _ => Err(format!("Unknown aggregate function: {}", s)),
        }
    }
    
}

/// Defines comparison criteria for file aggregation operations.
///
/// This enum specifies which file attribute should be used when performing
/// comparison-based aggregations such as finding minimum or maximum values.
/// Each variant corresponds to a specific file property that can be compared.
///
/// # Variants
/// - `Size`: Compare files by their size in bytes
/// - `Modified`: Compare files by their last modification time
/// - `Accessed`: Compare files by their last access time
/// - `Created`: Compare files by their creation time
#[derive(Debug, Clone)]
pub enum ComparingAggregator {
    Size,
    Modified,
    Accessed,
    Created,
}

impl ComparingAggregator {
    /// Compares two files based on the specified aggregation criterion.
    ///
    /// This method performs a comparison between two files using the attribute
    /// specified by the enum variant. It returns a `std::cmp::Ordering` that
    /// indicates the relative ordering of the two files.
    ///
    /// # Arguments
    ///
    /// * `a` - The first file to compare
    /// * `b` - The second file to compare
    ///
    /// # Returns
    ///
    /// A `std::cmp::Ordering` indicating whether the first file is less than,
    /// equal to, or greater than the second file according to the specified criterion.
    pub fn compare(&self, a: &File, b: &File) -> std::cmp::Ordering {
        match self {
            ComparingAggregator::Size => a.size.cmp(&b.size),
            ComparingAggregator::Modified => a.modified.cmp(&b.modified),
            ComparingAggregator::Accessed => a.accessed.cmp(&b.accessed),
            ComparingAggregator::Created => a.created.cmp(&b.created),
        }
    }
}

impl FromStr for ComparingAggregator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "size" | "s" => Ok(ComparingAggregator::Size),
            "modified" | "mod" | "m" => Ok(ComparingAggregator::Modified),
            "accessed" | "acc" | "a" => Ok(ComparingAggregator::Accessed),
            "created" | "cre" | "c" => Ok(ComparingAggregator::Created),
            _ => Err(format!("Unknown comparing aggregator: {}", s)),
        }
    }
}

impl Display for ComparingAggregator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ComparingAggregator::Size => "Size",
            ComparingAggregator::Modified => "Modified",
            ComparingAggregator::Accessed => "Accessed",
            ComparingAggregator::Created => "Created",
        };
        write!(f, "{}", name)
    }
}


/// Finds the file with the maximum value for the specified comparison criterion.
///
/// This function searches through a collection of grouped files and returns, for each group,
/// the file that has the highest value according to the specified `ComparingAggregator`.
/// For example, it can find the largest file, the most recently modified file,
/// or the most recently accessed file in each group.
///
/// # Arguments
///
/// * `files` - A map from group key to a vector of file references
/// * `aggregator` - The comparison criterion to use for finding the maximum
///
/// # Returns
///
/// A `HashMap<String, &File>` mapping each group key to the file with the maximum value.
pub fn max<'a>(files: &'a HashMap<String, Vec<&'a File>>, aggregator: ComparingAggregator) -> HashMap<String, &'a File> {
    files
        .iter()
        .map(|(key, file_list)| {
            let max_file = file_list
                .iter()
                .cloned()
                .max_by(|a, b| aggregator.compare(a, b))
                .unwrap();
            (key.clone(), max_file)
        })
        .collect()
}

/// Finds the file with the minimum value for the specified comparison criterion.
///
/// This function searches through a collection of grouped files and returns, for each group,
/// the file that has the lowest value according to the specified `ComparingAggregator`.
/// For example, it can find the smallest file, the oldest modified file,
/// or the least recently accessed file in each group.
///
/// # Arguments
///
/// * `files` - A map from group key to a vector of file references
/// * `aggregator` - The comparison criterion to use for finding the minimum
///
/// # Returns
///
/// A `HashMap<String, &File>` mapping each group key to the file with the minimum value.
pub fn min<'a>(files: &'a HashMap<String, Vec<&'a File>>, aggregator: ComparingAggregator) -> HashMap<String, &'a File> {
    files
        .iter()
        .map(|(key, file_list)| {
            let min_file = file_list
                .iter()
                .cloned()
                .min_by(|a, b| aggregator.compare(a, b))
                .unwrap();
            (key.clone(), min_file)
        })
        .collect()
}

/// Defines arithmetic aggregation criteria for file operations.
///
/// This enum specifies which numeric file attribute should be used when
/// performing arithmetic operations such as sum or average calculations.
/// Currently focused on size-based calculations but can be extended for
/// other numeric properties.
///
/// # Variants
/// - `Size`: Perform arithmetic operations on file sizes in bytes
#[derive(Debug, Clone)]
pub enum ArithmeticAggregator {
    Size,
}

impl Display for ArithmeticAggregator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ArithmeticAggregator::Size => "Size",
        };
        write!(f, "{}", name)
    }
}

/// Calculates the sum of a numeric property across all files.
///
/// This function aggregates a numeric value from all files in each group according to the specified
/// `ArithmeticAggregator`. Currently supports summing file sizes, but can be extended for other numeric properties.
///
/// # Arguments
///
/// * `files` - A map from group key to a vector of file references
/// * `aggregator` - The arithmetic criterion specifying which property to sum
///
/// # Returns
///
/// A `HashMap<String, u64>` mapping each group key to the sum for that group.
pub fn sum(files: &HashMap<String, Vec<&File>>, aggregator: ArithmeticAggregator) -> HashMap<String, u64> {
    files
        .iter()
        .map(|(key, file_list)| {
            let total: u64 = file_list.iter().map(|file| match aggregator {
                ArithmeticAggregator::Size => file.size,
            }).sum();
            (key.clone(), total)
        })
        .collect()
}

/// Calculates the average of a numeric property across all files in each group.
///
/// This function computes the arithmetic mean of a numeric value from all files in each group
/// according to the specified `ArithmeticAggregator`. The calculation uses the `sum` function
/// internally and divides by the count of files in each group.
///
/// # Arguments
///
/// * `files` - A map from group key to a vector of file references
/// * `aggregator` - The arithmetic criterion specifying which property to average
///
/// # Returns
///
/// A `HashMap<String, f64>` mapping each group key to the average value for that group.
pub fn avg(files: &HashMap<String, Vec<&File>>, aggregator: ArithmeticAggregator) -> HashMap<String, f64> {
    let sum = sum(files, aggregator.clone());

    files
        .iter()
        .map(|(key, file_list)| {
            let count = file_list.len() as f64;
            if count == 0.0 {
                (key.clone(), 0.0) // Avoid division by zero
            } else {
                let total = sum.get(key).unwrap_or(&0);
                (key.clone(), *total as f64 / count)
            }
        })
        .collect()
}

/// Counts the number of files in each group.
///
/// # Arguments
///
/// * `files` - A map from group key to a vector of file references
///
/// # Returns
///
/// A `HashMap<String, u64>` mapping each group key to the count of files in that group.
pub fn count(files: &HashMap<String, Vec<&File>>) -> HashMap<String, u64> {
    files
        .iter()
        .map(|(key, file_list)| (key.clone(), file_list.len() as u64))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Local, TimeZone};
    use std::collections::HashMap;

    fn dt(secs: i64) -> DateTime<Local> {
        Local.timestamp_opt(secs, 0).unwrap()
    }

    fn sample_files() -> Vec<File> {
        let now = dt(1_000_000);
        let earlier = dt(1_000_000 - 3600);
        let oldest = dt(1_000_000 - 7200);

        vec![
            File {
                name: "file1.txt".to_string(),
                extension: "txt".to_string(),
                size: 1000,
                modified: now,
                accessed: now,
                created: now,
                file_type: "file".to_string(),
            },
            File {
                name: "file2.rs".to_string(),
                extension: "rs".to_string(),
                size: 2048,
                modified: earlier,
                accessed: earlier,
                created: earlier,
                file_type: "file".to_string(),
            },
            File {
                name: "file3.txt".to_string(),
                extension: "txt".to_string(),
                size: 4096,
                modified: oldest,
                accessed: oldest,
                created: oldest,
                file_type: "file".to_string(),
            },
        ]
    }

    fn group_by_ext(files: &[File]) -> HashMap<String, Vec<&File>> {
        let mut map: HashMap<String, Vec<&File>> = HashMap::new();
        for file in files {
            map.entry(file.extension.clone()).or_default().push(file);
        }
        map
    }

    #[test]
    fn test_max_size() {
        let files = sample_files();
        let grouped = group_by_ext(&files);
        let max_map = max(&grouped, ComparingAggregator::Size);
        assert_eq!(max_map["txt"].size, 4096);
        assert_eq!(max_map["rs"].size, 2048);
    }

    #[test]
    fn test_min_size() {
        let files = sample_files();
        let grouped = group_by_ext(&files);
        let min_map = min(&grouped, ComparingAggregator::Size);
        assert_eq!(min_map["txt"].size, 1000);
        assert_eq!(min_map["rs"].size, 2048);
    }

    #[test]
    fn test_sum_size() {
        let files = sample_files();
        let grouped = group_by_ext(&files);
        let sum_map = sum(&grouped, ArithmeticAggregator::Size);
        assert_eq!(sum_map["txt"], 1000 + 4096);
        assert_eq!(sum_map["rs"], 2048);
    }

    #[test]
    fn test_average_size() {
        let files = sample_files();
        let grouped = group_by_ext(&files);
        let avg_map = avg(&grouped, ArithmeticAggregator::Size);
        assert!((avg_map["txt"] - ((1000.0 + 4096.0) / 2.0)).abs() < 1e-6);
        assert!((avg_map["rs"] - 2048.0).abs() < 1e-6);
    }

    #[test]
    fn test_count() {
        let files = sample_files();
        let grouped = group_by_ext(&files);
        let count_map = count(&grouped);
        assert_eq!(count_map["txt"], 2);
        assert_eq!(count_map["rs"], 1);
    }

    #[test]
    fn test_average_empty_group() {
        let grouped: HashMap<String, Vec<&File>> = HashMap::new();
        let avg_map = avg(&grouped, ArithmeticAggregator::Size);
        assert!(avg_map.is_empty());
    }
}