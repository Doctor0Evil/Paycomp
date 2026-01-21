use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let aln_dir = "aln";
    let dest = Path::new(&out_dir).join("aln_schemas.rs");

    let mut generated = String::new();
    if let Ok(entries) = fs::read_dir(aln_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "yaml" || ext == "yml" {
                    let name = path.file_stem().unwrap().to_string_lossy().to_string();
                    generated.push_str(&format!(
                        "pub const {}: &str = include_str!(\"../aln/{}\");\n",
                        name.to_uppercase(),
                        path.file_name().unwrap().to_string_lossy()
                    ));
                }
            }
        }
    }

    fs::write(&dest, generated).expect("failed to write aln_schemas.rs");
    println!("cargo:rerun-if-changed=aln");
}
