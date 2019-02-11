extern crate clap;
extern crate cloudflare_bypasser;
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

mod conf;
mod sites;

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
            .arg(Arg::with_name("show")
                .long("show")
                .help("Show configurations")
                .conflicts_with("proxy"))
            .arg(Arg::with_name("proxy")
                .long("proxy")
                .value_name("ADDRESS")
                .help("Use proxy with specify ADDRESS, format: [URL][PORT] http://127.0.0.1:1080")
                .conflicts_with("show")))
        .arg(Arg::with_name("site")
            .short("s")
            .long("site")
            .value_name("NAME")
            .possible_values(&["cosplayjav", "japonx"])
            .help("The site that you want"))
        .arg(Arg::with_name("fetch")
            .short("f")
            .long("fetch")
            .help("Fetch all posts")
            .requires("site")
            .conflicts_with("parse"))
        .arg(Arg::with_name("thread")
            .short("t")
            .long("thread")
            .value_name("NUM")
            .help("Specify crawler's thread NUM, the max thread amount depends on the page's posts amount")
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
            .help("Specify the post's url")
            .requires("site")
            .conflicts_with("fetch"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("config") {
        // --- custom ---
        use conf::CONF;

        let mut conf = CONF.lock().unwrap();

        if matches.is_present("show") {
            println!("Conf: {}", serde_json::to_string_pretty(&*conf).unwrap());
            return;
        }

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
        if let Some(post) = site.parse_post(url) {
            println!("{}", post);
            return;
        } else { println!("Invalid post"); }
    }

    if matches.is_present("fetch") {
        if let Some(num) = matches.value_of("thread") { site.thread(num.parse().unwrap()); }
        if let Some(date) = matches.value_of("after") { site.after(date.parse().unwrap()); }
        if let Some(num) = matches.value_of("recent") { site.recent(num.parse().unwrap()); }

        site.fetch_all();
        return;
    }
}
