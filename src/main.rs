use csv::{ReaderBuilder, StringRecord};
use serde_json;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_dir> <output_dir>", args[0]);
        std::process::exit(1);
    }

    let input_dir = &args[1];
    let output_dir = &args[2];

    for entry in WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && e.path().extension().and_then(|s| s.to_str()) == Some("csv"))
    {
        let path = entry.path();
        println!("Processing file: {}", path.display());
        process_csv_file(path, output_dir)?;
    }

    Ok(())
}

fn process_csv_file(path: &Path, output_dir: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let headers = rdr.headers()?.clone();
    let mut records = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let record = record_to_map(&headers, &record);
        records.push(record);
    }

    let json = serde_json::to_string_pretty(&records)?;

    // Create output folder
    fs::create_dir_all(output_dir)?;

    // Create Json
    let json_file_name = path.file_stem().unwrap().to_str().unwrap().to_owned() + ".json";
    let json_path = Path::new(output_dir).join(json_file_name);
    let mut json_file = File::create(json_path)?;
    write!(json_file, "{}", json)?;

    Ok(())
}

fn record_to_map(headers: &StringRecord, record: &StringRecord) -> HashMap<String, String> {
    headers
        .iter()
        .zip(record.iter())
        .map(|(header, field)| (header.to_string(), field.to_string()))
        .collect()
}
