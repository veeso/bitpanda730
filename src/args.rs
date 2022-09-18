//! # Args
//!
//! CLI arguments

use argh::FromArgs;
use chrono::{DateTime, FixedOffset};

use std::path::PathBuf;

#[derive(FromArgs)]
#[argh(
    description = "Please, report issues to <https://github.com/veeso/bitpanda730>
Please, consider supporting the author <https://ko-fi.com/veeso>"
)]
pub struct Args {
    #[argh(option, description = "set start date range")]
    pub since: DateTime<FixedOffset>,
    #[argh(option, description = "set end date range")]
    pub to: DateTime<FixedOffset>,
    #[argh(switch, short = 'D', description = "enable TRACE log level")]
    pub debug: bool,
    #[argh(switch, short = 'v', description = "verbose mode")]
    pub verbose: bool,
    #[argh(switch, short = 'V', description = "print version")]
    pub version: bool,
    #[argh(positional, description = "the csv file to read trades from")]
    pub csv_file: PathBuf,
}
