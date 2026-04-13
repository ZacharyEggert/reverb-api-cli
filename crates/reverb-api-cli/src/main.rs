//! Reverb CLI (`revcli`)
//!
//! A schema-driven CLI for the Reverb.com API.

mod auth;
mod auth_commands;
mod commands;
mod error;
mod executor;
mod formatter;
mod helpers;
mod logging;
mod schema_cmd;

use clap::Command;
// use clap::{Arg, Command};
use dotenvy::dotenv;
use error::print_error;
use reverb_api::services;

#[tokio::main]
async fn main() {
    // Load .env if present
    let _ = dotenv();

    logging::init();

    let cmd = Command::new("revcli")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Command-line interface for the Reverb.com API")
        .arg_required_else_help(true)
        .subcommand(auth_commands::command())
        .subcommand(schema_cmd::command())
        .subcommand(Command::new("--list-resources").about("List all available API resources"));

    // First-pass: extract subcommand name to determine resource
    let raw_args: Vec<String> = std::env::args().collect();
    let subcommand_name = raw_args.get(1).cloned().unwrap_or_default();

    // Handle built-in subcommands before dynamic dispatch
    match subcommand_name.as_str() {
        "auth" => {
            let matches = cmd.get_matches();
            if let Some(auth_matches) = matches.subcommand_matches("auth") {
                if let Err(e) = auth_commands::handle(auth_matches).await {
                    print_error(&e);
                    std::process::exit(e.exit_code());
                }
            }
            return;
        }
        "schema" => {
            let matches = cmd.get_matches();
            if let Some(schema_matches) = matches.subcommand_matches("schema") {
                schema_cmd::handle(schema_matches);
            }
            return;
        }
        "--list-resources" => {
            for svc in services::SERVICES {
                println!("{:<20} {}", svc.name, svc.description);
            }
            return;
        }
        "" | "help" | "--help" | "-h" | "--version" | "-V" => {
            cmd.get_matches();
            return;
        }
        _ => {}
    }

    // Dynamic dispatch: look up resource, build command tree, re-parse
    let resource_name = subcommand_name.as_str();
    if services::find_service(resource_name).is_none() {
        eprintln!("error: unknown resource '{resource_name}'");
        eprintln!("Run 'revcli --help' to see available resources.");
        std::process::exit(1);
    }

    let api_key = match auth::resolve_api_key() {
        Ok(k) => k,
        Err(e) => {
            print_error(&e);
            std::process::exit(e.exit_code());
        }
    };

    let dynamic_cmd = commands::build_resource_command(resource_name);
    let full_cmd = Command::new("revcli")
        .subcommand(dynamic_cmd)
        .subcommand(auth_commands::command())
        .subcommand(schema_cmd::command());

    let matches = full_cmd.get_matches();

    if let Some((resource, resource_matches)) = matches.subcommand() {
        if let Err(e) = executor::execute(resource, resource_matches, &api_key).await {
            print_error(&e);
            std::process::exit(e.exit_code());
        }
    }
}
