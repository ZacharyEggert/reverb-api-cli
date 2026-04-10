use clap::{ArgMatches, Command};
use reverb::RevError;

use crate::auth;

pub fn command() -> Command {
    Command::new("auth")
        .about("Manage Reverb API authentication")
        .arg_required_else_help(true)
        .subcommand(Command::new("set-key").about("Save your Reverb API key to the config file"))
        .subcommand(Command::new("status").about("Check whether a valid API key is configured"))
        .subcommand(Command::new("remove").about("Remove the stored API key from the config file"))
}

pub async fn handle(matches: &ArgMatches) -> Result<(), RevError> {
    match matches.subcommand() {
        Some(("set-key", _)) => {
            let key = prompt_api_key()?;
            auth::store_api_key(&key)?;
            eprintln!("API key saved.");
        }
        Some(("status", _)) => match auth::resolve_api_key() {
            Ok(_) => println!("Authenticated — API key is configured."),
            Err(e) => {
                println!("Not authenticated: {e}");
                std::process::exit(2);
            }
        },
        Some(("remove", _)) => {
            auth::remove_api_key()?;
            eprintln!("API key removed.");
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn prompt_api_key() -> Result<String, RevError> {
    eprint!("Enter your Reverb API key: ");
    let key = rpassword_or_stdin()?;
    if key.trim().is_empty() {
        return Err(RevError::Validation("API key cannot be empty".into()));
    }
    Ok(key.trim().to_string())
}

/// Read a line from stdin, hiding input if a TTY.
fn rpassword_or_stdin() -> Result<String, RevError> {
    // Simple stdin read; replace with rpassword crate for hidden input
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .map_err(|e| RevError::Auth(format!("failed to read input: {e}")))?;
    Ok(buf)
}
