mod file_opt;

use clap::Parser;
use color_eyre::Result;

use file_opt::*;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// List JSON or arb files keys to checked
    #[arg(short, long, value_delimiter = ' ', required = true)]
    file: Vec<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    if cli.file.len() < 2 {
        return Err(color_eyre::eyre::eyre!("provide at least two files"));
    }

    check_file_extension(&cli.file)?;

    for file in &cli.file {
        check_files_exist(file)?;
    }

    let mut file_vec: Vec<Data> = vec![];
    for file in &cli.file {
        let res = read_json(file)?;
        file_vec.push(res);
    }

    check_key_length(&file_vec)?;
    check_files_equal(file_vec)?;

    Ok(())
}
