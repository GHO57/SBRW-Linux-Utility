use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunnerVersion {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuntimeVersion {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub wine: WineComponent,
    pub dxvk: DXVKComponent,
    pub game: GameComponent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_wine_versions: Option<Vec<RunnerVersion>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_dxvk_versions: Option<Vec<RuntimeVersion>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WineComponent {
    pub version: String,
    pub path: String,
    pub esync: bool,
    pub fsync: bool,
    pub feral_gamemode: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DXVKComponent {
    pub version: String,
    pub path: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameComponent {
    pub prefix: String,
    pub directory: String,
    pub launcher: String,
    pub mangohud: bool,
    pub environment_variables: Vec<KeyValue>,
    pub dll_overrides: Vec<KeyValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl Config {
    pub fn new(wine: WineComponent, dxvk: DXVKComponent, game: GameComponent) -> Self {
        Self {
            wine,
            dxvk,
            game,
            available_wine_versions: None,
            available_dxvk_versions: None,
        }
    }
}
