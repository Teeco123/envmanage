use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    // Create a new file for writing
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("test.txt")
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
