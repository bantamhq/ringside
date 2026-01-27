use std::path::Path;
use std::process::{Command, Output};

use crate::error::{Result, RingsideError};

#[must_use]
fn is_local_path(source: &str) -> bool {
    source.starts_with('/')
        || source.starts_with("./")
        || source.starts_with("../")
        || source.starts_with('~')
        || (source.len() > 1 && source.chars().nth(1) == Some(':'))
}

fn run_git(cmd: &mut Command, error_context: &str) -> Result<Output> {
    let output = cmd.output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(RingsideError::GitError(format!(
            "{error_context}: {stderr}"
        )));
    }
    Ok(output)
}

pub fn clone_repo(
    source: &str,
    target_dir: &Path,
    git_ref: Option<&str>,
    sparse_path: Option<&str>,
) -> Result<()> {
    if let (false, Some(path)) = (is_local_path(source), sparse_path) {
        return sparse_clone(source, target_dir, git_ref, path);
    }

    let mut cmd = Command::new("git");
    cmd.arg("clone");

    if !is_local_path(source) {
        cmd.args(["--depth", "1"]);
    }

    if let Some(r) = git_ref {
        cmd.args(["--branch", r]);
    }

    cmd.arg(source).arg(target_dir);
    run_git(&mut cmd, &format!("Failed to clone {source}"))?;
    Ok(())
}

fn sparse_clone(source: &str, target_dir: &Path, git_ref: Option<&str>, path: &str) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.args(["clone", "--filter=blob:none", "--sparse", "--depth", "1"]);

    if let Some(r) = git_ref {
        cmd.args(["--branch", r]);
    }

    cmd.arg(source).arg(target_dir);
    run_git(&mut cmd, &format!("Failed to clone {source}"))?;

    let mut cmd = Command::new("git");
    cmd.args(["sparse-checkout", "set", path])
        .current_dir(target_dir);
    run_git(
        &mut cmd,
        &format!("Failed to set sparse-checkout for path '{path}'"),
    )?;

    Ok(())
}

pub fn remove_git_dir(repo_dir: &Path) -> Result<()> {
    let git_dir = repo_dir.join(".git");
    if git_dir.exists() {
        std::fs::remove_dir_all(&git_dir)?;
    }
    Ok(())
}

/// Get the current commit SHA for a remote ref (branch/tag)
/// Returns None for local paths
pub fn get_remote_commit(url: &str, git_ref: Option<&str>) -> Option<String> {
    if is_local_path(url) {
        return None;
    }

    let ref_name = git_ref.unwrap_or("HEAD");
    let output = Command::new("git")
        .args(["ls-remote", url, ref_name])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .split_whitespace()
        .next()
        .map(std::string::ToString::to_string)
}
