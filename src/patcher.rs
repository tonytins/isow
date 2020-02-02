/*
 * This project is licensed under the MPL 2.0 license.
 * See the LICENSE file in the project root for more information.
 */
#![allow(dead_code)]
#![cfg(feature = "updater")]
use anyhow::Result;
use clap::crate_version;
use self_update::backends::github::{ReleaseList, Update};
use std::error::Error;

pub const REPO_NAME: &str = "isow";
pub const REPO_OWNER: &str = "tonytins";

pub struct Patcher {
    repo_name: String,
    repo_owner: String,
    bin_name: String,
}

impl Default for Patcher {
    fn default() -> Self {
        Patcher {
            repo_name: REPO_NAME.to_string(),
            repo_owner: REPO_OWNER.to_string(),
            bin_name: REPO_NAME.to_string(),
        }
    }
}

impl Patcher {
    pub fn new<S: Into<String>>(name: S, owner: S, bin: S) -> Patcher {
        Patcher {
            repo_name: name.into(),
            repo_owner: owner.into(),
            bin_name: bin.into(),
        }
    }

    pub fn release_list(self) -> Result<(), Box<dyn Error>> {
        let releases = ReleaseList::configure()
            .repo_name(self.repo_name.as_str())
            .repo_owner(self.repo_owner.as_str())
            .build()?
            .fetch()?;

        println!("Releases: {:#?}", releases);
        Ok(())
    }

    pub fn update(self) -> Result<(), Box<dyn Error>> {
        // Self Update oddly isn't aware of platform-specific extensions
        let is_exe = if cfg!(target_os = "windows") {
            format!("{}.exe", self.bin_name)
        } else {
            self.bin_name
        };

        let download = Update::configure()
            .repo_name(self.repo_name.as_str())
            .repo_owner(self.repo_owner.as_str())
            .bin_name(is_exe.as_str())
            .current_version(crate_version!())
            .show_download_progress(true)
            .build()?
            .update()?;

        println!("Update status: {}", download.version());

        Ok(())
    }
}
