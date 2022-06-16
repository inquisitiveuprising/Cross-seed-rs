mod config;

use config::Config;

use std::path::{Path, PathBuf};
use std::error::Error;

use lava_torrent::torrent::v1::Torrent;

use torznab::Client as TorznabClient;

fn read_torrents(path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut torrents = Vec::new();
    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            if filename.ends_with(".torrent") {
                torrents.push(path);
            }
        } else {
            let mut inner = read_torrents(&path)?;
            torrents.append(&mut inner);
        }
    }

    return Ok(torrents);
}

fn main() {
    // Get config and debug the torrents
    let config = Config::new();//.expect("Failed to get config");
    println!("Searching torrents in: {}", config.torrents_path_str());

    println!("Searching {} trackers: ", config.indexers.len());
    for indexer in config.indexers.iter() {
        println!("  {}: {}", indexer.name, indexer.url);
    }

    let torrents = read_torrents(config.torrents_path()).unwrap();

    for torrent_path in torrents.iter() {
        let torrent = Torrent::read_from_file(torrent_path).unwrap();
        println!("{}:", torrent.name);

        /* for indexer in config.indexers.iter() {
            if indexer.enabled {
                let client = TorznabClient::new(indexer.url.clone());
                let results = client.search(&torrent).unwrap();
                println!("{}", results);
            }
        } */
        //TorznabClient

        /*if let Some(announce) = torrent.announce {
            println!("  Announce: {}", announce);
        }
        if let Some(announce_list) = torrent.announce_list {
            println!("  Announce list:");
            for announce in announce_list {
                for ann in announce {
                    println!("    {}", ann);
                }
            }
        }
        println!("  Files:");
        
        if let Some(files) = torrent.files {
            for file in files.iter() {
                println!("    {}", file.path.to_str().unwrap());
            }
        } */
    }
}