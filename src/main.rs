extern crate clap;
extern crate cloudflare_bypasser;
extern crate colored;
#[macro_use]
extern crate lazy_static;
extern crate postgres;
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
        .version("v0.1.2-beta version")
        .author("Xavier Lau <c.estlavie@icloud.com>")
        .about("🎉🎉 Sexy Time 🎉🎉")
        .subcommand(SubCommand::with_name("config")
            .about("Configurations of sexy")
            .arg(Arg::with_name("show")
                .long("show")
                .help("Show configurations")
                .conflicts_with_all(&["proxy", "database"]))
            .arg(Arg::with_name("database")
                .long("database")
                .value_name("URL")
                .help("Set database URL, format: postgresql://user[:password]@host[:port][/database][?param1=val1[[&param2=val2]...]]")
                .conflicts_with("show"))
            .arg(Arg::with_name("proxy")
                .long("proxy")
                .value_name("URL")
                .help("Use proxy with specify URL, format: [URL][PORT] http://127.0.0.1:1080")
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
            .conflicts_with("parse")
            .requires("site"))
        .arg(Arg::with_name("thread")
            .short("t")
            .long("thread")
            .value_name("NUM")
            .help("Specify crawler's thread NUM, the max thread amount depends on the page's posts amount")
            .conflicts_with("parse")
            .requires_all(&["site", "fetch"]))
        .arg(Arg::with_name("after")
            .long("after")
            .value_name("DATE")
            .help("Fetch the posts whose date after specify DATE, format: [year][month][day] 20190101")
            .conflicts_with("parse")
            .requires_all(&["site", "fetch"]))
        .arg(Arg::with_name("recent")
            .long("recent")
            .value_name("NUM")
            .help("Fetch recent specify NUM posts")
            .conflicts_with("parse")
            .requires_all(&["site", "fetch"]))
        .arg(Arg::with_name("database")
            .long("database")
            .help("Save to database")
            .conflicts_with("parse")
            .requires_all(&["site", "fetch"]))
        .arg(Arg::with_name("silent")
            .long("silent")
            .help("Capture the output")
            .conflicts_with("parse")
            .requires_all(&["site", "fetch"]))
        .arg(Arg::with_name("parse")
            .short("p")
            .long("parse")
            .value_name("URL")
            .help("Specify the post's url")
            .conflicts_with("fetch")
            .requires("site"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("config") {
        let mut conf = conf::CONF.clone();

        if matches.is_present("show") {
            println!("Conf: {}", serde_json::to_string_pretty(&conf).unwrap());
            return;
        }

        if let Some(url) = matches.value_of("proxy") {
            if url.is_empty() { conf.proxy = None; } else { conf.proxy = Some(url.to_owned()); }
            conf.save_to_json_file();
        }
        if let Some(url) = matches.value_of("database") {
            if url.is_empty() { conf.database = None; } else { conf.database = Some(url.to_owned()); }
            conf.save_to_json_file();
        }

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
            post.print_pretty();
            return;
        } else { println!("Invalid post"); }
    }

    if matches.is_present("fetch") {
        if matches.is_present("database") { site.database(); }
        if matches.is_present("silent") { site.silent(); }
        if let Some(num) = matches.value_of("thread") { site.thread(num.parse().unwrap()); }
        if let Some(date) = matches.value_of("after") { site.after(date.parse().unwrap()); }
        if let Some(num) = matches.value_of("recent") { site.recent(num.parse().unwrap()); }

        site.fetch_all();

        return;
    }
}
