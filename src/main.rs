use directories::UserDirs;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

fn main() -> std::io::Result<()> {
    let mut path_buff = PathBuf::new();

    if let Some(user_dirs) = UserDirs::new() {
        let home = user_dirs.home_dir();
        let file = Path::new("test.txt");
        path_buff.push(home);
        path_buff.push(file);
    }

    // Create a new file for writing
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path_buff)
        .unwrap();

    // Create a buffered writer to write to the file
    let mut writer = BufWriter::new(file);

    // Write some data to the file
    writer.write_all(b"Hello, world!\n")?;
    writer.write_all(b"Rust is awesome.\n")?;

    // Flush the writer to ensure all data is written to disk

    println!("Updated ENV VARs");
    Ok(())
}
