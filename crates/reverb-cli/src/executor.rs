use clap::ArgMatches;
use reverb::client::{execute_with_retry, get_client};
use reverb::RevError;
use serde_json::Value;
use std::io::Write;
use std::time::Duration;

const BASE_URL: &str = "https://api.reverb.com/api";

/// Main entry point: execute the matched resource method.
pub async fn execute(resource: &str, matches: &ArgMatches, api_key: &str) -> Result<(), RevError> {
    // Check if a helper wants to handle this first
    if let Some(helper) = crate::helpers::get_helper(resource) {
        if helper.handle(matches, api_key).await? {
            return Ok(());
        }
    }

    let Some((method_name, method_matches)) = matches.subcommand() else {
        return Err(RevError::Validation("no method specified".into()));
    };

    let query: Option<Value> = method_matches
        .get_one::<String>("query")
        .map(|s| s.clone())
        .map(Value::String);
    let params: Option<Value> = method_matches
        .get_one::<String>("params")
        .map(|s| serde_json::from_str(s))
        .transpose()
        .map_err(|e| RevError::Validation(format!("invalid --params JSON: {e}")))?;

    let body: Option<Value> = method_matches
        .get_one::<String>("json")
        .map(|s| serde_json::from_str(s))
        .transpose()
        .map_err(|e| RevError::Validation(format!("invalid --json: {e}")))?;

    let dry_run = method_matches.get_flag("dry-run");
    let per_page: Option<String> = method_matches
        .get_one::<String>("per-page")
        .map(|s| s.clone());
    let page_all = method_matches.get_flag("page-all");
    let page_limit: usize = method_matches
        .get_one::<String>("page-limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    let page_delay: u64 = method_matches
        .get_one::<String>("page-delay")
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);
    let format = method_matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("json");
    let output_path = method_matches.get_one::<String>("output");

    if dry_run {
        eprintln!("dry-run: would call {resource}.{method_name}");
        if let Some(p) = &params {
            eprintln!("  params: {}", serde_json::to_string_pretty(p).unwrap());
        }
        if let Some(b) = &body {
            eprintln!("  body:   {}", serde_json::to_string_pretty(b).unwrap());
        }
        return Ok(());
    }

    // Resolve HTTP method and path from schema
    let (http_method, path) = resolve_method(resource, method_name)?;
    let (url, extra_query) = build_url(&path, &params)?;

    let mut writer: Box<dyn Write> = match output_path {
        Some(path) => Box::new(
            std::fs::File::create(path)
                .map_err(|e| RevError::Other(anyhow::anyhow!("cannot open output file: {e}")))?,
        ),
        None => Box::new(std::io::stdout()),
    };

    let mut page = 0usize;
    let mut cursor: Option<String> = None;

    loop {
        let result = execute_with_retry(|| {
            let mut req = get_client()
                .request(http_method.clone(), &url)
                .header("Authorization", format!("Bearer {api_key}"))
                .header("Accept", "application/hal+json")
                .header("Accept-Version", "3.0");

            if let Some(ref c) = cursor {
                req = req.query(&[("page", c.as_str())]);
            }

            if let Some(ref pp) = per_page {
                req = req.query(&[("per_page", pp.as_str())]);
            }

            if !extra_query.is_empty() {
                req = req.query(&extra_query);
            }

            if let Some(ref q) = query {
                req = req.query(&[("query", q.as_str())]);
            }
            if let Some(ref b) = body {
                req = req.json(b);
            }

            req
        })
        .await?;

        let status = result.status();
        let body_text = result.text().await.map_err(|e| RevError::Other(e.into()))?;
        tracing::trace!(%status, body = %body_text, "raw API response");
        let response_body: Value = serde_json::from_str(&body_text).map_err(|e| {
            RevError::Other(anyhow::anyhow!(
                "failed to parse response as JSON: {e} \n raw body: {body_text}"
            ))
        })?;

        if !status.is_success() {
            let message = response_body["message"]
                .as_str()
                .unwrap_or("unknown error")
                .to_string();
            return Err(RevError::Api {
                code: status.as_u16(),
                message,
            });
        }

        crate::formatter::print(&response_body, format, page, &mut *writer)?;
        page += 1;

        // Pagination
        if page_all && page < page_limit {
            if let Some(next) = response_body.get("current_page").and_then(|v| {
                let current = v.as_u64()?;
                let total = response_body["total_pages"].as_u64()?;
                if current < total {
                    Some((current + 1).to_string())
                } else {
                    None
                }
            }) {
                cursor = Some(next);
                tokio::time::sleep(Duration::from_millis(page_delay)).await;
                continue;
            }
        }

        break;
    }

    Ok(())
}

fn resolve_method(resource: &str, method: &str) -> Result<(reqwest::Method, String), RevError> {
    // TODO: load from schema registry
    let http_method = match method {
        "list" | "get" | "show" => reqwest::Method::GET,
        "create" => reqwest::Method::POST,
        "update" => reqwest::Method::PUT,
        "delete" => reqwest::Method::DELETE,
        _ => reqwest::Method::GET,
    };

    let path = match method {
        "list" => format!("{BASE_URL}/{resource}"),
        "get" | "show" => format!("{BASE_URL}/{resource}/{{id}}"),
        "create" => format!("{BASE_URL}/{resource}"),
        "update" => format!("{BASE_URL}/{resource}/{{id}}"),
        "delete" => format!("{BASE_URL}/{resource}/{{id}}"),
        _ => {
            return Err(RevError::Schema(format!(
                "unknown method '{method}' for resource '{resource}'"
            )))
        }
    };

    Ok((http_method, path))
}

/// Returns `(url, extra_query_params)`.
/// Params matching `{key}` placeholders are substituted into the path;
/// all others are returned as query params.
fn build_url(
    path: &str,
    params: &Option<serde_json::Value>,
) -> Result<(String, Vec<(String, String)>), RevError> {
    let mut url = path.to_string();
    let mut extra_query: Vec<(String, String)> = Vec::new();

    if let Some(Value::Object(map)) = params {
        for (k, v) in map {
            let placeholder = format!("{{{k}}}");
            let val = v.as_str().unwrap_or(&v.to_string()).to_string();
            if url.contains(&placeholder) {
                reverb::validate::validate_resource_name(&val)?;
                url = url.replace(
                    &placeholder,
                    &percent_encoding::utf8_percent_encode(
                        &val,
                        percent_encoding::NON_ALPHANUMERIC,
                    )
                    .to_string(),
                );
            } else {
                extra_query.push((k.clone(), val));
            }
        }
    }

    Ok((url, extra_query))
}
