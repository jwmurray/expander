use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("book_map.rs");
    let mut f = File::create(&dest_path).unwrap();

    let files = vec![
        ("bofm_books.txt", "bofm"),
        ("nt_books.txt", "nt"),
        ("ot_books.txt", "ot"),
        ("dc-testament.txt", "dc-testament"),
        ("pgp.txt", "pgp"),
    ]; // Add more files as needed

    writeln!(
        f,
        "static BOOK_MAP: phf::Map<&'static str, &'static str> = phf_map! {{"
    )
    .unwrap();
    for (file_name, _series) in &files {
        if let Ok(input_file) = File::open(file_name) {
            let reader = BufReader::new(input_file);

            for line in reader.lines() {
                let line = line.unwrap();
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    let abbreviation = parts[0];
                    let keys: Vec<&str> = parts[1].split(',').collect();
                    for key in keys {
                        if key.trim().is_empty() {
                            continue;
                        }
                        writeln!(
                            f,
                            "    \"{}\" => \"{}\",",
                            key.trim().to_ascii_lowercase(),
                            abbreviation.trim()
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
    writeln!(f, "}};").unwrap();

    writeln!(
        f,
        "static SERIES_MAP: phf::Map<&'static str, &'static str> = phf_map! {{"
    )
    .unwrap();
    for (file_name, series) in &files {
        if let Ok(input_file) = File::open(file_name) {
            let reader = BufReader::new(input_file);

            for line in reader.lines() {
                let line = line.unwrap();
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    let abbreviation = parts[0];
                    writeln!(f, "    \"{}\" => \"{}\",", abbreviation.trim(), series).unwrap();
                }
            }
        }
    }
    writeln!(f, "}};").unwrap();
}
