use std::error::Error;
use std::env;
use std::process;

use gw2lib::{Client, Requester};
use gw2lib::model::authenticated::{
	characters::{Character, CharacterId},
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() || args.len() > 5 || args[1] == "-h" || args[1] == "--help" {
        println!("Usage: {} <api-key> [<character-name> [<equipment-tab> <build-tab>]]", args[0]);
		println!("  extract an equipment and build tab into website format (currently AW2 only)");
		println!("  if only api-key is provided, displays a list of characters");
		println!("  if only api-key and character-name are specified, display a numbered list of equipment and build tabs");
		println!("  if equipment-tab and build-tab is 'x', display current equipment/build");
        process::exit(1);
    }

	let key = &args[1];
	if args.len() == 2 {
        eprintln!("Getting available characters...");
		buildwars::print_available_characters_detailed(key);

	} else if args.len() >= 3 {
		let id = CharacterId::from(&args[2]);
		let client = Client::default().api_key(key);
		if let Ok(c) = client.single::<Character, CharacterId>(id.clone()) {

			if args.len() < 4 {
				// only list equipment/build tabs
                eprintln!("Getting available tabs on {}...", id);
				println!("{} - {:?} {:?} {:?}\n\nEquipment Tabs:", id, c.core.gender, c.core.race, c.core.profession, );
				for (i, tab) in c.equipment_tabs.iter().enumerate() {
					println!("{}: {}", i+1, tab.name);
				}
				println!("\nBuild Tabs:");
				for (i, tab) in c.build_tabs.clone().into_iter().enumerate() {
					println!("{}: {}", i+1, tab.build.name.unwrap_or(String::from("-unnamed-")));
				}
			} else {
				// indices given
				println!("{}", buildwars::create_page(&c, &args[3], &args[4]).unwrap_or(String::from("invalid build")));
			}

		} else {
			eprintln!("Character {id} not found. Available characters:");
			buildwars::print_available_characters(key);
		}
	}
    Ok(())
}
