/*
 *  nwd-rust, New Working Directory written in the Rust programming language
 *  Copyright (C) 2020 Callum McGregor
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */


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


// Flag strusure for the program
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
    if args.len() < 2 { exit_usage(); }

    // get name
    let project_name = &args[1];

    // verify with regex
    let name_regex = Regex::new(r"^[_0-9A-Za-z][-_0-9A-Za-z\.]*$").unwrap();
    if !name_regex.is_match(&project_name) {
        println!("Project name '{}' not valid.", project_name);
        exit_usage();
    }


    // Default flags
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
                    pf.language = &args_vec.next().unwrap();
                }
            },
            "-L"|"--license"    => {
                if args_vec.peek().is_some() {
                    pf.license = args_vec.next();
                }
            },
            "-g"|"--init-git"   => {
                pf.init_git = true;
            },
            "-I"|"--no-init-files" => {
                pf.init_files = false;
            },
            "-r"|"--git-remote" => {
                if args_vec.peek().is_some() {
                    pf.git_remote = args_vec.next();
                }
            },
            "-D"|"--no-init-docs"  => {
                pf.init_docs = false;
            },
            "-v"|"--verbose"        => {
                // TODO: change to u8 and have different levels of verbose
                pf.v = true;
            },
            _   => {
                println!("Unrecognised argument '{}'", arg);
                exit_usage();
            },
        }
    }


    // Set home for languages etc
    let program_home = dirs::data_dir().unwrap().join("nwd");
    pf.program_home = Some(&program_home);
    if pf.v { println!("nwd home set to: {}", pf.program_home.unwrap().to_str().unwrap()); }


    // Get the language home and check if it exists
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
        println!("Language '{}' not valid. Valid languages are: ", pf.language);
        for l in languages.iter() { println!("\t{}", l); }
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
        if ! licenses.iter().any(|l| &l == &pf.license.unwrap()) {
            println!("License '{}' not valid. Available licenses are:", pf.license.unwrap());
            for l in licenses.iter() { println!("\t{}", l); }
            process::exit(1);
        }
    }
    } // Licenses


    // Check if directory can be made
    let project_path = env::current_dir().expect("Cannot get current dir").join(project_name);
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
    let mut tree_dirs = fs::read_to_string(pf.program_home.unwrap().join("dirs.txt"))
        .expect("Couldn't create directory tree");

    if ! pf.init_docs {
        tree_dirs = tree_dirs.replace("\ndocs/", "");
    }


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
        if pf.v { println!("Copying and modifying docs"); }
        copy_and_sed_docs(pf.v, pf.program_home.unwrap(), pf.project_path.unwrap(),
            "PROJECT_NAME", &project_name)
            .expect("Can't copy over docs");
    }

    if pf.v { println!("Copying readme"); }
    copy_and_sed(&pf.program_home.unwrap().join("docs/README.md"), &pf.project_path.unwrap().join("README"),
        "PROJECT_NAME", &project_name)
        .expect("Can't copy over readme");

    if pf.license.is_some() {
        if pf.v { println!("Copying license {}", pf.license.unwrap()); }
        fs::copy(pf.program_home.unwrap().join("licenses").join(&pf.license.unwrap()),
            pf.project_path.unwrap().join("LICENSE.md"))
            .expect("Can't copy license!");
    }


    // Call language bash script
    let do_init = if pf.init_files {"1"} else {"0"};
    let script = String::from( language_home.join(&pf.language).join("specifics.sh").to_str().unwrap() );
    let script = format!("\"{s}\" \"{h}\" {n} {i}", 
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

} // End of main


/*
 *  Get the file name from a &Path
 */
fn get_file_name(p: &Path) -> Option<String> {
    let f = p.file_name()?;
    let s = f.to_str()?;

    Some(String::from(s))
}


/*
 *  Print shell command status, stdout and stderr
 */
fn log_command(v: bool, cmd_return: &Output, message: &str) {
    if v {
        println!("{} returned with:\n\t{}\n\tstdout: {}\n\tstderr: {}",
                message, 
                cmd_return.status,
                str::from_utf8(&cmd_return.stdout).unwrap(),
                str::from_utf8(&cmd_return.stderr).unwrap());
    }
}


/*
 *  List directory and return the names in a Vector
 *  TODO: probably breaks if asked to list a file
 */
fn ls_dir(path: &Path) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for entry in fs::read_dir(&path).unwrap() {
        let fpath: &Path = &entry.unwrap().path();
        let file_name = get_file_name(fpath).unwrap();
        files.push(file_name);
    }

    files
}


/*
 *  Take docs from `src_home`, replace the token with a given string, then
 *  place them in the `dst_home`
 */
fn copy_and_sed_docs(v: bool, src_home: &Path, dst_home: &Path, replace_token: &str, replace_str: &str) -> std::io::Result<()> {
    let docs_src = src_home.join("docs/");
    let docs_dst = dst_home.join("docs/");

    if v { println!("Copying docs to: {}", docs_dst.to_str().unwrap()); }

    if ! docs_dst.is_dir() {
        println!("Can't copy docs if docs directory isn't created!");
        process::exit(1);
    }


    copy_and_sed(&docs_src.join("DESIGN.md"), &docs_dst.join("DESIGN.md"), replace_token, replace_str)?;
    copy_and_sed(&docs_src.join("ISSUES.md"), &docs_dst.join("ISSUES.md"), replace_token, replace_str)?;
    copy_and_sed(&docs_src.join("MANUAL.md"), &docs_dst.join("MANUAL.md"), replace_token, replace_str)?;
    copy_and_sed(&docs_src.join("TODO.md"),   &docs_dst.join("TODO.md"),   replace_token, replace_str)?;
    
    Ok(())
}


/*
 *  Take a single file from `src`, replace all instances of `replace_token` with `replace_str` 
 *  and write it to `dst`
 */
fn copy_and_sed(src: &Path, dst: &Path, replace_token: &str, replace_str: &str) -> std::io::Result<()> {
    let mut contents = fs::read_to_string(src)?;
    contents = contents.replace(replace_token, replace_str);
    fs::write(dst, contents)?;

    Ok(())
}


/*
 *  Exit and show usage
 */
fn exit_usage() {
    println!("
    Usage:
        nwd <project-name> [options]

    Options:
        -l <language>, --language <language>
            Specifies the language to use for the project.
            Defaults to c.

        -L <license>
            Add the specified license file to the project.
            Default none.

        -I
            Do not add init source files to the project.
            Default creates init files.

        -D 
            Do not create docs other than README and optional license.
            Default creates docs.

        -g, --git
            Initialises a git repo for the project and adds a gitignore.
            Also creates an initial commit.
            Default off.

        -r <path/to/remote>, --remote <path/to/remote>
            Specifies a remote git repository 
            Default none.

        -v 
            Verbose, prints debug information about the process of
            creating the project.
            Default off.
    ");

    process::exit(1);
}

