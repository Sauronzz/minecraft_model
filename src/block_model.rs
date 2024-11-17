use std::collections::HashMap;

use model_merge_derive::ModelMerge;
use model_merge::ModelMerge;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, ModelMerge, Clone)]
struct RawModelTransform {
    scale: Option<[f32; 3]>,
    rotation: Option<[f32; 3]>,
    translation: Option<[f32; 3]>,
}

#[derive(Serialize, Deserialize, Debug, ModelMerge, Clone)]
struct RawModelElementRotation {
    origin: [f32; 3],
    axis: String,
    angle: f32,
    rescale: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, ModelMerge, Clone)]
struct RawModelFaceAttr {
    uv: Option<[f32; 4]>,
    texture: String,
    tintindex: Option<f32>,
    rotation: Option<f32>,
    cullface: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ModelMerge, Clone)]
struct RawModelElement {
    shade: Option<bool>,
    rotation: Option<RawModelElementRotation>,
    from: [f32; 3],
    to: [f32; 3],
    faces: HashMap<String, RawModelFaceAttr>,
}

#[derive(Serialize, Deserialize, Debug, ModelMerge, Clone)]
pub struct RawBlockModel {
    pub parent: Option<String>,
    display: Option<HashMap<String, RawModelTransform>>,
    ambientocclusion: Option<bool>,
    gui_light: Option<String>,
    textures: Option<HashMap<String, String>>,
    elements: Option<Vec<RawModelElement>>,
    // overrides: Vec<>
}

impl RawBlockModel {
    fn get_texture(&self, key: &str) -> Option<String> {
        if let Some(textures) = &self.textures {
            let tex = textures.get(key)?;
            if tex.starts_with('#') {
                return Some(self.get_texture(&tex[1..])?);
            } else {
                Some(tex.clone())
            }
        } else {
            None
        }
    }

    pub fn get_top_texture(&self) -> Option<String> {
        self.get_texture("top")
    }

    pub fn get_bottom_texture(&self) -> Option<String> {
        self.get_texture("bottom")
    }

    pub fn get_north_texture(&self) -> Option<String> {
        self.get_texture("north")
    }

    pub fn get_south_texture(&self) -> Option<String> {
        self.get_texture("south")
    }

    pub fn get_east_texture(&self) -> Option<String> {
        self.get_texture("east")
    }

    pub fn get_west_texture(&self) -> Option<String> {
        self.get_texture("west")
    }
}