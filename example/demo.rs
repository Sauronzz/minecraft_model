use std::collections::HashSet;

use minecraft_model::ModelManager;
use structopt::StructOpt;
use model_merge::ModelMerge;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    assets_path: std::path::PathBuf
}

fn main() {
    let args = Cli::from_args();

    let model_manager = ModelManager::from_dir(args.assets_path).unwrap();

    println!("loaded models: {}", model_manager.get_models().len());

    // println!("{:#?}", model_manager.get_models().keys());

    let cube_model = model_manager.get_models().get("minecraft:block/furnace").unwrap();
    // let block_model = model_manager.get_models().get("minecraft:block/block").unwrap();

    
    println!("{:#?}", vec![
        cube_model.get_top_texture(),
        cube_model.get_bottom_texture(),
        cube_model.get_east_texture(),
        cube_model.get_west_texture(),
        cube_model.get_south_texture(),
        cube_model.get_north_texture(),
    ]);
    // let mut all_top = HashSet::new();
    // for key in model_manager.get_models().keys() {
    //     all_top.insert(model_manager.get_model_top_parent(key));
    // }
    // println!("all top num {}: \n{:#?}", all_top.len(), all_top);

    // cube_model.merge(&block_model);
    // println!("merge: \n{:#?}", cube_model);
}