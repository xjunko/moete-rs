<img src="https://my-anime-waifu.needs.rest/r/moete-blue.png"  height="200" align="right" style="float: right; margin: 0 10px 0 0;" >

# moete, the next version   |  ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?style=for-the-badge&logo=discord&logoColor=white)
a ~~(hopefully)~~ (proven) better and improved version of Moete, written in the Rust Programming Language.

## why
- memory constraints 
- to learn rust
- to make it go _"blazingly fast"_

## requirements
- rust
- postgresql server (optional)
- a discord bot

## running
before running, make sure to create a `.env` file and fill it based on the contents of [[moete-core/config]](moete-core/src/config.rs), like so:
```env
# This holds all the possible configuration for the bot.

# [Discord]
INSTANCE_NAME = "Moete"
#INSTANCE_TOKEN_DISCORD = "..."
INSTANCE_TOKEN_DISCORD = "..."
INSTANCE_TOKEN_CDN = "..."
INSTANCE_PREFIXES = "; : !"

# [Database]
INSTANCE_DB_URL = "..."

# [Flags]
IS_DEBUG = true
IS_MINIMAL = false

# [Commands]
INSTANCE_WORD_LISTS = "WORD1:ALT1,ALT2|WORD2:ALT3,ALT4"
```

then you can run with
```
MOETE_FILTER=info cargo run
```
or if you had `moete-ext` set up
```
MOETE_FILTER=info cargo run --features macros
```

## disclaimer
the source code is public but i don't intend to provide support ever as this is only a personal bot of mine, feel free to make use of the code though, its [[MIT]](LICENSE).