use std::net::{SocketAddr, ToSocketAddrs};

#[derive(Debug)]
pub struct Targets {
    uris: Vec<SocketAddr>,
    curr: usize,
}

impl Targets {
    pub fn from_strings(uris: Vec<String>) -> std::io::Result<Targets> {
        let valid_uris: Vec<SocketAddr> = uris
            .iter()
            .flat_map(|a| a.to_socket_addrs().expect("Unable to resolve domain"))
            .collect();
        Ok(Targets {
            uris: valid_uris,
            curr: 0,
        })
    }

    pub fn next(&mut self) -> SocketAddr {
        let socket_address = self.uris.get(self.curr).unwrap();
        self.curr = (self.curr + 1) % self.uris.len();
        *socket_address
    }
}

#[cfg(test)]
mod test {
    use crate::targets::models::Targets;
    use std::net::SocketAddr;

    #[test]
    fn targets_can_be_created_from_str() {
        let targets = Targets::from_strings(vec![
            "127.0.0.1:8081".to_string(),
            "127.0.0.1:8082".to_string(),
        ]);
        assert!(targets.is_ok());
        let targets = targets.expect("Failed to resolve targets");
        assert_eq!(targets.uris.len(), 2);
    }

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

        let addr = targets.next();
        assert_eq!(addr, address1);
        assert_eq!(targets.curr, 1);

        let addr = targets.next();
        assert_eq!(addr, address2);
        assert_eq!(targets.curr, 0);

        let addr = targets.next();
        assert_eq!(addr, address1);
        assert_eq!(targets.curr, 1);
    }
}
