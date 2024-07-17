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