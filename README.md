# loco

A command-line interface for the [localise.biz](https://localise.biz) translation management API.

## Features

- **Pull/push translations** in 30+ formats (JSON, PO, XLIFF, Strings, YAML, and more)
- **Interactive project setup** with guided configuration
- **Translation progress dashboard** showing completion per locale
- **Full CRUD** for strings, locales, and tags
- **Shell completions** for bash, zsh, fish, and PowerShell
- **JSON output mode** for scripting and CI pipelines
- **Cross-platform** support for macOS, Linux, and Windows

## Installation

### From GitHub Releases

Download the latest binary for your platform from
[Releases](https://github.com/appswithlove/loco/releases/latest).

### From source

```sh
cargo install --git https://github.com/appswithlove/loco
```


## Quick Start

### 1. Set your API key

```sh
export LOCO_API_KEY=your-api-key-here
```

Or run interactive setup:

```sh
loco init
```

> **Security note:** Never commit API keys to version control. Use environment
> variables or a `.loco.toml` config file that is listed in your `.gitignore`.

### 2. Pull translations

```sh
loco pull --format json --locale en --path ./locales/{locale}.json
```

### 3. Push translations

```sh
loco push --file ./locales/en.json --locale en
```

### 4. Check progress

```sh
loco status
```

## Commands

| Command | Description |
|---|---|
| Command | Description |
|---|---|
| `init` | Interactive project setup |
| `pull` | Export translations to local files |
| `push` | Import local translation files |
| `status` | Show translation progress |
| `auth` | Authentication commands |
| `strings` | Manage translatable strings |
| `locales` | Manage project locales |
| `tags` | Manage tags |
| `completions` | Generate shell completions |

Use `loco <command> --help` for detailed usage of any command.

### `init`

Interactive setup wizard. Prompts for your API key, verifies it, and writes a `.loco.toml` config file with your chosen export format and output path.

```sh
loco init
```

### `pull`

Export translations from Loco to local files. Exports all locales by default with a progress bar.

```sh
# Export all locales as JSON
loco pull --format json --path ./locales/{locale}.json

# Export a single locale
loco pull --locale en --format po --path ./locales/{locale}.po

# Filter by tag or status
loco pull --filter mobile --status translated
```

| Flag | Description |
|---|---|
| `--format <FMT>` | Export format: `json`, `po`, `xlf`, `strings`, `yml`, `xml`, `csv`, etc. (default: `json`) |
| `--locale <CODE>` | Export a single locale (default: all) |
| `--path <TEMPLATE>` | Output path with `{locale}` and `{format}` placeholders (default: `./{locale}.{format}`) |
| `--filter <TAG>` | Filter by tag |
| `--status <STATUS>` | Filter by translation status |

Defaults can be set in `.loco.toml` under `[pull]`.

### `push`

Import a local translation file into Loco. Detects format from file extension if `--format` is omitted.

```sh
# Import a JSON file
loco push --file ./locales/en.json --locale en

# Import with async polling
loco push --file ./locales/de.po --locale de --async

# Tag newly created assets
loco push --file ./messages.json --tag-new v2
```

| Flag | Description |
|---|---|
| `--file <PATH>` | File to upload (required) |
| `--locale <CODE>` | Locale of the file |
| `--format <FMT>` | Format hint (auto-detected from extension) |
| `--tag-new <TAG>` | Tag strings created during import |
| `--async` | Run import asynchronously with progress polling |
| `--index <MODE>` | Key mapping: `id` or `text` |

### `status`

Show translation progress per locale with color-coded completion percentages.

```sh
# All locales
loco status

# Single locale
loco status --locale fr

# JSON output for CI
loco status --json
```

| Flag | Description |
|---|---|
| `--locale <CODE>` | Show progress for a single locale |

### `auth`

Authentication commands.

```sh
# Verify your API key works
loco auth verify

# Interactive API key setup (prompts, verifies, saves to .loco.toml)
loco auth init
```

### `strings`

Manage translatable strings and their translations.

**Add a string** — one-liner with inline translations:

```sh
loco strings add welcome.title en=Welcome de=Willkommen fr=Bienvenue
```

**Add a string** — interactive mode (just omit translations):

```sh
$ loco strings add welcome.title
Source text (en): Welcome
Add translations (LOCALE=TEXT, empty to finish):
> de=Willkommen
> fr=Bienvenue
>
```

**Other operations:**

```sh
# List all strings
loco strings list
loco strings list --filter mobile

# Get string details with all translations
loco strings get welcome.title

# Get a single translation
loco strings get welcome.title fr

# Set a translation
loco strings set welcome.title fr --text "Bienvenue"

# Set a translation, creating the string if needed
loco strings set welcome.title fr --text "Bienvenue" --create

# Remove a single translation
loco strings rm welcome.title fr

# Delete a string and all its translations
loco strings delete welcome.title

# Tag / untag
loco strings tag welcome.title mobile
loco strings untag welcome.title mobile

# Flag / unflag a translation
loco strings flag welcome.title fr --flag fuzzy
loco strings unflag welcome.title fr
```

**Subcommands:**

| Subcommand | Description |
|---|---|
| `list [--filter TAG]` | List all strings |
| `get <ID> [LOCALE]` | Get string details, or a single translation |
| `add <ID> [LOCALE=TEXT...] [--type --context --notes --update]` | Add a new string with translations |
| `set <ID> <LOCALE> --text TEXT [--create]` | Set a translation |
| `rm <ID> <LOCALE>` | Remove a single translation |
| `delete <ID>` | Delete a string and all translations |
| `tag <ID> <TAG>` | Add a tag |
| `untag <ID> <TAG>` | Remove a tag |
| `flag <ID> <LOCALE> [--flag VALUE]` | Flag a translation |
| `unflag <ID> <LOCALE>` | Unflag a translation |

### `locales`

Manage project locales.

```sh
loco locales list
loco locales get fr-FR
loco locales create fr-FR
loco locales delete fr-FR
```

| Subcommand | Description |
|---|---|
| `list` | List all locales |
| `get <CODE>` | Get locale details |
| `create <CODE>` | Create a new locale |
| `delete <CODE>` | Delete a locale |

### `tags`

Manage project tags.

```sh
loco tags list
loco tags create mobile
loco tags rename mobile app
loco tags delete app
```

| Subcommand | Description |
|---|---|
| `list` | List all tags |
| `create <NAME>` | Create a tag |
| `rename <OLD> <NEW>` | Rename a tag |
| `delete <NAME>` | Delete a tag |

### `completions`

Generate shell completions. Use `--install` to write to the standard location automatically.

```sh
# Auto-install (recommended)
loco completions zsh --install
loco completions bash --install
loco completions fish --install

# Or pipe manually
loco completions zsh > ~/.zfunc/_loco
loco completions bash > ~/.bash_completion.d/loco
loco completions fish > ~/.config/fish/completions/loco.fish
loco completions powershell > _loco.ps1
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
