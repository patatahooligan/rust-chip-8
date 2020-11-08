use std::env;
use std::fs::File;

use rust_chip_8::*;

struct ProgramOptions {
    rom_path: String,
}

fn parse_program_options(mut args: std::env::Args)
        -> Result<ProgramOptions, String> {
    match args.len() {
        0 => Err("Could not get arguments from OS. Running in non-compliant \
                 environment?".to_owned()),
        2 => Ok(ProgramOptions {
            rom_path: args.nth(1).unwrap(),
        }),
        _ => Err("Usage: ".to_owned() + &args.nth(0).unwrap() + " path/to/rom"),
    }
}

fn main() -> Result<(), String> {
     let options = parse_program_options(env::args())?;

     return Ok(());
}
