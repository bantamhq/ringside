# Ringside

**Your AI context, your way.**

Ringside is a CLI for curating and syncing AI skills, documentation, and context into your projects. Think of it as a personal package manager for everything that makes AI assistants more useful in your codebase.

## Why Ringside?

AI coding assistants work best with context — skills, prompts, documentation, and conventions that help them understand your project. Tools like [npx skills](https://github.com/vercel-labs/skills) are great for pulling from a shared ecosystem, but what about:

- Your team's internal best practices?
- That open-source skill repo that isn't in any registry?
- Your personal collection of prompts you've refined over months?
- A single folder from a massive monorepo?

Ringside makes it easy to **bring context from anywhere** — GitHub, GitLab, self-hosted repos (via [Cutman](https://github.com/bantamhq/cutman)), or your local machine — and sync it exactly where you want it.

Ringside isn't a replacement for npx skills — it's an extension. Use both. Pull popular skills from the ecosystem, then layer in your own.

## Key Benefits

**Pull from anywhere.** GitHub, GitLab, private repos, local folders. If it's a git repo, Ringside can sync it.

**Extract exactly what you need.** Want one folder from a 500-skill monorepo? Ringside uses sparse checkout to grab just that path — fast and efficient.

**One-way sync, zero conflicts.** Ringside creates snapshots, not submodules. Edit locally without worrying about merge conflicts or pushing back upstream. Need to make permanent changes? Just unlink.

**Stay up to date.** `ringside status` tells you which sources have updates available. Sync when you're ready.

**Parallel everything.** Multiple repos sync concurrently. Your time matters.

## Installation

```bash
cargo install --path .
```

## Quick Start

```bash
# Initialize config (creates ringside.toml)
ringside init

# Add a source
ringside add vercel-labs/agent-skills/skills/react-best-practices --dest skills/react

# Sync all sources
ringside sync

# Check for updates
ringside status
```

## Example: Building a Tauri Project

Say you're building a Tauri app and want to bring in context from multiple sources:

```bash
# React best practices from the skills ecosystem
ringside add vercel-labs/agent-skills/skills/react-best-practices --dest skills/react

# Rust conventions from your team's internal repo
ringside add git@gitlab.company.com:team/rust-conventions.git --dest skills/rust

# Your personal prompt collection from a local folder
ringside add ~/prompts/tauri-patterns --dest skills/tauri

# Sync everything
ringside sync
```

Your `.agents/` folder now has everything your AI assistant needs to understand your stack — pulled from three completely different sources.

## Commands

### `ringside init [ROOT]`

Initialize a new `ringside.toml` file.

| Argument | Description | Default |
|----------|-------------|---------|
| `ROOT` | Root directory for synced sources | `.agents` |

```bash
ringside init              # root = ".agents"
ringside init .claude      # root = ".claude"
```

### `ringside add <URL> [--dest <PATH>]`

Add a new source to the config. Creates `ringside.toml` if it doesn't exist.

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

### `ringside.toml`

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

### `ringside.lock`

Automatically maintained lock file tracking synced commits:

```toml
[sources."https://github.com/octocat/Hello-World.git:hello"]
url = "https://github.com/octocat/Hello-World.git"
commit = "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d"
synced_at = "2024-01-15T10:30:00+00:00"
```

## Roadmap

- **Templates** — Define reusable source bundles (e.g., a "Tauri" template that pulls both Rust and React skills with one command)

## License

MIT
