use clap::{Arg, Command};

/// Build the clap subcommand tree for a given resource name.
/// Methods are loaded from the schema registry.
pub fn build_resource_command(resource: &str) -> Command {
    let methods = crate::schema_cmd::methods_for(resource);

    let mut resource_cmd = Command::new(resource.to_string())
        .about(format!("Operations on {resource}"))
        .arg_required_else_help(true);

    for method in methods {
        resource_cmd = resource_cmd.subcommand(build_method_command(&method));
    }

    // Inject any helper-specific subcommands
    if let Some(helper) = crate::helpers::get_helper(resource) {
        resource_cmd = helper.inject_commands(resource_cmd);
    }

    resource_cmd
}

/// Build a clap subcommand for a single API method.
fn build_method_command(method: &str) -> Command {
    Command::new(method.to_string())
        .about(format!("{method} operation"))
        .arg(
            Arg::new("query")
                .long("query")
                .value_name("STRING")
                .help("Search/filter query string (e.g. 'Gibson Les Paul'))"),
        )
        .arg(
            Arg::new("params")
                .long("params")
                .value_name("JSON")
                .help("URL/query parameters as a JSON object"),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .value_name("JSON")
                .help("Request body as a JSON object (for POST/PUT/PATCH)"),
        )
        .arg(
            Arg::new("page-all")
                .long("page-all")
                .action(clap::ArgAction::SetTrue)
                .help("Automatically paginate through all results (outputs NDJSON)"),
        )
        .arg(
            Arg::new("page-limit")
                .long("page-limit")
                .value_name("N")
                .default_value("10")
                .help("Maximum number of pages to fetch"),
        )
        .arg(
            Arg::new("page-delay")
                .long("page-delay")
                .value_name("MS")
                .default_value("100")
                .help("Milliseconds to wait between page requests"),
        )
        .arg(
            Arg::new("per-page")
                .long("per-page")
                .value_name("N")
                .help("Number of results per page (sent as per_page query param)"),
        )
        .arg(
            Arg::new("format")
                .long("format")
                .value_name("FORMAT")
                .default_value("json")
                .value_parser(["json", "table", "yaml", "csv"])
                .help("Output format"),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .value_name("PATH")
                .help("Write output to file instead of stdout"),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .action(clap::ArgAction::SetTrue)
                .help("Validate inputs without sending the request"),
        )
}
