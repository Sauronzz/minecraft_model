use std::{collections::BTreeMap, fs, path::PathBuf};

use anyhow::Result;
use block_model::RawBlockModel;
use model_merge::ModelMerge;

pub mod block_model;
pub mod block_state;
pub mod math;

pub struct ModelManager {
    models: BTreeMap<String, RawBlockModel>
}

impl ModelManager {

    fn load_complete_model(models: &mut BTreeMap<String, RawBlockModel>, model_name: &str, models_dir: &PathBuf) -> Result<RawBlockModel> {
        let mut model_path = models_dir.clone();
        model_path.push(format!("{}.json", model_name));
        // println!("try load {:?}", model_path);

        let mut incomplete_model: RawBlockModel = serde_json::from_str(&fs::read_to_string(model_path)?)?;
        
        if let Some(parent) = incomplete_model.parent.clone() {
            if let Some(parent_model) = models.get(&parent) {
                incomplete_model.merge(parent_model);
            } else {
                let parent_name = parent.split('/').last().unwrap();
                let parent_model = ModelManager::load_complete_model(models, parent_name, models_dir)?;
                incomplete_model.merge(&parent_model);
                models.insert(format!("minecraft:block/{}", parent_name), parent_model);
            }
        }
        
        Ok(incomplete_model)
    }

    pub fn from_dir(models_dir: PathBuf) -> Result<Self> {
        let mut models = BTreeMap::new();

        let mut blocks = models_dir.clone();

        blocks.push("block");
        for dir in fs::read_dir(blocks.clone())? {
            let dir = dir?;
            let model_path = dir.path();
            let file_name = format!("minecraft:block/{}", model_path.file_stem().unwrap().to_string_lossy());

            if models.get(&file_name).is_some() {
                continue;
            }

            // if file_name != "minecraft:block/grass_block_snow" {
            //     continue;
            // }

            let model = ModelManager::load_complete_model(&mut models, &model_path.file_stem().unwrap().to_string_lossy(), &blocks)?;

            models.insert(file_name, model);
        }

        // todo: items

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