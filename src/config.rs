use std::env;
use tracing::{event, Level};

#[derive(Debug, Default)]
pub struct Config {
    pub tsh_proxy: String,
    pub tsh_cluster: String,
}

impl Config {
    pub fn new() -> Config {
        return Config {
            tsh_proxy: String::new(), 
            tsh_cluster: String::new(),
        }
    }

    pub fn load(&mut self) {
        self.get_tsh_proxy();
        self.get_tsh_cluster();
    }

    fn get_tsh_proxy(&mut self) {
        if let Ok(kith_tsh_proxy) = env::var("KITH_TSH_PROXY") {

            event!(Level::DEBUG, "Your teleport proxy value is: {}", kith_tsh_proxy);
            self.tsh_proxy = kith_tsh_proxy;
        } else {
            event!(Level::DEBUG, "The KITH_TSH_PROXY environment variable is not set.");
        }
    }

    fn get_tsh_cluster(&mut self) {
        if let Ok(kith_tsh_cluster) = env::var("KITH_TSH_CLUSTER") {
            event!(Level::DEBUG, "Your teleport cluster value is: {}", kith_tsh_cluster);
            self.tsh_cluster = kith_tsh_cluster;
        } else {
            event!(Level::DEBUG, "The KITH_TSH_CLUSTER environment variable is not set.");
        }
    }
}
