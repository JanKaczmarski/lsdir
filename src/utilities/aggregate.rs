use std::time::SystemTime;

use crate::file::File;

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

/// Finds the file with the maximum value for the specified comparison criterion.
///
/// This function searches through a collection of files and returns the file
/// that has the highest value according to the specified `ComparingAggregator`.
/// For example, it can find the largest file, the most recently modified file,
/// or the most recently accessed file.
///
/// # Arguments
///
/// * `files` - A slice of files to search through
/// * `aggregator` - The comparison criterion to use for finding the maximum
///
/// # Returns
///
/// An `Option<File>` containing the file with the maximum value, or `None` if
/// the input slice is empty.
pub fn max(files: &[File], aggregator: ComparingAggregator) -> Option<File> {
    files
        .iter()
        .cloned()
        .max_by(|a, b| aggregator.compare(a, b))
}

/// Finds the file with the minimum value for the specified comparison criterion.
///
/// This function searches through a collection of files and returns the file
/// that has the lowest value according to the specified `ComparingAggregator`.
/// For example, it can find the smallest file, the oldest modified file,
/// or the least recently accessed file.
///
/// # Arguments
///
/// * `files` - A slice of files to search through
/// * `aggregator` - The comparison criterion to use for finding the minimum
///
/// # Returns
///
/// An `Option<File>` containing the file with the minimum value, or `None` if
/// the input slice is empty.
pub fn min(files: &[File], aggregator: ComparingAggregator) -> Option<File> {
    files
        .iter()
        .cloned()
        .min_by(|a, b| aggregator.compare(a, b))
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

/// Calculates the sum of a numeric property across all files.
///
/// This function aggregates a numeric value from all files in the collection
/// according to the specified `ArithmeticAggregator`. Currently supports
/// summing file sizes, but can be extended for other numeric properties.
///
/// # Arguments
///
/// * `files` - A slice of files to aggregate
/// * `aggregator` - The arithmetic criterion specifying which property to sum
///
/// # Returns
///
/// The sum as a `u64` value. Returns 0 if the input slice is empty.
pub fn sum(files: &[File], aggregator: ArithmeticAggregator) -> u64 {
    files.iter().fold(0, |acc, file| match aggregator {
        ArithmeticAggregator::Size => acc + file.size,
    })
}

/// Calculates the average of a numeric property across all files.
///
/// This function computes the arithmetic mean of a numeric value from all files
/// in the collection according to the specified `ArithmeticAggregator`. The
/// calculation uses the `sum` function internally and divides by the count of files.
///
/// # Arguments
///
/// * `files` - A slice of files to aggregate
/// * `aggregator` - The arithmetic criterion specifying which property to average
///
/// # Returns
///
/// An `Option<f64>` containing the average value, or `None` if the input slice
/// is empty (to avoid division by zero).
pub fn average(files: &[File], aggregator: ArithmeticAggregator) -> Option<f64> {
    if files.is_empty() {
        return None;
    }
    let total = sum(files, aggregator);
    Some(total as f64 / files.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    fn sample_files() -> Vec<File> {
        let now = SystemTime::now();
        let earlier = now - Duration::from_secs(3600);
        let oldest = now - Duration::from_secs(7200);

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

    #[test]
    fn test_max_size() {
        let files = sample_files();
        let max_file = max(&files, ComparingAggregator::Size).unwrap();
        assert_eq!(max_file.size, 4096);
    }

    #[test]
    fn test_min_size() {
        let files = sample_files();
        let min_file = min(&files, ComparingAggregator::Size).unwrap();
        assert_eq!(min_file.size, 1000);
    }

    #[test]
    fn test_max_modified() {
        let files = sample_files();
        let max_file = max(&files, ComparingAggregator::Modified).unwrap();
        // The most recently modified file is the first one
        assert_eq!(max_file.name, "file1.txt");
    }

    #[test]
    fn test_min_modified() {
        let files = sample_files();
        let min_file = min(&files, ComparingAggregator::Modified).unwrap();
        // The oldest modified file is the last one
        assert_eq!(min_file.name, "file3.txt");
    }

    #[test]
    fn test_sum_size() {
        let files = sample_files();
        let total = sum(&files, ArithmeticAggregator::Size);
        assert_eq!(total, 1000 + 2048 + 4096);
    }

    #[test]
    fn test_average_size() {
        let files = sample_files();
        let avg = average(&files, ArithmeticAggregator::Size).unwrap();
        assert!((avg - ((1000.0 + 2048.0 + 4096.0) / 3.0)).abs() < 1e-6);
    }

    #[test]
    fn test_average_empty() {
        let files: Vec<File> = vec![];
        assert_eq!(average(&files, ArithmeticAggregator::Size), None);
    }
}
