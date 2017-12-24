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

        fn action_type(action: &str) -> Result<ActionKind, String> {
            Ok(match action {
                "gray" => ActionKind::Gray,
                "thumb" => ActionKind::Thumb,
                "rotate" => ActionKind::Rotate,
                "crop" => ActionKind::Crop,
                e @ _ => return Err(format!("Unknown action: {}", e)),
            })
        }

        let action = action_type(env_action)?;

        Ok( Config{ image_path, action } )
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {

    let _ = process(&config.image_path, &config.action); // is this idiomatic rust to ignore the output?
    Ok(())
}

pub fn process(file: &String, action: &ActionKind) -> Result<(), Box<Error>> {

    let ref mut img = image::open(file)?;
    let save_location = Path::new("./images").join(file);

    Ok(match action {
        &ActionKind::Gray => imageops::grayscale(img).save(&save_location)?,
        &ActionKind::Crop => imageops::crop(img, 0, 0, 250, 250).to_image().save(&save_location)?,
        &ActionKind::Rotate => println!("Not implemented"),
        &ActionKind::Thumb => println!("Not implemented"),
    })
}