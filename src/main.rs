#[macro_use]
extern crate log;

use clap::{App, AppSettings, Arg, SubCommand};
use std::io::Write;
mod commands;

const SCRIPTS_DIR: &str = "/Users/anon/Desktop/repos/0-hacking/storm/scripts";

fn init_log(verbosity: u64)
{
    // default value for RUST_LOG is "info"
    let log_level;
    match verbosity
    {
        1 => log_level = log::Level::Debug,
        2 => log_level = log::Level::Trace,
        _ => log_level = log::Level::Info,
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
            Arg::with_name("scripts-dir")
                .long("scripts-dir")
                .default_value(SCRIPTS_DIR),
        )
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
                .arg(Arg::with_name("name"))
                .arg(Arg::with_name("args").multiple(true)),
        )
        .get_matches();

    init_log(matches.occurrences_of("v"));

    if let Some(sub_matches) = matches.subcommand_matches("script")
    {
        let scripts_dir = matches.args["scripts-dir"].vals[0].to_str().unwrap();
        commands::script::run(scripts_dir, sub_matches);
    }
}
