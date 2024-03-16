use gw2lib::{Client, Requester};
use gw2lib::model::authenticated::{
	// characters::{BuildTemplate, Skillset, TraitLine, Equip, Slot, Stats},
	characters::{Character, CharacterId},
};

use buildwars;
use buildwars::chatlink::ChatlinkMarkup;

#[test]
fn get_chatlink_basic() { 
	let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
	let id = CharacterId::from("Johnny Vicious");
	let c: Character = client.single(id).unwrap();

	let buildidx = "3".parse::<usize>().unwrap_or(c.active_build_tab.unwrap());
	let build = &c.build_tabs[buildidx-1].build;
	assert_eq!(build.to_chatlink().unwrap(), String::from("[&DQILLjM3EjmmAAAAsAAAAK0BAABqAAAAwhIAAAAAAAAAAAAAAAAAAAAAAAA=]"));
}

#[test]
#[ignore = "not implemented yet"]
fn get_chatlink_pets() { 
	let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
	let id = CharacterId::from("Sigrún Magnúsdóttir");
	let c: Character = client.single(id).unwrap();

	let buildidx = "1".parse::<usize>().unwrap_or(c.active_build_tab.unwrap());
	let build = &c.build_tabs[buildidx-1].build;
	assert_eq!(build.to_chatlink().unwrap(), String::from("..."));
}

#[test]
#[ignore = "not implemented yet"]
fn get_chatlink_legends() { 
	let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
	let id = CharacterId::from("Johnny Socotra");
	let c: Character = client.single(id).unwrap();

	let buildidx = "1".parse::<usize>().unwrap_or(c.active_build_tab.unwrap());
	let build = &c.build_tabs[buildidx-1].build;
	assert_eq!(build.to_chatlink().unwrap(), String::from("..."));
}

#[test]
#[ignore = "not implemented yet"]
fn get_chatlink_with_aquatic() { 
	let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
	let id = CharacterId::from("Johnny Vicious");
	let c: Character = client.single(id).unwrap();

	let buildidx = "3".parse::<usize>().unwrap_or(c.active_build_tab.unwrap());
	let build = &c.build_tabs[buildidx-1].build;
	assert_eq!(build.to_chatlink().unwrap(), String::from("[&DQILLjM3EjmmAKcAsACtAK0BsgBqAGoAwhKcAAAAAAAAAAAAAAAAAAAAAAADNQBnAFkAAA==]"));
}
