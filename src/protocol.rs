use std::fmt::Display;

use clap::ValueEnum;


#[derive(ValueEnum, Clone)]
pub enum Protocol {
    Http,
    Https,
}

impl Default for Protocol {
    fn default() -> Self { Protocol::Https }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Http => write!(f, "http"),
            Protocol::Https => write!(f, "https"),
        }
    }
}