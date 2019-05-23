mod urls { pub const LATEST_POSTS_PAGE: &'static str = "https://www.japonx.vip/portal/index/search/new/1.html?page="; }

// --- std ---
use std::{
    collections::HashSet,
    fmt::{Formatter, Display, self},
};
// --- external ---
use select::{
    document::Document,
    predicate::{Attr, Class, Name, Predicate},
};
// --- custom ---
use super::{CRAWLER, Post as PostTrait, Site};

#[derive(Debug, Default)]
struct PostContent {
    id: String,
    length: u32,
    release_date: u32,
    actress: String,
    company: String,
    subtitles: String,
    r#type: Vec<String>,
}

#[derive(Debug)]
pub struct Post {
    id: u32,
    title: String,
    intro: String,
    cover: String,
    content: PostContent,
}

impl Display for Post {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // --- external ---
        use colored::*;

        let content = {
            let mut s = String::new();

            s.push_str(&format!(
                "        {}: {}\n        {}: {}\n        {}: {}\n        {}: {}\n        {}: {}\n        {}: {}",
                "Id".cyan(),
                &self.content.id,
                "Length".cyan(),
                &self.content.length,
                "Release date".cyan(),
                &self.content.release_date,
                "Company".cyan(),
                &self.content.company,
                "Subtitles".cyan(),
                &self.content.subtitles,
                "Type".cyan(),
                &self.content.r#type.join(", "),
            ));

            s
        };

        write!(
            f,
            "{} https://www.japonx.vip/portal/index/detail/id/{}.html:\n    {}: {}\n    {}: {}\n    {}: {}\n    {}:\n{}",
            "Post".cyan(),
            self.id,
            "Title".yellow(),
            self.title,
            "Intro".yellow(),
            self.intro,
            "Cover".yellow(),
            self.cover,
            "Content".yellow(),
            content,
        )
    }
}

impl PostTrait for Post {
    fn print(&self) { println!("{}", self); }

    fn save_to_db(&self, conn: &postgres::Connection) {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct Japonx {
    database: bool,
    verbose: bool,
    thread: u32,
    after: Option<u32>,
    recent: Option<u32>,
    likes: HashSet<u32>,
    views: HashSet<u32>,
}

impl Japonx {
    pub fn new() -> Japonx {
        Japonx {
            database: false,
            verbose: true,
            thread: 1,
            after: None,
            recent: None,
            likes: HashSet::new(),
            views: HashSet::new(),
        }
    }
}

impl Site for Japonx {
    fn is_database(&self) -> bool { self.database }
    fn is_verbose(&self) -> bool { self.verbose }

    fn database(&mut self) { self.database = true; }
    fn silent(&mut self) { self.verbose = false; }
    fn after(&mut self, date: u32) { self.after = Some(date); }
    fn recent(&mut self, num: u32) { self.recent = Some(num); }
    fn thread(&mut self, num: u32) { self.thread = num; }

    fn parse_post(&self, url: &str) -> Option<Box<dyn PostTrait + Send>> {
        // --- external ---
        use regex::Regex;

        let re = Regex::new(r"id/(\d+).html").unwrap();
        let id = re.captures(url)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let html = CRAWLER.get_text(url);
        let document = Document::from(html.as_str());

        let title = document.find(Name("h1"))
            .next()
            .unwrap()
            .text()
            .to_owned();
        let intro = document.find(Class("tx-comment"))
            .next()
            .unwrap()
            .text();
        let cover = document.find(Attr("id", "do_play_1"))
            .next()
            .unwrap()
            .attr("src")
            .unwrap()
            .to_owned();
        let content = {
            let mut c = PostContent::default();
            for (i, info) in document.find(Class("desc").descendant(Name("dd"))).enumerate() {
                match i {
                    0 => c.id = info.find(Name("a"))
                        .next()
                        .unwrap()
                        .text(),
                    1 => c.length = info.find(Name("a"))
                        .next()
                        .unwrap()
                        .text()
                        .parse()
                        .unwrap(),
                    2 => c.actress = info.find(Name("a"))
                        .next()
                        .unwrap()
                        .text(),
                    3 => {
                        let release_date = info.find(Name("a"))
                            .next()
                            .unwrap()
                            .text()
                            .replace('-', "")
                            .parse()
                            .unwrap();

                        if let Some(after) = self.after { if after > release_date { return None; } }

                        c.release_date = release_date;
                    }
                    4 => for info in info.find(Name("a")) { c.r#type.push(info.text()); }
                    5 => c.company = info.find(Name("a"))
                        .next()
                        .unwrap()
                        .text(),
                    6 => {
                        c.subtitles = info.find(Name("a"))
                            .next()
                            .unwrap()
                            .text();

                        break;
                    }
                    _ => unreachable!()
                }
            }

            c
        };

        Some(Box::new(Post { id, title, intro, cover, content }))
    }

    fn parse_posts_page(&self, document: Document) -> bool {
        // --- std ---
        use std::{
            sync::Arc,
            thread::spawn,
        };

        let japonx = Arc::new(self.clone());
        let mut handles = vec![];

        for (i, work) in document.find(Attr("id", "works").descendant(Name("li"))).enumerate() {
            let url = work.find(Name("a"))
                .next()
                .unwrap()
                .attr("href")
                .unwrap()
                .to_owned();

            {
                let japonx = japonx.clone();
                handles.push(spawn(move || japonx.parse_post(&format!("https://www.japonx.vip{}", url))));
                if handles.len() as u32 == self.thread { self.collect_posts(&mut handles); }
            }

            if let Some(recent) = self.recent {
                if i as u32 + 1 == recent {
                    self.collect_posts(&mut handles);
                    return true;
                }
            }
        }

        self.collect_posts(&mut handles);
        false
    }

    fn fetch_posts_pages(&self, last_page: u32, url: &str) {
        for page_num in 1..last_page {
            let html = CRAWLER.get_text(&format!("{}{}", url, page_num));
            if self.parse_posts_page(Document::from(html.as_str())) { return; }
        }
    }

    fn fetch_all(&self) {
        let last_page: u32 = {
            let html = CRAWLER.get_text(&format!("{}{}", urls::LATEST_POSTS_PAGE, 1));
            let document = Document::from(html.as_str());

            document.find(Class("bx-pagination"))
                .next()
                .unwrap()
                .find(Name("a"))
                .skip(6)
                .next()
                .unwrap()
                .text()
                .parse()
                .unwrap()
        };

        self.fetch_posts_pages(last_page, urls::LATEST_POSTS_PAGE);
    }
}
