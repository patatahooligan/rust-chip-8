use std::env;
use std::fs::File;

use rust_chip_8::*;

struct ProgramOptions {
    rom_path: String,
}

fn parse_program_options(args: std::env::Args)
        -> Result<ProgramOptions, &'static str> {
    return Err(&"");
}

fn main() {
     parse_program_options(env::args());
}
