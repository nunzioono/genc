use std::fs::remove_file;

use crate::{ 
    utils::{
        cmake_helpers::cmake_helpers::install_file,
        env_helpers::env_helpers::{
            add_to_path,
            get_project_name,
            get_user_data_dir_path, remove_from_path
        }, file_helpers::file_helpers::{create_folder, remove_folder}
    },
    CMAKE_SCAFFOLDER_DIR,
    SCAFFOLDER_FILE_NAME,
    scaffolder::Scaffolder
};

use anyhow::{anyhow, Result};

pub struct ScaffolderBuilder {
    user_data_dir_path: Option<std::path::PathBuf>,
    scaffolder_dir_path: Option<std::path::PathBuf>,
    scaffolder_file_path: Option<std::path::PathBuf>,
}

impl ScaffolderBuilder {
    pub fn new() -> Self {
        ScaffolderBuilder {
            user_data_dir_path: None,
            scaffolder_dir_path: None,
            scaffolder_file_path: None,
        }
    }

    fn with_user_data_dir_path(mut self) -> Result<Self> {
        self.user_data_dir_path = Some(get_user_data_dir_path().expect("Unable to retrieve programs default dir"));
        println!("Found root programs path: {:?}",self.user_data_dir_path);
        Ok(self)
    }

    fn with_scaffolder_dir_path(mut self) -> Result<Self> {
        if let Some(ref user_data_dir_path) = self.user_data_dir_path {
            self.scaffolder_dir_path = Some(user_data_dir_path.join(CMAKE_SCAFFOLDER_DIR));
            println!("Found scaffolder dir path: {:?}",self.scaffolder_dir_path);
            return Ok(self);
        } else {
            return Err(anyhow!("Unable to retrieve a valid path for GenC dir"));
        }
    }

    fn with_scaffolder_file_path(mut self) -> Result<Self> {
        if let Some(ref scaffolder_dir_path) = self.scaffolder_dir_path {
            self.scaffolder_file_path = Some(scaffolder_dir_path.join(SCAFFOLDER_FILE_NAME));
            println!("Found scaffolder file path: {:?}",self.scaffolder_file_path);
            return Ok(self);
        } else {
            return Err(anyhow!("Unable to retrieve a valid path for GenC cmake file"));
        }
    }

    fn create_scaffolder_dir_if_not_exists(self) -> Result<Self> {
        if let Some(ref scaffolder_dir_path) = self.scaffolder_dir_path {
            if !scaffolder_dir_path.exists() {
                create_folder(self.user_data_dir_path.clone().unwrap().as_path(), CMAKE_SCAFFOLDER_DIR)?;
            } else {
                println!("Scaffolder dir already exists");
            }
            return Ok(self);
        } else {
            return Err(anyhow!("Unable to retrieve a valid dir path"));
        }
    }

    fn remove_scaffolder_dir_if_exists(self) -> Result<Self> {
        if let Some(ref scaffolder_dir_path) = self.scaffolder_dir_path {
            if scaffolder_dir_path.exists() {
                remove_folder(self.user_data_dir_path.clone().unwrap().as_path(), CMAKE_SCAFFOLDER_DIR)?;
            } else {
                println!("Scaffolder dir do not exists");
            }
            return Ok(self);
        } else {
            return Err(anyhow!("Unable to retrieve a valid dir path"));
        }
    }

    fn install_scaffolder_file_if_not_exists(self) -> Result<Self> {
        if let Some(ref scaffolder_file_path) = self.scaffolder_file_path {
            if !scaffolder_file_path.exists() {
                install_file(scaffolder_file_path)?;
            }
        }
        Ok(self)
    }

    fn remove_scaffolder_file_if_exists(self) -> Result<Self> {
        if let Some(ref scaffolder_file_path) = self.scaffolder_file_path {
            if scaffolder_file_path.exists() {
                remove_file(scaffolder_file_path)?;
            }
        }
        Ok(self)
    }

    fn add_scaffolder_dir_to_path(self) -> Result<Self> {
        if let Some(ref scaffolder_dir_path) = self.scaffolder_dir_path {
            add_to_path(scaffolder_dir_path)?;
        }
        Ok(self)
    }

    fn remove_scaffolder_dir_from_path(self) -> Result<Self> {
        if let Some(ref scaffolder_dir_path) = self.scaffolder_dir_path {
            remove_from_path(scaffolder_dir_path)?;
        }
        Ok(self)
    }

    pub fn build(self) -> Result<Scaffolder> {
        let builder = self
        .with_user_data_dir_path()?
        .with_scaffolder_dir_path()?
        .with_scaffolder_file_path()?
        .create_scaffolder_dir_if_not_exists()?
        .install_scaffolder_file_if_not_exists()?
        .add_scaffolder_dir_to_path()?;
        
        let project_name = get_project_name()?;

        if let Some(scaffolder_file_path) = builder.scaffolder_file_path {
            Ok(Scaffolder::new(scaffolder_file_path, project_name))
        } else {
            Err(anyhow::anyhow!("Scaffolder file path is not set"))
        }
    }

    pub fn build_from_scratch(self) -> Result<Scaffolder> {
        let builder = self
        .with_user_data_dir_path()?
        .with_scaffolder_dir_path()?
        .with_scaffolder_file_path()?
        .remove_scaffolder_dir_if_exists()?
        .remove_scaffolder_file_if_exists()?
        .remove_scaffolder_dir_from_path()?
        .create_scaffolder_dir_if_not_exists()?
        .install_scaffolder_file_if_not_exists()?
        .add_scaffolder_dir_to_path()?;
        
        let project_name = get_project_name()?;

        if let Some(scaffolder_file_path) = builder.scaffolder_file_path {
            Ok(Scaffolder::new(scaffolder_file_path, project_name))
        } else {
            Err(anyhow::anyhow!("Scaffolder file path is not set"))
        }
    }
}
