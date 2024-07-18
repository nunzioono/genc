// env_helpers.rs
pub mod env_helpers {
    use std::{env, path::PathBuf};
    use std::path::Path;
    use anyhow::{Context, Result};

    pub fn get_user_data_dir_path() -> Result<PathBuf> {
        if cfg!(target_os = "windows") {
            env::var("LOCALAPPDATA")
                .or_else(|_| env::var("APPDATA"))
                .map(PathBuf::from)
                .context("Failed to get LOCALAPPDATA or APPDATA environment variable")
        } else if cfg!(target_os = "macos") {
            env::var("HOME")
                .map(|home| Path::new(&home).join("Library/Application Support"))
                .context("Failed to get HOME environment variable")
        } else if cfg!(target_os = "linux") {
            env::var("XDG_DATA_HOME")
                .or_else(|_| env::var("HOME"))
                .map(PathBuf::from)
                .context("Failed to get XDG_DATA_HOME or HOME environment variable")
        } else {
            Err(anyhow::anyhow!("Unsupported operating system"))
        }
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

    pub fn remove_from_path(path: &Path) -> Result<()> {
        let mut current_path = env::var("PATH")
            .context("Failed to read PATH environment variable")?;
        let path_str = path.to_str().context("Failed to convert path to string")?;
        
        // Determine the path separator based on the operating system
        let separator = if cfg!(target_os = "windows") { ";" } else { ":" };
        
        if current_path.contains(path_str) {
            current_path = current_path.split(separator)
                .filter(|p| p != &path_str)
                .collect::<Vec<_>>()
                .join(separator);
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


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_user_data_dir_path() {
            if cfg!(target_os = "windows") {
                // Set a dummy LOCALAPPDATA environment variable
                env::set_var("LOCALAPPDATA", "mock/local/appdata/path");
    
                let result = get_user_data_dir_path().unwrap();
                assert_eq!(result, PathBuf::from("mock/local/appdata/path"));
    
                // Clean up
                env::remove_var("LOCALAPPDATA");
    
                // Test fallback to APPDATA
                env::set_var("APPDATA", "mock/appdata/path");
    
                let result = get_user_data_dir_path().unwrap();
                assert_eq!(result, PathBuf::from("mock/appdata/path"));
    
                // Clean up
                env::remove_var("APPDATA");
    
            } else if cfg!(target_os = "macos") {
                // Set a dummy HOME environment variable
                env::set_var("HOME", "mock/home/path");
    
                let result = get_user_data_dir_path().unwrap();
                assert_eq!(result, Path::new("mock/home/path").join("Library/Application Support"));
    
                // Clean up
                env::remove_var("HOME");
    
            } else if cfg!(target_os = "linux") {
                // Set a dummy XDG_DATA_HOME environment variable
                env::set_var("XDG_DATA_HOME", "mock/xdg/data/home/path");
    
                let result = get_user_data_dir_path().unwrap();
                assert_eq!(result, PathBuf::from("mock/xdg/data/home/path"));
    
                // Clean up
                env::remove_var("XDG_DATA_HOME");
    
                // Test fallback to HOME
                env::set_var("HOME", "mock/home/path");
    
                let result = get_user_data_dir_path().unwrap();
                assert_eq!(result, Path::new("mock/home/path").join(".local/share"));
    
                // Clean up
                env::remove_var("HOME");
            }
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

    }
}

