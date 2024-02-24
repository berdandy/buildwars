use std::error::Error;
use std::env;
use std::process;

use itertools::Itertools;

use gw2lib::{Client, Requester};
use gw2lib::model::items::{Item, ItemId, Details};
use gw2lib::model::items::itemstats::{ItemStat, StatsId};
use gw2lib::model::authenticated::{
	characters::{Character, CharacterId, BuildTemplate, Profession, Skillset, TraitLine, Equip, Slot, Stats},
};

trait FrontmatterMarkup {
	fn to_frontmatter(&self) -> String;
}

impl FrontmatterMarkup for BuildTemplate {
	fn to_frontmatter(&self) -> String
	{
		let spec = match self.specializations[2].id.unwrap() {
			// HoT elites
			5 => String::from("druid"),
			7 => String::from("daredevil"),
			18 => String::from("berserker"),
			27 => String::from("dragonhunter"),
			34 => String::from("reaper"),
			40 => String::from("chronomancer"),
			43 => String::from("scrapper"),
			48 => String::from("tempest"),
			52 => String::from("herald"),
			
			// PoF elites
			55 => String::from("soulbeast"),
			56 => String::from("weaver"),
			57 => String::from("holosmith"),
			58 => String::from("deadeye"),
			59 => String::from("mirage"),
			60 => String::from("scourge"),
			61 => String::from("spellbreaker"),
			62 => String::from("firebrand"),
			63 => String::from("renegade"),

			// EoD elites
			64 => String::from("harbinger"),
			65 => String::from("willbender"),
			66 => String::from("virtuoso"),
			67 => String::from("catalyst"),
			68 => String::from("bladesworn"),
			69 => String::from("vindicator"),
			70 => String::from("mechanist"),
			71 => String::from("specter"),
			72 => String::from("untamed"),

			// core or no third spec
			_ => match self.profession.as_ref() {
				Some(Profession::Elementalist) => String::from("elementalist"), 
				Some(Profession::Engineer) => String::from("engineer"), 
				Some(Profession::Guardian) => String::from("guardian"), 
				Some(Profession::Mesmer) => String::from("mesmer"), 
				Some(Profession::Necromancer) => String::from("necromancer"), 
				Some(Profession::Ranger) => String::from("ranger"), 
				Some(Profession::Revenant) => String::from("revenant"), 
				Some(Profession::Thief) => String::from("thief"), 
				Some(Profession::Warrior) => String::from("warrior"), 
				None => String::from("unknown")
			}
		};
		format!("spec: {spec}")
	}
}

trait ArmoryMarkup {
	fn to_markup(&self) -> Option<String>;
}

// note: this only really works in the context of a full BuildTemplate
impl ArmoryMarkup for TraitLine {
	fn to_markup(&self) -> Option<String>
	{
		Some(format!("data-armory-{spec}-traits='{trait1},{trait2},{trait3}' ",
			spec=self.id?,
			trait1=self.traits?[0]?,
			trait2=self.traits?[1]?,
			trait3=self.traits?[2]?,
		))
	}
}

impl ArmoryMarkup for Skillset {
	fn to_markup(&self) -> Option<String>
	{
		Some(format!("<div data-armory-embed='skills' data-armory-ids='{healing},{utility1},{utility2},{utility3},{elite}'></div>",
			healing=self.heal?,
			utility1=self.utilities[0]?,
			utility2=self.utilities[1]?,
			utility3=self.utilities[2]?,
			elite=self.elite?,
		))
	}
}

impl ArmoryMarkup for BuildTemplate {
	fn to_markup(&self) -> Option<String>
	{
		Some(format!(concat!(
			"{skills}",
			"<div ",
			  "data-armory-embed='specializations' ",
			  "data-armory-ids='{spec1},{spec2},{spec3}' ",
			  "{traitline1} {traitline2} {traitline3}",
			">",
			"</div>"),
			skills=self.skills.to_markup()?,
			spec1=self.specializations[0].id?,
			spec2=self.specializations[1].id?,
			spec3=self.specializations[2].id?,
			traitline1=self.specializations[0].to_markup()?,
			traitline2=self.specializations[1].to_markup()?,
			traitline3=self.specializations[2].to_markup()?,
		))
	}
}

impl ArmoryMarkup for ItemId
{
	fn to_markup(&self) -> Option<String>
	{
		let client = Client::default();
		let result = client.single::<Item, ItemId>(*self);
		match result {
			Ok(item) => match item.details {
				Details::Weapon(details) => Some(format!("{:?}", details._type)),
				_ => Some(item.name),
			}
			_ => None
		}
	}
}

impl ArmoryMarkup for Vec<ItemId>
{
	fn to_markup(&self) -> Option<String>
	{
		Some(self.iter().flat_map(|e| e.to_markup()).join(", "))
	}
}

impl ArmoryMarkup for Vec<Equip>
{
	fn to_markup(&self) -> Option<String>
	{
		Some(self.iter().flat_map(|e| e.to_markup()).join("\n"))
	}
}

/*
// generic
impl<T> ArmoryMarkup for Vec<T>
	where T: ArmoryMarkup
{
	fn to_markup(&self) -> Option<String>
	{
		Some(self.iter().flat_map(|e| e.to_markup()).join(", "))
	}
}
*/

impl ArmoryMarkup for Stats {
	fn to_markup(&self) -> Option<String>
	{
		let client = Client::default();
		let result = client.single::<ItemStat, StatsId>(self.id);
		match result {
			Ok(stat) => Some(stat.name),
			_ => None
		}
	}
}

impl ArmoryMarkup for Equip {
	fn to_markup(&self) -> Option<String>
	{
		Some(match (self.slot.clone(), self.stats.clone(), self.upgrades.clone()) {
			(Some(Slot::Backpack), Some(s), _)		=> Some(format!("- Backpack: {}", s.to_markup()?)),
			(Some(Slot::Accessory1), Some(s), _)	=> Some(format!("- Accessory 1: {}", s.to_markup()?)),
			(Some(Slot::Accessory2), Some(s), _)	=> Some(format!("- Accessory 2: {}", s.to_markup()?)),
			(Some(Slot::Ring1), Some(s), _)			=> Some(format!("- Ring 1: {}", s.to_markup()?)),
			(Some(Slot::Ring2), Some(s), _)			=> Some(format!("- Ring 2: {}", s.to_markup()?)),
			(Some(Slot::Amulet), Some(s), _)		=> Some(format!("- Amulet: {}", s.to_markup()?)),

			(Some(Slot::Helm), Some(s), Some(u))		=> Some(format!("- {} Helm, {}", s.to_markup()?, u.to_markup()?)),
			(Some(Slot::Shoulders), Some(s), Some(u))	=> Some(format!("- {} Shoulders, {}", s.to_markup()?, u.to_markup()?)),
			(Some(Slot::Coat), Some(s), Some(u))		=> Some(format!("- {} Coat, {}", s.to_markup()?, u.to_markup()?)),
			(Some(Slot::Gloves), Some(s), Some(u))		=> Some(format!("- {} Gloves, {}", s.to_markup()?, u.to_markup()?)),
			(Some(Slot::Leggings), Some(s), Some(u))	=> Some(format!("- {} Leggings, {}", s.to_markup()?, u.to_markup()?)),
			(Some(Slot::Boots), Some(s), Some(u))		=> Some(format!("- {} Boots, {}", s.to_markup()?, u.to_markup()?)),

			(Some(Slot::WeaponA1), Some(s), Some(u))	=> Some(format!("- Weapon A1: {}, {}, {}", self.id.to_markup()?, s.to_markup()?, u.to_markup()?)),
			(Some(Slot::WeaponA2), Some(s), Some(u))	=> Some(format!("- Weapon A2: {}, {}, {}", self.id.to_markup()?, s.to_markup()?, u.to_markup()?)),
			(Some(Slot::WeaponB1), Some(s), Some(u))	=> Some(format!("- Weapon B1: {}, {}, {}", self.id.to_markup()?, s.to_markup()?, u.to_markup()?)),
			(Some(Slot::WeaponB2), Some(s), Some(u))	=> Some(format!("- Weapon B2: {}, {}, {}", self.id.to_markup()?, s.to_markup()?, u.to_markup()?)),

			(Some(Slot::Relic), _, _)		=> Some(format!("- Relic: {}", self.id.to_markup()?)),

			(Some(Slot::HelmAquatic),_,_) => None,
			(Some(Slot::WeaponAquaticA),_,_) => None,
			(Some(Slot::WeaponAquaticB),_,_) => None,

			(Some(slot), _, Some(u))		=> Some(format!("- Unknown {:?}, {}", slot, u.to_markup()?)),
			(Some(slot),_,_)				=> Some(format!("- Unknown {:?}", slot)),
			(None,_,_) => None,
		}?)
	}
}

fn create_page(c: &Character, gear_arg: &String, build_arg: &String) -> Option<String> {

	if c.core.profession == Profession::Revenant {
		todo!("Revenant legend support");
	}

	let gearidx = gear_arg.parse::<usize>().unwrap_or(c.active_equipment_tab.unwrap());
	let buildidx = build_arg.parse::<usize>().unwrap_or(c.active_build_tab.unwrap());

	let gear = &c.equipment_tabs[gearidx-1].equipment;
	let build = &c.build_tabs[buildidx-1].build;

	Some(format!(concat!(
			"---\n",
			"layout: build\n",
			"author: AW2\n",
			"credit: AW2\n",
			"editor: AW2\n",
			"title: {id}\n",
			"tags: POWER_CONDI_HYBRID PROFESSION SPECIALIZATION EXPANSION LOWCOG LOWPHYS LOWREP\n",
			"tagline: FROM API\n",
			"balance: January 2024\n",
			"{build_frontmatter}\n",
			"---\n\n",
			"INSERT SUMMARY HERE",
			"\n\n",
			"## Gearing\n\n",
			"{gear}",
			"\n\n",
			"`CHATLINK`", // #TODO
			"\n\n",
			"{skills_and_traits}",
			"\n\n",
			"## Notes\n\n",
			"INSERT NOTES HERE\n\n",
			"## Other References\n\n",
			"{{% include video id=\"YOUTUBE_ID\" provider=\"youtube\" %}}",
			"\n\n",
			),
		id=c.core.name,
		build_frontmatter=build.to_frontmatter(),
		gear=gear.to_markup()?,
		skills_and_traits=build.to_markup()?
	))
}

fn print_available_characters(key: &String) {
	let client = Client::default().api_key(key);
	client.ids::<Character, CharacterId>().unwrap()
		.into_iter()
		.for_each(|name| println!("- {name}"));
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 || args.len() > 5 || args[1] == "-h" || args[1] == "--help" {
        println!("Usage: {} <api-key> [<character-name> [<equipment-tab> <build-tab>]]", args[0]);
		println!("  extract an equipment and build tab into website format (currently AW2/Armory only)");
		println!("  if only api-key is provided, displays a list of characters");
		println!("  if only api-key and character-name are specified, display a numbered list of equipment and build tabs");
		println!("  if equipment-tab and build-tab is 'x', display current equipment/build");
        process::exit(1);
    }

	let key = &args[1];
	if args.len() == 2 {
		print_available_characters(key);

	} else if args.len() >= 3 {
		let id = CharacterId::from(&args[2]);
		let client = Client::default().api_key(key);
		if let Ok(c) = client.single::<Character, CharacterId>(id.clone()) {

			if args.len() < 4 {
				// only list equipment/build tabs
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
				println!("{}", create_page(&c, &args[3], &args[4]).unwrap_or(String::from("invalid build")));
			}

		} else {
			eprintln!("Character {id} not found. Available characters:");
			print_available_characters(key);
		}
	}
    Ok(())
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn get_character_names() {
		let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
		let names: Vec<CharacterId> = client.ids::<Character, CharacterId>().unwrap();
		assert!(names.contains(&CharacterId::from("Johnny Vicious")));
	}

	#[test]
	#[ignore]
	fn get_equipment() { 
		/// #TODO
		let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
		let id = CharacterId::from("Johnny Vicious");
		let c: Character = client.single(id).unwrap();
	}

	#[test]
	#[ignore]
	fn get_known_character() { 
		// relatively expensive
		let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
		let id = CharacterId::from("Johnny Vicious");
		let c: Character = client.single(id).unwrap();
		assert_eq!(c.core.name, "Johnny Vicious");
	}

	#[test]
	#[ignore]
	fn get_bulk_characters() {
		let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
		let characters: Vec<Character> = client.all().unwrap();
		assert_eq!(characters.len(), 15);
	}
}
