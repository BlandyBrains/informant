use std::env;
use std::io::{self};
use std::slice::Iter;

use meta::Meta;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let iter: Iter<'_, String> = args.iter();

    let file_path: &str = match iter.clone().last() {
        Some(f) => {
            f
        },
        None => {
            println!("no file -- ");
            return Ok(());
        }
    };
    match Meta::capture(file_path) {
        Ok(m) => {
            println!("{:#?}", m);
            println!("{:#?}", serde_json::to_string(&m)?);
        },
        Err(e) => {
            println!("error {:#?}", e);
        }
    }
    Ok(())
}