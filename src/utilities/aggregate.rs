use std::time::SystemTime;

use crate::file::File;

#[derive(Debug, Clone)]
pub enum ComparingAggregator {
    Size,
    Modified,
    Accessed,
    Created,
}

impl ComparingAggregator {
    pub fn compare(&self, a: &File, b: &File) -> std::cmp::Ordering {
        match self {
            ComparingAggregator::Size => a.size.cmp(&b.size),
            ComparingAggregator::Modified => a.modified.cmp(&b.modified),
            ComparingAggregator::Accessed => a.accessed.cmp(&b.accessed),
            ComparingAggregator::Created => a.created.cmp(&b.created),
        }
    }
}



pub fn max(files: &[File], aggregator: ComparingAggregator) -> Option<File> {
    files.iter().cloned().
        max_by(|a, b| aggregator.compare(a, b))
}

pub fn min(files: &[File], aggregator: ComparingAggregator) -> Option<File> {
    files.iter().cloned().
        min_by(|a, b| aggregator.compare(a, b))
}

#[derive(Debug, Clone)]
pub enum ArithmeticAggregator {
    Size,
}

pub fn sum(files: &[File], aggregator: ArithmeticAggregator) -> u64 {
    files.iter().fold(0, |acc, file| {
        match aggregator {
            ArithmeticAggregator::Size => acc + file.size,
        }
    })
}

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
    use std::time::{SystemTime, Duration};

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