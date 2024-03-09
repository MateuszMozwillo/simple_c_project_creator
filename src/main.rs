use std::{env::{self, current_exe}, process::{exit, Command}};

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

    let pwd_out_as_char = 
        String::from_utf8(command_out_pwd.stdout).expect("char* conversion fail");
    
    let pwd_trimmed = pwd_out_as_char.trim();

    let project_dir = format!("{}/{}", &pwd_trimmed, &project_name);
    println!("project created in: {}", &project_dir);
    
    Command::new("mkdir").arg(&project_dir)
        .output().expect("mkdir err");

    let exe_dir = current_exe().expect("cant get dir of exe")
        .into_os_string().into_string().expect("cant convert os_string to string");
    let exe_dir_stripped = exe_dir.strip_suffix("/c-project-creator").expect("couldnt remove suffix");

    Command::new("cp").arg("-r").arg(
            format!("{}/project_template/", &exe_dir_stripped)
        )
        .arg(&project_dir).output().expect("copy err");
}
