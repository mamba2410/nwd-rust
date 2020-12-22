
#![allow(unused_assignments, unused_variables, dead_code)]

extern crate dirs;
extern crate regex;

use std::env;
use std::fs;
use std::process;

use std::path::Path;
use regex::Regex;

fn main() {

    let args: Vec<String> = env::args().collect();
    //println!("{:#?}", args);

    // get name
    let project_name = &args[1];

    // verify with regex
    let name_regex = Regex::new(r"^[0-9A-Za-z][-0-9A-Za-z\.]*$").unwrap();
    if !name_regex.is_match(&project_name) {
        exit_usage();
    }


    // Default flags
    let mut language: &String = &String::from("c");
    let mut license: Option<&String> = None;
    let mut init_git: bool = false;
    let mut init_files: bool = true;
    let mut git_remote: Option<&String> = None;


    // Argument loop
    // TODO: be able to combine args in one flag
    let mut args_vec = args.iter().skip(2).peekable();
    while args_vec.peek().is_some() {
        let arg = &args_vec.next().unwrap();
        //println!("{:#?}", arg);
        match arg.as_str() {
            // TODO: tidy this up, there has to be a better way
            "-l"|"--language"   => {
                if args_vec.peek().is_some() {
                    language = &args_vec.next().unwrap();
                    println!("Language set: {}", language);
                }
            },
            "--license"         => {
                if args_vec.peek().is_some() {
                    license = args_vec.next();
                    println!("License set: {}", license.unwrap());
                }
            },
            "-g"|"--init-git"   => {
                init_git = true;
                println!("Git init set: {}", init_git);
            },
            "-i"|"--no-init-files" => {
                init_files = false;
                println!("Files init set: {}", init_files);
            },
            "-r"|"--git-remote" => {
                if args_vec.peek().is_some() {
                    git_remote = args_vec.next();
                    println!("Git remote set: {}", git_remote.unwrap());
                }
            }
            _   => {
                println!("Unrecognised argument '{}'", arg);
                exit_usage();
            },
        }
    }


    // Set home for languages etc
    let user_home = &dirs::home_dir().unwrap();
    let program_home = &dirs::data_dir().unwrap().join("nwd");
    let common_home = program_home.join("common");
    let language_home = program_home.join("languages");
    let docs_home = program_home.join("docs");
    //println!("{:#?}", program_home);

    // listing example
    if ! language_home.exists() {
        panic!("Language home does not exist");
    }
    let mut languages: Vec<&String> = Vec::new();
    for entry in fs::read_dir(language_home).unwrap() {
        let path = entry.unwrap().path();
        //println!("{:#?}", path);
        languages.push(&String::from(path.file_name().unwrap().to_str().unwrap()));

    }
    println!("{:#?}", languages);

    // Check if language is valid
    // Check if license is valid
    //
    // Check if directory can be made
    //
    // Create dir and cd to it
    // Create tree
    // Copy docs
    // Change docs
    // Call language bash script
    // 
    // check and init git
    // check and set remote

}


fn exit_usage() {
    println!("
    Usage:
    ");

    process::exit(1);
}

