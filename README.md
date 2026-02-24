# loco-cli

A command-line interface for the [localise.biz](https://localise.biz) translation management API.

## Features

- **Pull/push translations** in 30+ formats (JSON, PO, XLIFF, Strings, YAML, and more)
- **Interactive project setup** with guided configuration
- **Translation progress dashboard** showing completion per locale
- **Full CRUD** for assets, locales, translations, and tags
- **Shell completions** for bash, zsh, fish, and PowerShell
- **JSON output mode** for scripting and CI pipelines
- **Cross-platform** support for macOS, Linux, and Windows

## Installation

### From GitHub Releases

Download the latest binary for your platform from
[Releases](https://github.com/user/loco-cli/releases/latest).

### From source

```sh
cargo install --git https://github.com/user/loco-cli
```

### From crates.io (when published)

```sh
cargo install loco-cli
```

## Quick Start

### 1. Set your API key

```sh
export LOCO_API_KEY=your-api-key-here
```

Or run interactive setup:

```sh
loco-cli init
```

> **Security note:** Never commit API keys to version control. Use environment
> variables or a `.loco.toml` config file that is listed in your `.gitignore`.

### 2. Pull translations

```sh
loco-cli pull --format json --locale en --path ./locales/{locale}.json
```

### 3. Push translations

```sh
loco-cli push --file ./locales/en.json --locale en
```

### 4. Check progress

```sh
loco-cli status
```

## Configuration

Create a `.loco.toml` file in your project root:

```toml
[project]
api_key = "loco_..."      # or use LOCO_API_KEY env var

[pull]
format = "json"
path = "./locales/{locale}.json"

[push]
format = "json"
```

> Add `.loco.toml` to your `.gitignore` if it contains your API key.

## Commands

| Command | Description |
|---|---|
| `init` | Interactive project setup |
| `pull` | Export translations to local files |
| `push` | Import local translation files |
| `status` | Show translation progress |
| `auth verify` | Verify your API key |
| `assets list\|get\|create\|delete\|tag\|untag` | Manage translation assets |
| `locales list\|get\|create\|delete` | Manage project locales |
| `translations list\|get\|set\|delete\|flag\|unflag` | Manage translations |
| `tags list\|create\|rename\|delete` | Manage tags |
| `completions <shell>` | Generate shell completions |

Use `loco-cli <command> --help` for detailed usage of any command.

## Global Flags

| Flag | Description |
|---|---|
| `--key <KEY>` | API key (overrides config and env) |
| `--config <PATH>` | Path to config file |
| `--json` | Output as JSON |
| `--quiet` | Suppress non-essential output |
| `--verbose` | Enable verbose output |
| `--no-color` | Disable colored output |

## Shell Completions

```sh
# bash
loco-cli completions bash > ~/.bash_completion.d/loco-cli

# zsh
loco-cli completions zsh > ~/.zfunc/_loco-cli

# fish
loco-cli completions fish > ~/.config/fish/completions/loco-cli.fish

# PowerShell
loco-cli completions powershell > _loco-cli.ps1
```

## License

MIT
