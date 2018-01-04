extern crate image;

use image::*;

use std::error::Error;
use std::path::Path;
use std::str::FromStr;
use std::string::String;


#[derive(Debug, PartialEq)]
pub enum ActionKind {
    Gray,
    Thumb,
    Rotate,
    Crop
}

impl FromStr for ActionKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gray" => Ok(ActionKind::Gray),
            "thumb" => Ok(ActionKind::Thumb),
            "rotate" => Ok(ActionKind::Rotate),
            "crop" => Ok(ActionKind::Crop),
            e @ _ => Err(format!("Unknown action: {}", e)),
        }
    }
}

#[derive(Debug, PartialEq)]
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

        let action : ActionKind = env_action.parse()?;

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


    Ok(match *action {
        ActionKind::Gray => imageops::grayscale(img)
            .save(&save_location)?,
        ActionKind::Crop => imageops::crop(img, x, y, 250, 250)
            .to_image()
            .save(&save_location)?,
        ActionKind::Rotate => imageops::rotate180(img)
            .save(&save_location)?,
        ActionKind::Thumb => imageops::crop(img, x, y, 28, 28)
            .to_image()
            .save(&save_location)?,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_action() {

        let action : ActionKind = "gray".parse().unwrap();
        assert_eq!( action, ActionKind::Gray );
    }

    #[test]
    fn invalid_action() {

        let invalid : Result<ActionKind, _> = "invalid".parse();
        assert!( invalid.is_err() );
    }

    #[test]
    fn simulate_not_enough_args() {

        let args : Vec<String> = vec![ "only 1 arg".to_string() ];
        let config : Result<Config, _> = Config::new(&args);
        assert!( config.is_err() );
    }

    #[test]
    fn simulate_valid_num_args() {

        let args : Vec<String> = vec![ "arg1".to_string(), "image.jpg".to_string(), "gray".to_string() ];
        let config : Result<Config, _> = Config::new(&args);
        assert!( config.is_ok() );
    }
}
