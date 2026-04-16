use crate::core::config::Config;
use crate::core::script::Script;
use anyhow::Result;

pub trait Storage {
    fn load_config(&self) -> Result<Config>;
    fn save_config(&self, config: &Config) -> Result<()>;
    fn add_script(&self, script: Script) -> Result<()>;
    fn update_script(&self, name: &str, script: Script) -> Result<()>;
    fn remove_script(&self, name: &str) -> Result<()>;
    fn get_script(&self, name: &str) -> Result<Script>;
    fn list_scripts(&self) -> Result<Vec<Script>>;
}

pub mod fs;
