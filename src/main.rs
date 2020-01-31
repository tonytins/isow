/*
 * This project is licensed under the MPL 2.0 license.
 * See the LICENSE file in the project root for more information.
 */
#![allow(dead_code)]
mod flags;
mod patcher;

use chrono::{Datelike, Local, Utc};
use clap::{crate_authors, crate_description, crate_version, load_yaml, App};
use flags::*;
#[cfg(feature = "updater")]
use patcher::*;
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

    match matches.subcommand_name() {
        #[cfg(feature = "updater")]
        Some("update") => {
            let patcher = Patcher::default();
            let is_status = upd.is_present(LIST_FLAG);

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
        }
        _ => {
            let is_utc = matches.is_present(UTC_FLAG);

            let dt = Local::now();
            let dt_utc = Utc::now();
            let day = if is_utc { dt_utc.day() } else { dt.day() };
            let year_week = if is_utc {
                dt_utc.iso_week()
            } else {
                dt.iso_week()
            };
            let week = format!("W{:02}", year_week.week());
            let year = year_week.year();
            let iso_date = format!("{:?}-{}", year_week, day);

            let input = {
                let (is_day, is_week, is_year) = (
                    matches.is_present(DAY_FLAG),
                    matches.is_present(WEEK_FLAG),
                    matches.is_present(YEAR_FLAG),
                );

                match (is_year, is_week, is_day) {
                    (true, true, true) => format!("{}", iso_date),
                    (_, true, true) => format!("{}-{}", week, day),
                    (true, true, _) => format!("{:?}", year_week),
                    (true, _, true) => format!("{}-{}", year, day),
                    (_, _, true) => format!("{}", day),
                    (_, true, _) => format!("{}", week),
                    (true, _, _) => format!("{}", year),
                    _ => format!("{}", iso_date),
                }
            };

            println!("{}", input);
        }
    }
}
