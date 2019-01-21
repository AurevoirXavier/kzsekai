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
    subtitles: String,
    r#type: Vec<String>,
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

#[derive(Debug)]
pub struct Post {
    id: u32,
    title: String,
    intro: String,
    cover: String,
    content: PostContent,
}

#[derive(Debug)]
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
                    5 => {
                        c.actress = info.find(Name("a"))
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

        let post = Post { id, title, intro, cover, content };
        println!("\n🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉 🎉\n\n{}", post);
        SitePost::Japonx(post)
    }

    fn parse_posts(&self, html: String) -> (Vec<SitePost>, bool) {
        let document = Document::from(html.as_str());
        let mut is_lage_page = true;
        let mut posts = vec![];

        for (i, work) in document.find(Attr("id", "works").descendant(Name("li"))).enumerate() {
            is_lage_page = false;

            let url = work.find(Name("a"))
                .next()
                .unwrap()
                .attr("href")
                .unwrap();

            posts.push(self.parse_post(url));

            if let Some(recent) = self.recent { if i as u32 + 1 == recent { return (posts, true); } }
        }

        (posts, is_lage_page)
    }

    fn fetch_posts(&self) {
        for page_num in 1u32.. {
            let html = CRAWLER.get_text(&format!("{}{}", urls::LATEST_POSTS_PAGE, page_num));
            let (_posts, is_last_page) = self.parse_posts(html);

            if is_last_page { return; }
        }
    }
}
