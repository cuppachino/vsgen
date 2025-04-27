use std::time::Instant;

use clap::Parser;
use data::Manifest;
use file::ManifestIter;

mod cli;
mod data;
mod datagen;
mod file;
mod error;

fn main() -> Result<(), error::Error> {
    let time_now = Instant::now();

    let args = cli::Args::parse();

    for manifest in args.paths().iter_manifests() {
        let manifest = manifest?;

        println!("Processing manifest: {:#?}", manifest);

        let datagen = datagen::DataGen::new(&manifest)?;
        let recipes = datagen.generate()?;

        if args.is_dry_run() {
            for recipe in recipes {
                println!("{:?}", recipe);
            }
            println!("Dry run complete. No files were written.");
        }
    }

    let elapsed_time = time_now.elapsed();
    println!("Finished all tasks in {:.2?}", elapsed_time);

    Ok(())
}
