extern crate clap;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;

use std::fs::File;
use std::io::prelude::*;

use std::process::Command;
use clap::{Arg, App, SubCommand};

#[derive(Serialize, Deserialize, Debug)]
struct CmdLineParameter {
    flag: String, 
    values: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Configuration {
    command: String,
    arguments: Vec<CmdLineParameter>,
}

fn main() {

    // arg parsing
    let matches = App::new("sprinter")
        .version("1.0")
        .author("Adam Harries <adam.harries@ed.ac.uk>")
        .about("Run command line programs quickly, with combinations of arguments")
        .arg(Arg::with_name("CONFIG")
            .help("The configuration file specifying the program + arguments to be run")
            .takes_value(true)
            .required(true)
            .index(1))
        .get_matches();

    // get the filename from the argument parser
    let filename = matches.value_of("CONFIG").unwrap_or("config.json");
    println!("Using config from file: {}", filename);

    // read the file contents into a string
    let mut file = match File::open(filename) {
        Err(why) => panic!("couldn't open {}: {}", filename,
                                                   why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
         Err(why) => panic!("couldn't read {}: {}", filename,
                                                   why),
        Ok(_) => println!("{} contains:\n{}", filename, contents),
    };

    // try and parse a command out of the string that we've got
    let res: Result<Configuration, Error> = serde_json::from_str(&*contents);

    let structure = match res {
        Err(err) => 
            panic!("Error while parsing JSON: {}", err), 
        Ok(clp) => clp,
    };

    println!("Got struture: {:?}", structure);

    // now we have the structure, call it

    // we build the argument "backwards", starting with a vector of arguments:
    let mut argv: Vec<String> = Vec::new();
    argv.push(structure.command);
    for arg in structure.arguments {
        argv.push(arg.flag);

        for value in arg.values {
            argv.push(value);
        }
    }

    // get the output from the command, with our arguments, joined
    let output = Command::new("sh")
        .arg("-c")
        .arg(argv.join(" "))
        .output()
        .expect("Failed to run process");
    println!("Output: {:?}", output);

    // let example_data = r#"{
    //     "flag": "-i",
    //     "values": [
    //         "10", 
    //         "20"
    //     ] 
    // }"#;

    // let res : Result<CmdLineParameter, Error> = serde_json::from_str(example_data);

    // match res {
    //     Err(err) => 
    //         println!("Found error: {}", err), 
    //     Ok(clp) => 
    //         println!("Got flag: {}, with {} values", clp.flag, clp.values.len()),
    // }

    



}
