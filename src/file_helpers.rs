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