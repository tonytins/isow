/*
 * This project is licensed under the MPL 2.0 license.
 * See the LICENSE file in the project root for more information.
 */
#[allow(dead_code)]
mod flags;

use flags::*;
use clap::{crate_authors, crate_description,
           crate_version, load_yaml, App};
use chrono::{Datelike, Local, Utc};

fn main() {
    let yaml = load_yaml!("isow.yml");
    let matches = App::from_yaml(yaml)
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();

    // let debug = matches.is_present(DEBUG_FLAG);
    let utc = matches.is_present(UTC_FLAG);
    let (is_week, is_year, is_day) = (
        matches.is_present(WEEK_FLAG),
        matches.is_present(YEAR_FLAG),
        matches.is_present(DAY_FLAG)
    );

    let dt= Local::now();
    let utc_dt = Utc::now();
    let day = if utc { utc_dt.day() } else { dt.day() };
    let iso_week = if utc { utc_dt.iso_week() } else { dt.iso_week() };

    match (is_day, is_week, is_year) {
        (true, _, _) => println!("{:02}", day),
        (_, true, _) => println!("W{:02}", iso_week.week()),
        (_, _, true) => println!("{}", iso_week.year()),
        _ => println!("{:?}-{:02}", iso_week, day),
    }
}
