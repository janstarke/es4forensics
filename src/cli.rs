use std::fmt::Display;

use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone)]
pub enum Protocol {
    Http,
    Https,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Http => write!(f, "http"),
            Protocol::Https => write!(f, "https"),
        }
    }
}

#[derive(clap::Subcommand)]
enum Action {
    // create a new index
    Create,

    // import timeline data
    Import {
        /// number of timeline entries to combine in one bulk operation
        #[clap(long("bulk-size"), default_value_t=1000)]
        bulk_size: usize
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    action: Action,

    /// strict mode: do not only warn, but abort if an error occurs
    #[clap(long("strict"), display_order(500))]
    pub(crate) strict_mode: bool,

    /// name of the elasticsearch index
    #[clap(long("index"), display_order = 800)]
    #[cfg(feature = "elastic")]
    pub(crate) index_name: Option<String>,

    /// server name or IP address of elasticsearch server
    #[clap(
        short('H'),
        long("host"),
        display_order = 810,
        default_value = "localhost"
    )]
    pub(crate) host: String,

    /// API port number of elasticsearch server
    #[clap(short('P'), long("port"), display_order = 820, default_value_t = 9200)]
    pub(crate) port: u16,

    /// protocol to be used to connect to elasticsearch
    #[clap(long("proto"), display_order=830, default_value_t=Protocol::Https)]
    pub(crate) protocol: Protocol,

    /// omit certificate validation
    #[clap(
        short('k'),
        long("insecure"),
        display_order = 840,
        default_value_t = false
    )]
    pub(crate) omit_certificate_validation: bool,

    /// username for elasticsearch server
    #[clap(short('U'), long("username"), display_order=850, default_value=Some("elastic"))]
    pub(crate) username: String,

    /// password for authenticating at elasticsearch
    #[clap(short('W'), long("password"), display_order = 860)]
    pub(crate) password: Option<String>,

    #[clap(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}