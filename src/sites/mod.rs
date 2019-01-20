pub mod cosplayjav_pl;
pub mod japonx_vip;

// --- external ---
use reqwest::{Client, ClientBuilder};

pub trait Site {
    // crawler
    fn parse_post(&self, url: &str);
    fn fetch_posts(&self);

    // conf
    fn thread(&mut self, num: u32);
    fn after(&mut self, date: u32);
    fn last(&mut self, num: u32);
}

struct Crawler { request: Client }

impl Crawler {
    fn default_builder() -> ClientBuilder {
        // --- std ---
        use std::time::Duration;

        ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .gzip(true)
            .timeout(Duration::from_secs(10))
    }

    fn new() -> Crawler { Crawler { request: Crawler::default_builder().build().unwrap() } }

    pub fn new_with_proxy(address: &str) -> Self {
        Crawler {
            request: Crawler::default_builder()
                .proxy(reqwest::Proxy::http(address).unwrap())
                .build()
                .unwrap()
        }
    }

    fn get_text(&self, url: &str) -> String {
        loop {
            match self.request.get(url).send() {
                Ok(mut resp) => if let Ok(text) = resp.text() { return text; } else { continue; }
                Err(_) => continue,
            }
        }
    }

    fn get_bytes(&self, url: &str) -> Vec<u8> {
        // --- std ---
        use std::io::Read;

        loop {
            let mut v = vec![];
            match self.request.get(url).send() {
                Ok(mut resp) => if let Ok(_) = resp.read_to_end(&mut v) { return v; } else { continue; }
                Err(_) => continue,
            }
        }
    }
}

lazy_static! { static ref CRAWLER: Crawler = if let Some(address) = &super::CONF.proxy { Crawler::new_with_proxy(address) } else { Crawler::new() }; }
