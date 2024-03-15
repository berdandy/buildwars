use gw2lib::{Client, Requester};
// use gw2lib::model::items::{Item, ItemId, Details};
// use gw2lib::model::items::itemstats::{ItemStat, StatsId};
use gw2lib::model::authenticated::{
	// characters::{BuildTemplate, Skillset, TraitLine, Equip, Slot, Stats},
	characters::{Character, CharacterId, Profession},
};

use buildwars;
use buildwars::Aw2Markup;
use buildwars::FrontmatterMarkup;

#[test]
#[ignore = "expensive"]
fn get_engineer() { 
	let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
	let id = CharacterId::from("Jonyeigh");
	let c: Character = client.single(id).unwrap();
	assert_eq!(c.core.name, "Jonyeigh");
	assert_eq!(c.core.profession, Profession::Engineer);

	let gearidx = "1".parse::<usize>().unwrap_or(c.active_equipment_tab.unwrap());
	let gear = &c.equipment_tabs[gearidx-1].equipment;
	assert!(gear.to_markup().is_some());

	let buildidx = "2".parse::<usize>().unwrap_or(c.active_build_tab.unwrap());
	let build = &c.build_tabs[buildidx-1].build;
	assert!(build.to_frontmatter().is_some());
	assert!(build.to_markup().is_some());
}

#[test]
#[ignore = "expensive"]
fn get_ranger() {
	let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
	let id = CharacterId::from("Johnny Silvermane");
	let c: Character = client.single(id).unwrap();
	assert_eq!(c.core.profession, Profession::Ranger);

	let buildidx = "1".parse::<usize>().unwrap_or(c.active_build_tab.unwrap());
	let build = &c.build_tabs[buildidx-1].build;
	assert!(build.to_markup().is_some());
	assert!(build.to_markup().unwrap().contains("Pets: Juvenile"));
}

#[test]
#[ignore = "not yet implemented"]
fn get_revenant() {
	let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
	let id = CharacterId::from("Johnny Socotra");
	let c: Character = client.single(id).unwrap();
	assert_eq!(c.core.profession, Profession::Revenant);

	let buildidx = "1".parse::<usize>().unwrap_or(c.active_build_tab.unwrap());
	let build = &c.build_tabs[buildidx-1].build;
	println!("{:?}", build);
	assert!(build.to_markup().is_some());
	assert!(build.to_markup().unwrap().contains("Legends"));
}

