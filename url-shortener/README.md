![General CI](https://github.com/ferranjr/build-your-own-in-rust/actions/workflows/url-shortener-general.yml/badge.svg)

# Build Your Own URL Shortener
Inspired by https://codingchallenges.fyi/challenges/challenge-url-shortener my ongoing attempt to build a url-shortener using RUST as a way to learn more about the language and gain some experience.

Currently, using:
* https://tokio.rs/ as async runtime
* https://actix.rs/ as http framework
* https://www.mongodb.com/docs/ as database
* Using the template for hexagonal architecture defined in https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust
* Using https://rust.testcontainers.org/ for the integration tests

# Current State
 - [x] Set up
 - [x] Unit Tests run locally and in CI env
 - [X] Integration Tests run locally
 - [X] Running Integration Tests in CI
 - [ ] Pack Service using docker and add to docker-compose
 - [ ] Improve startup/running locally
 - [ ] Use tracing/logging

# Running Locally

