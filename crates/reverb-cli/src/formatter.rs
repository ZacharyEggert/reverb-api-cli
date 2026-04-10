use reverb::RevError;
use serde_json::Value;

/// Print a JSON value in the requested format.
/// `page` is 0-indexed; headers/separators are only emitted on page 0.
pub fn print(value: &Value, format: &str, page: usize) -> Result<(), RevError> {
    match format {
        "json" | "" => print_json(value, page),
        "table" => print_table(value, page),
        "yaml" => print_yaml(value, page),
        "csv" => print_csv(value, page),
        other => Err(RevError::Validation(format!("unknown format '{other}'"))),
    }
}

fn print_json(value: &Value, page: usize) -> Result<(), RevError> {
    if page == 0 {
        println!("{}", serde_json::to_string_pretty(value).unwrap());
    } else {
        // NDJSON for subsequent pages
        println!("{}", serde_json::to_string(value).unwrap());
    }
    Ok(())
}

fn print_table(value: &Value, page: usize) -> Result<(), RevError> {
    let rows = extract_rows(value);
    if rows.is_empty() {
        if page == 0 {
            println!("(no results)");
        }
        return Ok(());
    }

    // Collect all column keys
    let mut columns: Vec<String> = Vec::new();
    for row in &rows {
        if let Value::Object(map) = row {
            for k in map.keys() {
                if !columns.contains(k) {
                    columns.push(k.clone());
                }
            }
        }
    }

    // Compute column widths
    let mut widths: Vec<usize> = columns.iter().map(|c| c.len()).collect();
    for row in &rows {
        if let Value::Object(map) = row {
            for (i, col) in columns.iter().enumerate() {
                let val = cell_str(map.get(col));
                widths[i] = widths[i].max(val.len());
            }
        }
    }

    if page == 0 {
        let header = columns
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{:<width$}", c, width = widths[i]))
            .collect::<Vec<_>>()
            .join("  ");
        println!("{header}");
        println!("{}", "-".repeat(header.len()));
    }

    for row in &rows {
        if let Value::Object(map) = row {
            let line = columns
                .iter()
                .enumerate()
                .map(|(i, col)| {
                    format!("{:<width$}", cell_str(map.get(col)), width = widths[i])
                })
                .collect::<Vec<_>>()
                .join("  ");
            println!("{line}");
        }
    }

    Ok(())
}

fn print_yaml(value: &Value, page: usize) -> Result<(), RevError> {
    if page > 0 {
        println!("---");
    }
    // Simple YAML-like output; replace with serde_yaml if added as a dep
    println!("{}", serde_json::to_string_pretty(value).unwrap());
    Ok(())
}

fn print_csv(value: &Value, page: usize) -> Result<(), RevError> {
    let rows = extract_rows(value);
    if rows.is_empty() {
        return Ok(());
    }

    let mut columns: Vec<String> = Vec::new();
    for row in &rows {
        if let Value::Object(map) = row {
            for k in map.keys() {
                if !columns.contains(k) {
                    columns.push(k.clone());
                }
            }
        }
    }

    if page == 0 {
        println!("{}", columns.join(","));
    }

    for row in &rows {
        if let Value::Object(map) = row {
            let line = columns
                .iter()
                .map(|col| csv_escape(&cell_str(map.get(col))))
                .collect::<Vec<_>>()
                .join(",");
            println!("{line}");
        }
    }

    Ok(())
}

/// Extract the inner data array from a response, or wrap the value in a vec.
fn extract_rows(value: &Value) -> Vec<&Value> {
    // Common Reverb response shapes: { "listings": [...] } or just [...]
    if let Value::Array(arr) = value {
        return arr.iter().collect();
    }
    if let Value::Object(map) = value {
        for key in &[
            "listings", "orders", "conversations", "items", "results", "data",
        ] {
            if let Some(Value::Array(arr)) = map.get(*key) {
                return arr.iter().collect();
            }
        }
    }
    vec![value]
}

fn cell_str(v: Option<&Value>) -> String {
    match v {
        None => String::new(),
        Some(Value::String(s)) => s.clone(),
        Some(Value::Null) => String::new(),
        Some(other) => other.to_string(),
    }
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
