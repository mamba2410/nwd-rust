
#![allow(unused_assignments, unused_variables, dead_code)]

extern crate dirs;
extern crate regex;

use std::env;
use std::fs;
use std::process;
use std::str;

use std::path::Path;
use std::process::Command;
use std::process::Output;
use regex::Regex;


struct Flags<'a> {
    language: &'a String,
    license: Option<&'a String>,
    init_git: bool,
    init_files: bool,
    init_docs: bool,
    v: bool,
    git_remote: Option<&'a String>,
    program_home: Option<&'a Path>,
    project_path: Option<&'a Path>,
}


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
    //let mut language: &String = &String::from("c");
    //let mut license: Option<&String> = None;
    //let mut init_git: bool = false;
    //let mut init_files: bool = true;
    //let mut init_docs: bool = true;
    //let mut v: bool = false;
    //let mut git_remote: Option<&String> = None;

    let mut pf = Flags {
        language: &String::from("c"),
        license: None,
        init_git: false,
        init_files: true,
        init_docs: true,
        v: false,
        git_remote: None,
        program_home: None,
        project_path: None,
    };


    // Argument loop
    // TODO: be able to combine args in one flag
    // TODO: tidy this up, there has to be a better way
    let mut args_vec = args.iter().skip(2).peekable();
    while args_vec.peek().is_some() {
        let arg = &args_vec.next().unwrap();
        match arg.as_str() {
            "-l"|"--language"   => {
                if args_vec.peek().is_some() {
                    //language = &args_vec.next().unwrap();
                    pf.language = &args_vec.next().unwrap();
                    //println!("Language set: {}", language);
                }
            },
            "-L"|"--license"    => {
                if args_vec.peek().is_some() {
                    //license = args_vec.next();
                    pf.license = args_vec.next();
                    //println!("License set: {}", license.unwrap());
                }
            },
            "-g"|"--init-git"   => {
                //init_git = true;
                pf.init_git = true;
                //println!("Git init set: {}", init_git);
            },
            "-I"|"--no-init-files" => {
                //init_files = false;
                pf.init_files = false;
                //println!("Files init set: {}", init_files);
            },
            "-r"|"--git-remote" => {
                if args_vec.peek().is_some() {
                    //git_remote = args_vec.next();
                    pf.git_remote = args_vec.next();
                    //println!("Git remote set: {}", git_remote.unwrap());
                }
            },
            "-D"|"--no-init-docs"  => {
                //init_docs = false;
                pf.init_docs = false;
                //println!("Docs init set: {}", init_docs);
            },
            "-v"|"--verbose"        => {
                // TODO: change to u8 and have different levels of verbose
                //v = true;
                pf.v = true;
            },
            _   => {
                println!("Unrecognised argument '{}'", arg);
                exit_usage();
            },
        }
    }


    // Set home for languages etc
    //let user_home = &dirs::home_dir().unwrap();
    let program_home = dirs::data_dir().unwrap().join("nwd");
    //pf.program_home = Some(&dirs::data_dir().unwrap().join("nwd"));
    pf.program_home = Some(&program_home);
    // TODO: remove these or move them to somewhere more relevant. They don't need such a long
    // lifetime

    if pf.v { println!("nwd home set to: {}", pf.program_home.unwrap().to_str().unwrap()); }


    let language_home = pf.program_home.unwrap().join("languages");
    if ! language_home.exists() {
        println!("Language home does not exist. Please copy your data over to '{}'",
                 language_home.to_str().unwrap());
        process::exit(1);
    }

    // Get available languages
    let languages: Vec<String> = ls_dir(&language_home);
    if pf.v {
        println!("Available languages:");
        for l in languages.iter() {
            println!("\t{}", l);
        }
    }

    // Check if language is valid
    if ! languages.iter().any(|l| &l == &pf.language) {
        println!("Language {} not valid", pf.language);
        process::exit(1);
    }


    { // Licenses
    let license_home = program_home.join("licenses");
    if ! license_home.exists() {
        println!("License home does not exist. Please copy your data over to '{}'",
                 license_home.to_str().unwrap());
        process::exit(1);
    }
    // Get available licences
    let licenses: Vec<String> = ls_dir(&license_home);
    if pf.v {
        println!("Available licenses:");
        for l in licenses.iter() {
            println!("\t{}", l);
        }
    }
    
    // Check if license is valid
    if pf.license.is_some() {
        //let license = license.unwrap();
        if ! licenses.iter().any(|l| &l == &pf.license.unwrap()) {
            println!("License {} not valid", pf.license.unwrap());
            process::exit(1);
        }
    }
    } // Licenses

    // Check if directory can be made
    let project_path =env::current_dir().expect("Cannot get current dir").join(project_name);
    pf.project_path = Some(&project_path);
    if pf.project_path.unwrap().exists() {
        println!("Project path exists!");
        process::exit(1);
    }
    
    if pf.v { println!("Project path: {}", pf.project_path.unwrap().to_str().unwrap()); }

    // Create dir and cd to it
    match fs::create_dir(pf.project_path.unwrap()) {
        Ok(_)   => env::set_current_dir(pf.project_path.unwrap()).expect("Unable to change directory!"),
        Err(_)  => {
            println!("Couldn't make new project directory!");
            process::exit(1);
        },
    };

    // Create tree
    let tree_dirs = fs::read_to_string(pf.program_home.unwrap().join("dirs.txt"))
        .expect("Couldn't create directory tree");
    

    if pf.v { println!("Creating tree: "); }
    for dir in tree_dirs.lines() {
        if dir.len() < 1 { continue; }
        let path = &pf.project_path.unwrap().join(dir);
        match fs::create_dir(path) {
            Ok(_)   => { if pf.v { println!("\t{}", path.to_str().unwrap())} },
            Err(_)  => {
                println!("Could not create {} in directory tree", dir);
                process::exit(1);
            },
        }
    }

    // Copy docs
    if pf.init_docs {
        copy_docs(pf.v, pf.program_home.unwrap(), pf.project_path.unwrap())
            .expect("Copying docs failed");
    }

    if pf.v { println!("Copying readme"); }
    fs::copy(pf.program_home.unwrap().join("docs/README.md"), pf.project_path.unwrap().join("README.md"))
        .expect("Can't copy README!");

    if pf.license.is_some() {
        if pf.v { println!("Copying license {}", pf.license.unwrap()); }
        fs::copy(pf.program_home.unwrap().join("licenses").join(&pf.license.unwrap()),
            pf.project_path.unwrap().join("LICENSE.md"))
            .expect("Can't copy license!");
    }

    // Change docs
    // TODO: bug when passing in -D flag, separate license and readme from docs
    if pf.v { println!("Modifying docs"); } 
    match sed_docs(&pf.project_path.unwrap(), "PROJECT_NAME", &project_name) {
        Ok(())  => {},
        Err(e)  => {
            println!("{:#?}", e);
            process::exit(1);
        },
    }

    let common_home = program_home.join("common");
    let docs_home = program_home.join("docs");

    // Call language bash script
    let do_init = if pf.init_files {"1"} else {"0"};
    let script = String::from( language_home.join(&pf.language).join("specifics.sh").to_str().unwrap() );
    let script = format!("{s} {h} {n} {i}", 
                    s=script, h=pf.program_home.unwrap().to_str().unwrap(), n=project_name, i=do_init);
    if pf.v { println!("Calling language bash script {}", script); }
    let mut script_cmd = Command::new("sh");
    script_cmd.arg("-c").arg(&script);
    let script_return = script_cmd.output().expect("More shit broke");
    log_command(pf.v, &script_return, "Language script");
    

    // check and init git
    if pf.init_git {
        if pf.v { println!("Initialising git"); }
        let cmd_return = Command::new("sh").arg("-c").arg("git init .")
            .output().expect("Couldn't initialise git repo");
        log_command(pf.v, &cmd_return, "Git init");

        let cmd_return = Command::new("sh").arg("-c").arg("git add .")
            .output().expect("Couldn't add git files");
        log_command(pf.v, &cmd_return, "Git add");

        let cmd_return = Command::new("sh").arg("-c").arg("git commit -m \"Initial commit\"")
            .output().expect("Couldn't make first commit");
        log_command(pf.v, &cmd_return, "Git commit");

    }

    // check and set remote
    if pf.git_remote.is_some() {
        if ! pf.init_git { println!("Can't add repo if git isn't initialised! Skipping"); }
        else {
            if pf.v { println!("Adding git remote {} as origin", pf.git_remote.unwrap()); }
            let cmd_string = String::from("git remote add origin ") + pf.git_remote.unwrap();
            let mut cmd = Command::new("sh");
            cmd.arg("-c").arg(cmd_string);

            let cmd_return = cmd.output().expect("Couldn't add git remote");
            log_command(pf.v, &cmd_return, "Git add remote");
        }
    }

}



fn get_file_name(p: &Path) -> Option<String> {
    let f = p.file_name()?;
    let s = f.to_str()?;

    Some(String::from(s))
}


fn log_command(v: bool, cmd_return: &Output, message: &str) {
    if v {
        println!("{} returned with:\n\t{}\n\tstdout: {}\n\tstderr: {}",
                message, 
                cmd_return.status,
                str::from_utf8(&cmd_return.stdout).unwrap(),
                str::from_utf8(&cmd_return.stderr).unwrap());
    }
}


fn ls_dir(path: &Path) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for entry in fs::read_dir(&path).unwrap() {
        let fpath: &Path = &entry.unwrap().path();
        let file_name = get_file_name(fpath).unwrap();
        files.push(file_name);
    }

    files
}


// TODO: combine copy and sed
fn copy_docs(v: bool, src_home: &Path, dst_home: &Path) -> std::io::Result<()> {
    let docs_dst = dst_home.join("docs/");

    if v { println!("Copying docs to: {}", docs_dst.to_str().unwrap()); }

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

