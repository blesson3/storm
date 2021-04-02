#![feature(command_access)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate dotenv_codegen;

use clap::{App, AppSettings, Arg, SubCommand};
use std::io::Write;

mod commands;
mod models;
mod services;

fn init_log(verbosity: u64)
{
    // default value for RUST_LOG is "info"
    let log_level;
    match verbosity
    {
        0 => log_level = log::Level::Info,
        1 => log_level = log::Level::Debug,
        _ => log_level = log::Level::Trace,
    }

    if let Err(_) = std::env::var("RUST_LOG")
    {
        std::env::set_var("RUST_LOG", format!("{}", log_level));
    }

    let mut builder = env_logger::builder();
    // don't print to stdout, messes up piping output of scripts
    // builder.target(env_logger::Target::Stdout);
    builder.format(|buf, record| match record.level()
    {
        log::Level::Info => writeln!(buf, "[+] {}", record.args()),
        log::Level::Error => writeln!(buf, "[-] {}", record.args()),
        log::Level::Warn | log::Level::Debug | log::Level::Trace =>
        {
            writeln!(
                buf,
                "[{}:{}] {}",
                record.level(),
                record.target(),
                record.args()
            )
        }
    });
    let _ = builder.try_init();
}

fn main()
{
    let matches = App::new("storm")
        .about("a cli used for general hacking shortcuts and data processing")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("script")
                .setting(AppSettings::TrailingVarArg)
                .about("run nodejs scripts on data. list scripts by omitting the `name` param")
                .arg(
                    Arg::with_name("scripts-dir")
                        .long("scripts-dir")
                        .default_value(dotenv!("SCRIPS_DIR")),
                )
                .arg(Arg::with_name("name"))
                .arg(Arg::with_name("args").multiple(true)),
        )
        .subcommand(
            SubCommand::with_name("workflow")
                .about("run predefined workflows on zap data")
                .subcommand(
                    SubCommand::with_name("analyze-api").arg(
                        Arg::with_name("base-url")
                            .long("base-url")
                            .takes_value(true)
                            .required(true),
                    ),
                ),
        )
        .subcommand(
            SubCommand::with_name("zap")
                .about("automate primitive zap commands")
                .arg(
                    Arg::with_name("zap-jar-path")
                        .takes_value(true)
                        .default_value(dotenv!("ZAP_JAR_PATH")),
                )
                .subcommand(
                    SubCommand::with_name("start-and-load-session").arg(
                        Arg::with_name("session-file")
                            .short("s")
                            .takes_value(true)
                            .required(true),
                    ),
                )
                .subcommand(
                    SubCommand::with_name("start").arg(
                        Arg::with_name("ignore-session-files")
                            .long("ignore-session-files")
                            .short("i"),
                    ),
                ),
        )
        .get_matches();

    init_log(matches.occurrences_of("v"));

    // TODO: clean up subcommand matching here (use match?)
    if let Some(script_matches) = matches.subcommand_matches("script")
    {
        let scripts_dir = script_matches.args["scripts-dir"].vals[0].to_str().unwrap();
        commands::script::run(scripts_dir, script_matches);
    }
    else if let Some(workflow_matches) = matches.subcommand_matches("workflow")
    {
        if let Some(analyze_api_matches) = workflow_matches.subcommand_matches("analyze-api")
        {
            let base_url = analyze_api_matches.args["base-url"].vals[0]
                .to_str()
                .unwrap();
            commands::workflow::analyze_api::run(base_url);
        }
    }
    else if let Some(zap_matches) = matches.subcommand_matches("zap")
    {
        if let Some(start_and_load_session_matches) =
            zap_matches.subcommand_matches("start-and-load-session")
        {
            let zap_jar_path = zap_matches.args["zap-jar-path"].vals[0].to_str().unwrap();
            let session_file = start_and_load_session_matches.args["session-file"].vals[0]
                .to_str()
                .unwrap();
            commands::zap::start_zap_and_load_session(zap_jar_path, Some(session_file));
        }
        else if let Some(start_matches) = zap_matches.subcommand_matches("start")
        {
            let zap_jar_path = zap_matches.args["zap-jar-path"].vals[0].to_str().unwrap();
            let ignore_session_files = start_matches.args.get("ignore-session-files");
            commands::zap::start_zap(zap_jar_path, ignore_session_files.is_some());
        }
    }
}
