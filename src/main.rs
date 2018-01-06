//! Build a CLI that can do some basic image processing
//!     - convert to gray scale
//!     - thumbnail converter
//!     - rotate
//!     - crop
//!
//! Need to be able to pass in the filename and then the expected action
//!
//! Next step, add in some concurrency for bulk image actions just to use it and see it working
//!

extern crate imagecli;

use std::env;
use imagecli::Config;
use std::process;

fn main() {

    // get the commands used
    let args : Vec<String> = env::args().collect();

    // build a new Config object or error
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });

    // run that object and get file
    if let Err(e) = imagecli::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}
