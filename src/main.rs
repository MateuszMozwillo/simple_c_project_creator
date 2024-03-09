use std::{env::{self}, process::{exit, Command}};

const CONFIG_DIR: &str = "/Users/mai/Documents/My_bin/config";

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("too many arguments");
        exit(1);
    } if args.len() < 2 {
        println!("too few arguments");
        exit(1);
    } if args[1] == "-h" || args[1] == "--h" || args[1] == "-help" || args[1] == "--help" {
        println!("to create a c/c++ project provide project name as argument");
        exit(1);
    } if args[1].starts_with("-") || args[1].starts_with("--") {
        println!("argument not supprted");
        exit(1);
    }

    let project_name: String = args[1].clone();

    let command_out_pwd = Command::new("pwd")
        .output().expect("pwd fail");
    if command_out_pwd.stderr.len() > 0 {
        println!("{}", String::from_utf8(command_out_pwd.stderr).unwrap());
        exit(1);
    }
    let pwd_out_as_char = 
        String::from_utf8(command_out_pwd.stdout).unwrap();
    let pwd_trimmed = pwd_out_as_char.trim();

    let project_dir = format!("{}/{}", &pwd_trimmed, &project_name);
    println!("project created in: {}", &project_dir);
    
    let mkdir_cmnd_out = Command::new("mkdir").arg(&project_dir)
        .output().expect("mkdir err");
    if mkdir_cmnd_out.stderr.len() > 0 {
        println!("{}", String::from_utf8(mkdir_cmnd_out.stderr).unwrap());
        exit(1);
    }

    let copy_cmnd_out = Command::new("cp").arg("-r").arg(
            format!("{}/c-project-creator/project_template/", CONFIG_DIR)
        ).arg(&project_dir).output().expect("copy err");
    if copy_cmnd_out.stderr.len() > 0 {
        println!("{}", String::from_utf8(copy_cmnd_out.stderr).unwrap());
        exit(1);
    }
}
