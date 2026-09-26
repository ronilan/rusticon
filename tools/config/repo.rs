use std::process::Command;

/// Discover `owner/repo` from the local git remote. No flags, no config.
pub fn discover_repo() -> Option<String> {
    parse_github_repo(&git_origin_url()?)
}

fn git_origin_url() -> Option<String> {
    let out = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8(out.stdout).ok()?;
    let s = s.trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn parse_github_repo(url: &str) -> Option<String> {
    let url = url.trim();
    let path = if let Some(rest) = url.strip_prefix("git@github.com:") {
        rest.to_string()
    } else if let Some(rest) = url.strip_prefix("ssh://git@github.com/") {
        rest.to_string()
    } else if let Some(idx) = url.find("github.com/") {
        url[idx + "github.com/".len()..].to_string()
    } else if !url.contains("://") && !url.contains('@') {
        url.to_string()
    } else {
        return None;
    };
    let path = path.strip_suffix(".git").unwrap_or(&path);
    let path = path.trim_matches('/');
    let mut parts = path.split('/');
    let owner = parts.next()?.trim();
    let repo = parts.next()?.trim();
    if parts.next().is_some() || owner.is_empty() || repo.is_empty() {
        return None;
    }
    let owner_ok = owner
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    let repo_ok = repo
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.');
    if !owner_ok || !repo_ok {
        return None;
    }
    Some(format!("{}/{}", owner, repo))
}

#[cfg(test)]
mod tests {
    use super::parse_github_repo;

    #[test]
    fn parses_ssh() {
        assert_eq!(
            parse_github_repo("git@github.com:ronilan/shape-sorting.git"),
            Some("ronilan/shape-sorting".to_string())
        );
    }

    #[test]
    fn parses_https() {
        assert_eq!(
            parse_github_repo("https://github.com/ronilan/shape-sorting.git"),
            Some("ronilan/shape-sorting".to_string())
        );
        assert_eq!(
            parse_github_repo("https://github.com/ronilan/shape-sorting"),
            Some("ronilan/shape-sorting".to_string())
        );
    }

    #[test]
    fn parses_https_with_credentials() {
        assert_eq!(
            parse_github_repo("https://user:token@github.com/ronilan/shape-sorting.git"),
            Some("ronilan/shape-sorting".to_string())
        );
    }

    #[test]
    fn parses_bare() {
        assert_eq!(
            parse_github_repo("ronilan/shape-sorting"),
            Some("ronilan/shape-sorting".to_string())
        );
    }

    #[test]
    fn rejects_non_github() {
        assert_eq!(parse_github_repo("git@gitlab.com:ronilan/x.git"), None);
        assert_eq!(parse_github_repo(""), None);
        assert_eq!(parse_github_repo("https://github.com/onlyowner"), None);
        assert_eq!(
            parse_github_repo("https://github.com/a/b/c"),
            None
        );
    }
}
