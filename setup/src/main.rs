use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use anyhow::Context;
use anyhow::Result;

const SCAFFOLDER_FILE_NAME: &str = "scaffolder.cmake";
const CMAKE_SCAFFOLDER_DIR: &str = "CMakeScaffolder";

fn main() -> Result<()> {

    let root_programs_path: PathBuf = env::var("PROGRAMFILES")
    .context("Unable to retrieve PROGRAMFILES environment variable.")?
    .into(); // Convert String to PathBuf
    let cmake_scaffolder_path = root_programs_path.join(CMAKE_SCAFFOLDER_DIR);
    let scaffolder_file_path = cmake_scaffolder_path.join(SCAFFOLDER_FILE_NAME);
    let source_file_path = Path::new(SCAFFOLDER_FILE_NAME);

    // Check if the CMakeScaffolder folder exists
    if !cmake_scaffolder_path.exists() {
        // Create the folder if it does not exist
        fs::create_dir_all(&cmake_scaffolder_path)
            .with_context(|| format!("Failed to create directory {:?}", cmake_scaffolder_path))?;
    }

    // Check if the scaffolder.cmake file exists in the folder
    if !scaffolder_file_path.clone().exists() {
        // Copy the scaffolder.cmake file into the folder if it does not exist
        fs::copy(source_file_path, scaffolder_file_path.clone())
            .with_context(|| format!("Failed to copy file from {:?} to {:?}", source_file_path, scaffolder_file_path))?;
    }

    // Get the current PATH environment variable
    let mut path = env::var("PATH")
        .with_context(|| "Failed to read PATH environment variable")?;

    // Add the new directory to the PATH if it's not already included
    let cmake_scaffolder_path_str = cmake_scaffolder_path.to_str().context("Failed to convert path to string")?;
    if !path.contains(cmake_scaffolder_path_str) {
        path.push_str(&format!(";{}", cmake_scaffolder_path_str));
        env::set_var("PATH", path);
    }

    

    // Define the path to the scaffolder file
    let scaffolder_path = Path::new(&root_programs_path).join("CMakeScaffolder").join("scaffolder.cmake");

    // Check if the scaffolder file exists
    if !scaffolder_path.exists() {
        anyhow::bail!("scaffolder.cmake file does not exist in the CMakeScaffolder directory.");
    }

    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a project name was provided
    if args.len() < 2 {
        anyhow::bail!("Usage: {} <PROJECT_NAME>", args[0]);
    }

    // Extract the project name from the arguments
    let project_name = &args[1];

    // Construct the cmake command
    let status = Command::new("cmake")
        .args(&[
            "-D", &format!("PROJECT_NAME={}", project_name),
            "-P", scaffolder_path.to_str().context("Invalid scaffolder path format")?,
        ])
        .status()
        .with_context(|| format!("Failed to execute cmake command with project name '{}'", project_name))?;

    // Check if the command was successful
    if status.success() {
        println!("CMake command executed successfully.");
    } else {
        anyhow::bail!("CMake command failed with status: {:?}", status.code());
    }

    Ok(())
}
