![General CI](https://github.com/ferranjr/build-your-own-in-rust/actions/workflows/web-server-general.yml/badge.svg)

# Build Your Own Web Server
Inspired by https://codingchallenges.fyi/challenges/challenge-web-server/ my ongoing attempt to build a Web Server using RUST as a way to learn more about the language and gain some experience.

Currently, using:
* https://tokio.rs/ as async runtime

# Current State
* [x] Step 0. We are using RUST
* [x] Step 1. Http Server listening on port `8080` and handle single connection
* [x] Step 2. Returning HTML
* [x] Step 3. Add Concurrency
* [x] Step 4. Ensure we only server documents within `www` directory
* [ ] Extra. Using [nom](https://docs.rs/nom/latest/nom/) crate for stream parsing.
