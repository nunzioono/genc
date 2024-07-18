use crate::utils::cmake_helpers::cmake_helpers::run_cmake_command;
use anyhow::Result;

pub trait ScaffolderStrategy {
    fn run(&self, scaffolder_file_path: &std::path::Path, project_name: &str) -> Result<()>;
}

pub struct NewStrategy;

impl ScaffolderStrategy for NewStrategy {
    fn run(&self, scaffolder_file_path: &std::path::Path, project_name: &str) -> Result<()> {
        run_cmake_command(scaffolder_file_path, project_name.to_owned())
    }
}
