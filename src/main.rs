use std::error::Error;
use std::fs;

use fast_config::Config;
use serde::{Serialize, Deserialize};
use clap::Parser;

use gw2lib::{Client, Requester};
use gw2lib::model::authenticated::{
	characters::{Character, CharacterId},
};

use buildwars;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// API key from account.arena.net/applications. If used, it will be saved in $HOME/.buildwars/buildwars.toml
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
    pub key: Option<String>
}

fn main() -> Result<(), Box<dyn Error>> {

    let args = Args::parse(); 

    let settings = Settings {
        key: args.key.clone()
    };

    let config_dir = String::from(env!("HOME")) + "/.buildwars";
    fs::create_dir_all(&config_dir)?;

    let config_path = config_dir + "/buildwars.toml";
    let config = Config::new(&config_path, settings).unwrap();

    let finalized_key = match (args.key, config.data.key.clone()) {
        (Some(akey), _) => Some(akey),
        (None, Some(ckey)) => Some(ckey),
        (_,_) => None
    };

    match (finalized_key, args.name) {
        (None, _) => {
            println!("ERROR: No API key. Use -k option or put in {}", config_path)
        }
        (Some(key), None) => {
            eprintln!("Getting available characters...");
            buildwars::print_available_characters_detailed(&key);

            config.save().unwrap();
        },
        (Some(key), Some(name)) => {
            let id = CharacterId::from(name);
            let client = Client::default().api_key(&key);
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
                buildwars::print_available_characters(&key);
            }

            config.save().unwrap();
        }
    }
    Ok(())
}
