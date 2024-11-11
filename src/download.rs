use bytes::Bytes;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use std::{env, fs};
use tokio::runtime::Runtime;
use tokio::task::JoinSet;

#[derive(Clone)]
pub struct Libraries {
    libraries: Vec<Library>,
}

#[derive(Clone)]
pub struct Library {
    pub path: String,
    pub url: String,
}

impl Libraries {
    pub async fn download(self) {
        // Download Libraries
        let mut set = JoinSet::new();
        for library in self.libraries {
            set.spawn(async move {
                download_library(library.clone()).await;
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
}

async fn download_library(library: Library) {
    // Download
    let current_dir = env::current_dir().unwrap();
    let path = current_dir
        .join("download/libraries/")
        .join(&library.path)
        .to_string_lossy()
        .to_string();
    get_file(&library.url, path).await;
}

async fn get_file(url: &String, path: String) {
    let response = reqwest::get(url).await.unwrap();
    let bytes = response.bytes().await.unwrap();
    save_file(bytes, path);
}

fn save_file(bytes: Bytes, path: String) {
    let as_path = Path::new(&path);
    let parent = as_path.parent().unwrap();
    fs::create_dir_all(parent).unwrap();
    let mut file = File::create(path).unwrap();
    let mut content = Cursor::new(bytes);
    std::io::copy(&mut content, &mut file).unwrap();
}

pub struct Server {
    path: String,
    url: String,
    libraries: Libraries,
}

impl Server {
    pub fn download(&mut self) {
        // Download Server Jar
        let server_jar_path = format!("{}/server.jar", self.path);
        let rt = Runtime::new().unwrap();
        rt.block_on(get_file(&self.url, server_jar_path));
        rt.block_on(self.libraries.clone().download());
    }
}
