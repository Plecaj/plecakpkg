use std::env::home_dir;
use std::path::PathBuf;

pub fn resolve_version(version: Option<String>) -> String {
    version.unwrap_or("newest".to_string())
}

pub fn extract_name_from_url(url: &str) -> Result<String, String> {
    url.rsplit('/')
        .find(|s| !s.is_empty())
        .map(|s| s.to_string())
        .ok_or_else(|| "Invalid URL".to_string())
}

pub fn get_home_dir() -> Result<PathBuf, String> {
    let home = home_dir()
        .ok_or_else(|| "Home directory not found".to_owned())?;
    Ok(home)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_version() {
        assert_eq!(resolve_version(None), "newest");
        assert_eq!(resolve_version(Some("1.2.3".into())), "1.2.3");
    }

    #[test]
    fn test_extract_name_from_url() {
        let name = extract_name_from_url("https://github.com/fmtlib/fmt").unwrap();
        assert_eq!(name, "fmt");

        let name2 = extract_name_from_url("https://github.com/fmtlib/fmt/").unwrap();
        assert_eq!(name2, "fmt");

        let err = extract_name_from_url("").unwrap_err();
        assert_eq!(err, "Invalid URL");
    }
}