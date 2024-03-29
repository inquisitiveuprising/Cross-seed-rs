use serde::{Deserialize,Serialize};
use std::path::Path;
use std::env;
use std::collections::HashMap;
use figment::{Figment, providers::{Format, Toml, Env}};
use figment::value::Value as FigmentValue;

use crate::torznab::TorznabClient;

use super::CliProvider;

#[derive(Deserialize, Serialize)]
pub struct Config {
    /// The path of the torrents to search.
    torrents_path: String,
    /// The output path of the torrents.
    output_path: Option<String>,
    
    //pub indexers: HashMap<String, Indexer>,

    
    /// Used for deserializing the indexers into a Vec<Indexer>.
    #[serde(rename = "indexers")]
    indexers_map: HashMap<String, FigmentValue>,

    /// The indexers to search.
    #[serde(skip)]
    pub indexers: Vec<Indexer>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Indexer {
    #[serde(skip_deserializing)]
    pub name: String,
    pub enabled: Option<bool>,
    pub url: String,
    pub api_key: String,

    #[serde(skip)]
    pub client: Option<TorznabClient>,
}

impl Indexer {
    pub async fn create_client(&mut self) -> Result<&TorznabClient, crate::torznab::ClientError> {
        if self.client.is_none() {
            self.client = Some(TorznabClient::new(self.name.clone(), &self.url, &self.api_key).await?);
        }

        Ok(self.client.as_ref().unwrap())
    }
}

// Allow dead code for functions. We should probably remove this later on.
#[allow(dead_code)]
impl Config {
    pub fn new() -> Config {
        // The path of the config file without the file extension
        let path = match env::var("CROSS_SEED_CONFIG") {
            Ok(path) => path,
            Err(_) => "config".to_string(),
        };

        // TODO: Create a command line argument `Provider` (https://docs.rs/figment/0.10.6/figment/trait.Provider.html)
        // TODO: Figure out priority
        // Merge the config files
        let figment = Figment::new()
            .join(CliProvider::new())
            .join(Env::prefixed("CROSS_SEED_"))
            .join(Toml::file(format!("{}.toml", path)));

        let mut config: Config = figment.extract().unwrap();

        // Parse the indexers map into a vector.
        for (name, value) in &mut config.indexers_map {
            let mut indexer: Indexer = value.deserialize().unwrap();
            indexer.name = name.to_owned();

            config.indexers.push(indexer);
        }

        config
    }

    pub fn torrents_path(&self) -> &Path {
        Path::new(&self.torrents_path)
    }

    pub fn torrents_path_str(&self) -> &String {
        &self.torrents_path
    }

    pub fn output_path(&self) -> Option<&Path> {
        match self.output_path {
            Some(ref path) => Some(Path::new(path)),
            None => None,
        }
    }

    pub fn output_path_str(&self) -> Option<&String> {
        self.output_path.as_ref()
    }
}