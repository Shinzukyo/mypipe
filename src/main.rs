#[macro_use]
extern crate clap;
use clap::{App,};
use std::process::Command;


fn main() {
    let yaml = load_yaml!("../conf/pipe.yml");
    let matches = App::from_yaml(yaml).get_matches(); 

    let input = matches.value_of("in").unwrap();
    let output = matches.value_of("out").unwrap();

    let res_in = Command::new(input.to_string())
                            .output()
                            .expect("failed to execute process");

    let res = res_in.stdout;

    let output_res = Command::new(output.to_string())
                    .arg(String::from_utf8_lossy(&res).to_string())    
                    .output()
                    .expect("failed to execute process");

    print!("{}", String::from_utf8(output_res.stdout).unwrap());
}

