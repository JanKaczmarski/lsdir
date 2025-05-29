use crate::file_type::FileType;

pub enum Predicate {
    Name(String),
    Size(u64),
}

pub fn filter<'a>(
    paths: &[&'a FileType],
    predicate: Predicate
) -> Vec<&'a FileType> {
    paths
        .iter()
        .filter(|entry_ref| {
            let entry: &FileType = *entry_ref;
            match &predicate {
                Predicate::Name(name) => {
                    entry.name == *name
                }
                Predicate::Size(size) => {
                    entry.size == *size
                }
            }
        })
        .copied()
        .collect()
}