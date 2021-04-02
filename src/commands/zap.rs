use std::process::exit;
use std::{
    borrow::Cow,
    process::{Command, Stdio},
};

fn cmd_display(cmd: &Command) -> String
{
    format!(
        "{} {}",
        cmd.get_program().to_string_lossy(),
        cmd.get_args()
            .map(|x| x.to_string_lossy())
            .collect::<Vec<Cow<'_, str>>>()
            .join(" ")
    )
}

pub fn start_zap_and_load_session(zap_jar_path: &str, session_file: Option<&str>)
{
    let mut cmd = Command::new("/usr/bin/java");

    cmd.arg("-jar").arg(zap_jar_path);

    // include session file if provided
    if let Some(session_file) = session_file
    {
        debug!("starting zap using session file: {}", session_file);

        cmd.arg(session_file);
    }

    info!("$ {}", cmd_display(&cmd));

    // finishing running the session
    cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
}

pub fn start_zap(zap_jar_path: &str, ignore_session_files: bool)
{
    // try to auto find the session files unless `ignore_session_files` is present
    if !ignore_session_files
    {
        // use `gfind` (GNU variant of the `find` cmd)
        //
        // NOTE: this may not work on all systems, I would rather not add another crate
        // dependency if I don't have to. this should be more universal if this becomes
        // a well-used public project
        let output = Command::new("/usr/local/bin/gfind")
            .arg("-maxdepth")
            .arg("2")
            .arg("-name")
            .arg("*.session")
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        let session_files: Vec<&str> = stdout
            .split("\n")
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect();
        trace!("session files found: {:?}", session_files);

        let mut err_str = None;
        match session_files.len()
        {
            0 => err_str = Some("no session files found"),
            1 =>
            {}
            _ => err_str = Some("too many session files found"),
        }

        if let Some(err_str) = err_str
        {
            error!("{}", err_str);
            exit(1);
        }

        // exactly one session file found
        start_zap_and_load_session(zap_jar_path, Some(session_files[0]))
    }
    else
    {
        start_zap_and_load_session(zap_jar_path, None)
    }
}
