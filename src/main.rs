extern crate clap;

use std::process::Command;
use clap::{Arg, App, SubCommand};

fn main() {
    println!("Hello, world!");

    // arg parsing
    let matches = App::new("sprinter")
        .version("1.0")
        .author("Adam Harries <adam.harries@ed.ac.uk>")
        .about("Run command line programs quickly, with combinations of arguments");

    // run git and find out where we're
    let output = Command::new("sh")
        .arg("-c")
        .arg("git rev-parse --verify HEAD")
        .output()
        .expect("Failed to run process");
    println!("Output: {:?}", output);


}
