# moniker

`moniker` is a terminal user interface (TUI) for matching television episode filenames against online metadata and renaming them consistently.

The application recursively scans a directory for configured video extensions, searches a metadata provider for a show, previews the resulting episode names, and can rename one file or all matching files.

## Features

- Recursive directory scanning
- Interactive file selection with arrow keys or `j`/`k`
- Show and episode lookup through TVMaze
- Configurable filename templates
- Preview of proposed episode names before renaming
- Rename a selected file or all matching files
- Rescan without restarting the application
- TUI help overlay and source selection
- Daily application logs in `logs/`

## Requirements

- Rust stable with Cargo
- Network access for metadata searches

## Installation and usage

Clone the repository and run the application with Cargo:

```text
cargo run --release -- --dir /path/to/episodes
```

If no directory is supplied, the current directory is scanned:

```text
cargo run --release
```

The compiled binary is created at `target/release/moniker` (or `target\release\moniker.exe` on Windows).

## Keyboard controls

| Key | Action |
| --- | --- |
| `↑` / `k` | Select the previous file |
| `↓` / `j` | Select the next file |
| `r` | Rescan the directory |
| `m` / `/` | Search for a show |
| `s` | Select a metadata source |
| `n` | Rename the selected file |
| `a` | Rename all matching files |
| `F1` / `?` | Toggle help |
| `q` | Quit |

In dialogs, use `↑` and `↓` to move, `Enter` to confirm, and `Esc` to close or cancel.

## Configuration

On first launch, `moniker` creates `config.toml` in the operating system's user configuration directory for the application name `moniker`.

The default configuration is:

```toml
[general]
files = ["mkv", "mp4"]
format = "{n} - {s00e00} - {t}"
```

`files` contains the filename extensions to scan. `format` supports these placeholders:

| Placeholder | Meaning |
| --- | --- |
| `{n}` | Show name |
| `{s00e00}` | Season and episode, such as `S02E03` |
| `{s}` | Season number |
| `{e}` | Episode number |
| `{t}` | Episode title |

For example, the default format renames an episode to:

```text
Example Show - S02E03 - Episode Title.mkv
```

Extensions are preserved, and generated filenames are sanitized before the rename is attempted.

## Metadata providers

TVMaze is currently implemented and is the default usable provider. TheTVDB and AniDB are listed in the source selector but their search functions are not implemented yet.

## Development

Format and check the project with:

```text
cargo fmt --check
cargo check
```

Run the application in development mode with:

```text
cargo run -- --dir /path/to/episodes
```

Logging uses the `RUST_LOG` environment variable when set. Logs are written to a daily file under `logs/`.

## Notes

- Matching currently relies on season/episode markers such as `S02E03` in existing filenames.
- Renaming changes files on disk; review the preview pane before using `n` or `a`.
- The application needs network access when performing a metadata search.
