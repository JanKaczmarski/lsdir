use std::{collections::HashMap, time::SystemTime};
use chrono::{DateTime, Local, Datelike, Timelike};

use crate::file::File;

#[derive(Debug, Clone)]
pub enum SizeMagnitude {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
    Terabytes,
}

impl SizeMagnitude {
    pub fn convert(&self, size: u64) -> String {
        match self {
            SizeMagnitude::Bytes => format!("{} B", size),
            SizeMagnitude::Kilobytes => format!("{:.2} KB", size / 1024),
            SizeMagnitude::Megabytes => format!("{:.2} MB", size / (1024 * 1024)),
            SizeMagnitude::Gigabytes => format!("{:.2} GB", size / (1024 * 1024 * 1024)),
            SizeMagnitude::Terabytes => format!("{:.2} TB", size / (1024 * 1024 * 1024 * 1024)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimeGrouping {
    pub year: bool,
    pub month: bool,
    pub day: bool,
    pub hour: bool,
    pub minute: bool,
    pub second: bool,
}

impl TimeGrouping {
    pub fn format(&self, time: SystemTime) -> String {
        let datetime: DateTime<Local> = time.into();
        String::from(format!(
            "{}.{}.{} {}:{}:{}",
            if self.day { format!("{:02}", datetime.day()) } else { String::from("*") },
            if self.month { format!("{:02}", datetime.month()) } else { String::from("*") },
            if self.year { format!("{:04}", datetime.year()) } else { String::from("*") },
            if self.hour { format!("{:02}", datetime.hour()) } else { String::from("*") },
            if self.minute { format!("{:02}", datetime.minute()) } else { String::from("*") },
            if self.second { format!("{:02}", datetime.second()) } else { String::from("*") }
        ))
    }
}


#[derive(Debug, Clone)]
pub enum GroupingOperator {
    Extension,
    Size(SizeMagnitude),
    Modified(TimeGrouping),
    Accessed(TimeGrouping),
    Created(TimeGrouping),
    FileType,
}

fn group(
    files: &[File],
    operator: GroupingOperator,
) -> Vec<Vec<&File>> {
    let mut groups: HashMap<String, Vec<&File>> = HashMap::new();

    for file in files {
        let group_key = match &operator {
            GroupingOperator::Extension => file.extension.clone(),
            GroupingOperator::Size(magnitude) => {
                magnitude.convert(file.size)
            }
            GroupingOperator::Modified(time_grouping) => {
                time_grouping.format(file.modified)
            }
            GroupingOperator::Accessed(time_grouping) => {
                time_grouping.format(file.accessed)
            }
            GroupingOperator::Created(time_grouping) => {
                time_grouping.format(file.created)
            }
            GroupingOperator::FileType => file.file_type.clone(),
        };

        groups.entry(group_key)
            .or_insert_with(Vec::new)
            .push(file);
    }

    groups.into_iter()
        .map(|(_, group)| group)
        .collect()
}