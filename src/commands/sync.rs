use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use fs_extra::dir::{self, CopyOptions};
use rayon::prelude::*;
use tempfile::TempDir;

use crate::config::{Config, Source};
use crate::error::{Result, RingsideError};
use crate::git;
use crate::lock::{lock_key, LockFile, LockedSource};
use crate::source::parse_source;

pub fn run() -> Result<()> {
    let config = Config::load()?;

    if config.sources.is_empty() {
        println!("No sources configured. Add sources to .ringside/config.toml");
        return Ok(());
    }

    let root_path = PathBuf::from(&config.root);
    if !root_path.exists() {
        fs::create_dir_all(&root_path).map_err(|e| {
            RingsideError::DirectoryCreation(format!("Failed to create {}: {}", config.root, e))
        })?;
    }

    let lock = Mutex::new(LockFile::load()?);
    let errors: Mutex<Vec<String>> = Mutex::new(Vec::new());

    config
        .sources
        .par_iter()
        .for_each(|source| match sync_source(source, &root_path) {
            Ok(locked) => {
                if let Some(entry) = locked {
                    let mut lock = lock.lock().unwrap();
                    lock.set(entry.0, entry.1);
                }
            }
            Err(e) => {
                errors.lock().unwrap().push(e.to_string());
            }
        });

    let lock = lock.into_inner().unwrap();
    lock.save()?;

    let errors = errors.into_inner().unwrap();
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("Error: {e}");
        }
        return Err(RingsideError::GitError(format!(
            "{} source(s) failed to sync",
            errors.len()
        )));
    }

    println!("Sync complete!");
    Ok(())
}

fn sync_source(source: &Source, root_path: &Path) -> Result<Option<(String, LockedSource)>> {
    let parsed = parse_source(&source.url);
    println!("Syncing {}...", parsed.url);

    let subpath = source.path.as_deref().or(parsed.path.as_deref());
    let git_ref = source.git_ref.as_deref().or(parsed.git_ref.as_deref());

    let commit = git::get_remote_commit(&parsed.url, git_ref);

    let temp_dir = TempDir::new()?;
    let clone_path = temp_dir.path().join("repo");

    git::clone_repo(&parsed.url, &clone_path, git_ref, subpath)?;
    git::remove_git_dir(&clone_path)?;

    let source_path = match subpath {
        Some(path) => {
            let full_path = clone_path.join(path);
            if !full_path.exists() {
                return Err(RingsideError::FileCopy(format!(
                    "Path '{}' not found in {}",
                    path, parsed.url
                )));
            }
            full_path
        }
        None => clone_path,
    };

    let dest_path = match &source.dest {
        Some(dest) => root_path.join(dest),
        None => root_path.to_path_buf(),
    };

    if source.dest.is_some() && dest_path.exists() {
        fs::remove_dir_all(&dest_path).map_err(|e| {
            RingsideError::FileCopy(format!(
                "Failed to remove existing directory {}: {}",
                dest_path.display(),
                e
            ))
        })?;
    }

    if !dest_path.exists() {
        fs::create_dir_all(&dest_path).map_err(|e| {
            RingsideError::DirectoryCreation(format!(
                "Failed to create {}: {}",
                dest_path.display(),
                e
            ))
        })?;
    }

    let options = CopyOptions {
        overwrite: true,
        content_only: true,
        ..Default::default()
    };
    dir::copy(&source_path, &dest_path, &options).map_err(|e| {
        RingsideError::FileCopy(format!("Failed to copy to {}: {}", dest_path.display(), e))
    })?;

    println!("  -> {}", dest_path.display());

    let lock_entry = commit.map(|c| {
        let key = lock_key(&parsed.url, source.dest.as_deref());
        let entry = LockedSource {
            url: parsed.url.clone(),
            commit: c,
            synced_at: chrono::Utc::now().to_rfc3339(),
        };
        (key, entry)
    });

    Ok(lock_entry)
}
