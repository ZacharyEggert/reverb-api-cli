use reverb::RevError;
use std::fs;
use std::path::PathBuf;

const ENV_VAR: &str = "REVERB_API_KEY";
const KEY_FILE_NAME: &str = "api_key";

/// Resolve the Reverb API key from (in priority order):
/// 1. `REVERB_API_KEY` environment variable
/// 2. Config file at `~/.config/revcli/api_key`
pub fn resolve_api_key() -> Result<String, RevError> {
    // 1. Environment variable
    if let Ok(key) = std::env::var(ENV_VAR) {
        let key = key.trim().to_string();
        if !key.is_empty() {
            return Ok(key);
        }
    }

    // 2. Config file
    if let Some(path) = config_key_path() {
        if path.exists() {
            let key = fs::read_to_string(&path)
                .map_err(|e| RevError::Auth(format!("failed to read api key file: {e}")))?;
            let key = key.trim().to_string();
            if !key.is_empty() {
                return Ok(key);
            }
        }
    }

    Err(RevError::Auth(format!(
        "no API key found — set {ENV_VAR} or run 'revcli auth set-key'"
    )))
}

/// Persist the API key to the config file.
pub fn store_api_key(key: &str) -> Result<(), RevError> {
    let path = config_key_path()
        .ok_or_else(|| RevError::Auth("could not determine config directory".into()))?;

    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)
            .map_err(|e| RevError::Auth(format!("failed to create config dir: {e}")))?;
    }

    fs::write(&path, key.trim())
        .map_err(|e| RevError::Auth(format!("failed to write api key: {e}")))?;

    // Restrict permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&path, perms)
            .map_err(|e| RevError::Auth(format!("failed to set file permissions: {e}")))?;
    }

    Ok(())
}

/// Remove the stored API key.
pub fn remove_api_key() -> Result<(), RevError> {
    if let Some(path) = config_key_path() {
        if path.exists() {
            fs::remove_file(&path)
                .map_err(|e| RevError::Auth(format!("failed to remove api key: {e}")))?;
        }
    }
    Ok(())
}

fn config_dir() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("REVERB_CLI_CONFIG_DIR") {
        return Some(PathBuf::from(dir));
    }
    dirs::config_dir().map(|d| d.join("revcli"))
}

fn config_key_path() -> Option<PathBuf> {
    config_dir().map(|d| d.join(KEY_FILE_NAME))
}
