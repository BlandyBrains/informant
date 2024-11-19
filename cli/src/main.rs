mod builder;

use builder::{AssetBuilder, AssetParts, Builder};
use storage::{Asset, Database};
use std::{env, error::Error, fs, path::PathBuf};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    file: String,

    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Hash,
    Scrub,

    #[command(name = "archive", about = "Archive an asset")]
    Archive(ArchiveOpts)
}

#[derive(Parser, Debug)]
pub struct ArchiveOpts {
    #[arg(short, long, help = "Sqlite DB file")]
    db_file: Option<String>,

    #[arg(short, long, help = "Sqlite DB file")]
    create_database: Option<bool>,

    #[arg(long, help = "Directory for archiving")]
    directory: String,
}


fn main() -> Result<(), Box<dyn Error>>{
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Hash => {
            let hash: String = Builder::open(&cli.file)?.get_hash()?;
            println!("HASH: {}", hash);
        }, 
        Commands::Scrub => {
            Builder::open(&cli.file)?.scrub()?;
            println!("Scrubbed: {}", &cli.file);
        }
        Commands::Archive(opts) => {
            // find sqlite db file
            let db_file: String = match &opts.db_file {
                Some(x) => x.clone(),
                None => {
                    println!("Using env INF_DB_FILE for Sqlite.");
                    env::var("INF_DB_FILE".to_string())?
                }
            };

            // check if db exists
            match fs::exists(&db_file) {
                Ok(_) => (),
                Err(e) => {
                    if !opts.create_database.unwrap_or(false) {
                        return Err(e.into());
                    }
                }
            }

            // connection
            let db: Database = Database::open(&db_file)?;

            if opts.create_database.unwrap_or(false) {
                db.migrate()?;
            }

            // builder
            let mut builder: AssetBuilder<Asset, AssetParts> = Builder::open(&cli.file)?.build(&db)?;

            // only normalize tags on applicable file types
            // if !builder.asset.extension.contains("gif") {
            //     builder.normalize_tags()?;
            // }
            
            let mut real_path: PathBuf = PathBuf::from(&opts.directory);
            real_path.push(builder.asset.path.clone().unwrap());

            // Provision Path
            fs::create_dir_all(&real_path)?;
            
            real_path.push(&builder.asset.name);

            let full_path: String = real_path.display().to_string();
            println!("Renaming {} to {}", cli.file, full_path);

            // Relocate Asset
            fs::rename(cli.file, full_path)?;

            // Update Asset record
            builder.asset.available(&db)?
        }
    }

    Ok(())
}


#[cfg(test)]
mod test {

    #[test]
    fn test_asset() {
        let file = "../testdata/original/horse.jpg";

        let meta = rexiv2::Metadata::new_from_path(&file).unwrap();

        // println!("{:#?}", meta.has_exif());
        // println!("{:#?}", meta.has_iptc());
        // println!("{:#?}", meta.has_xmp());

        // println!("{:#?}", meta.supports_exif());
        // println!("{:#?}", meta.supports_iptc());
        // println!("{:#?}", meta.supports_xmp());

        println!("{:#?}", meta.get_exif_tags());
        // println!("{:#?}", meta.get_iptc_tags());
        // println!("{:#?}", meta.get_xmp_tags());

        // println!("{:#?}", meta.get_media_type());

        let values: Vec<&str> = vec!["TEST DESCRIPTION"];

        // meta.set_tag_multiple_strings("Exif.Image.ImageDescription", &values).unwrap();
        
        // // Important
        // println!("{:#?}", meta.get_tag_multiple_strings("Xmp.dc.subject"));
        // println!("{:#?}", meta.get_tag_multiple_strings("Iptc.Application2.Keywords"));
    
        // // Remove
        // println!("{:#?}", meta.erase_thumbnail());
        // println!("{:#?}", meta.clear_tag("Xmp.MicrosoftPhoto.DateAcquired"));

        println!("{:#?}", meta);
        
        // meta.save_to_file("../testdata/brian_football.jpg").unwrap()
    }
}
