// env_helpers.rs
pub mod env_helpers {
    use std::{env, path::PathBuf};
    use std::path::Path;
    use anyhow::{Context, Result};

    pub fn get_root_programs_path() -> Result<PathBuf> {
        Ok(env::var("PROGRAMFILES")
            .context("Unable to retrieve PROGRAMFILES environment variable.")?
            .into())
    }

    pub fn add_to_path(path: &Path) -> Result<()> {
        let mut current_path = env::var("PATH")
            .context("Failed to read PATH environment variable")?;
        let path_str = path.to_str().context("Failed to convert path to string")?;
        if !current_path.contains(path_str) {
            current_path.push_str(&format!(";{}", path_str));
            env::set_var("PATH", current_path);
        }
        Ok(())
    }

    pub fn get_project_name() -> Result<String> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            anyhow::bail!("Usage: {} <PROJECT_NAME>", args[0]);
        }
        Ok(args[1].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_root_programs_path() {
        // Set a dummy PROGRAMFILES environment variable
        env::set_var("PROGRAMFILES", "mock/program/files/path");

        let result = get_root_programs_path().unwrap();
        assert_eq!(result, PathBuf::from("mock/program/files/path"));

        // Clean up
        env::remove_var("PROGRAMFILES");
    }

    #[test]
    fn test_add_to_path() {
        let temp_path = PathBuf::from("mock/path/to/add");
        let original_path = env::var("PATH").unwrap();

        // Ensure the path is not in PATH
        assert!(!env::var("PATH").unwrap().contains(temp_path.to_str().unwrap()));

        // Call add_to_path
        assert!(add_to_path(&temp_path).is_ok());

        // Verify the path was added
        let updated_path = env::var("PATH").unwrap();
        assert!(updated_path.contains(temp_path.to_str().unwrap()));

        // Restore original PATH
        env::set_var("PATH", original_path);
    }

    #[test]
    fn test_get_project_name_success() {
        // Simulate command-line arguments
        let args: Vec<String> = vec!["program".to_string(), "test_project".to_string()];
        let original_args = std::env::args().collect::<Vec<String>>();
        std::env::set_args(args);

        assert_eq!(get_project_name().unwrap(), "test_project");

        // Restore original arguments
        std::env::set_args(original_args);
    }

    #[test]
    fn test_get_project_name_failure() {
        // Simulate command-line arguments with no project name
        let args: Vec<String> = vec!["program".to_string()];
        let original_args = std::env::args().collect::<Vec<String>>();
        std::env::set_args(args);

        let result = get_project_name();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Usage: program <PROJECT_NAME>");

        // Restore original arguments
        std::env::set_args(original_args);
    }
}