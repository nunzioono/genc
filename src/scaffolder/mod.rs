pub mod scaffolder_builder;
pub mod scaffolder_strategy;

use scaffolder_strategy::ScaffolderStrategy;
use anyhow::Result;

pub struct Scaffolder {
    scaffolder_file_path: std::path::PathBuf,
    project_name: String,
}

impl Scaffolder {
    pub fn new(scaffolder_file_path: std::path::PathBuf, project_name: String) -> Self {
        Scaffolder {
            scaffolder_file_path,
            project_name,
        }
    }

    pub fn run(&self, strategy: &dyn ScaffolderStrategy) -> Result<()> {
        strategy.run(&self.scaffolder_file_path, &self.project_name)
    }
}
