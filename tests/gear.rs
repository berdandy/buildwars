use gw2lib::{Client, Requester};
use gw2lib::model::authenticated::characters::{Character, CharacterId};

use buildwars;
use buildwars::Aw2Markup;

#[test]
fn trinkets() { 
	let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
	let id = CharacterId::from("Johnny Somebody");
	let c: Character = client.single(id).unwrap();

	let gearidx = "1".parse::<usize>().unwrap_or(c.active_equipment_tab.unwrap());
	let gear = &c.equipment_tabs[gearidx-1].equipment;
	assert!(gear.to_markup().is_some());
}
