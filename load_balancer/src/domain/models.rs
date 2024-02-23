use std::net::SocketAddr;

#[derive(Debug)]
pub struct Targets {
    uris: Vec<SocketAddr>,
    curr: usize,
}

impl Targets {
    pub fn new(uris: Vec<SocketAddr>) -> Targets {
        Targets { uris, curr: 0 }
    }
}

impl Iterator for Targets {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        let socket_address = self.uris.get(self.curr);
        self.curr = (self.curr + 1) % self.uris.len();
        socket_address.copied()
    }
}

#[cfg(test)]
mod test {
    use crate::domain::models::Targets;
    use std::net::SocketAddr;

    #[test]
    fn targets_next_should_return_address_in_round_robin_manner() {
        let address1: SocketAddr = "127.0.0.1:8081"
            .parse()
            .expect("Failed to parse socket addr");
        let address2: SocketAddr = "127.0.0.1:8082"
            .parse()
            .expect("Failed to parse socket addr");
        let mut targets = Targets {
            uris: vec![address1, address2],
            curr: 0,
        };

        let addr = targets.next().unwrap();
        assert_eq!(addr, address1);
        assert_eq!(targets.curr, 1);

        let addr = targets.next().unwrap();
        assert_eq!(addr, address2);
        assert_eq!(targets.curr, 0);

        let addr = targets.next().unwrap();
        assert_eq!(addr, address1);
        assert_eq!(targets.curr, 1);
    }
}
