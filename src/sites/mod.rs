pub mod cosplayjav_pl;
pub mod japonx_vip;

// --- external ---
use postgres::Connection;
use reqwest::{
    Client, ClientBuilder,
    header::HeaderMap,
};
// --- custom ---
use crate::conf::CONF;

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

    pub fn new_with_proxy(url: &str) -> Self {
        Crawler {
            request: Crawler::default_builder()
                .proxy(reqwest::Proxy::http(url).unwrap())
                .build()
                .unwrap()
        }
    }

    fn get_status(&self, url: &str) -> u16 {
        loop {
            match self.request
                .get(url)
                .send() {
                Ok(resp) => return resp.status().as_u16(),
                Err(e) => println!("At get_status() send(), {:?}", e),
            }
        }
    }

    fn get_text(&self, url: &str) -> String {
        loop {
            match self.request
                .get(url)
                .send() {
                Ok(mut resp) => match resp.text() {
                    Ok(text) => return text,
                    Err(e) => println!("At get_text() text(), {:?}", e),
                }
                Err(e) => println!("At get_text() send(), {:?}", e),
            }
        }
    }

    fn get_text_with_headers(&self, url: &str, headers: &HeaderMap) -> String {
        loop {
            match self.request
                .get(url)
                .headers(headers.clone())
                .send() {
                Ok(mut resp) => match resp.text() {
                    Ok(text) => return text,
                    Err(e) => println!("At get_text_with_headers() text(), {:?}", e),
                }
                Err(e) => println!("At get_text_with_headers() send(), {:?}", e),
            }
        }
    }

    fn _get_bytes(&self, url: &str) -> Vec<u8> {
        // --- std ---
        use std::io::Read;

        loop {
            let mut v = vec![];
            match self.request
                .get(url)
                .send() {
                Ok(mut resp) => match resp.read_to_end(&mut v) {
                    Ok(_) => return v,
                    Err(e) => println!("At get_bytes() read_to_end(), {:?}", e),
                }
                Err(e) => println!("At get_bytes() send(), {:?}", e),
            }
        }
    }
}

lazy_static! { static ref CRAWLER: Crawler = if let Some(ref url) = CONF.proxy { Crawler::new_with_proxy(url) } else { Crawler::new() };}

pub trait Post {
    fn print(&self);
    fn print_pretty(&self) {
        println!("\n🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉\n");
        self.print();
    }

    fn save_to_db(&self, conn: &Connection);
}

pub trait Site {
    // get
    fn is_database(&self) -> bool;
    fn is_verbose(&self) -> bool;

    // set
    fn database(&mut self);
    fn silent(&mut self);
    fn thread(&mut self, num: u32);
    fn after(&mut self, date: u32);
    fn recent(&mut self, num: u32);

    // collect
    fn collect_posts(&self, handles: &mut Vec<std::thread::JoinHandle<Option<Box<dyn Post + Send>>>>) {
        while let Some(handle) = handles.pop() {
            if let Some(post) = handle.join().unwrap() {
//                if self.is_verbose() { post.print_pretty(); }
//                if self.is_database() {
//                    let conn = if let Some(ref url) = crate::conf::CONF.database {
//                        Connection::connect(url.as_str(), postgres::TlsMode::None).unwrap()
//                    } else { panic!("please config database first"); };
//                    post.save_to_db(&conn);
//                }
            }
        }
    }

    // fetch and parse
    fn parse_post(&self, url: &str) -> Option<Box<dyn Post + Send>>;
    fn parse_posts_page(&self, html: String) -> bool;
    fn fetch_posts_pages(&self, last_page: u32, url: &str);

    // fetch
    fn fetch_all(&self);
}
