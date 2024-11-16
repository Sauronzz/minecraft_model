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

    
    println!("furnace: \n{:#?}", cube_model);
    // println!("block: \n{:#?}", block_model);

    // cube_model.merge(&block_model);
    // println!("merge: \n{:#?}", cube_model);
}