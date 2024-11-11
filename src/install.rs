use config::{Config, ConfigError, Environment};
use serde::Deserialize;

fn get_main_url(
    loader: Loader,
    loader_ver: Option<String>,
    installer_ver: Option<String>,
    minecraft_ver: String,
) -> String {
    match loader {
        Loader::Vanilla(_) => {
            String::from("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        }
        Loader::Fabric(_) => String::from(format!(
            "https://meta.fabricmc.net/v2/versions/loader/{}/{}/{}/server/jar",
            minecraft_ver,
            loader_ver.unwrap(),
            installer_ver.unwrap()
        )),
        _ => todo!(),
    }
}

pub struct Vanilla;
pub struct Fabric;
pub struct Forge;
pub struct Neoforge;

pub enum Loader {
    Vanilla(Vanilla),
    Fabric(Fabric),
    Forge(Forge),
    Neoforge(Neoforge),
}

pub trait Distro {
    fn install();
}

impl Distro for Vanilla {
    fn install() {}
}

impl Distro for Fabric {
    fn install() {}
}

impl Distro for Forge {
    fn install() {}
}

impl Distro for Neoforge {
    fn install() {}
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
            .try_deserialize()?;

        Ok(config)
    }
}
