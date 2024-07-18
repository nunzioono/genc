// cmake_helpers.rs
pub mod cmake_helpers {
    use std::io::Write;
    use std::{fs, process::Command};
    use std::path::Path;
    use anyhow::{Context, Result};

    const FILE_CONTENT: &[u8] = include_bytes!("../../scaffolder.cmake");

    pub fn run_cmake_command(scaffolder_path: &Path, project_name: String) -> Result<()> {
        let status = Command::new("cmake")
            .args(&[
                "-D", &format!("PROJECT_NAME={}", project_name),
                "-P", scaffolder_path.to_str().context("Invalid scaffolder path format")?,
            ])
            .status()
            .with_context(|| format!("Failed to execute cmake command with project name '{}'", project_name))?;
        if status.success() {
            println!("CMake command executed successfully.");
        } else {
            anyhow::bail!("CMake command failed with status: {:?}", status.code());
        }
        Ok(())
    }

    pub fn install_file(destination: &Path) -> Result<()> {
        println!("Creating file at path: {:?}",destination);
        let mut file = fs::File::create(destination)?;
        file.write_all(FILE_CONTENT)?;
        Ok(())
    }

    pub fn remove_file(file_path: &Path) -> Result<()> {
        println!("Removing file at path: {:?}", file_path);
        fs::remove_file(file_path)?;
        Ok(())
    }
    

    #[cfg(test)]
    mod tests {
        use std::io::Read;

        use tempfile::TempDir;

        use super::*;

        #[test]
        fn test_install_file() {
            // Create a temporary directory
            let temp_dir = TempDir::new().unwrap();
            let file_path = temp_dir.path().join("scaffolder.cmake");

            // Ensure the file does not exist
            assert!(!file_path.exists(), "File should not exist before installation");

            // Call install_file
            assert!(install_file(&file_path).is_ok());

            // Verify that the file was created
            assert!(file_path.exists(), "File should be created");

            // Verify the content of the file
            let mut file_content = Vec::new();
            fs::File::open(&file_path)
                .unwrap()
                .read_to_end(&mut file_content)
                .unwrap();

            assert_eq!(file_content, *FILE_CONTENT, "File content does not match");
        }

        #[test]
        fn test_run_cmake_command_success() {
            // Create a temporary directory
            let temp_dir = TempDir::new().unwrap();
            let scaffolder_path = temp_dir.path().join("scaffolder.cmake");

            // Create a dummy scaffolder file
            install_file(&scaffolder_path).unwrap();

            // Define a dummy project name
            let project_name = "test_project".to_string();

            // Execute the CMake command
            let result = run_cmake_command(&scaffolder_path, project_name.clone());

            // Check if the command was executed successfully
            assert!(result.is_ok(), "CMake command should succeed");

            // Optionally, you might want to check the actual output or behavior
            // of the CMake command if running in a real environment.
            // This depends on your specific test environment and setup.
        }

        #[test]
        fn test_run_cmake_command_failure() {
            // Create a temporary directory
            let temp_dir = TempDir::new().unwrap();
            let invalid_path = temp_dir.path().join("invalid.cmake");

            // Define a dummy project name
            let project_name = "test_project".to_string();

            // Execute the CMake command with an invalid file path
            let result = run_cmake_command(&invalid_path, project_name.clone());

            // Check if the command returns an error
            assert!(result.is_err(), "CMake command should fail with an invalid file path");
        }
    }
}
