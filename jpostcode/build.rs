use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    println!("cargo:rerun-if-changed=src/json");

    let json_dir = Path::new("src/json");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("address_data.json");

    let mut merged_data = HashMap::new();

    for entry in WalkDir::new(json_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file()
            && entry.path().extension().map_or(false, |ext| ext == "json")
        {
            let content = fs::read_to_string(entry.path()).unwrap();
            let file_data: HashMap<String, Value> = serde_json::from_str(&content).unwrap();

            let prefix = entry.path().file_stem().unwrap().to_str().unwrap();
            for (suffix, data) in file_data {
                let full_postcode = format!("{}{}", prefix, suffix);
                merged_data.insert(full_postcode, data);
            }
        }
    }

    fs::write(dest_path, serde_json::to_string(&merged_data).unwrap()).unwrap();
}
