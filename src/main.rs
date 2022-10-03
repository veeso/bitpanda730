#[macro_use]
extern crate log;
#[macro_use]
extern crate rust_decimal_macros;
#[macro_use]
extern crate serde;

use env_logger::Builder as LogBuilder;
use log::LevelFilter;

mod app;
mod args;
mod database;
mod finance;
mod module730;
mod tax;

#[cfg(test)]
mod mock;

use app::App;
use args::Args;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() -> anyhow::Result<()> {
    // parse arguments
    let args: Args = argh::from_env();
    // setup logging
    LogBuilder::new()
        .filter_level(if args.debug {
            LevelFilter::Debug
        } else if args.verbose {
            LevelFilter::Info
        } else {
            LevelFilter::Off
        })
        .init();
    // print version
    if args.version {
        anyhow::bail!("bitpanda730 {} - developed by {}", APP_VERSION, APP_AUTHORS)
    }
    // run app
    App::setup(args.year, &args.csv_file)?.run()
}
