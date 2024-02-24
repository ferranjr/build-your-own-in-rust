use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Server {
    pub uri: SocketAddr,
    pub healthy: bool,
}

impl Server {
    pub fn new(uri: SocketAddr) -> Server {
        Server { uri, healthy: true }
    }

    pub fn check_status_address(&self) -> String {
        format!("http://{}/private/status", &self.uri.to_string())
    }
}

#[derive(Debug)]
pub struct Targets {
    pub servers: Vec<Arc<Mutex<Server>>>,
    pub curr: usize,
}

impl Targets {
    pub fn new(uris: Vec<SocketAddr>) -> Self {
        let servers: Vec<_> = uris
            .into_iter()
            .map(|uri| {
                Arc::new(Mutex::new(Server {
                    uri,
                    healthy: false,
                }))
            })
            .collect();

        Targets { servers, curr: 0 }
    }

    pub async fn next_available_server(&mut self) -> SocketAddr {
        let mut index = self.curr;
        for _ in 0..self.servers.len() {
            let server = self.servers[index].clone();
            let server = server.lock().await;

            if server.healthy {
                self.curr = (self.curr + 1) % self.servers.len();
                return server.uri;
            }

            index = (index + 1) % self.servers.len();
        }
        self.servers[index].clone().lock().await.uri
    }
}

#[cfg(test)]
mod test {
    use crate::domain::models::Targets;
    use std::net::SocketAddr;
    use std::sync::Arc;

    #[test]
    fn targets_new_should_init_struct_correctly() {
        let address1: SocketAddr = "127.0.0.1:8081"
            .parse()
            .expect("Failed to parse socket addr");
        let address2: SocketAddr = "127.0.0.1:8082"
            .parse()
            .expect("Failed to parse socket addr");

        let targets = Targets::new(vec![address1, address2]);

        assert_eq!(targets.servers.len(), 2);
        assert_eq!(targets.curr, 0);
    }

    #[tokio::test]
    async fn targets_next_should_return_address_in_round_robin_manner_when_multiple_healthy() {
        let address1: SocketAddr = "127.0.0.1:8081"
            .parse()
            .expect("Failed to parse socket addr");
        let address2: SocketAddr = "127.0.0.1:8082"
            .parse()
            .expect("Failed to parse socket addr");
        let mut targets = Targets::new(vec![address1, address2]);
        for s in targets.servers.iter() {
            let server = Arc::clone(&s);
            server.lock().await.healthy = true;
        }

        let addr = targets.next_available_server().await;
        assert_eq!(addr, address1);
        assert_eq!(targets.curr, 1);

        let addr = targets.next_available_server().await;
        assert_eq!(addr, address2);
        assert_eq!(targets.curr, 0);

        let addr = targets.next_available_server().await;
        assert_eq!(addr, address1);
        assert_eq!(targets.curr, 1);
    }
}
