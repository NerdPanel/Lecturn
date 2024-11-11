use config::{Config, ConfigError, Environment};
use serde::Deserialize;

fn get_main_url(loader: Loader, loader_ver: Option<String>, installer_ver: Option<String>, minecraft_ver: String) -> String {
    match loader {
        Loader::VANILLA => String::from("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json"),
        Loader::FABRIC => String::from(format!("https://meta.fabricmc.net/v2/versions/loader/{}/{}/{}/server/jar", minecraft_ver, loader_ver.unwrap(), installer_ver.unwrap())),
        _ => todo!(),
    }
}

pub struct vanilla;
pub struct fabric;
pub struct forge;
pub struct neoforge;

pub enum Loader {
    VANILLA (vanilla),
    FABRIC (fabric),
    FORGE (forge),
    NEOFORGE (neoforge),
}

pub trait Distro {
    fn install();
}

impl Distro for vanilla {
    fn install() {

    }
}

impl Distro for fabric {
    fn install() {

    }
}

impl Distro for forge {
    fn install() {

    }
}

impl Distro for neoforge {
    fn install() {

    }
}

#[derive(Clone, Debug, Deserialize)]
struct InstallConfig {
    pub loader: String,
    pub mc_ver: String,
    pub loader_ver: String,
}

impl InstallConfig {
    pub fn get() -> Result<InstallConfig, ConfigError> {
        let source = Environment::default();

        let config: InstallConfig = Config::builder()
            .add_source(source)
            .build()?
            .try_into()?;

        Ok(config)
    }
}

