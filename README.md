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

## Commands

| Command | Description |
|---|---|
| `init` | Interactive project setup |
| `pull` | Export translations to local files |
| `push` | Import local translation files |
| `status` | Show translation progress |
| `auth verify` | Verify your API key |
| `auth init` | Set up and save your API key |
| `assets` | Manage translation assets |
| `locales` | Manage project locales |
| `translations` | Manage translations |
| `tags` | Manage tags |
| `completions` | Generate shell completions |

Use `loco-cli <command> --help` for detailed usage of any command.

### `init`

Interactive setup wizard. Prompts for your API key, verifies it, and writes a `.loco.toml` config file with your chosen export format and output path.

```sh
loco-cli init
```

### `pull`

Export translations from Loco to local files. Exports all locales by default with a progress bar.

```sh
# Export all locales as JSON
loco-cli pull --format json --path ./locales/{locale}.json

# Export a single locale
loco-cli pull --locale en --format po --path ./locales/{locale}.po

# Filter by tag or status
loco-cli pull --filter mobile --status translated
```

| Flag | Description |
|---|---|
| `--format <FMT>` | Export format: `json`, `po`, `xlf`, `strings`, `yml`, `xml`, `csv`, etc. (default: `json`) |
| `--locale <CODE>` | Export a single locale (default: all) |
| `--path <TEMPLATE>` | Output path with `{locale}` and `{format}` placeholders (default: `./{locale}.{format}`) |
| `--filter <TAG>` | Filter assets by tag |
| `--status <STATUS>` | Filter by translation status |

Defaults can be set in `.loco.toml` under `[pull]`.

### `push`

Import a local translation file into Loco. Detects format from file extension if `--format` is omitted.

```sh
# Import a JSON file
loco-cli push --file ./locales/en.json --locale en

# Import with async polling
loco-cli push --file ./locales/de.po --locale de --async

# Tag newly created assets
loco-cli push --file ./messages.json --tag-new v2
```

| Flag | Description |
|---|---|
| `--file <PATH>` | File to upload (required) |
| `--locale <CODE>` | Locale of the file |
| `--format <FMT>` | Format hint (auto-detected from extension) |
| `--tag-new <TAG>` | Tag assets created during import |
| `--async` | Run import asynchronously with progress polling |
| `--index <MODE>` | Key mapping: `id` or `text` |

### `status`

Show translation progress per locale with color-coded completion percentages.

```sh
# All locales
loco-cli status

# Single locale
loco-cli status --locale fr

# JSON output for CI
loco-cli status --json
```

| Flag | Description |
|---|---|
| `--locale <CODE>` | Show progress for a single locale |

### `auth`

Authentication commands.

```sh
# Verify your API key works
loco-cli auth verify

# Interactive API key setup (prompts, verifies, saves to .loco.toml)
loco-cli auth init
```

### `assets`

Manage translatable assets. Maps to the [Loco Assets API](https://localise.biz/api/docs/assets).

```sh
# List all assets
loco-cli assets list

# List assets filtered by tag
loco-cli assets list --filter mobile

# List translations for a specific asset
loco-cli assets list welcome.title

# Get asset details with all translations
loco-cli assets get welcome.title

# Create an asset
loco-cli assets create welcome.title --text "Welcome" --type text --notes "Homepage heading"

# Create with inline translations
loco-cli assets create welcome.title --text "Welcome" -t de="Willkommen" -t fr="Bienvenue"

# Create or update if exists
loco-cli assets create welcome.title --text "Welcome" --update

# Set a single translation (optionally create the asset)
loco-cli assets set welcome.title en --text "Welcome" --create

# Delete an asset
loco-cli assets delete welcome.title

# Tag / untag
loco-cli assets tag welcome.title mobile
loco-cli assets untag welcome.title mobile
```

**Subcommands:**

| Subcommand | Description |
|---|---|
| `list [ID] [--filter TAG]` | List all assets, or translations for a specific asset |
| `get <ID>` | Get asset details with translations |
| `create <ID> [--text TEXT] [--type TYPE] [--context CTX] [--notes NOTES] [-t LOCALE=TEXT...] [--update]` | Create a new asset |
| `set <ID> <LOCALE> --text TEXT [--create]` | Set a translation for an asset |
| `delete <ID>` | Delete an asset |
| `tag <ID> <TAG>` | Add a tag to an asset |
| `untag <ID> <TAG>` | Remove a tag from an asset |

### `locales`

Manage project locales. Maps to the [Loco Locales API](https://localise.biz/api/docs/locales).

```sh
loco-cli locales list
loco-cli locales get fr-FR
loco-cli locales create fr-FR
loco-cli locales delete fr-FR
```

| Subcommand | Description |
|---|---|
| `list` | List all locales |
| `get <CODE>` | Get locale details |
| `create <CODE>` | Create a new locale |
| `delete <CODE>` | Delete a locale |

### `translations`

Manage individual translations. Maps to the [Loco Translations API](https://localise.biz/api/docs/translations).

```sh
# List all translations for an asset
loco-cli translations list welcome.title

# Get a specific translation
loco-cli translations get welcome.title fr

# Set a translation
loco-cli translations set welcome.title fr --text "Bienvenue"

# Delete a translation
loco-cli translations delete welcome.title fr

# Flag / unflag
loco-cli translations flag welcome.title fr --flag fuzzy
loco-cli translations unflag welcome.title fr
```

| Subcommand | Description |
|---|---|
| `list <ASSET_ID>` | List translations for an asset |
| `get <ASSET_ID> <LOCALE>` | Get a specific translation |
| `set <ASSET_ID> <LOCALE> --text TEXT` | Set a translation value |
| `delete <ASSET_ID> <LOCALE>` | Delete a translation |
| `flag <ASSET_ID> <LOCALE> [--flag VALUE]` | Flag a translation |
| `unflag <ASSET_ID> <LOCALE>` | Unflag a translation |

### `tags`

Manage project tags. Maps to the [Loco Tags API](https://localise.biz/api/docs/tags).

```sh
loco-cli tags list
loco-cli tags create mobile
loco-cli tags rename mobile app
loco-cli tags delete app
```

| Subcommand | Description |
|---|---|
| `list` | List all tags |
| `create <NAME>` | Create a tag |
| `rename <OLD> <NEW>` | Rename a tag |
| `delete <NAME>` | Delete a tag |

### `completions`

Generate shell completions.

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

## Global Flags

| Flag | Description |
|---|---|
| `--key <KEY>` | API key (overrides config and env) |
| `--config <PATH>` | Path to config file |
| `--json` | Output as JSON |
| `--quiet` | Suppress non-essential output |
| `--verbose` | Enable verbose output |
| `--no-color` | Disable colored output |

## Configuration

### Config file

The CLI looks for `.loco.toml` by walking up from the current directory. You can also specify a path with `--config`.

```toml
[api]
key = "your-api-key"       # or use LOCO_API_KEY env var

[pull]
format = "json"
path = "./locales/{locale}.json"

[push]
index = "id"
```

### API key resolution order

1. `--key` CLI flag
2. `LOCO_API_KEY` environment variable
3. `key` in `.loco.toml` `[api]` section

### Environment variables

| Variable | Description |
|---|---|
| `LOCO_API_KEY` | API key for authentication |
| `LOCO_API_URL` | Base URL override (default: `https://localise.biz/api`) |

> Add `.loco.toml` to your `.gitignore` if it contains your API key.

## Exit Codes

| Code | Meaning |
|---|---|
| `0` | Success |
| `1` | General error |
| `2` | Authentication failure |
| `3` | Configuration error |

## API Reference

This CLI wraps the [Loco REST API](https://localise.biz/api/docs). See the [Swagger spec](https://localise.biz/api/swagger) for full endpoint documentation.

## License

MIT
