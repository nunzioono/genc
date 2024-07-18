pub mod scaffolder;
pub mod utils;

use anyhow::Result;
use scaffolder::{scaffolder_builder::ScaffolderBuilder, scaffolder_strategy::NewStrategy};

pub const SCAFFOLDER_FILE_NAME: &str = "scaffolder.cmake";
pub const CMAKE_SCAFFOLDER_DIR: &str = "GenC";

fn main() -> Result<()> {
    let scaffolder = ScaffolderBuilder::new()
        .build()?;

    let strategy = NewStrategy;  // Instantiate your strategy
    scaffolder.run(&strategy)
}