use std::error::Error;
use std::path::PathBuf;

use clap::ArgMatches;
use comic_sort::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let matches: ArgMatches = get_matches()?;
    let config_file: &str = matches.value_of("config").unwrap();

    let file: PathBuf = read(PathBuf::from(config_file))?;

    let mut config: Config = Config::load(file)?;

    let _ = config.get_files().expect("Couldn't read the download folder");

    for mapping in &mut config.mappings {
        let _ = mapping.make_patterns();
    }

    for file in &config.files {
        &config.process(file);
    };

    Ok(())
}
