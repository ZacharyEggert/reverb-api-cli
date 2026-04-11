use clap::{Arg, ArgMatches, Command};
use reverb_api::services;

pub fn command() -> Command {
    Command::new("schema")
        .about("Inspect the schema for a resource or method")
        .arg(
            Arg::new("target")
                .help("Resource or method to inspect (e.g. 'listings', 'listings.list')")
                .required(true),
        )
}

pub fn handle(matches: &ArgMatches) {
    let target = matches.get_one::<String>("target").unwrap();
    let parts: Vec<&str> = target.splitn(2, '.').collect();

    let resource_name = parts[0];
    let method_name = parts.get(1).copied();

    if services::find_service(resource_name).is_none() {
        eprintln!("unknown resource '{resource_name}'");
        std::process::exit(4);
    }

    if let Some(method) = method_name {
        println!("Schema for {resource_name}.{method}:");
        println!("  (schema details would be loaded from the registry here)");
    } else {
        println!("Resource: {resource_name}");
        println!("Methods:  {}", methods_for(resource_name).join(", "));
    }
}

/// Return the list of method names for a resource.
/// TODO: load from a machine-readable schema registry.
pub fn methods_for(resource: &str) -> Vec<String> {
    match resource {
        "listings" => vec!["list", "get", "create", "update", "delete"],
        "orders" => vec!["list", "get"],
        "conversations" => vec!["list", "get", "create"],
        "shop" => vec!["get", "update"],
        "categories" => vec!["list"],
        "handpicked" => vec!["list", "get"],
        "priceguide" => vec!["get"],
        "shipping" => vec!["list", "create", "update", "delete"],
        "feedback" => vec!["list"],
        "webhooks" => vec!["list", "create", "delete"],
        _ => vec!["list", "get"],
    }
    .into_iter()
    .map(String::from)
    .collect()
}
