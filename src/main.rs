/*
 * This project is licensed under the MPL 2.0 license.
 * See the LICENSE file in the project root for more information.
 */
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
    let week = matches.is_present(WEEK_FLAG);
    let year = matches.is_present(YEAR_FLAG);

    let dt= Local::now();
    let utc_dt = Utc::now();
    let isow = if utc { utc_dt.iso_week() } else { dt.iso_week() };

    if week {
        println!("W{}", isow.week());
    } else if year {
        println!("{}", isow.year());
    } else {
        println!("{:?}", isow);
    }

}
