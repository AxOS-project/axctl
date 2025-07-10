use std::{fs, path::Path};

use crate::internal::utils;

pub fn update_releases() {

    let lsb_release: &'static str = "/etc/lsb-release";
    let os_release: &'static str = "/etc/lsb-release";
    let axos_version_path: &'static str = "/etc/axos-version";

    let axos_version = fs::read_to_string(axos_version_path).ok().unwrap_or_else(|| "24.08".to_string());

    // will check for "DISTRIB_RELEASE" in lsb-release and then return it's value. If it doesn't exists, return unknown
    let lsb_version = fs::read_to_string(lsb_release)
        .ok()
        .and_then(|content| {
            content
            .lines()
            .find(|line| line.to_uppercase().starts_with("DISTRIB_RELEASE"))
            .map(|line| line.split('=').nth(1).unwrap_or("").trim_matches('"').to_lowercase())
        })
        .unwrap_or_else(|| "unknown".to_string());

    // same here but for os-release
    let os_version = fs::read_to_string(os_release)
        .ok()
        .and_then(|content| {
            content
            .lines()
            .find(|line| line.to_uppercase().starts_with("VERSION"))
            .map(|line| line.split("=").nth(1).unwrap_or("").trim_end_matches('"').to_lowercase())
        })
        .unwrap_or_else(|| "unknown".to_string());

    // If the file exists and the version is known, sync the versions
    if Path::new(lsb_release).exists() && lsb_version != "unknown" {
        if lsb_version != axos_version {
            utils::sed_in_file(lsb_release, &lsb_version, &axos_version).unwrap()
        }
    }
    if Path::new(os_release).exists() && os_version != "unknown" {
        if os_version != axos_version {
            utils::sed_in_file(os_release, &os_version, &axos_version).unwrap()
        }
    }
}