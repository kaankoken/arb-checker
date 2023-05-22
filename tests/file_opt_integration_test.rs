mod file_opt {
    include!("../src/file_opt.rs");
}

mod file_opt_integration_test {
    use crate::file_opt::*;

    use color_eyre::Result;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test1() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_check_extension_and_files_exist() -> Result<()> {
        let dir = tempdir().unwrap();
        let file_path1 = dir.path().join("test1.json");
        let file_path2 = dir.path().join("test2.arb");

        let files = vec![
            file_path1.as_path().to_str().unwrap().to_string(),
            file_path2.as_path().to_str().unwrap().to_string(),
        ];

        let _ = File::create(&file_path1).unwrap();
        let _ = File::create(&file_path2).unwrap();

        assert!(check_file_extension(&files).is_ok());

        for file in files {
            assert!(check_files_exist(&file).is_ok());
        }

        Ok(())
    }

    #[test]
    fn test_check_extension_and_files_not_exist() -> Result<()> {
        let dir = tempdir().unwrap();
        let file_path1 = dir.path().join("test1.json");
        let file_path2 = dir.path().join("test2.arb");

        let files = vec![
            file_path1.as_path().to_str().unwrap().to_string(),
            file_path2.as_path().to_str().unwrap().to_string(),
        ];

        assert!(check_file_extension(&files).is_ok());

        for file in files {
            assert!(check_files_exist(&file).is_err());
        }

        Ok(())
    }

    #[test]
    fn test_check_files_same_key_length() -> Result<()> {
        let dir = tempdir().unwrap();
        let file_path1 = dir.path().join("test1.json");
        let file_path2 = dir.path().join("test2.arb");

        let files = vec![
            file_path1.as_path().to_str().unwrap().to_string(),
            file_path2.as_path().to_str().unwrap().to_string(),
        ];

        let mut file1 = File::create(&file_path1).unwrap();
        writeln!(file1, r#"{{"key1": "value1", "key2": "value2"}}"#).unwrap();
        let mut file2 = File::create(&file_path2).unwrap();
        writeln!(file2, r#"{{"key1": "value1", "key2": "value2"}}"#).unwrap();

        assert!(check_file_extension(&files).is_ok());

        for file in &files {
            assert!(check_files_exist(file).is_ok());
        }

        // Skipping the read file part because it's already tested in the unit test.

        let mut file_vec: Vec<Data> = vec![];
        for file in &files {
            let res = read_json(file)?;
            file_vec.push(res);
        }

        assert!(check_key_length(&file_vec).is_ok());

        Ok(())
    }

    #[test]
    fn test_check_files_different_key_length() -> Result<()> {
        let dir = tempdir().unwrap();
        let file_path1 = dir.path().join("test1.json");
        let file_path2 = dir.path().join("test2.arb");

        let files = vec![
            file_path1.as_path().to_str().unwrap().to_string(),
            file_path2.as_path().to_str().unwrap().to_string(),
        ];

        let mut file1 = File::create(&file_path1).unwrap();
        writeln!(file1, r#"{{"key1": "value1", "key2": "value2"}}"#).unwrap();
        let mut file2 = File::create(&file_path2).unwrap();
        writeln!(file2, r#"{{"key1": "value1"}}"#).unwrap();

        assert!(check_file_extension(&files).is_ok());

        for file in &files {
            assert!(check_files_exist(file).is_ok());
        }

        // Skipping the read file part because it's already tested in the unit test.

        let mut file_vec: Vec<Data> = vec![];
        for file in &files {
            let res = read_json(file)?;
            file_vec.push(res);
        }

        assert!(check_key_length(&file_vec).is_err());

        Ok(())
    }

    #[test]
    fn test_correct_app_flow() -> Result<()> {
        let dir = tempdir().unwrap();
        let file_path1 = dir.path().join("test1.json");
        let file_path2 = dir.path().join("test2.arb");

        let files = vec![
            file_path1.as_path().to_str().unwrap().to_string(),
            file_path2.as_path().to_str().unwrap().to_string(),
        ];

        let mut file1 = File::create(&file_path1).unwrap();
        writeln!(file1, r#"{{"key1": "value1", "key2": "value2"}}"#).unwrap();
        let mut file2 = File::create(&file_path2).unwrap();
        writeln!(file2, r#"{{"key1": "value1", "key2": "value2"}}"#).unwrap();

        assert!(check_file_extension(&files).is_ok());

        for file in &files {
            assert!(check_files_exist(file).is_ok());
        }

        // Skipping the read file part because it's already tested in the unit test.

        let mut file_vec: Vec<Data> = vec![];
        for file in &files {
            let res = read_json(file)?;
            file_vec.push(res);
        }

        assert!(check_key_length(&file_vec).is_ok());
        assert!(check_files_equal(file_vec).is_ok());
        Ok(())
    }

    #[test]
    fn test_wrong_app_flow() -> Result<()> {
        let dir = tempdir().unwrap();
        let file_path1 = dir.path().join("test1.json");
        let file_path2 = dir.path().join("test2.arb");

        let files = vec![
            file_path1.as_path().to_str().unwrap().to_string(),
            file_path2.as_path().to_str().unwrap().to_string(),
        ];

        let mut file1 = File::create(&file_path1).unwrap();
        writeln!(file1, r#"{{"key1": "value1", "key2": "value2"}}"#).unwrap();
        let mut file2 = File::create(&file_path2).unwrap();
        writeln!(file2, r#"{{"key3": "value1", "key2": "value2"}}"#).unwrap();

        assert!(check_file_extension(&files).is_ok());

        for file in &files {
            assert!(check_files_exist(file).is_ok());
        }

        // Skipping the read file part because it's already tested in the unit test.

        let mut file_vec: Vec<Data> = vec![];
        for file in &files {
            let res = read_json(file)?;
            file_vec.push(res);
        }

        assert!(check_key_length(&file_vec).is_ok());
        assert!(check_files_equal(file_vec).is_err());
        Ok(())
    }
}
