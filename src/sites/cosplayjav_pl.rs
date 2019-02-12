mod urls {
    pub const HOMEPAGE: &'static str = "http://cosplayjav.pl";
    pub const POSTS_PAGE: &'static str = "http://cosplayjav.pl/page/";
}

// --- std ---
use std::{
    collections::HashSet,
    fmt::{Formatter, Display, self},
};
// --- external ---
use reqwest::header::{COOKIE, USER_AGENT, HeaderMap};
use select::{
    document::Document,
    predicate::{Attr, Class, Name, Predicate},
};
// --- custom ---
use super::{CRAWLER, Post as PostTrait, Site};

#[derive(Debug, Default)]
struct Content {
    id: Option<String>,
    title: Option<String>,
    alternative_title: Option<String>,
    company: Option<String>,
    actress: Option<String>,
    in_premium_section_to: Option<String>,
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

impl PostType {
    fn to_i16(&self) -> i16 {
        // --- custom ---
        use self::PostType::*;

        match self {
            CosplayVideos => 0,
            OnlyImages => 1,
            Premium => 2,
            Wishlist => 3,
        }
    }
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

impl PostTrait for Post {
    fn print(&self) { println!("{}", self); }

    fn save_to_db(&self, conn: &postgres::Connection) {
        conn.execute(
            "INSERT INTO cosplayjav (\
                post_id,\
                post_likes,\
                post_title,\
                post_cover,\
                post_parts,\
                post_type,\
                content_id,\
                content_title,\
                conten_alternative_title,\
                content_company,\
                content_actress,\
                contetn_in_premium_section_to,\
                content_anmie_or_game_series,\
                content_character_cosplay,\
                content_info\
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)\
                ON CONFLICT (post_id) DO NOTHING;",
            &[
                &(self.id as i32),
                &(self.likes as i32),
                &self.title,
                &self.cover,
                &self.parts,
                &(self.r#type.to_i16()),
                &self.content.id,
                &self.content.title,
                &self.content.alternative_title,
                &self.content.company,
                &self.content.actress,
                &self.content.in_premium_section_to,
                &self.content.anmie_or_game_series,
                &self.content.character_cosplay,
                &self.content.info,
            ],
        ).unwrap();
    }
}

#[derive(Clone, Debug)]
pub struct Cosplayjav {
    database: bool,
    verbose: bool,
    thread: u32,
    after: Option<u32>,
    recent: Option<u32>,
    likes: HashSet<u32>,
    views: HashSet<u32>,
    headers: HeaderMap,
}

impl Cosplayjav {
    pub fn new() -> Cosplayjav {
        let headers = if CRAWLER.get_status(urls::HOMEPAGE) == 503 {
            // --- external ---
            use cloudflare_bypasser::Bypasser;

            let mut bypasser = Bypasser::new()
                .retry(10000)
                .user_agent("Mozilla/5.0");
            if let Some(ref proxy) = crate::conf::CONF.proxy { bypasser = bypasser.proxy(proxy); }

            println!("We're trying to bypass the cloudflare's anti-bot page, it might takes few seconds...");
            let mut h = HeaderMap::new();
            loop {
                if let Ok((c, ua)) = bypasser.bypass(urls::HOMEPAGE) {
                    h.insert(COOKIE, c);
                    h.insert(USER_AGENT, ua);
                    break;
                }
            }

            h
        } else { HeaderMap::new() };

        Cosplayjav {
            database: false,
            verbose: true,
            thread: 1,
            after: None,
            recent: None,
            likes: HashSet::new(),
            views: HashSet::new(),
            headers,
        }
    }
}

impl Site for Cosplayjav {
    fn is_database(&self) -> bool { self.database }
    fn is_verbose(&self) -> bool { self.verbose }

    fn database(&mut self) { self.database = true; }
    fn silent(&mut self) { self.verbose = false; }
    fn thread(&mut self, num: u32) { self.thread = num; }
    fn after(&mut self, date: u32) { self.after = Some(date); }
    fn recent(&mut self, num: u32) { self.recent = Some(num); }

    fn parse_post(&self, url: &str) -> Option<Box<dyn PostTrait + Send>> {
        // --- external ---
        use regex::Regex;

        let html = CRAWLER.get_text_with_headers(url, &self.headers);
        let document = Document::from(html.as_str());

        let re = Regex::new(r"cosplayjav.pl/(\d+)").unwrap();
        let id = re.captures(url)
            .unwrap()[1]
            .to_string()
            .parse()
            .unwrap();
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
            use std::{
                sync::Arc,
                thread::spawn,
            };

            match r#type {
                PostType::Premium | PostType::Wishlist => vec![],
                _ => {
                    let headers = Arc::new(self.headers.clone());
                    let mut v = vec![];
                    let mut handles = vec![];

                    for item_part in document.find(Class("item-parts").descendant(Name("a"))) {
                        let url = item_part.attr("href").unwrap().to_owned();
                        if url.ends_with("torrents") { v.push(url); } else if url.ends_with("alternative") { continue; } else {
                            let headers = headers.clone();
                            handles.push(spawn(move || {
                                let download_page = CRAWLER.get_text_with_headers(&url, &headers);
                                let document = Document::from(download_page.as_str());

                                document.find(Attr("class", "btn btn-primary btn-download"))
                                    .next()
                                    .unwrap()
                                    .attr("href")
                                    .unwrap()
                                    .to_owned()
                            }));
                        }
                    }

                    for handle in handles { v.push(handle.join().unwrap()); }
                    if v.is_empty() { return None; } else { v }
                }
            }
        };
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
        let content = {
            let mut c = Content::default();
            if let Some(item_info) = document.find(Class("item-info")).next() {
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

                for info in item_info.find(Name("table").descendant(Name("tr"))) {
                    let mut kv = info.find(Name("td"));
                    if let Some(k) = kv.next() {
                        if let Some(v) = kv.next() {
                            match k.text().replace('–', "").trim() {
                                "ID" => c.id = Some(v.text()),
                                "TITLE" => c.title = Some(v.text()),
                                "ALTERNATIVE TITLE" => c.alternative_title = Some(v.text()),
                                "COMPANY" => c.company = Some(v.text()),
                                "ACTRESS" | "ACRESS" => c.actress = Some(v.text()),
                                k if k.contains("PREMIUM") => c.in_premium_section_to = Some(v.text()),
                                "ANIME/GAME SERIES" => c.anmie_or_game_series = Some(split_multi_info(v)),
                                "CHARACTER COSPLAY" => c.character_cosplay = Some(split_multi_info(v)),
                                "INFO" => c.info = Some(split_multi_info(v)),
                                _ => continue
                            }
                        }
                    } else { continue; }
                }
            }

            c
        };

        Some(Box::new(Post { id, likes, title, cover, parts, r#type, content }))
    }

    fn parse_posts_page(&self, html: String) -> bool {
        // --- std ---
        use std::{
            sync::Arc,
            thread::spawn,
        };

        let cosplayjav = Arc::new(self.clone());
        let document = Document::from(html.as_str());
        let mut handles = vec![];

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
                        _ => unreachable!()
                    };
                    let year = date.next().unwrap();

                    format!("{}{}{}", year, month, day).parse().unwrap()
                };

                if after > date {
                    self.collect_posts(&mut handles);
                    return true;
                }
            }

            let url = article.find(Name("a"))
                .next()
                .unwrap()
                .attr("href")
                .unwrap()
                .to_owned();

            {
                let cosplayjav = cosplayjav.clone();
                handles.push(spawn(move || cosplayjav.parse_post(&url)));
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
            let html = CRAWLER.get_text_with_headers(&format!("{}{}", url, page_num), &self.headers);
            if self.parse_posts_page(html) { return; }
        }
    }

    fn fetch_all(&self) {
        let last_page: u32 = {
            let html = CRAWLER.get_text_with_headers(&format!("{}{}", urls::POSTS_PAGE, 1), &self.headers);
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

        self.fetch_posts_pages(last_page, urls::POSTS_PAGE);
    }
}
