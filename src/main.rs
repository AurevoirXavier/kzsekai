extern crate clap;
extern crate colored;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate select;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod sites;

#[derive(Serialize, Deserialize)]
struct Conf { proxy: Option<String> }

impl Conf {
    fn path() -> String {
        // --- std ---
        use std::env::current_exe;

        format!(
            "{}/sexy_conf.json",
            current_exe().unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
        )
    }

    fn load_from_json_file() -> Conf {
        // --- std ---
        use std::{
            fs::File,
            path::Path,
        };

        let path = Conf::path();
        if Path::new(&path).is_file() { serde_json::from_reader(File::open(&path).unwrap()).unwrap() } else { Conf::default() }
    }

    fn save_to_json_file(&self) {
        // --- std ---
        use std::fs::File;

        serde_json::to_writer_pretty(&mut File::create(&Conf::path()).unwrap(), self).unwrap()
    }
}

impl Default for Conf { fn default() -> Conf { Conf { proxy: None } } }

lazy_static! { static ref CONF: Conf = Conf::load_from_json_file(); }

fn main() {
    // --- external ---
    use clap::{Arg, App, SubCommand};
    // --- custom ---
    use sites::Site;

    let matches = App::new("sexy")
        .version("0.1.0 alpha version")
        .author("Xavier Lau <c.estlavie@icloud.com>")
        .about("🎉🎉 Sexy Time 🎉🎉")
        .subcommand(SubCommand::with_name("config")
            .about("Configurations of sexy")
            .arg(Arg::with_name("proxy")
                .long("proxy")
                .value_name("ADDRESS")
                .help("Use proxy with specify ADDRESS, format: [URL][PORT] http://127.0.0.1:1080")))
        .arg(Arg::with_name("site")
            .short("s")
            .long("site")
            .value_name("NAME")
            .possible_values(&["cosplayjav", "japonx"])
            .help("The site that you want"))
        .arg(Arg::with_name("fetch")
            .short("f")
            .long("fetch")
            .help("Fetch all Posts")
            .requires("site")
            .conflicts_with("parse"))
        .arg(Arg::with_name("thread")
            .short("t")
            .long("thread")
            .value_name("NUM")
            .help("Specify crawler's thread NUM")
            .requires_all(&["site", "fetch"])
            .conflicts_with("parse"))
        .arg(Arg::with_name("after")
            .long("after")
            .value_name("DATE")
            .help("Fetch the posts whose date after specify DATE, format: [year][month][day] 20190101")
            .requires_all(&["site", "fetch"])
            .conflicts_with("parse"))
        .arg(Arg::with_name("recent")
            .long("recent")
            .value_name("NUM")
            .help("Fetch recent specify NUM posts")
            .requires_all(&["site", "fetch"])
            .conflicts_with("parse"))
        .arg(Arg::with_name("parse")
            .short("p")
            .long("parse")
            .value_name("URL")
            .help("Specify the Post's url")
            .requires("site")
            .conflicts_with("fetch"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("config") {
        let mut conf = Conf::load_from_json_file();

        if let Some(address) = matches.value_of("proxy") { if address.is_empty() { conf.proxy = None; } else { conf.proxy = Some(address.to_owned()); } }

        conf.save_to_json_file();

        return;
    }

    let mut site: Box<dyn Site> = if let Some(site) = matches.value_of("site") {
        match site {
            "cosplayjav" => Box::new(sites::cosplayjav_pl::Cosplayjav::new()),
            "japonx" => Box::new(sites::japonx_vip::Japonx::new()),
            site => panic!("Not support {}", site)
        }
    } else {
        println!("{}", matches.usage());
        return;
    };

    if let Some(url) = matches.value_of("parse") {
        site.parse_post(url);
        return;
    }

    if matches.is_present("fetch") {
        if let Some(num) = matches.value_of("thread") { site.thread(num.parse().unwrap()); }
        if let Some(date) = matches.value_of("after") { site.after(date.parse().unwrap()); }
        if let Some(num) = matches.value_of("recent") { site.recent(num.parse().unwrap()); }

        site.fetch_posts();
        return;
    }
}
