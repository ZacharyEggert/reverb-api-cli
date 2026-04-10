use crate::error::RevError;

/// Reject strings containing dangerous Unicode or control characters.
/// Call this on any untrusted CLI input before processing.
pub fn check_safe_string(s: &str) -> Result<(), RevError> {
    for ch in s.chars() {
        let cp = ch as u32;
        // Control characters
        if cp < 0x20 || (0x7F..=0x9F).contains(&cp) {
            return Err(RevError::Validation(format!(
                "input contains control character U+{cp:04X}"
            )));
        }
        // Bidirectional overrides
        if (0x202A..=0x202E).contains(&cp)
            || (0x2066..=0x2069).contains(&cp)
            || cp == 0x200F
            || cp == 0x200E
        {
            return Err(RevError::Validation(format!(
                "input contains bidirectional override character U+{cp:04X}"
            )));
        }
        // Zero-width characters
        if cp == 0x200B || cp == 0xFEFF {
            return Err(RevError::Validation(format!(
                "input contains zero-width character U+{cp:04X}"
            )));
        }
    }
    Ok(())
}

/// Validate a resource identifier (listing ID, shop slug, etc.) for safe URL embedding.
/// Rejects path traversal sequences and URL-breaking characters.
pub fn validate_resource_name(s: &str) -> Result<&str, RevError> {
    check_safe_string(s)?;
    if s.contains("..") {
        return Err(RevError::Validation(
            "resource name contains path traversal sequence".into(),
        ));
    }
    for ch in ['?', '#', '/', '\\'] {
        if s.contains(ch) {
            return Err(RevError::Validation(format!(
                "resource name contains invalid character '{ch}'"
            )));
        }
    }
    Ok(s)
}

/// Validate a relative file path for safe output (no traversal, no absolute paths).
pub fn validate_safe_output_path(path: &str) -> Result<(), RevError> {
    check_safe_string(path)?;
    if std::path::Path::new(path).is_absolute() {
        return Err(RevError::Validation("output path must be relative".into()));
    }
    if path.contains("..") {
        return Err(RevError::Validation(
            "output path contains path traversal sequence".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_control_chars() {
        assert!(check_safe_string("hello\x00world").is_err());
    }

    #[test]
    fn rejects_traversal_in_resource() {
        assert!(validate_resource_name("../../etc/passwd").is_err());
    }

    #[test]
    fn accepts_valid_resource() {
        assert!(validate_resource_name("12345678").is_ok());
        assert!(validate_resource_name("my-shop-slug").is_ok());
    }

    #[test]
    fn rejects_absolute_output_path() {
        assert!(validate_safe_output_path("/etc/hosts").is_err());
    }
}
