mod days;

use crate::days::Day;
use clap::Parser;
use color_eyre::eyre::Error;
use color_eyre::Result;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    match args.day {
        1 => days::D1::run(open_input(1)?, args.part),
        _ => Err(Error::msg("Invalid or unimplemented day")),
    }
}

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    day: u16,
    #[clap(short, long)]
    part: u8,
}

fn open_input(day: u16) -> Result<Vec<u8>> {
    let mut f = fs::File::open(PathBuf::from("./input").join(format!("{day}.txt")))?;

    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}
