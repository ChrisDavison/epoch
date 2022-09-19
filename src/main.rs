use anyhow::Result;
use chrono::{TimeZone, Utc};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long, default_value = "%Y-%m-%d %H:%M:%S")]
    /// Format for time output
    format: String,
    /// Seconds since unix epoch
    epoch: String,
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    let epochsec: i64 = args.epoch.parse()?;
    let dt = Utc.timestamp(epochsec, 0);
    println!("{}", dt.format(&args.format));
    Ok(())
}
