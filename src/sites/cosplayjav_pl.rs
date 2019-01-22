mod urls { pub const POSTS_PAGE: &'static str = "http://cosplayjav.pl/page/"; }

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
struct Content {
    id: Option<String>,
    title: Option<String>,
    alternative_title: Option<String>,
    company: Option<String>,
    actress: Option<String>,
    anmie_or_game_series: Option<Vec<String>>,
    character_cosplay: Option<Vec<String>>,
    info: Option<Vec<String>>,
}

#[derive(Debug)]
enum PostType {
    CosplayVideos,
    OnlyImages,
    Premium,
    Wishlist,
}

impl Display for PostType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PostType::CosplayVideos => "Cosplay Videos",
                PostType::OnlyImages => "Only Images",
                PostType::Premium => "Premium",
                PostType::Wishlist => "WISHLIST",
            }
        )
    }
}

#[derive(Debug)]
pub struct Post {
    id: u32,
    likes: u32,
    title: String,
    cover: String,
    parts: Vec<String>,
    r#type: PostType,
    content: Content,
}

impl Display for Post {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // --- external ---
        use colored::*;

        let parts = self.parts
            .iter()
            .enumerate()
            .map(|(i, part)| format!("        {}: {}", format!("{} {}", "Part".cyan(), i + 1), part))
            .collect::<Vec<_>>()
            .join("\n");
        let content = {
            let mut s = String::new();

            if let Some(id) = &self.content.id { s.push_str(&format!("        {}: {}\n", "Id".cyan(), id)); }
            if let Some(title) = &self.content.title { s.push_str(&format!("        {}: {}\n", "Title".cyan(), title)); }
            if let Some(alternative_title) = &self.content.alternative_title { s.push_str(&format!("        {}: {}\n", "Alternative title".cyan(), alternative_title)); }
            if let Some(company) = &self.content.company { s.push_str(&format!("        {}: {}\n", "Company".cyan(), company)); }
            if let Some(actress) = &self.content.actress { s.push_str(&format!("        {}: {}\n", "Actress".cyan(), actress)); }
            if let Some(anmie_or_game_series) = &self.content.anmie_or_game_series { s.push_str(&format!("        {}: {}\n", "Anmie/Game eries".cyan(), anmie_or_game_series.join(", "))); }
            if let Some(character_cosplay) = &self.content.character_cosplay { s.push_str(&format!("        {}: {}\n", "Character cosplay".cyan(), character_cosplay.join(", "))); }
            if let Some(info) = &self.content.info { s.push_str(&format!("        {}: {}\n", "Info".cyan(), info.join(", "))); }

            s
        };

        write!(
            f,
            "{} http://cosplayjav.pl/{}:\n    {}: {}\n    {}: {}\n    {}: {}\n    {}: {}\n    {}:\n{}    {}:\n{}",
            "Post".cyan(),
            self.id,
            "Title".yellow(),
            self.title,
            "Type".yellow(),
            self.r#type,
            "Cover".yellow(),
            self.cover,
            "Likes".yellow(),
            self.likes,
            "Content".yellow(),
            content,
            "Parts".yellow(),
            parts,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Cosplayjav {
    thread: u32,
    after: Option<u32>,
    recent: Option<u32>,
    last_view: Option<u32>,
    likes: HashSet<u32>,
    views: HashSet<u32>,
}

impl Cosplayjav {
    pub fn new() -> Cosplayjav {
        Cosplayjav {
            thread: 1,
            after: None,
            recent: None,
            last_view: None,
            likes: HashSet::new(),
            views: HashSet::new(),
        }
    }
}

impl Site for Cosplayjav {
    fn thread(&mut self, num: u32) { self.thread = num; }
    fn after(&mut self, date: u32) { self.after = Some(date); }
    fn recent(&mut self, num: u32) { self.recent = Some(num); }

    fn parse_post(&self, url: &str) -> SitePost {
        // --- external ---
        use regex::Regex;

        let re = Regex::new(r"cosplayjav.pl/(\d+?)/").unwrap();
        let id = re.captures(url)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let html = CRAWLER.get_text(url);
        let document = Document::from(html.as_str());

        let likes = document.find(Class("favourites-li"))
            .next()
            .unwrap()
            .text()
            .parse()
            .unwrap();
        let title = document.find(Name("h1"))
            .next()
            .unwrap()
            .text()
            .to_owned();
        let cover = document.find(Class("post-thumb").descendant(Name("img")))
            .next()
            .unwrap()
            .attr("src")
            .unwrap()
            .to_owned();
        let r#type = {
            let s = document.find(Class("post-aside"))
                .next()
                .unwrap()
                .text();

            if s.contains("videos") {
                PostType::CosplayVideos
            } else if s.contains("images") {
                PostType::OnlyImages
            } else if s.contains("Premium") {
                PostType::Premium
            } else {
                PostType::Wishlist
            }
        };
        let parts = {
            // --- std ---
            use std::thread::spawn;

            match r#type {
                PostType::Premium | PostType::Wishlist => vec![],
                _ => {
                    let parts_len = document.find(Class("item-parts").descendant(Name("a")))
                        .into_iter()
                        .fold(0u8, |acc, _| acc + 1);

                    let mut handles = vec![];
                    for part in 1..=parts_len {
                        handles.push(spawn(move || {
                            let download_page = CRAWLER.get_text(&format!("http://cosplayjav.pl/download/?forPost={}&part={}", id, part));
                            let document = Document::from(download_page.as_str());

                            document.find(Attr("class", "btn btn-primary btn-download"))
                                .next()
                                .unwrap()
                                .attr("href")
                                .unwrap()
                                .to_owned()
                        }));
                    }

                    let mut v = vec![];
                    for handle in handles { v.push(handle.join().unwrap()); }

                    v
                }
            }
        };
        let content = {
            // --- external ---
            use select::node::Node;

            fn split_multi_info(infos: Node) -> Vec<String> {
                let v: Vec<_> = infos.find(Name("a"))
                    .into_iter()
                    .map(|info| info.text())
                    .collect();

                if v.is_empty() {
                    let infos = infos.text();
                    if infos.contains('\n') { infos.lines().map(|line| line.to_owned()).collect() } else {
                        infos.split('/')
                            .map(|info| info.trim().to_owned())
                            .filter(|info| !info.is_empty())
                            .collect()
                    }
                } else { v }
            }

            let mut c = Content::default();
            for info in document.find(
                Class("item-info").descendant(Name("table").descendant(Name("tr")))) {
                let (k, v) = {
                    let mut kv = info.find(Name("td"));
                    (kv.next().unwrap().text(), kv.next().unwrap())
                };

                match k.trim_end_matches(" – ") {
                    "ID" => c.id = Some(v.text()),
                    "TITLE" => c.title = Some(v.text()),
                    "ALTERNATIVE TITLE" => c.alternative_title = Some(v.text()),
                    "COMPANY" => c.company = Some(v.text()),
                    "ACTRESS" => c.actress = Some(v.text()),
                    "ANIME/GAME SERIES" => c.anmie_or_game_series = Some(split_multi_info(v)),
                    "CHARACTER COSPLAY" => c.character_cosplay = Some(split_multi_info(v)),
                    "INFO" => c.info = Some(split_multi_info(v)),
                    not_covered => panic!("Found not covered field {}", not_covered)
                }
            }

            c
        };

        SitePost::Cosplayjav(Post { id, likes, title, cover, parts, r#type, content })
    }

    fn parse_posts(&self, html: String) -> Vec<SitePost> {
        let document = Document::from(html.as_str());
        let mut posts = vec![];

        for (i, article) in document.find(Attr("id", "main-section").descendant(Name("article"))).enumerate() {
            if let Some(after) = self.after {
                let date = article.find(Class("post-aside").descendant(Name("a")))
                    .next()
                    .unwrap()
                    .text();
                let date: u32 = {
                    let mut date = date.split(' ');
                    let day = date.next().unwrap();
                    let month = match date.next().unwrap() {
                        "January" => "01",
                        "February" => "02",
                        "March" => "03",
                        "April" => "04",
                        "May" => "05",
                        "June" => "06",
                        "July" => "07",
                        "August" => "08",
                        "September" => "09",
                        "October" => "10",
                        "November" => "11",
                        "December" => "12",
                        _ => panic!("Invalid month")
                    };
                    let year = date.next().unwrap();

                    format!("{}{}{}", year, month, day).parse().unwrap()
                };


                if after > date { return posts; }
            }

            let url = article.find(Name("a"))
                .next()
                .unwrap()
                .attr("href")
                .unwrap();

            let post = self.parse_post(url);
            println!("{}", post);
            posts.push(post);

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
            let html = CRAWLER.get_text(&format!("{}{}", urls::POSTS_PAGE, 1));
            let document = Document::from(html.as_str());

            document.find(Attr("id", "pagination-elem"))
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
        let cosplayjav = Arc::new(self.clone());
        let page_num = Mutex::new(1);

        'fetch: loop {
            let mut handles = vec![];
            for _ in 0..self.thread {
                let page_num = {
                    let mut page_num = page_num.lock().unwrap();
                    *page_num += 1;

                    page_num.clone()
                };

                let cosplayjav = cosplayjav.clone();
                handles.push(spawn(move || {
                    let html = CRAWLER.get_text(&format!("{}{}", urls::POSTS_PAGE, page_num));
                    let _posts = cosplayjav.parse_posts(html);
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
