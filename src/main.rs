use clap::Parser;
use directories::UserDirs;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

#[derive(Parser)]
struct Args{
    path: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut path_buff = PathBuf::new();

    if let Some(user_dirs) = UserDirs::new() {
        let home = user_dirs.home_dir();
        let file = Path::new(".zprofile");
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
    writer.write_all(b"export PATH=")?;
    writer.write_all(b"\"")?;
    writer.write_all(args.path.as_bytes())?;
    writer.write_all(b":$PATH\"\n")?;
    // Flush the writer to ensure all data is written to disk

    println!("Updated ENV VARs");
    Ok(())
}
