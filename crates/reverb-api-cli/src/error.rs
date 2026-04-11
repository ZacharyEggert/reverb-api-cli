use reverb_api::RevError;

/// Print a structured error to stdout (JSON) and a human-readable message to stderr.
pub fn print_error(e: &RevError) {
    // Machine-readable JSON on stdout
    let json = serde_json::json!({
        "error": e.to_string(),
        "exit_code": e.exit_code(),
    });
    println!("{}", serde_json::to_string(&json).unwrap());

    // Human-readable on stderr
    eprintln!("error: {e}");
}
