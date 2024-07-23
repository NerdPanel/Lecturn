use std::{env, fs};
use std::borrow::ToOwned;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use tokio::task::JoinSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
    sha1: String,
    #[serde(rename = "complianceLevel")]
    compliance_level: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinecraftVersions {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

impl Version {
    pub fn download_server(&mut self) {
        let data = get_version_data(self);
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join("download").join(&self.id).to_string_lossy().to_string().replace(".", "_");
        fs::create_dir_all(&path).unwrap();

        // Download Server Jar
        let server_jar_path = format!("{}/server.jar", &path);
        let rt = Runtime::new().unwrap();
        rt.block_on(Self::get_file(&data.downloads.server.url, server_jar_path));
        rt.block_on(Self::multithreaded_library_download(data))
    }

    async fn multithreaded_library_download(data: VersionData) {
        // Download Libraries
        let mut set = JoinSet::new();
        for library in data.libraries {
            set.spawn(async move {
                Self::download_library(library).await;
            });
        }
        let mut counter = 0;
        let total = set.len();
        while let Some(res) = set.join_next().await {
            counter += 1;
            println!("{} of {} downloads completed", counter, total);
        }
        println!("All downloads completed");
    }

    async fn download_library(library: Library) {
        if library.rules.is_some() {
            // Check for natives
            let mut apply = true;
            for rule in library.rules.unwrap() {
                let os = rule.os.unwrap().name;
                apply = Self::should_apply(os);
                break;
            }
            if !apply {
                return;
            }
        }
        // Download
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join("download/libraries/").join(&library.downloads.artifact.path).to_string_lossy().to_string();
        let url = library.downloads.artifact.url;
        Self::get_file(&url, path).await;
    }

    fn should_apply(os: String) -> bool {
        let mut current_os = env::consts::OS;
        if current_os == "macos" {
            current_os = "osx";
        }
        current_os == os
    }

    async fn get_file(url: &String, path: String) {
        let response = reqwest::get(url).await.unwrap();
        let bytes = response.bytes().await.unwrap();
        Self::save_file(bytes, path);
    }

    fn save_file(bytes: Bytes, path: String) {
        let as_path = Path::new(&path);
        let parent = as_path.parent().unwrap();
        fs::create_dir_all(parent).unwrap();
        let mut file = File::create(path).unwrap();
        let mut content =  Cursor::new(bytes);
        std::io::copy(&mut content, &mut file).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct VersionData {
    downloads: Downloads,
    #[serde(rename = "javaVersion")]
    java: JavaVersion,
    libraries: Vec<Library>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Download {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Downloads {
    server: Download,
}

#[derive(Debug, Serialize, Deserialize)]
struct JavaVersion {
    #[serde(rename = "majorVersion")]
    major_version: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Artifact {
    path: String,
    sha1: String,
    size: i32,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LibraryDownloads {
    artifact: Artifact,
}

#[derive(Debug, Serialize, Deserialize)]
struct OS {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Rule {
    action: String,
    os: Option<OS>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Library {
    downloads: LibraryDownloads,
    name: String,
    rules: Option<Vec<Rule>>,
}

pub fn get_version(mc_version: String) -> Option<Version> {
    let parsed = get_minecraft_versions();
    for version in parsed.versions {
        if version.id == mc_version {
            return Some(version);
        }
    }
    None
}

pub fn get_latest_versions() -> Latest {
    let parsed = get_minecraft_versions();
    parsed.latest
}

async fn get_json(url: &String) -> String {
    let resp = reqwest::get(url).await.unwrap().text().await;
    resp.unwrap()
}

pub fn get_minecraft_versions() -> MinecraftVersions {
    let rt = Runtime::new().unwrap();
    let response = rt.block_on(get_json(&"https://piston-meta.mojang.com/mc/game/version_manifest_v2.json".to_owned()));
    serde_json::from_str(&response).unwrap()
}

fn get_version_data(version: &mut Version) -> VersionData {
    let rt = Runtime::new().unwrap();
    let response = rt.block_on(get_json(&version.url));
    serde_json::from_str(&response).unwrap()
}