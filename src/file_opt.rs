use color_eyre::{eyre::Context, eyre::ContextCompat, Result};

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub type Data = HashMap<String, String>;

/// Checks the file extensions of a given list of file paths.
///
/// This function takes a reference to a vector of file paths as input, and returns a `Result`
/// with an empty tuple `()` as the `Ok` variant if all file paths have the correct file
/// extensions (`.json` or `.arb`). If any file path has an incorrect extension, it returns
/// an `Err` variant containing an error message.
///
/// # Arguments
///
/// * `file_paths` - A reference to a `Vec<String>` containing file paths to be checked.
///
/// # Errors
///
/// This function will return an error if any file path in `file_paths` does not have a `.json`
/// or `.arb` file extension.
///
/// # Examples
///
/// ```rust
/// use file_opt::check_file_ext;
///
/// let file_paths = vec![
///     String::from("file1.json"),
///     String::from("file2.arb"),
/// ];
/// assert!(check_file_ext(&file_paths).is_ok());
///
/// let invalid_file_paths = vec![
///     String::from("file1.txt"),
///     String::from("file2.arb"),
/// ];
/// assert!(check_file_ext(&invalid_file_paths).is_err());
/// ```
///
pub fn check_file_extension(file_paths: &Vec<String>) -> Result<()> {
    for file_path in file_paths {
        if !file_path.ends_with(".json") && !file_path.ends_with(".arb") {
            return Err(color_eyre::eyre::eyre!(
                "file name must end with .json or .arb"
            ));
        }
    }
    Ok(())
}

/// Check if all `Data` objects in the given vector have the same keys.
///
/// This function takes a vector of `Data` objects and ensures that all objects have the same
/// set of keys. It returns a `Result` that is `Ok(())` if the keys are the same, or an error
/// if the keys differ.
///
/// # Arguments
///
/// * `data` - A vector of `Data` objects to be checked for key consistency.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The keys are not the same for all `Data` objects in the vector.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use my_module::{check_files_equal, Data};
///
/// let data1: Data = [("key1", "value1"), ("key2", "value2")].iter().cloned().collect();
/// let data2: Data = [("key1", "value1"), ("key2", "value2")].iter().cloned().collect();
/// let data_vec: Vec<Data> = vec![data1, data2];
///
/// match check_files_equal(data_vec) {
///     Ok(_) => {
///         println!("All Data objects have the same keys.");
///     }
///     Err(e) => {
///         eprintln!("Error: {}", e);
///     }
/// }
/// ```
pub fn check_files_equal(data: Vec<Data>) -> Result<()> {
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

/// Checks if the given path is a file and if it exists.
///
/// This function takes a generic parameter `P` that implements the `AsRef<Path>` trait, allowing
/// it to accept various types that can be converted into a `Path` reference. The function returns
/// a `Result` with an empty tuple `()` as the `Ok` variant if the given path is a file and exists.
/// If the path does not exist or is not a file, it returns an `Err` variant containing an error
/// message.
///
/// # Arguments
///
/// * `file_path` - A value of type `P` that implements the `AsRef<Path>` trait, representing the
///   file path to be checked.
///
/// # Errors
///
/// This function will return an error if the given `file_path` does not exist or is not a file.
/// It may also return an error if there are file permission related issues.
///
/// # Examples
///
/// ```rust
/// use file_opt::check_files_exist;
/// use std::path::Path;
///
/// let valid_path = Path::new("file1.json");
/// assert!(check_files_exist(valid_path).is_ok());
///
/// let invalid_path = Path::new("non_existent_file.txt");
/// assert!(check_files_exist(invalid_path).is_err());
/// ```
pub fn check_files_exist<P: AsRef<Path>>(file_path: P) -> Result<()> {
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

/// Check if all `Data` objects in the given slice have the same key-value pair count.
///
/// This function takes a slice of `Data` objects and ensures that all objects have the same
/// number of key-value pairs. It returns a `Result` that is `Ok(())` if the key-value pair
/// count is the same, or an error if the count differs or any `Data` object is empty.
///
/// # Arguments
///
/// * `data` - A slice of `Data` objects to be checked for key-value pair count consistency.
///
/// # Errors
///
/// This function will return an error if:
///
/// * Any `Data` object in the slice is empty.
/// * The key-value pair count is not the same for all `Data` objects in the slice.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use my_module::{check_key_length, Data};
///
/// let data1: Data = [("key1", "value1"), ("key2", "value2")].iter().cloned().collect();
/// let data2: Data = [("key1", "value1"), ("key2", "value2")].iter().cloned().collect();
/// let data_slice: &[Data] = &[data1, data2];
///
/// match check_key_length(data_slice) {
///     Ok(_) => {
///         println!("All Data objects have the same key-value pair count.");
///     }
///     Err(e) => {
///         eprintln!("Error: {}", e);
///     }
/// }
/// ```
pub fn check_key_length(data: &[Data]) -> Result<()> {
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

/// Read the JSON data from a file and deserialize it into a `Data` object.
///
/// This function takes a file path as input and attempts to read the JSON
/// contents of the file, returning a `Result` containing the deserialized `Data`
/// object or an error if reading the file or parsing the JSON fails.
///
/// # Arguments
///
/// * `file_path` - The path to the JSON file to be read.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The file cannot be opened (e.g., due to permission issues or the file does not exist).
/// * The contents of the file cannot be deserialized into a `Data` object (e.g., due to malformed JSON).
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
/// use my_module::{read_json, Data};
///
/// let data = read_json(Path::new("path/to/your/file.json"));
///
/// match data {
///     Ok(parsed_data) => {
///         println!("Data: {:?}", parsed_data);
///     }
///     Err(e) => {
///         eprintln!("Error: {}", e);
///     }
/// }
/// ```
pub fn read_json<P: AsRef<Path>>(file_path: P) -> Result<Data> {
    let file = File::open(file_path).wrap_err("could not open file")?;
    let reader = BufReader::new(file);

    let json: Data = serde_json::from_reader(reader).wrap_err("could not read json or arb file")?;

    Ok(json)
}

#[cfg(test)]
#[path = "tests/file_opt_unit_test.rs"]
mod file_opt_tests;
