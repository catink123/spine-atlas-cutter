use std::io::Read;
use image::io::Reader as ImageReader;
use clap::Parser;
use clio::{Input, ClioPath};
use spine_atlas_cutter::{atlas_parser::AtlasParser, atlas_cutter::cut_up_atlas};

/// Atlas-cutter for spine atlases
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Texture
    #[arg(value_parser, short, long)]
    image: ClioPath,

    /// Output folder
    #[arg(value_parser, short, long)]
    output_dir: ClioPath,

    /// Atlas
    #[arg(value_parser, short, long)]
    atlas: Input
}

fn main() {
    let mut args = Args::parse();
    let image = ImageReader::open(args.image.path()).unwrap().decode().unwrap();
    let mut atlas = String::new();
    args.atlas.read_to_string(&mut atlas).unwrap();

    let mut atlas_parser = AtlasParser::new();
    let atlas = atlas_parser.parse_str(&atlas);

    if !args.output_dir.is_dir() {
        println!("Invalid output directory!");
        return;
    }

    match atlas {
        Ok(atlas) => match cut_up_atlas(image, atlas, args.output_dir) {
            Ok(_) => println!("Done!"),
            Err(e) => println!("Error while cutting up the atlas: {}", e)
        },
        Err(e) => println!("Error while parsing the atlas: {}", e)
    }
}
