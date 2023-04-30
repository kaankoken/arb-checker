#[cfg(test)]
mod tests {
    use crate::file_opt::*;

    /// Unit test of `check_file_extension` function.
    #[test]
    fn test_check_file_extension_valid() {
        let file_paths = vec![
            String::from("file1.json"),
            String::from("file2.arb"),
            String::from("file3.json"),
        ];
        assert!(check_file_extension(&file_paths).is_ok());
    }

    #[test]
    fn test_check_file_extension_invalid() {
        let file_paths = vec![
            String::from("file1.json"),
            String::from("file2.arb"),
            String::from("file3.txt"),
        ];
        assert!(check_file_extension(&file_paths).is_err());
    }
    ///

    /// Unit test of `check_files_exist` function.
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_check_files_exist_file_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("existing_file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Test content").unwrap();

        assert!(check_files_exist(&file_path).is_ok());
    }

    #[test]
    fn test_check_files_exist_file_does_not_exist() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("non_existent_file.txt");

        assert!(check_files_exist(file_path).is_err());
    }

    #[test]
    fn test_check_files_exist_path_is_directory() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path().to_path_buf();

        assert!(check_files_exist(dir_path).is_err());
    }
    ///

    /// Unit test of `read_json` function.
    #[test]
    fn test_read_json_valid() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("valid.json");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, r#"{{ "key": "value" }}"#).unwrap();

        let result = read_json(&file_path);

        let mut expected_data = HashMap::new();
        expected_data.insert("key".to_string(), "value".to_string());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_data);
    }

    #[test]
    fn test_read_json_invalid() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("invalid.json");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, r#"{{ "invalid": 123 }}"#).unwrap();

        let result = read_json(&file_path);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("could not read json or arb file"));
    }

    #[test]
    fn test_read_json_nonexistent() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("nonexistent.json");

        let result = read_json(file_path);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("could not open file"));
    }
    ///

    /// Unit test of key-value pair count check.
    #[test]
    fn test_check_key_length_same_count() {
        let data1: Data = [
            (String::from("key1"), String::from("value1")),
            (String::from("key2"), String::from("value2")),
        ]
        .iter()
        .cloned()
        .collect();
        let data2: Data = [
            (String::from("key1"), String::from("value1")),
            (String::from("key2"), String::from("value2")),
        ]
        .iter()
        .cloned()
        .collect();
        let data_slice: &[Data] = &[data1, data2];

        let result = check_key_length(data_slice);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_key_length_different_count() {
        let data1: Data = [
            (String::from("key1"), String::from("value1")),
            (String::from("key2"), String::from("value2")),
        ]
        .iter()
        .cloned()
        .collect();
        let data2: Data = [(String::from("key1"), String::from("value1"))]
            .iter()
            .cloned()
            .collect();
        let data_slice: &[Data] = &[data1, data2];

        let result = check_key_length(data_slice);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("files does not have the same key-value pair"));
    }

    #[test]
    fn test_check_key_length_empty_data() {
        let data1: Data = HashMap::new();
        let data2: Data = [
            (String::from("key1"), String::from("value1")),
            (String::from("key2"), String::from("value2")),
        ]
        .iter()
        .cloned()
        .collect();
        let data_slice: &[Data] = &[data1, data2];

        let result = check_key_length(data_slice);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("file is empty"));
    }
    ///

    /// Unit test of `check_files_equal` function.
    #[test]
    fn test_check_files_equal_same_keys() {
        let data1: Data = [
            (String::from("key1"), String::from("value1")),
            (String::from("key2"), String::from("value2")),
        ]
        .iter()
        .cloned()
        .collect();
        let data2: Data = [
            (String::from("key1"), String::from("value1")),
            (String::from("key2"), String::from("value2")),
        ]
        .iter()
        .cloned()
        .collect();
        let data_vec: Vec<Data> = vec![data1, data2];

        let result = check_files_equal(data_vec);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_files_equal_different_keys() {
        let data1: Data = [
            (String::from("key1"), String::from("value1")),
            (String::from("key2"), String::from("value2")),
        ]
        .iter()
        .cloned()
        .collect();
        let data2: Data = [
            (String::from("key4"), String::from("value1")),
            (String::from("key3"), String::from("value2")),
        ]
        .iter()
        .cloned()
        .collect();
        let data_vec: Vec<Data> = vec![data1, data2];

        let result = check_files_equal(data_vec);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("files does not have the same keys"));
    }
}
