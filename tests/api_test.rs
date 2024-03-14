use gw2lib::{Client, Requester};
// use gw2lib::model::items::{Item, ItemId, Details};
// use gw2lib::model::items::itemstats::{ItemStat, StatsId};
use gw2lib::model::authenticated::{
	// characters::{BuildTemplate, Skillset, TraitLine, Equip, Slot, Stats},
	characters::{Character, CharacterId},
};

#[test]
fn get_character_names() {
	let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
	let names: Vec<CharacterId> = client.ids::<Character, CharacterId>().unwrap();
	assert!(names.contains(&CharacterId::from("Johnny Vicious")));
}

#[test]
#[ignore = "expensive"]
fn get_bulk_characters() {
	let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
	let characters: Vec<Character> = client.all().unwrap();
	assert_eq!(characters.len(), 15);
}

#[test]
#[ignore = "not yet implemented"]
fn get_chatlink() {
	todo!("get chatlink from api");
}
