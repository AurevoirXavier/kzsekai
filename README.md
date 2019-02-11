#### Intro

Just Sexy.

#### Installation

1.  Complie from source (recommend)
   - `git clone https://github.com/AurevoirXavier/sexy.git`
   - `cargo build --release` (rust version 1.34.0 nightly)
2.  Download release
   - [OS X Mojave (10.14.3 18D42)](https://github.com/AurevoirXavier/xmly-exporter/releases/download/1.0/xmly-exporter)
   - [Windows](#): Not yet
   - [Linux](#): Not yet
   
 #### Require
 
 - **Node.js**: to pass the cloudflare's anti-bot test.

#### Usage

`cargo run --release -- --help` or `sexy --help` or `sexy.exe --help`:

```text
sexy 0.1.0 alpha version
Xavier Lau <c.estlavie@icloud.com>
🎉🎉 Sexy Time 🎉🎉

USAGE:
    sexy [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
        --database    Save to database
    -f, --fetch       Fetch all posts
    -h, --help        Prints help information
        --silent      Capture the output
    -V, --version     Prints version information

OPTIONS:
        --after <DATE>    Fetch the posts whose date after specify DATE, format: [year][month][day] 20190101
    -p, --parse <URL>     Specify the post's url
        --recent <NUM>    Fetch recent specify NUM posts
    -s, --site <NAME>     The site that you want [possible values: cosplayjav, japonx]
    -t, --thread <NUM>    Specify crawler's thread NUM, the max thread amount depends on the page's posts amount

SUBCOMMANDS:
    config    Configurations of sexy
    help      Prints this message or the help of the given subcommand(s)
```

#### Screenshot

![screenshot](demo.png)
