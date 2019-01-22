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
use super::{CRAWLER, Post as SitePost, Site};

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

#[derive(Clone, Debug)]
pub struct Japonx {
    thread: u32,
    after: Option<u32>,
    recent: Option<u32>,
    last_view: Option<u32>,
    likes: HashSet<u32>,
    views: HashSet<u32>,
}

impl Japonx {
    pub fn new() -> Japonx {
        Japonx {
            thread: 1,
            after: None,
            recent: None,
            last_view: None,
            likes: HashSet::new(),
            views: HashSet::new(),
        }
    }
}

impl Site for Japonx {
    fn thread(&mut self, num: u32) { self.thread = num; }
    fn after(&mut self, date: u32) { self.after = Some(date); }
    fn recent(&mut self, num: u32) { self.recent = Some(num); }

    fn parse_post(&self, url: &str) -> SitePost {
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
                    3 => c.release_date = info.find(Name("a"))
                        .next()
                        .unwrap()
                        .text()
                        .replace('-', "")
                        .parse()
                        .unwrap(),
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

        SitePost::Japonx(Post { id, title, intro, cover, content })
    }

    fn parse_posts(&self, html: String) -> Vec<SitePost> {
        let document = Document::from(html.as_str());
        let mut posts = vec![];

        for (i, work) in document.find(Attr("id", "works").descendant(Name("li"))).enumerate() {
            let url = work.find(Name("a"))
                .next()
                .unwrap()
                .attr("href")
                .unwrap();

            let post = self.parse_post(&format!("https://www.japonx.vip{}", url));
            if let Some(after) = self.after {
                match post {
                    SitePost::Japonx(
                        Post {
                            content: PostContent { release_date, .. },
                            ..
                        }
                    ) => if after > release_date { return posts; }
                    _ => unreachable!()
                }

                println!("{}", post);
                posts.push(post);
            } else {
                println!("{}", post);
                posts.push(self.parse_post(&format!("https://www.japonx.vip{}", url)));
            }

            if let Some(recent) = self.recent { if i as u32 + 1 == recent { return posts; } }
        }

        posts
    }

    fn fetch_posts(&self) {
        // --- std ---
        use std::{
            thread::spawn,
            sync::{Arc, Mutex},
        };

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
        let japonx = Arc::new(self.clone());
        let page_num = Mutex::new(1);

        'fetch: loop {
            let mut handles = vec![];
            for _ in 0..self.thread {
                let page_num = {
                    let mut page_num = page_num.lock().unwrap();
                    *page_num += 1;

                    page_num.clone()
                };

                let japonx = japonx.clone();
                handles.push(spawn(move || {
                    let html = CRAWLER.get_text(&format!("{}{}", urls::LATEST_POSTS_PAGE, page_num));
                    let _posts = japonx.parse_posts(html);
                }));

                if page_num > last_page {
                    for handle in handles { handle.join().unwrap(); }
                    break 'fetch;
                }
            }

            for handle in handles { handle.join().unwrap(); }
        }
    }
}
