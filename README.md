# Ringside

A CLI tool that syncs git repositories into a local project directory.

## Installation

```bash
cargo install --path .
```

## Quick Start

```bash
# Initialize config (creates .ringside/config.toml)
ringside init

# Add a source
ringside add vercel-labs/agent-skills/skills/react-best-practices --dest skills/react

# Sync all sources
ringside sync

# Check for updates
ringside status
```

## Commands

### `ringside init [ROOT]`

Initialize a new `.ringside/config.toml` file.

| Argument | Description | Default |
|----------|-------------|---------|
| `ROOT` | Root directory for synced sources | `.agents` |

```bash
ringside init              # root = ".agents"
ringside init .claude      # root = ".claude"
```

### `ringside add <URL> [--dest <PATH>]`

Add a new source to the config. Creates `.ringside/config.toml` if it doesn't exist.

| Argument | Description |
|----------|-------------|
| `URL` | Repository URL (see URL Formats below) |
| `--dest`, `-d` | Destination path within root |

```bash
ringside add octocat/Hello-World --dest hello
ringside add vercel-labs/agent-skills/skills/react-best-practices --dest skills/react
```

### `ringside sync`

Sync all configured sources. Clones repositories in parallel and updates the lock file.

```bash
ringside sync
```

### `ringside status`

Show the status of synced sources (up-to-date, outdated, or not synced).

```bash
ringside status
# [up-to-date] https://github.com/octocat/Hello-World.git -> hello
# [outdated] https://github.com/vercel-labs/agent-skills.git -> skills/react
#
# 1 up-to-date, 1 outdated, 0 unknown
```

## URL Formats

Ringside supports multiple URL formats:

### GitHub Shorthand

```
owner/repo
```

```bash
ringside add octocat/Hello-World --dest hello
# -> https://github.com/octocat/Hello-World.git
```

### GitHub Shorthand with Path

```
owner/repo/path/to/folder
```

```bash
ringside add vercel-labs/agent-skills/skills/react-best-practices --dest skills/react
# -> clones https://github.com/vercel-labs/agent-skills.git
# -> extracts skills/react-best-practices
```

### GitHub URL with Tree Path

```
https://github.com/owner/repo/tree/branch/path/to/folder
```

```bash
ringside add "https://github.com/vercel-labs/agent-skills/tree/main/skills/react-best-practices" --dest skills/react
# -> clones at ref "main"
# -> extracts skills/react-best-practices
```

### Full Git URL

```
https://github.com/owner/repo.git
```

```bash
ringside add "https://github.com/octocat/Hello-World.git" --dest hello
```

### GitLab URLs

```
https://gitlab.com/owner/repo/-/tree/branch/path
```

```bash
ringside add "https://gitlab.com/owner/repo/-/tree/main/src/lib" --dest lib
```

### Local Paths

```
/path/to/local/repo
./relative/path
../parent/path
```

```bash
ringside add /path/to/local/repo --dest local
```

## Configuration

### `.ringside/config.toml`

```toml
root = ".agents"

[[sources]]
url = "https://github.com/octocat/Hello-World.git"
dest = "hello"

[[sources]]
url = "https://github.com/vercel-labs/agent-skills.git"
path = "skills/react-best-practices"
dest = "skills/react"
ref = "main"
```

| Field | Description | Required |
|-------|-------------|----------|
| `root` | Base directory for all synced sources | Yes |
| `sources[].url` | Git repository URL | Yes |
| `sources[].path` | Subfolder to extract from repo | No |
| `sources[].dest` | Destination path within root | No (defaults to root) |
| `sources[].ref` | Branch or tag to clone | No (defaults to default branch) |

### `.ringside/lock.toml`

Automatically maintained lock file tracking synced commits:

```toml
[sources."https://github.com/octocat/Hello-World.git:hello"]
url = "https://github.com/octocat/Hello-World.git"
commit = "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d"
synced_at = "2024-01-15T10:30:00+00:00"
```

## Features

- **Parallel syncing** - Multiple repositories sync concurrently
- **Sparse checkout** - Only downloads the specific path you need (faster for large repos)
- **Lock file** - Tracks synced commits for update detection
- **GitHub shorthand** - Use `owner/repo` instead of full URLs
- **Path extraction** - Sync specific folders from repositories
- **Branch/tag support** - Pin to specific refs
