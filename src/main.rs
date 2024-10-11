use std::error::Error;

use fast_config::Config;
use serde::{Serialize, Deserialize};
use clap::Parser;

use gw2lib::{Client, Requester};
use gw2lib::model::authenticated::{
	characters::{Character, CharacterId},
};

use buildwars;

#[derive(Parser)]
pub struct Args {
    /// API key from account.arena.net/applications. If used, it will be saved in buildwars.toml
    #[arg(short, long)]
    key: Option<String>,

    /// Character Name
    name: Option<String>,

    /// Equipment Tab index
    equipment: Option<String>,

    /// Build Tab index
    build: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub key: String
}

fn main() -> Result<(), Box<dyn Error>> {

    let args = Args::parse(); 

    let settings = Settings {
        key: match args.key {
            Some(key) => key,
            None => String::from("API-KEY-HERE")
        }
    };
    let config = Config::new("buildwars.toml", settings).unwrap();

    match args.name {
        None => {
            eprintln!("Getting available characters...");
            buildwars::print_available_characters_detailed(&config.data.key);
        },
        Some(name) => {
            let id = CharacterId::from(name);
            let client = Client::default().api_key(config.data.key.clone());
            if let Ok(c) = client.single::<Character, CharacterId>(id.clone()) {
                match (args.equipment, args.build) {
                    (Some(e), Some(b)) => println!("{}", buildwars::create_page(&c, &e, &b).unwrap_or(String::from("invalid build"))),
                    _ => {
                        // list equipment/build tabs
                        eprintln!("Getting available tabs on {}...", id);
                        println!("{} - {:?} {:?} {:?}\n\nEquipment Tabs:", id, c.core.gender, c.core.race, c.core.profession, );
                        for (i, tab) in c.equipment_tabs.iter().enumerate() {
                            println!("{}: {}", i+1, tab.name);
                        }
                        println!("\nBuild Tabs:");
                        for (i, tab) in c.build_tabs.clone().into_iter().enumerate() {
                            println!("{}: {}", i+1, tab.build.name.unwrap_or(String::from("-unnamed-")));
                        }
                    }
                }
            } else {
                eprintln!("Character {id} not found. Available characters:");
                buildwars::print_available_characters(&config.data.key);
            }
        }
    }
    config.save().unwrap();
    Ok(())
}
