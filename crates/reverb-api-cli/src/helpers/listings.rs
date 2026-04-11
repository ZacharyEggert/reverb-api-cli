use clap::{Arg, ArgMatches, Command};
use reverb_api::RevError;
use std::future::Future;
use std::pin::Pin;

use crate::helpers::Helper;

pub struct ListingsHelper;

impl Helper for ListingsHelper {
    fn inject_commands(&self, cmd: Command) -> Command {
        cmd.subcommand(
            Command::new("+draft")
                .about("Create a listing in draft state with guided prompts")
                .arg(
                    Arg::new("make")
                        .long("make")
                        .required(true)
                        .help("Instrument make (e.g. Fender)"),
                )
                .arg(
                    Arg::new("model")
                        .long("model")
                        .required(true)
                        .help("Instrument model (e.g. Stratocaster)"),
                )
                .arg(
                    Arg::new("price")
                        .long("price")
                        .required(true)
                        .help("Listing price in USD (e.g. 999.00)"),
                )
                .arg(
                    Arg::new("condition")
                        .long("condition")
                        .required(true)
                        .value_parser([
                            "mint",
                            "excellent",
                            "very-good",
                            "good",
                            "fair",
                            "poor",
                            "non-functioning",
                        ])
                        .help("Item condition"),
                ),
        )
    }

    fn handle<'a>(
        &'a self,
        matches: &'a ArgMatches,
        api_key: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, RevError>> + Send + 'a>> {
        Box::pin(async move {
            if let Some(draft_matches) = matches.subcommand_matches("+draft") {
                handle_draft(draft_matches, api_key).await?;
                return Ok(true);
            }
            Ok(false)
        })
    }
}

async fn handle_draft(matches: &ArgMatches, _api_key: &str) -> Result<(), RevError> {
    let make = matches.get_one::<String>("make").unwrap();
    let model = matches.get_one::<String>("model").unwrap();
    let price = matches.get_one::<String>("price").unwrap();
    let condition = matches.get_one::<String>("condition").unwrap();

    // TODO: map condition name → Reverb condition UUID, then POST to /listings
    eprintln!("Creating draft listing: {make} {model} @ ${price} ({condition})");
    eprintln!("(Not yet implemented — this is a skeleton placeholder)");
    Ok(())
}
