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
 - [X] Create Short Url, idempotency
 - [X] Redirect to original url
 - [X] Delete Short Url
 - [x] Unit Tests run locally and in CI env
 - [X] Integration Tests run locally
 - [X] Running Integration Tests in CI
 - [X] Pack Service using docker and add to docker-compose
 - [X] Improve startup/running locally
 - [ ] Use tracing/logging

# Testing
Using `cargo test` or, if installed, using `cargo nextest run` will run unit and integration tests.
Since we use test-containers, no need for setting up the DB, in integration tests we are creating the required indexes.

# Running Locally
Using docker-compose we can easily start the application locally:
```
docker-compose up
```
After that you can run curl commands against the service:
```shell
curl -XPOST "http://127.0.0.1:8080/" --header 'content-type: application/json'  -d'{ "long_url": "https://github.com/ferranjr/build-your-own-in-rust" }'
```
