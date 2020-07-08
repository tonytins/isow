#![allow(unused_imports)]
use clap::{crate_authors, crate_description, crate_version, Clap};

#[derive(Clap)]
pub struct Options {
    #[clap(short, long)]
    pub utc: bool,
    #[clap(short, long)]
    pub year: bool,
    #[clap(short, long)]
    pub day: bool,
    #[clap(short, long)]
    pub week: bool,
    #[clap(short, long)]
    pub time: bool,
    /*#[clap(subcommand)]
    pub update: Update,*/
}

#[derive(Clap)]
pub enum Update {
    Patch(Patcher)
}

#[derive(Clap)]
pub struct Patcher {
    #[clap(short, long)]
    pub list: bool,
}