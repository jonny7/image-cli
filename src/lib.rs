extern crate image;

use image::*;

use std::error::Error;
use std::path::Path;

#[derive(Debug)]
pub enum ActionKind {
    Gray,
    Thumb,
    Rotate,
    Crop
}

#[derive(Debug)]
pub struct Config {
    pub image_path: String,
    pub action: ActionKind
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("Not enough arguments".into());
        }

        let image_path = args[1].clone();
        let env_action = &args[2];

        let action = match env_action.as_ref() {
            "gray" => ActionKind::Gray,
            "thumb" => ActionKind::Thumb,
            "rotate" => ActionKind::Rotate,
            "crop" => ActionKind::Crop,
            e @ _ => return Err(format!("Unknown action: {}", e)),
        };

        Ok( Config{ image_path, action } )
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {

    process(&config.image_path, &config.action)?;
    Ok(())
}

pub fn process(file: &String, action: &ActionKind) -> Result<(), Box<Error>> {

    let ref mut img = image::open(file)?;
    let save_location = Path::new("./images").join(file);
    let x = img.width() / 2;
    let y = img.height() / 2;


    Ok(match action {
        &ActionKind::Gray => imageops::grayscale(img)
            .save(&save_location)?,
        &ActionKind::Crop => imageops::crop(img, x, y, 250, 250)
            .to_image()
            .save(&save_location)?,
        &ActionKind::Rotate => imageops::rotate180(img)
            .save(&save_location)?,
        &ActionKind::Thumb => imageops::crop(img, x, y, 28, 28)
            .to_image()
            .save(&save_location)?,
    })
}