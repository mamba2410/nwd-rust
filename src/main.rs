
#![allow(unused_assignments, unused_variables, dead_code)]

extern crate dirs;
extern crate regex;

use std::env;
use std::fs;
use std::process;

use std::path::Path;
use std::process::Command;
use regex::Regex;

fn main() {

    let args: Vec<String> = env::args().collect();
    //println!("{:#?}", args);

    if args.len() < 2 {
        exit_usage();
    }

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
    let mut init_docs: bool = true;
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
            "-I"|"--no-init-files" => {
                init_files = false;
                println!("Files init set: {}", init_files);
            },
            "-r"|"--git-remote" => {
                if args_vec.peek().is_some() {
                    git_remote = args_vec.next();
                    println!("Git remote set: {}", git_remote.unwrap());
                }
            },
            "-D"|"--no-init-docs"  => {
                init_docs = false;
                println!("Docs init set: {}", init_docs);
            },
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
    let license_home = program_home.join("licenses");
    let docs_home = program_home.join("docs");
    //println!("{:#?}", program_home);

    if ! language_home.exists() {
        println!("Language home does not exist. Please copy your data over to '{}'", language_home.to_str().unwrap());
        process::exit(1);
    }

    // Get available languages
    let mut languages: Vec<String> = Vec::new();
    for entry in fs::read_dir(&language_home).unwrap() {
        let path: &Path = &entry.unwrap().path();
        let file_name = get_file_name(path).unwrap();
        languages.push(file_name);
    }
    //println!("{:#?}", languages);

    // Check if language is valid
    if ! languages.iter().any(|l| &l == &language) {
        println!("Language {} not valid", language);
        process::exit(1);
    }

    // Get available licences
    let mut licenses: Vec<String> = Vec::new();
    for entry in fs::read_dir(&license_home).unwrap() {
        let path: &Path = &entry.unwrap().path();
        let file_name = get_file_name(path).unwrap();
        licenses.push(file_name);
    }
    //println!("{:#?}", licenses);
    
    // Check if license is valid
    if license.is_some() {
        let license = license.unwrap();
        if ! licenses.iter().any(|l| &l == &license) {
            println!("License {} not valid", license);
            process::exit(1);
        }
    }

    // Check if directory can be made
    let project_path = &env::current_dir().expect("Cannot get current dir").join(project_name);
    if project_path.exists() {
        println!("Project path exists!");
        process::exit(1);
    }

    //println!("{}", project_path.to_str().unwrap());

    // Create dir and cd to it
    match fs::create_dir(project_path) {
        Ok(_)   => env::set_current_dir(project_path).expect("Unable to change directory!"),
        Err(_)  => {
            println!("Couldn't make new project directory!");
            process::exit(1);
        },
    };

    // Create tree
    let tree_dirs = fs::read_to_string(program_home.join("dirs.txt"))
        .expect("Couldn't create directory tree");
    //println!("{}", tree_dirs);

    for dir in tree_dirs.lines() {
        if dir.len() < 1 { continue; }
        let path = &project_path.join(dir);
        match fs::create_dir(path) {
            Ok(_)   => {},
            Err(_)  => {
                println!("Could not create {} in directory tree", dir);
                process::exit(1);
            },
        }
    }

    // Copy docs
    if init_docs {
        copy_docs(program_home, project_path)
            .expect("Copying docs failed");
    }

    fs::copy(program_home.join("docs/README.md"), project_path.join("README.md"))
        .expect("Can't copy README!");

    if license.is_some() {
        fs::copy(program_home.join("licenses").join(&license.unwrap()), project_path.join("LICENSE.md"))
            .expect("Can't copy license!");
    }

    // Change docs
    // TODO: bug when passing in -D flag, separate license and readme from docs
    match sed_docs(&project_path, "PROJECT_NAME", &project_name) {
        Ok(())    => {},
        Err(e)  => {
            println!("{:#?}", e);
            process::exit(1);
        },
    }

    // Call language bash script
    let do_init = if init_files {"1"} else {"0"};
    let script = String::from( language_home.join(&language).join("specifics.sh").to_str().unwrap() );
    let script = format!("{s} {h} {n} {i}", 
                    s=script, h=program_home.to_str().unwrap(), n=project_name, i=do_init);
    let mut script_cmd = Command::new("sh");
    script_cmd.arg("-c").arg(&script);
    let script_return = script_cmd.output().expect("More shit broke");
    
    //println!("{:#?}", script_return);

    // check and init git
    if init_git {
        println!("Initialising git");
        let cmd_return = Command::new("sh").arg("-c").arg("git init .")
            .output().expect("Couldn't initialise git repo");
        println!("{:#?}", cmd_return);
        let cmd_return = Command::new("sh").arg("-c").arg("git add .")
            .output().expect("Couldn't add git files");
        println!("{:#?}", cmd_return);
        let cmd_return = Command::new("sh").arg("-c").arg("git commit -m \"Initial commit\"")
            .output().expect("Couldn't make first commit");
        println!("{:#?}", cmd_return);

    }

    // check and set remote
    if git_remote.is_some() {
        if ! init_git { println!("Can't add repo if git isn't initialised! Skipping"); }
        else {
            let cmd_string = String::from("git remote add origin ") + git_remote.unwrap();
            let mut cmd = Command::new("sh");
            cmd.arg("-c").arg(cmd_string);

            let cmd_return = cmd.output().expect("Couldn't add git remote");
            println!("{:#?}", cmd_return);
        }
    }

}



fn get_file_name(p: &Path) -> Option<String> {
    let f = p.file_name()?;
    let s = f.to_str()?;

    Some(String::from(s))
}



// TODO: combine copy and sed
fn copy_docs(src_home: &Path, dst_home: &Path) -> std::io::Result<()> {
    let docs_dst = dst_home.join("docs/");
    if ! docs_dst.is_dir() {
        println!("Can't copy docs if docs directory isn't created!");
        process::exit(1);
    }


    // TODO: Make nicer, maybe a docs file like dirs.txt
    fs::copy(src_home.join("docs/DESIGN.md"), dst_home.join("docs/DESIGN.md"))?;
    fs::copy(src_home.join("docs/ISSUES.md"), dst_home.join("docs/ISSUES.md"))?;
    fs::copy(src_home.join("docs/MANUAL.md"), dst_home.join("docs/MANUAL.md"))?;
    fs::copy(src_home.join("docs/TODO.md"),   dst_home.join("docs/TODO.md"))?;


    Ok(())
}



fn sed_docs(project_home: &Path, replace_token: &str, replace_str: &str) -> std::io::Result<()> {
    let docs_dir: &Path = &project_home.join("docs/");

    //sed_file(&project_home.join("README.md"), replace_token, replace_str)?;
    sed_file(&docs_dir.join("DESIGN.md"), replace_token, replace_str)?;
    sed_file(&docs_dir.join("ISSUES.md"), replace_token, replace_str)?;
    sed_file(&docs_dir.join("MANUAL.md"), replace_token, replace_str)?;
    sed_file(&docs_dir.join("TODO.md"), replace_token, replace_str)?;

    Ok(())
}

fn sed_file(file: &Path, replace_token: &str, replace_str: &str) -> std::io::Result<()> {
    let mut contents = fs::read_to_string(file)?;
    contents = contents.replace(replace_token, replace_str);
    fs::write(file, contents)?;

    Ok(())
}


fn exit_usage() {
    println!("
    Usage:
    ");

    process::exit(1);
}

