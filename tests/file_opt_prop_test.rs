use proptest::collection::{hash_map, vec};
use proptest::prelude::*;

use std::fs::File;
use std::io::Write;

mod file_opt {
    include!("../src/file_opt.rs");
}

// TODO: diffrerent_size_maps() is not working
fn different_size_maps() -> impl Strategy<Value = Vec<file_opt::Data>> {
    vec(
        (1usize..10).prop_flat_map(|len| {
            vec(
                (
                    prop::string::string_regex(&format!("[A-Za-z0-9_]{{{}}}", len)).unwrap(),
                    "[A-Za-z0-9_]{1,8}".prop_map(String::from),
                ),
                1..10,
            )
            .prop_map(|vec| vec.into_iter().collect())
        }),
        1..10,
    )
}

fn same_size_maps() -> impl Strategy<Value = Vec<file_opt::Data>> {
    vec("[A-Za-z0-9_]{1,8}".prop_map(String::from), 1..50).prop_flat_map(|v| {
        let len = v.len();
        vec(
            hash_map(
                "[A-Za-z0-9_]{1,8}".prop_map(String::from),
                "[A-Za-z0-9_]{1,8}".prop_map(String::from),
                len..=len,
            ),
            1..50,
        )
    })
}

proptest! {
    /// check_file_extension prop tests
    #[test]
    fn test_check_file_extension_correct(ref ext in prop::sample::select(vec![".json", ".arb"])) {
        let file_path = format!("/tmp/test{}", ext);
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "writing to file").unwrap();

        // Clean up
        std::fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn test_check_file_extension_incorrect(ref ext in "-[A-Za-z0-9_]{1,3}") {
        let file_path = format!("/tmp/test.{}", ext);
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "writing to file").unwrap();

        // File has incorrect extension
        assert!(file_opt::check_file_extension(&vec![file_path.clone()]).is_err());

        // Clean up
        std::fs::remove_file(&file_path).unwrap();
    }
    ///

    /// check_files_exist prop tests
    #[test]
    fn test_check_files_exist(ref path in "[A-Za-z0-9_]{1,8}") {
        let file_path = format!("/tmp/{}", path);
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "writing to file").unwrap();

        // Path exists and is a file
        assert!(file_opt::check_files_exist(&file_path).is_ok());

        // Clean up
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_check_files_exist_file_not_exist(ref path in "-[A-Za-z0-9_]{1,8}") {
        let file_path = format!("/tmp/{}", path);

        // Path does not exist
        assert!(file_opt::check_files_exist(file_path).is_err());
    }
    ///

    /// check_files_equal prop tests
    #[test]
    fn test_check_files_equal_same_keys(ref keys in prop::collection::hash_set("[a-z]{1,5}", 1..5), ref vals in "[a-z]{1,5}")  {
        let data: Vec<file_opt::Data> = vec![
            keys.iter().map(|key| (key.clone(), vals.clone())).collect(),
        ];

        // All data items have the same keys
        assert!(file_opt::check_files_equal(data).is_ok());
    }

    #[test]
    fn test_check_files_equal_different_keys(ref keys1 in prop::collection::hash_set("[a-z]{1,5}", 1..5), ref keys2 in prop::collection::hash_set("[a-z]{1,5}", 1..5), ref val in "[a-z]{1,5}") {
        let mut keys2: Vec<String> = keys2.iter().cloned().collect();
        keys2.push("unique_key".to_string());  // Add a unique key to the second set

        let data: Vec<file_opt::Data> = vec![
            keys1.iter().map(|key| (key.clone(), val.clone())).collect(),
            keys2.iter().map(|key| (key.clone(), val.clone())).collect()
        ];

        // At least one data item has different keys
        assert!(file_opt::check_files_equal(data).is_err());
    }
    ///

    /// check_key_length prop tests
    #[test]
    fn test_check_key_length_same_length(data in same_size_maps()) {
        // All HashMaps have the same number of key-value pairs, so the function should return Ok(())
        assert!(file_opt::check_key_length(&data).is_ok());
    }

    #[test]
    fn test_check_key_length_empty(ref data in Just(Vec::<file_opt::Data>::new())) {
        // The slice is empty, so the function should return Err
        assert!(file_opt::check_key_length(data).is_err());
    }

    #[test]
    #[ignore = "could not test due to cannot generate different size map"]
    // TODO: could not test due to cannot generate different size map
    fn test_check_key_length_different_length(data in different_size_maps()) {
        assert!(file_opt::check_key_length(&data).is_err());
    }
}
