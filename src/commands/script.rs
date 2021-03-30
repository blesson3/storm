use clap::ArgMatches;
use std::path::Path;
use std::process::Command;
use std::{fs, process::exit};

// fn get_built_in_scripts_list() -> Vec<String>
// {
//     return env!("BUILTIN_SCRIPTS_LIST")
//         .split(",")
//         .map(|p| p.to_owned())
//         .collect();
// }

fn get_script_filenames(scripts_dir: &str) -> Vec<String>
{
    fs::read_dir(scripts_dir)
        .unwrap()
        .into_iter()
        .map(|p| p.unwrap().path().display().to_string())
        .map(|p| p.replace(scripts_dir, ""))
        .map(|p| p.replace("/", ""))
        .collect()
}

fn run_script(scripts_dir: &str, script_name: &str, script_args: Vec<&str>)
{
    let script_path = Path::new(scripts_dir).join(script_name);
    // make sure it exists
    if !script_path.exists()
    {
        error!("script does not exist: {}", script_name);
        debug!("at path: {}", script_path.display());

        let all_scripts = get_script_filenames(scripts_dir);
        let matching_scripts: Vec<String> = all_scripts
            .into_iter()
            .filter(|name| name.contains(script_name))
            .collect();

        if matching_scripts.len() > 0
        {
            info!("Did you mean...?");
            println!("- {}", matching_scripts.join("\n- "));
        }
        else
        {
            error!("there are no scripts containing: {}", script_name);
        }

        exit(1);
    }

    info!("$ node {} {}", script_name, script_args.join(" "));
    eprintln!("------------------");

    let output = Command::new("/usr/local/bin/node")
        .arg(script_path)
        .args(script_args)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout.clone()).unwrap();
    let stderr = String::from_utf8(output.stderr.clone()).unwrap();

    if !stdout.is_empty()
    {
        println!("{}", stdout);
    }
    else
    {
        debug!("stdout empty");
    }

    if !stderr.is_empty()
    {
        eprintln!("{}", stderr);
    }
    else
    {
        debug!("stderr empty");
    }

    // exit with code if needed
    if output.status.code() != Some(0)
    {
        // exit with non-zero code
        exit(output.status.code().unwrap_or(1));
    }
}

pub fn run(scripts_dir: &str, matches: &ArgMatches)
{
    // get the name of the script to run (if exists)
    if let Some(name_arg) = &matches.args.get("name")
    {
        // find and run the script
        let script_name = name_arg.vals[0]
            .to_str()
            .expect("name arg is a valid UTF-8 string");

        // get script args
        let script_args: Vec<&str>;
        if let Some(args_matches) = matches.args.get("args")
        {
            script_args = args_matches
                .vals
                .iter()
                .map(|p| p.to_str().unwrap())
                .collect();
        }
        else
        {
            script_args = vec![];
        }

        run_script(scripts_dir, script_name, script_args);
    }
    else
    {
        // list all available scripts
        info!("scripts:");

        let paths_string = get_script_filenames(scripts_dir).join("\n- ");
        println!("- {}", paths_string)
    }
}
