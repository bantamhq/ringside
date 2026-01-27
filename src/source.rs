use std::sync::LazyLock;

use regex::Regex;

static GITHUB_TREE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^https?://github\.com/([^/]+)/([^/]+)/tree/([^/]+)(?:/(.+))?$").unwrap()
});

static GITLAB_TREE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^https?://gitlab\.com/([^/]+)/([^/]+)/-/tree/([^/]+)(?:/(.+))?$").unwrap()
});

static SHORTHAND_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([^/]+)/([^/]+)(?:/(.+))?$").unwrap());

#[derive(Debug, Clone)]
pub struct ParsedSource {
    pub url: String,
    pub path: Option<String>,
    pub git_ref: Option<String>,
}

#[must_use]
pub fn parse_source(input: &str) -> ParsedSource {
    parse_github_tree_url(input)
        .or_else(|| parse_gitlab_tree_url(input))
        .or_else(|| parse_shorthand(input))
        .unwrap_or_else(|| ParsedSource {
            url: input.to_string(),
            path: None,
            git_ref: None,
        })
}

fn parse_github_tree_url(input: &str) -> Option<ParsedSource> {
    let caps = GITHUB_TREE_RE.captures(input)?;
    let owner = caps.get(1)?.as_str();
    let repo = caps.get(2)?.as_str().trim_end_matches(".git");
    let git_ref = caps.get(3)?.as_str();
    let path = caps.get(4).map(|m| m.as_str().to_string());

    Some(ParsedSource {
        url: format!("https://github.com/{owner}/{repo}.git"),
        path,
        git_ref: Some(git_ref.to_string()),
    })
}

fn parse_gitlab_tree_url(input: &str) -> Option<ParsedSource> {
    let caps = GITLAB_TREE_RE.captures(input)?;
    let owner = caps.get(1)?.as_str();
    let repo = caps.get(2)?.as_str().trim_end_matches(".git");
    let git_ref = caps.get(3)?.as_str();
    let path = caps.get(4).map(|m| m.as_str().to_string());

    Some(ParsedSource {
        url: format!("https://gitlab.com/{owner}/{repo}.git"),
        path,
        git_ref: Some(git_ref.to_string()),
    })
}

fn parse_shorthand(input: &str) -> Option<ParsedSource> {
    if input.starts_with("http://")
        || input.starts_with("https://")
        || input.starts_with("git@")
        || input.starts_with('/')
        || input.starts_with("./")
        || input.starts_with("../")
        || input.starts_with('~')
    {
        return None;
    }

    let caps = SHORTHAND_RE.captures(input)?;
    let owner = caps.get(1)?.as_str();
    let repo = caps.get(2)?.as_str();
    let path = caps.get(3).map(|m| m.as_str().to_string());

    Some(ParsedSource {
        url: format!("https://github.com/{owner}/{repo}.git"),
        path,
        git_ref: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_shorthand() {
        let parsed = parse_source("vercel-labs/agent-skills");
        assert_eq!(
            parsed.url,
            "https://github.com/vercel-labs/agent-skills.git"
        );
        assert_eq!(parsed.path, None);
    }

    #[test]
    fn test_github_shorthand_with_path() {
        let parsed = parse_source("vercel-labs/agent-skills/skills/react-best-practices");
        assert_eq!(
            parsed.url,
            "https://github.com/vercel-labs/agent-skills.git"
        );
        assert_eq!(parsed.path, Some("skills/react-best-practices".to_string()));
    }

    #[test]
    fn test_github_tree_url() {
        let parsed = parse_source(
            "https://github.com/vercel-labs/agent-skills/tree/main/skills/react-best-practices",
        );
        assert_eq!(
            parsed.url,
            "https://github.com/vercel-labs/agent-skills.git"
        );
        assert_eq!(parsed.path, Some("skills/react-best-practices".to_string()));
        assert_eq!(parsed.git_ref, Some("main".to_string()));
    }

    #[test]
    fn test_plain_github_url() {
        let parsed = parse_source("https://github.com/octocat/Hello-World.git");
        assert_eq!(parsed.url, "https://github.com/octocat/Hello-World.git");
        assert_eq!(parsed.path, None);
    }

    #[test]
    fn test_local_path() {
        let parsed = parse_source("/path/to/local/repo");
        assert_eq!(parsed.url, "/path/to/local/repo");
        assert_eq!(parsed.path, None);
    }
}
