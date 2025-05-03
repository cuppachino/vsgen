use std::{ io::{ Seek, Write }, path::PathBuf, time::Instant };

use clap::Parser;
use file::ManifestIter;

mod cli;
mod data;
mod datagen;
mod file;
mod error;

fn main() -> Result<(), error::Error> {
    let time_now = Instant::now();

    let args = cli::Args::parse();

    if args.paths().is_empty() {
        eprintln!("No input files or directories specified.");
        std::process::exit(1);
    }

    let dist_path = args.dist();
    let dist_path = PathBuf::from(dist_path);
    std::fs::create_dir_all(&dist_path).map_err(error::Error::Io)?;

    for manifest in args.paths().iter_manifests() {
        let manifest = manifest?;
        let datagen = datagen::DataGen::new(&manifest)?;
        let recipes = datagen.generate()?;

        println!("Generated {} recipes", recipes.len());

        if args.is_dry_run() {
            for recipe in recipes {
                println!("Recipe: {recipe:#?}");
            }
            println!("Dry run complete. No files were written.");
        } else {
            let recipe_len = recipes.len();
            if recipe_len == 0 {
                println!("No recipes generated.");
                continue;
            }
            let file_path = dist_path.join(manifest.output.as_str());
            let file = std::fs::File::create(&file_path).map_err(error::Error::Io)?;
            let mut writer = std::io::BufWriter::new(file);
            writer.write_all(b"[\n").map_err(error::Error::Io)?;

            for recipe in recipes {
                recipe.write(&mut writer).map_err(error::Error::Io)?;
                writer.write_all(b",\n").map_err(error::Error::Io)?;
            }

            // Remove the last comma and newline
            writer.seek_relative(-2).map_err(error::Error::Io)?;
            writer.write_all(b"]").map_err(error::Error::Io)?;

            println!("Saved recipes to {}", file_path.display());
        }
    }

    let elapsed_time = time_now.elapsed();
    println!("Finished all tasks in {elapsed_time:.2?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::file::ManifestIterator;

    use super::*;

    const TEST: &str = "input/shovel.json";

    #[test]
    fn benchmark_deserialization() {
        let start = Instant::now();
        let paths: Vec<PathBuf> = vec![TEST.into()];
        let mut manifest_iter = paths.iter_manifests();
        for _ in &mut manifest_iter {
            // Do nothing, just iterate
        }
        let elapsed = start.elapsed();
        println!("Deserialization took: {elapsed:.2?}");
    }

    #[test]
    fn benchmark_serialization() {
        let start = Instant::now();
        let paths: Vec<PathBuf> = vec![TEST.into()];
        let mut manifest_iter = paths.iter_manifests();
        let mut buf = Vec::new();
        for manifest in &mut manifest_iter {
            let manifest = manifest.unwrap();
            let datagen = datagen::DataGen::new(&manifest).unwrap();
            let recipes = datagen.generate().unwrap();
            for recipe in recipes {
                recipe.write(&mut buf).unwrap();
            }
        }
        let elapsed = start.elapsed();
        println!("Serialization took: {elapsed:.2?}");
        println!("Serialized {} bytes", buf.len());
    }
}
