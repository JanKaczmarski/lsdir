mod file_type;
mod utilities;

use std::fs;

// fn get_directory_data()

//


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
