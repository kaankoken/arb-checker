use clap::Parser;
use color_eyre::{
    eyre::{Context, ContextCompat},
    Result,
};
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// List JSON or arb files keys to checked
    #[arg(short, long, value_delimiter = ' ', required = true)]
    file: Vec<String>,
}

type Data = HashMap<String, String>;

fn check_file_ext(file_paths: &Vec<String>) -> Result<()> {
    for file_path in file_paths {
        if !file_path.ends_with(".json") && !file_path.ends_with(".arb") {
            return Err(color_eyre::eyre::eyre!(
                "file name must end with .json or .arb"
            ));
        }
    }
    Ok(())
}

fn check_files_exist<P: AsRef<Path>>(file_path: P) -> Result<()> {
    let is_exist = file_path
        .as_ref()
        .try_exists()
        .wrap_err("file permission related issue")?;

    let is_file = file_path.as_ref().is_file();

    if !is_exist {
        return Err(color_eyre::eyre::eyre!("file does not exist"));
    }

    if !is_file {
        return Err(color_eyre::eyre::eyre!("given path is not a file"));
    }

    Ok(())
}

fn check_key_length(data: &[Data]) -> Result<()> {
    let mut key_lengths = data.iter().map(|d| d.len()).collect::<Vec<usize>>();
    key_lengths.sort();

    let first = key_lengths.first().wrap_err("no first item")?;
    let last = key_lengths.last().wrap_err("no last item")?;
    let zero = usize::try_from(0)?;

    if first == &zero || last == &zero {
        return Err(color_eyre::eyre::eyre!("file is empty"));
    }

    if first != last {
        return Err(color_eyre::eyre::eyre!(
            "files does not have the same key-value pair"
        ));
    }

    Ok(())
}

// TODO: structure error messages!
fn read_json<P: AsRef<Path>>(file_path: P) -> Result<Data> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let json: Data = serde_json::from_reader(reader)?;
    Ok(json)
}

fn check_files_equal(data: Vec<Data>) -> Result<()> {
    let mut key_iter = data
        .iter()
        .map(|d| {
            let mut keys = d.keys().collect::<Vec<_>>();
            keys.sort();

            keys
        })
        .collect::<Vec<_>>()
        .into_iter();

    let first = key_iter.next().wrap_err("could not get first item")?;

    for key in key_iter {
        if first.ne(&key) {
            return Err(color_eyre::eyre::eyre!("files does not have the same keys"));
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    if cli.file.len() < 2 {
        return Err(color_eyre::eyre::eyre!("provide at least two files"));
    }

    check_file_ext(&cli.file)?;

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
