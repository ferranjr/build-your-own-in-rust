![General CI](https://github.com/ferranjr/load_balancer/actions/workflows/general.yml/badge.svg)

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
 - [x] Load Balancer to keep track of servers health
 - [ ] Improve set up to start servers using CLI or docker so makes manual testing more flexible
 - [ ] Pack LB and add docker-compose
 - [ ] Improve error handling
 - [ ] Use tracing/logging

# Running Locally

Start docker compose which starts 3 servers running in ports 8081, 8082 and 8083
```shell
docker-compose up --build --remove-orphans
```
You can curl the servers to see the different messages on root path
```shell
curl http://127.0.0.1:8081/
curl http://127.0.0.1:8082/
curl http://127.0.0.1:8083/
```
Running the load_balancer/main.rs will set the LB pointing to those 2 servers.
```shell
cargo run --bin load_balancer
```
After that, curl to the load balancer will alternate between the two servers following Round Robin algorithm.
```shell
curl http://127.0.0.1:8080/
```
Sample output:
```
➜ curl http://127.0.0.1:8080/  
Hello from server R2D2%                                                                                                                                                                        ➜  load_balancer git:(main) ✗ curl http://127.0.0.1:8080/
➜ curl http://127.0.0.1:8080/  
Hello from server Chewbacca%                                                                                                                                                                   ➜  load_balancer git:(main) ✗ curl http://127.0.0.1:8080/
➜ curl http://127.0.0.1:8080/  
Hello from server C3P0%                                                                                                                                                                        ➜  load_balancer git:(main) ✗ curl http://127.0.0.1:8080/
➜ curl http://127.0.0.1:8080/  
Hello from server R2D2%    
```

Now you can stop and start servers and see LB working as usual,
some glitch connection might be caught.