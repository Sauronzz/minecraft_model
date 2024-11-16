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
struct RawModelDisplay {
    gui: Option<RawModelTransform>,
    ground: Option<RawModelTransform>,
    fixed: Option<RawModelTransform>,
    thirdperson_righthand: Option<RawModelTransform>,
    firstperson_righthand: Option<RawModelTransform>,
    firstperson_lefthand: Option<RawModelTransform>,
}

#[derive(Serialize, Deserialize, Debug, ModelMerge, Clone)]
struct RawModelTexture {
    particle: Option<String>,
    bottom: Option<String>,
    top: Option<String>,
    side: Option<String>,
    overlay: Option<String>,
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
struct RawModelFaces {
    down: Option<RawModelFaceAttr>,
    up: Option<RawModelFaceAttr>,
    north: Option<RawModelFaceAttr>,
    south: Option<RawModelFaceAttr>,
    west: Option<RawModelFaceAttr>,
    east: Option<RawModelFaceAttr>,
}

#[derive(Serialize, Deserialize, Debug, ModelMerge, Clone)]
struct RawModelElement {
    shade: Option<bool>,
    rotation: Option<RawModelElementRotation>,
    from: [f32; 3],
    to: [f32; 3],
    faces: RawModelFaces,
}

#[derive(Serialize, Deserialize, Debug, ModelMerge, Clone)]
pub struct RawBlockModel {
    parent: Option<String>,
    display: Option<RawModelDisplay>,
    ambientocclusion: Option<bool>,
    gui_light: Option<String>,
    textures: Option<RawModelTexture>,
    elements: Option<Vec<RawModelElement>>,
    // overrides: Vec<>
}
