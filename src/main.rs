use std::{env, fmt::format, process::Command};

fn main() {

    let args: Vec<String> = env::args().collect();
    let project_name: String = args[1].clone();

    let command_out_pwd = Command::new("pwd")
        .output().expect("pwd fail");

    let pwd_out_as_char = 
        String::from_utf8(command_out_pwd.stdout).expect("char* conversion fail");
    let pwd_trimmed = pwd_out_as_char.trim();

    let project_dir = format!("{}/{}", pwd_trimmed, &project_name);
    println!("{}", &project_dir);
    Command::new("mkdir").arg(&project_dir)
        .output().expect("mkdir err");

    Command::new("cp").arg("-r").arg("./project_template/")
        .arg(&project_dir).output().expect("copy err");
    
}
