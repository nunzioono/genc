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