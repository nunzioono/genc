mod env_helpers;
mod file_helpers;
mod cmake_helpers;

use anyhow::Result;

use cmake_helpers::cmake_helpers::{run_cmake_command, install_file};
use env_helpers::env_helpers::{add_to_path, get_project_name, get_root_programs_path};
use file_helpers::file_helpers::create_folder;

pub const SCAFFOLDER_FILE_NAME: &str = "scaffolder.cmake";
pub const CMAKE_SCAFFOLDER_DIR: &str = "CMakeScaffolder";

pub fn main() -> Result<()> {
    let root_programs_path = get_root_programs_path()?;
    let scaffolder_dir_path = root_programs_path.join(CMAKE_SCAFFOLDER_DIR);
    let scaffolder_file_path = scaffolder_dir_path.join(SCAFFOLDER_FILE_NAME);

    if !scaffolder_dir_path.exists() {
        create_folder(&scaffolder_dir_path, CMAKE_SCAFFOLDER_DIR)?;
    }

    if !scaffolder_file_path.exists() {
        install_file(&scaffolder_file_path)?;
    }

    add_to_path(&scaffolder_dir_path)?;

    let project_name = get_project_name()?;
    run_cmake_command(&scaffolder_file_path, project_name)?;

    Ok(())
}
