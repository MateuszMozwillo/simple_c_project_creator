use std::{env::{self, current_exe}, process::Command};

fn main() {

    let args: Vec<String> = env::args().collect();
    let project_name: String = args[1].clone();

    let command_out_pwd = Command::new("pwd")
        .output().expect("pwd fail");

    let pwd_out_as_char = 
        String::from_utf8(command_out_pwd.stdout).expect("char* conversion fail");
    
    let pwd_trimmed = pwd_out_as_char.trim();

    let project_dir = format!("{}/{}", &pwd_trimmed, &project_name);
    println!("{}", &project_dir);
    
    Command::new("mkdir").arg(&project_dir)
        .output().expect("mkdir err");
    let exe_dir = current_exe().expect("cant get dir of exe")
        .into_os_string().into_string().unwrap();
    println!("{}, suffix: {}", &exe_dir, format!("/{}", &project_name));
    let exe_dir_stripped = exe_dir.strip_suffix("/c-project-creator").expect("couldnt remove suffix");
    println!("{}", &exe_dir_stripped);
    Command::new("cp").arg("-r").arg(
            format!("{}/project_template/", &exe_dir_stripped)
        )
        .arg(&project_dir).output().expect("copy err");    
}
