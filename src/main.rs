/*
 * This project is licensed under the MPL 2.0 license.
 * See the LICENSE file in the project root for more information.
 */
#![allow(dead_code)]
mod flags;
mod patcher;

use flags::*;
use patcher::*;
use clap::{crate_authors, crate_description,
           crate_version, load_yaml, App};
use chrono::{Datelike, Local, Utc};
use std::error::Error;

fn exit_on_error(err: Box<dyn Error>) {
    eprintln!("[ERROR] {}", err);
    ::std::process::exit(1);
}

fn main() {
    let yaml = load_yaml!("isow.yml");
    let matches = App::from_yaml(yaml)
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();

    if let Some(upd) = matches.subcommand_matches("update") {

        let patcher = Patcher::default();
        let is_status = upd.is_present(STATUS_FLAG);

        match is_status {
            true => {
                if let Err(err) = patcher.release_list() {
                    exit_on_error(err);
                }
            }
            false => {
                if let Err(err) = patcher.update("isow") {
                    exit_on_error(err);
                }
            }
        }

    } else {

        let is_utc = matches.is_present(UTC_FLAG);
        let (is_day, is_week, is_year) = (
            matches.is_present(DAY_FLAG),
            matches.is_present(WEEK_FLAG),
            matches.is_present(YEAR_FLAG)
        );

        let dt = Local::now();
        let dt_utc = Utc::now();
        let day = if is_utc { dt_utc.day() } else { dt.day() };
        let iso_week = if is_utc { dt_utc.iso_week() } else { dt.iso_week() };

        match (is_day, is_week, is_year) {
            (true, _, _) => println!("{}", day),
            (_, true, _) => println!("W{:02}", iso_week.week()),
            (_, _, true) => println!("{}", iso_week.year()),
            _ => println!("{:?}-{}", iso_week, day),
        }
    }
}
