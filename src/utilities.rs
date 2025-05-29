use crate::file_type::FileType;

enum Predicate {
    Name(String),
    Size(u64),
}

fn filter<'a>(
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