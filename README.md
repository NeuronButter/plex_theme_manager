# Plex Theme Manager

## About

Plex themes are a nightmare to manage when using Sonarr and Radarr (where upgrades and cleanups will remove the theme.mp3 file). Additionally, there's no way to share themes easily, between your friend's servers, or between different Radarr/Sonarr instances (for 4K and 1080p media). This tiny CLI tool aims to fix these problems by having one central directory for themes, and automatically symlinking them to the correct locations.

## Installation

```bash
cargo install plex_theme_manager
```

## Requirements

- Unix-like OS (Linux, macOS etc). This is due to some weird Windows privilege issues with symlinks that I can't be bothered to understand.

## Usage

plex_theme_manager is wordy, but oh well, I'm not creative enough for a new name. To use this tool, you _**need**_ a configuration file, in a JSON format. Pass it in using `--config <path>`. The JSON must be formatted like this:

```json
{
    "folders": ["/path/to/TV", "/path/to/Movies"],
    "themes": "/path/to/Themes"
}
```

```rust
pub struct Config {
    pub folders: Vec<String>, // Array of Movies/TV folders.
    pub themes: String, // Path to the themes folder.
}
```

Then, use the commands `delete`, `derive`, or `update`. `delete` will remove all themes from the folders, `derive` will create symlinks to the themes folder, and `update` will first derive the themes (to find update missing themes), and then update the media folders with the theme songs.

**`update`** *is probably what you want to use 99.9% of the time.* The other commands are just there because I needed them for very niche cleaning up.

### Examples

```bash
ptm --config config.json update
```

## License

This code is licensed under the MIT license because I'm seeking a scholarship at MIT without an application. Also apparently `cargo` needs a license, and that seems the most sense. IANAL, but I don't care what you do with this code, just don't sue me.

## Contributing and Issues

You're free to open an issue or PR. This is actually a tool that I use more often, so I'm likely to have a look at your issue if you make one.

My code is sloppy, bodged and what some would find "radical". There's probably a few `expect()`s that are supposed to be `unwrap()`s, and that's primarily due to me not actually using Rust often, but oh well, it works

## About Me

Heyo 👋, my Github sits [here](https://github.com/NeuronButter), and my website's probably [here](https://neeron.dev) if it still exists.
