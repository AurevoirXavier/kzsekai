pub mod cosplayjav_pl;
pub mod japonx_vip;

// --- std ---
use std::fmt::{Formatter, Display, self};
// --- external ---
use reqwest::{Client, ClientBuilder};

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

    fn _get_bytes(&self, url: &str) -> Vec<u8> {
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

pub enum Post {
    Cosplayjav(cosplayjav_pl::Post),
    Japonx(japonx_vip::Post),
}

impl Display for Post {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Post::Cosplayjav(post) => write!(f, "\n🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉\n\n{}", post),
            Post::Japonx(post) => write!(f, "\n🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉\n\n{}", post),
        }
    }
}

pub trait Site {
    // conf
    fn thread(&mut self, num: u32);
    fn after(&mut self, date: u32);
    fn recent(&mut self, num: u32);

    // fetch and parse
    fn parse_post(&self, url: &str) -> Post;

    fn parse_posts_page(&self, html: String) -> (bool, Vec<Post>);
    fn fetch_posts_pages(&self, last_page: u32, url: &str) {
        for page_num in 1..last_page {
            let html = CRAWLER.get_text(&format!("{}{}", url, page_num));
            let (stop, _posts) = self.parse_posts_page(html);

            if stop { return; }
        }
    }

    // fetch
    fn fetch_all(&self);
}
