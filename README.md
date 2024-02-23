# Build Your Own Load Balancer
Inspired by https://codingchallenges.fyi/challenges/challenge-load-balancer/ my ongoing attempt to build a Load Balancer using RUST as a way to learn more about the language and gain some experience.

Currently, using:
* https://tokio.rs/ as async runtime
* https://hyper.rs/ as really low level http framework

# Current State
 - [x] Set up
 - [x] Server module starts two servers at 8081 and 8082
 - [x] Load Balancer running on port 8080
 - [x] Load Balancer using Round robin to redirect calls
 - [ ] Load Balancer to keep track of servers health