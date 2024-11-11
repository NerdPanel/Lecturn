use crate::parser;
use crate::parser::{get_latest_versions, get_version};

pub fn list_minecraft_versions_all(only_stable: bool) -> String {
    let mc_versions = parser::get_minecraft_versions();
    let mut combined: String = String::new();
    for version in mc_versions.versions {
        if only_stable {
            let release_type = version.version_type;
            if release_type != "release" {
                continue;
            }
        }
        combined.push_str(format!("{}, ", version.id).as_str());
    }
    combined
}

pub fn is_stable(version: String) -> bool {
    let mc_ver = get_version(version).unwrap();
    mc_ver.version_type == "release"
}

pub fn is_latest(version: String, only_stable: bool) -> bool {
    let latest = get_latest_versions();
    if only_stable {
        return version == latest.release;
    };
    version == latest.release || version == latest.snapshot
}

pub fn download(version: String) {
    let version = get_version(version);
    if let Some(mut mc_ver) = version {
        mc_ver.download_server();
    }
}
