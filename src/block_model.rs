use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct ModelTransform {
    scale: Option<[f32; 3]>,
    rotation: Option<[f32; 3]>,
    translation: Option<[f32; 3]>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelDisplay {
    gui: Option<ModelTransform>,
    ground: Option<ModelTransform>,
    fixed: Option<ModelTransform>,
    thirdperson_righthand: Option<ModelTransform>,
    firstperson_righthand: Option<ModelTransform>,
    firstperson_lefthand: Option<ModelTransform>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelTexture {
    particle: Option<String>,
    bottom: Option<String>,
    top: Option<String>,
    side: Option<String>,
    overlay: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelElementRotation {
    origin: [i32; 3],
    axis: String,
    angle: f32,
    rescale: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelFaceAttr {
    uv: Option<[i32; 4]>,
    texture: String,
    tintindex: Option<i32>,
    rotation: Option<i32>,
    cullface: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelFaces {
    down: Option<ModelFaceAttr>,
    up: Option<ModelFaceAttr>,
    north: Option<ModelFaceAttr>,
    south: Option<ModelFaceAttr>,
    west: Option<ModelFaceAttr>,
    east: Option<ModelFaceAttr>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelElement {
    shade: Option<bool>,
    rotation: Option<ModelElementRotation>,
    from: [f32; 3],
    to: [f32; 3],
    faces: ModelFaces,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockModel {
    display: Option<ModelDisplay>,
    ambientocclusion: Option<bool>,
    gui_light: Option<String>,
    textures: Option<ModelTexture>,
    elements: Option<Vec<ModelElement>>,
    // overrides: Vec<>
}