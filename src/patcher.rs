/*
 * This project is licensed under the MPL 2.0 license.
 * See the LICENSE file in the project root for more information.
 */
#![allow(dead_code)]
#![cfg(feature = "updater")]
use clap::crate_version;
use self_update::backends::github::{ReleaseList, Update};
use std::error::Error;

pub struct Patcher {
    repo_name: String,
    repo_owner: String,
}

impl Default for Patcher {
    fn default() -> Self {
        Patcher {
            repo_name: "isow".to_string(),
            repo_owner: "tonytins".to_string(),
        }
    }
}

impl Patcher {
    pub fn new<S: Into<String>>(name: S, owner: S) -> Patcher {
        Patcher {
            repo_name: name.into(),
            repo_owner: owner.into(),
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

    pub fn update<S: Into<String>>(self, bin_name: S) -> Result<(), Box<dyn Error>> {
        // Self Update oddly isn't aware of platform-specific extensions
        let is_exe = if cfg!(target_os = "windows") {
            format!("{}.exe", bin_name.into())
        } else {
            bin_name.into()
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
