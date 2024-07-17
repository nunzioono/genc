// cmake_helpers.rs
pub mod cmake_helpers {
    use std::io::Write;
    use std::{fs, process::Command};
    use std::path::Path;
    use anyhow::{Context, Result};

    const FILE_CONTENT: &[u8] = include_bytes!("C:/Users/nunzi/OneDrive/Desktop/genc/scaffolder.cmake");

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
        let mut file = fs::File::create(destination)?;
        file.write_all(FILE_CONTENT)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use mockall::mock;

    // Mock the Command module
    mock! {
        Command {
            pub fn new<S: AsRef<str>>(program: S) -> Self;
            pub fn args<I, S>(self, args: I) -> Self where I: IntoIterator<Item=S>, S: AsRef<str>;
            pub fn status(self) -> std::process::ExitStatus;
        }
    }

    #[test]
    fn test_install_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("scaffolder.cmake");

        assert!(!file_path.exists(), "File should not exist before installation");

        assert!(install_file(&file_path).is_ok());

        assert!(file_path.exists(), "File should be created");

        let mut file_content = Vec::new();
        fs::File::open(&file_path)
            .unwrap()
            .read_to_end(&mut file_content)
            .unwrap();

        assert_eq!(file_content, *FILE_CONTENT, "File content does not match");
    }

    #[test]
    fn test_run_cmake_command_success() {
        let _mock_command = MockCommand::new("cmake");
        _mock_command
            .expect_args()
            .return_once(|_| _mock_command.clone());
        _mock_command
            .expect_status()
            .return_once(|| std::process::ExitStatus::from_raw(0));

        let temp_dir = TempDir::new().unwrap();
        let scaffolder_path = temp_dir.path().join("scaffolder.cmake");
        let project_name = "test_project".to_string();

        assert!(run_cmake_command(&scaffolder_path, project_name).is_ok());
    }

    #[test]
    fn test_run_cmake_command_failure() {
        let _mock_command = MockCommand::new("cmake");
        _mock_command
            .expect_args()
            .return_once(|_| _mock_command.clone());
        _mock_command
            .expect_status()
            .return_once(|| std::process::ExitStatus::from_raw(1));

        let temp_dir = TempDir::new().unwrap();
        let scaffolder_path = temp_dir.path().join("scaffolder.cmake");
        let project_name = "test_project".to_string();

        let result = run_cmake_command(&scaffolder_path, project_name);
        assert!(result.is_err(), "CMake command should fail");
    }
}