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