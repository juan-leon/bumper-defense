use clap::{self, Arg, Command};

pub fn get_app() -> Command<'static> {
    Command::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .max_term_width(100)
        .arg(
            Arg::new("width")
                .long("width")
                .short('w')
                .help("Window width")
                .default_value("2560")
                .takes_value(true),
        )
        .arg(
            Arg::new("height")
                .long("height")
                .short('h')
                .help("Window height")
                .default_value("1440")
                .takes_value(true),
        )
        .arg(
            Arg::new("seed")
                .long("seed")
                .short('s')
                .help("Random seed")
                .default_value("0")
                .takes_value(true),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Show debug info")
                .takes_value(false),
        )
}
