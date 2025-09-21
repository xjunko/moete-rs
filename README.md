<img src="https://my-anime-waifu.needs.rest/r/moete-blue.png"  height="200" align="right" style="float: right; margin: 0 10px 0 0;" >

# moete, the next version   |  ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?style=for-the-badge&logo=discord&logoColor=white)
a (hopefully) better and improved version of Moete, written in the Rust Programming Language.

## why
- memory constraints (current version is written in python and we only have 1gb ram at best)
- to learn rust (i've been putting this off for years now)
- improve performance? (big cap because im sure most of the slowness is on the internet side)

## requirements
- rust
- postgresql server (optional)
- a discord bot

## running
before running, make sure to create a `.env` file and fill it based on the contents of [[core/config]](src/core/config.rs), like so:
```env
# The configuration file of the discord bot, Moete.
# [Discord]
INSTANCE_NAME = "Moete"
INSTANCE_TOKEN_DISCORD = "..."
INSTANCE_TOKEN_CDN = "..."
INSTANCE_PREFIXES = ";; !!'

# [Flags]
IS_DEBUG = true
IS_MINIMAL = false
```

then you can run with
```
MOETE_FILTER=info cargo run
```
