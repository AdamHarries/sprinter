extern crate clap;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;

use std::process::Command;
use clap::{Arg, App, SubCommand};

mod parser;

fn main() {

    // arg parsing
    let matches = App::new("sprinter")
        .version("1.0")
        .author("Adam Harries <adam.harries@ed.ac.uk>")
        .about(
            "Run command line programs quickly, with combinations of arguments",
        )
        .arg(
            Arg::with_name("CONFIG")
                .help(
                    "The configuration file specifying the program + arguments to be run",
                )
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .get_matches();

    // get the filename from the argument parser
    let filename = matches.value_of("CONFIG").unwrap_or("config.json");
    println!("Using config from file: {}", filename);

    // read the file contents into a string
    let mut file = match File::open(filename) {
        Err(why) => panic!("couldn't open {}: {}", filename, why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", filename, why),
        Ok(_) => println!("{} contains:\n{}", filename, contents),
    };

    let (cmd, argv) = parser::parse_string(&contents);

    // get the output from the command, with our arguments, joined
    let mut command = Command::new(cmd);
    for argvv in argv {
        command.arg(argvv);
    }

    let output = command.output().expect("Failed to run process");
    println!("Output: {:?}", output);

}
