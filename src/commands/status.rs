use crate::config::Config;
use crate::error::Result;
use crate::git;
use crate::lock::{lock_key, LockFile};
use crate::source::parse_source;

pub fn run() -> Result<()> {
    let config = Config::load()?;
    let lock = LockFile::load()?;

    if config.sources.is_empty() {
        println!("No sources configured.");
        return Ok(());
    }

    let mut up_to_date = 0;
    let mut outdated = 0;
    let mut unknown = 0;

    for source in &config.sources {
        let parsed = parse_source(&source.url);
        let key = lock_key(&parsed.url, source.dest.as_deref());
        let git_ref = source.git_ref.as_deref().or(parsed.git_ref.as_deref());

        let status = if let Some(locked) = lock.get(&key) {
            if let Some(remote_commit) = git::get_remote_commit(&parsed.url, git_ref) {
                if remote_commit == locked.commit {
                    up_to_date += 1;
                    "up-to-date"
                } else {
                    outdated += 1;
                    "outdated"
                }
            } else {
                unknown += 1;
                "unknown"
            }
        } else {
            unknown += 1;
            "not synced"
        };

        let dest_display = source.dest.as_deref().unwrap_or("/");
        println!("[{}] {} -> {}", status, parsed.url, dest_display);
    }

    println!();
    println!("{up_to_date} up-to-date, {outdated} outdated, {unknown} unknown");

    Ok(())
}
