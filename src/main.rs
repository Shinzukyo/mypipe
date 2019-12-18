use std::process::Command;
extern crate clap; 
use clap::{Arg, App};


fn main() {
    let yaml = load_yaml!("../configs/main.yml");
    let matches = App::from_yaml(yaml).get_matches(); 

    let input = matches.value_of("in").unwrap().to_string();
    let output = matches.value_of("out").unwrap().to_string();

    let in_cmd = std::process::Command::new("cmd");
    let out_cmd = std::process::Command::new("cmd");

    let res_in = in_cmd
                .args(input)
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("error");

    let out_res = out_cmd
                .args(output)
                .stdin(std::process::Stdio::from(stdin))
                .output()
                .expect("error");

    print!("{}", String::from_utf8(out_res.stdout).unwrap());
}

