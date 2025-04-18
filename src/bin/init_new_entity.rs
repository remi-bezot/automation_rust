use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("❗ Usage: cargo run --bin init_new_entity <species>");
        return;
    }

    let species = &args[1];
    let genome_path = format!("src/genomes/{}", species);
    let entities_path = format!("src/entities/");

    if !Path::new(&genome_path).exists() {
        eprintln!("❗ Espèce inconnue : '{}'", species);
        return;
    }

    let next_id = find_next_entity_id(&entities_path, species);
    let new_entity_path = format!("{}{}_{}", entities_path, species, next_id);

    match fs_extra::dir::copy(
        &genome_path,
        &entities_path,
        &fs_extra::dir::CopyOptions {
            overwrite: false,
            copy_inside: true,
            content_only: false,
            ..Default::default()
        },
    ) {
        Ok(_) => {
            fs::rename(format!("{}{}", entities_path, species), &new_entity_path)
                .expect("Échec du renommage");

            println!("✅ Nouvelle entité créée : {}", new_entity_path);
        }
        Err(e) => eprintln!("❌ Erreur: {}", e),
    }
}

fn find_next_entity_id(base_path: &str, species: &str) -> String {
    let entries =
        fs::read_dir(base_path).unwrap_or_else(|_| panic!("Impossible de lire {}", base_path));
    let mut max_id = 0;

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with(species) {
            if let Some(id_str) = name.split('_').last() {
                if let Ok(id) = id_str.parse::<u32>() {
                    max_id = max_id.max(id);
                }
            }
        }
    }

    format!("{:03}", max_id + 1)
}
