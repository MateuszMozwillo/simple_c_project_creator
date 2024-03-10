use std::{env::{self}, fs, io::Write, process::exit};

fn main() {

    let main_cpp_template = include_bytes!("./project_template/src/main.cpp");
    let gitignore_template = include_bytes!("./project_template/.gitignore");
    let makefile_template = include_bytes!("./project_template/Makefile");

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

    fs::create_dir_all(format!("./{}/src", &project_name)).unwrap();

    let mut makefile_file = fs::File::create(format!("./{}/Makefile", &project_name)).unwrap();
    makefile_file.write_all(makefile_template).unwrap();

    let mut gitignore_file = fs::File::create(format!("./{}/.gitignore", &project_name)).unwrap();
    gitignore_file.write_all(gitignore_template).unwrap();

    let mut main_file = fs::File::create(format!("./{}/src/main.cpp", &project_name)).unwrap();
    main_file.write_all(main_cpp_template).unwrap();
} 
