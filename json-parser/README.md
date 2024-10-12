![General CI](https://github.com/ferranjr/build-your-own-in-rust/actions/workflows/json-parser-general.yml/badge.svg)

# Build Your Own Json Parser
Inspired by https://codingchallenges.fyi/challenges/challenge-json-parser/ my ongoing attempt to build a Json Parser using RUST as a way to learn more about the language and gain some experience.

# Current State
 - [x] Set up
 - [x] Lexer created
 - [x] Parser created
 - [x] Basic CLI to test the parser, only accepting inline jsons
 - [ ] Improve error handling
 - [ ] Commas edge cases to be dealt with

# Running Locally

```shell
cargo test

cargo nextest run
```