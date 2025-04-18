use std::fs::{self, File};
use std::io::{Result, Write};
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let root = PathBuf::from("src");
    generate_mod_rs_recursively(&root)?;
    println!("✅ Tous les fichiers mod.rs ont été régénérés.");
    Ok(())
}

fn generate_mod_rs_recursively(dir: &Path) -> Result<()> {
    let mut mods = Vec::new();

    let entries = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    for entry in entries.iter() {
        let path = entry.path();
        let file_name_owned = entry.file_name().to_string_lossy().to_string();
        let file_name = file_name_owned.as_str();

        if file_name == "mod.rs" {
            continue;
        }

        if path.is_dir() {
            if path.join("mod.rs").exists() {
                mods.push(format!("pub mod {};", file_name));
                generate_mod_rs_recursively(&path)?;
            }
        } else if path.is_file() && path.extension().map(|e| e == "rs").unwrap_or(false) {
            if let Some(stem) = path.file_stem() {
                mods.push(format!("pub mod {};", stem.to_string_lossy()));
            }
        }
    }

    if !mods.is_empty() {
        let mut file = File::create(dir.join("mod.rs"))?;
        writeln!(file, "// Auto-generated mod.rs")?;
        for line in mods {
            writeln!(file, "{}", line)?;
        }
    }

    Ok(())
}
