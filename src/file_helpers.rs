// file_helpers.rs
pub mod file_helpers {
    use std::fs;
    use std::path::{Path, PathBuf};
    use anyhow::{Context, Result};

    pub fn create_folder(base_path: &Path, folder_name: &str) -> Result<PathBuf> {
        let new_folder_path = base_path.join(folder_name);
        if !new_folder_path.exists() {
            fs::create_dir_all(&new_folder_path)
                .with_context(|| format!("Failed to create directory {:?}", new_folder_path))?;
        }
        Ok(new_folder_path)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_folder_success() {
        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let folder_name = "test_folder";

        // Ensure the folder does not exist
        let folder_path = temp_dir.path().join(folder_name);
        assert!(!folder_path.exists(), "Folder should not exist before creation");

        // Call create_folder
        let result = create_folder(temp_dir.path(), folder_name).unwrap();
        
        // Verify that the folder was created
        assert_eq!(result, folder_path);
        assert!(folder_path.exists(), "Folder should be created");
    }

    #[test]
    fn test_create_folder_existing_folder() {
        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let folder_name = "test_folder";

        // Create the folder initially
        create_folder(temp_dir.path(), folder_name).unwrap();
        
        // Ensure the folder exists
        let folder_path = temp_dir.path().join(folder_name);
        assert!(folder_path.exists(), "Folder should exist after initial creation");

        // Call create_folder again
        let result = create_folder(temp_dir.path(), folder_name).unwrap();

        // Verify that the existing folder is not modified
        assert_eq!(result, folder_path);
    }

    #[test]
    fn test_create_folder_error_handling() {
        use std::io::ErrorKind;
        use std::fs::File;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let folder_name = "test_folder";

        // Create a file in the temporary directory to simulate an error
        let file_path = temp_dir.path().join(folder_name);
        File::create(&file_path).unwrap();

        // Call create_folder and expect an error
        let result = create_folder(temp_dir.path(), folder_name);
        
        assert!(result.is_err(), "create_folder should return an error if a file exists with the same name as the folder");
    }
}