use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_main_integration() {
    let temp_dir = TempDir::new().unwrap();
    let scaffolder_dir_path = temp_dir.path().join(CMAKE_SCAFFOLDER_DIR);
    let scaffolder_file_path = scaffolder_dir_path.join(SCAFFOLDER_FILE_NAME);

    // Ensure the directory does not exist
    assert!(!scaffolder_dir_path.exists());

    // Call the main function
    assert!(super::main().is_ok());

    // Check that the directory was created
    assert!(scaffolder_dir_path.exists());
    assert!(scaffolder_file_path.exists());
}
