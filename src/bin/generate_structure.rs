use std::fs::{self, File};
use std::io::{Result, Write};
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let root = PathBuf::from("src");
    let mut output = String::new();
    output.push_str("# 📂 Arborescence du projet `src/`\n\n```\n");
    build_tree(&root, "", &mut output)?;
    output.push_str("```\n");

    let mut file = File::create("current_structure.md")?;
    file.write_all(output.as_bytes())?;
    println!("✅ Fichier `current_structure.md` généré !");
    Ok(())
}

fn build_tree(path: &Path, prefix: &str, output: &mut String) -> Result<()> {
    let entries = fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|e| !e.file_name().to_str().unwrap_or("").starts_with('.'))
        .collect::<Vec<_>>();

    for (i, entry) in entries.iter().enumerate() {
        let name = entry.file_name().into_string().unwrap_or_default();
        let is_last = i == entries.len() - 1;
        let symbol = if is_last { "└── " } else { "├── " };
        output.push_str(&format!("{}{}{}\n", prefix, symbol, name));

        if entry.path().is_dir() {
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            build_tree(&entry.path(), &new_prefix, output)?;
        }
    }
    Ok(())
}
