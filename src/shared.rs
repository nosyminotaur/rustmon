extern crate serde_json;
use serde::Deserialize;

use std::io::prelude::*;
use std::fs::File;
use std::io::Error;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Configuration {
    process: String,
    process_dir: String,
    watch_directory: String
}

pub fn run_process() {
    let process_result = get_process();
    if process_result.is_err() {
        println!("Could not get understand process string!");
        return;
    }
    let process = process_result.unwrap();

    println!("Running {:?}", process);
    let mut v: Vec<&str> = process.split(" ").collect();
    let mut command = Command::new(v[0]);
    //remove the command name to separate arguments
    v.remove(0);
    let output = command.args(v).output().expect("Some error occured!");
    println!("{:?}", String::from_utf8(output.stdout));
}

fn get_configuration() -> Result<Configuration, Error> {
    let mut file = File::open("rm_conf.json")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let conf: Configuration = serde_json::from_str(&buffer)?;
    Ok(conf)
}

#[allow(dead_code)]
pub fn get_process_dir() -> Result<String, Error> {
    let conf = get_configuration()?;
    Ok(conf.process_dir)
}

pub fn get_process() -> Result<String, Error> {
    let conf = get_configuration()?;
    Ok(conf.process)
}

pub fn get_watch_dir() -> Result<String, Error> {
    let conf = get_configuration()?;
    Ok(conf.watch_directory)
}