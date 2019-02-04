## Intro

Just Sexy.

#### Installation

1.  Complie from source (recommend)
   - `git clone https://github.com/AurevoirXavier/sexy.git`
   - `cargo build --release` (rust version 1.33.0 nightly)
2.  Download release
   - [OS X Mojave (10.14.3 18D42)](https://github.com/AurevoirXavier/xmly-exporter/releases/download/1.0/xmly-exporter)
   - [Windows](#): Not yet
   - [Linux](#): Not yet

#### Usage

`cargo run --release -- --help` or `sexy --help` or `sexy.exe --help`:

```text
USAGE:
    sexy [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -f, --fetch      Fetch all posts
    -h, --help       Prints help information
    -V, --version    Prints version information

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

#### Example




#### Screenshot

![screenshot](demo.png)
