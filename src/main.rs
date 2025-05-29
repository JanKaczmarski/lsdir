use std::{fs, time::SystemTime};

// fn get_directory_data()

//

struct FileType {
    name: String,
    extension: String,
    size: String,
    modified: SystemTime,
    changed: SystemTime,
    accessed: SystemTime,
    created: SystemTime,
    file_type: String,
}

fn main() -> std::io::Result<()> {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let metadata = fs::metadata(&path)?;
        println!("Name: {}", path.display());
        println!("{:#?}", metadata);
    }

    Ok(())
}
