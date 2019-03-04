extern crate clap;
#[macro_use] extern crate derive_more;
extern crate secure_routing_stats;

use clap::App;
use clap::Arg;
use clap::SubCommand;
use secure_routing_stats::report::world::{
    self,
    WorldStatsOpts,
    WorldStatsReport
};
use secure_routing_stats::report::resources::{
    self,
    ResourceReporter,
    ResourceReportOpts
};

fn main() {
    match Options::create() {
        Err(e) => {
            eprintln!("{}", e);
            ::std::process::exit(1);
        },
        Ok(option) => {
            let res = match option {
                Options::WorldStats(opts) => {
                    WorldStatsReport::execute(&opts)
                        .map_err(Error::WorldReportError)
                }
                Options::ResourceStats(opts) => {
                    ResourceReporter::execute(&opts)
                        .map_err(Error::ResourceReportError)
                }
            };
            match res {
                Ok(()) => {},
                Err(e) => {
                    eprintln!("{}", e);
                    ::std::process::exit(1);
                }
            }
        }
    }
}

enum Options {
    WorldStats(WorldStatsOpts),
    ResourceStats(ResourceReportOpts)
}

impl Options {
    pub fn create() -> Result<Self, Error> {
        let matches = App::new("NLnet Labs RRDP Server")
            .version("0.1b")
            .about("Analyse ROA quality vs BGP")
            .arg(Arg::with_name("ris4")
                .short("4")
                .long("ris4")
                .value_name("FILE")
                .help("RIS dump v4.")
                .required(true))
            .arg(Arg::with_name("ris6")
                .short("6")
                .long("ris6")
                .value_name("FILE")
                .help("RIS dump v6.")
                .required(true))
            .arg(Arg::with_name("vrps")
                .short("v")
                .long("vrps")
                .value_name("FILE")
                .help("Validated ROAs Payloads CSV file.")
                .required(true))

            .subcommand(SubCommand::with_name("world")
                .about("Report ROA quality on a per country basis")
                .arg(Arg::with_name("stats")
                    .short("s")
                    .long("stats")
                    .value_name("FILE")
                    .help("Delegation stats (NRO extended delegated stats format).")
                    .required(true))
                .arg(Arg::with_name("format")
                    .short("f")
                    .long("format")
                    .value_name("json | html | text")
                    .help("Specify output format, defaults to json")
                    .required(false))
            )
            .subcommand(SubCommand::with_name("resources")
                .about("Report ROA quality on a resource basis")
                .arg(Arg::with_name("ips")
                    .short("i")
                    .long("ips")
                    .value_name("comma separated prefixes/ranges")
                    .help("Optional scope for invalid report. Default: all")
                    .required(false))
                .arg(Arg::with_name("asns")
                    .short("a")
                    .long("asns")
                    .value_name("comma separated ASNs / ASN ranges")
                    .help("Optional scope for invalid report. Default: all")
                    .required(false))
                .arg(Arg::with_name("format")
                    .short("f")
                    .long("format")
                    .value_name("json | text")
                    .help("Specify output format, defaults to json")
                    .required(false))
            )
            .get_matches();

        if matches.subcommand_matches("world").is_some() {
            Ok(Options::WorldStats(WorldStatsOpts::parse(&matches)?))
        } else if matches.subcommand_matches("resources").is_some() {
            Ok(Options::ResourceStats(ResourceReportOpts::parse(&matches)?))
        } else {
            Err(Error::msg("No sub-command given. See --help for options."))
        }
    }
}


//------------ Error --------------------------------------------------------

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "{}", _0)]
    WithMessage(String),

    #[display(fmt="{}", _0)]
    WorldReportError(world::Error),

    #[display(fmt="{}", _0)]
    ResourceReportError(resources::Error),
}

impl Error {
    pub fn msg(s: &str) -> Self {
        Error::WithMessage(s.to_string())
    }
}

impl From<world::Error> for Error {
    fn from(e: world::Error) -> Self { Error::WorldReportError(e) }
}

impl From<resources::Error> for Error {
    fn from(e: resources::Error) -> Self { Error::ResourceReportError(e) }
}

