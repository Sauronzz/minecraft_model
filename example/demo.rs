use std::fs;

use minecraft_model::block_model::BlockModel;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    file_path: std::path::PathBuf
}

fn main() {
    let args = Cli::from_args();

    let model_json = fs::read_to_string(args.file_path).unwrap();

    let model: BlockModel = serde_json::from_str(&model_json).unwrap();
    println!("{:#?}", model);
}