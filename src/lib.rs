use std::{collections::BTreeMap, fs, path::PathBuf};

use anyhow::Result;
use block_model::RawBlockModel;

pub mod block_model;
pub mod block_state;
pub mod math;

pub struct ModelManager {
    models: BTreeMap<String, RawBlockModel>
}

impl ModelManager {
    pub fn from_dir(models_dir: PathBuf) -> Result<Self> {
        let mut models = BTreeMap::new();

        let mut blocks = models_dir.clone();
        blocks.push("block");
        for dir in fs::read_dir(blocks)? {
            let dir = dir?;
            let model_path = dir.path();
            let file_name = model_path.file_stem().unwrap().to_string_lossy();
            let model: RawBlockModel = serde_json::from_str(&fs::read_to_string(model_path.clone())?)?;
            models.insert(format!("minecraft:block/{}", file_name), model);
        }

        Ok(Self {
            models
        })
    }

    pub fn get_models(&self) -> &BTreeMap<String, RawBlockModel> {
        &self.models
    }

    pub fn output(&self) -> Result<()> {

        Ok(())
    }
}