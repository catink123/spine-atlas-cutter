use core::fmt;
use std::path::Path;

use clio::ClioPath;
use image::DynamicImage;

use crate::atlas_parser::{Atlas, Part};


#[derive(Debug, Clone)]
pub struct CutUpError(String);

impl fmt::Display for CutUpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CutUpError {}

pub fn cut_up_atlas(image: DynamicImage, atlas: Atlas, output: ClioPath) -> Result<(), Box<dyn std::error::Error>> {
    // sanity check
    let (atlas_w, atlas_h) = atlas.size;
    if atlas_w != image.width() || atlas_h != image.height() {
        return Err(CutUpError("atlas text and atlas image don't match".to_owned()).into());
    }

    println!("Cutting up the image...");

    for part in atlas.parts.iter() {
        let Part { xy: (x, y), size: (width, height), name, rotate, .. } = part.clone();

        print!("Processing part '{name}'... ");

        let (width, height) = 
            match rotate {
                Some(90 | 270) => (height, width),
                _ => (width, height)
            };

        let mut part_image = image.crop_imm(x, y, width, height);

        part_image = match rotate {
            Some(90) => part_image.rotate90(),
            Some(180) => part_image.rotate180(),
            Some(270) => part_image.rotate270(),
            _ => part_image
        };

        let filename = format!("{}.png", name);
        let mut output = output.clone();
        output.push(filename);

        let output_path: &Path = &output;

        print!("saving to '{}'... ", output_path.display());

        part_image.save(output_path)?;

        println!("saved successfully.");
    }

    Ok(())
}