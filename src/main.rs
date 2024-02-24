/// use gw2lib::{Client, Requester};
/// use gw2lib_model::authenticated::{
///     account::Account,
///     characters::{Character, CharacterId},
/// };
///
/// let client = Client::default().api_key("<subtoken>");
/// let account: Account = client.get().unwrap();
/// let client = client.identifier(&account.id);
///
/// // make a request
/// let characters: Vec<CharacterId> = client.ids::<Character, CharacterId>().unwrap();
///
/// let client = Client::default().api_key("<different subtoken>");
/// let client = client.identifier(account.id);
///
/// // cache hit
/// let characters: Vec<CharacterId> = client.ids::<Character, CharacterId>().unwrap();

use gw2lib::{Client, Requester};
use gw2lib::model::authenticated::{
	characters::{Character, CharacterId, BuildTemplate, Profession},
};

trait ArmoryMarkup {
	fn to_frontmatter(&self) -> String;
	fn to_markup(&self) -> String;
}

impl ArmoryMarkup for BuildTemplate {

	fn to_frontmatter(&self) -> String
	{
		let spec = match self.specializations[2].id.unwrap() {
			5 => String::from("druid"),
			7 => String::from("daredevil"),
			18 => String::from("berserker"),
			27 => String::from("dragonhunter"),
			34 => String::from("reaper"),
			40 => String::from("chronomancer"),
			43 => String::from("scrapper"),
			48 => String::from("tempest"),
			52 => String::from("herald"),
			
			55 => String::from("soulbeast"),
			56 => String::from("weaver"),
			57 => String::from("holosmith"),
			58 => String::from("deadeye"),
			59 => String::from("mirage"),
			60 => String::from("scourge"),
			61 => String::from("spellbreaker"),
			62 => String::from("firebrand"),
			63 => String::from("renegade"),

			64 => String::from("harbinger"),
			65 => String::from("willbender"),
			66 => String::from("virtuoso"),
			67 => String::from("catalyst"),
			68 => String::from("bladesworn"),
			69 => String::from("vindicator"),
			70 => String::from("mechanist"),
			71 => String::from("specter"),
			72 => String::from("untamed"),

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

	fn to_markup(&self) -> String
	{
		format!(concat!(
			"<div ",
			  "data-armory-embed='skills' ",
			  "data-armory-ids='{healing},{utility1},{utility2},{utility3},{elite}'",
			"></div>",
			"<div ",
			  "data-armory-embed='specializations' ",
			  "data-armory-ids='{spec1},{spec2},{spec3}' ",
			  "data-armory-{spec1}-traits='{trait11},{trait12},{trait13}' ",
			  "data-armory-{spec2}-traits='{trait21},{trait22},{trait23}' ",
			  "data-armory-{spec3}-traits='{trait31},{trait32},{trait33}'",
			">",
			"</div>"),
			// TODO: make this less dumb
			healing=self.skills.heal.unwrap(),
			utility1=self.skills.utilities[0].unwrap(),
			utility2=self.skills.utilities[1].unwrap(),
			utility3=self.skills.utilities[2].unwrap(),
			elite=self.skills.elite.unwrap(),
			spec1=self.specializations[0].id.unwrap(),
			spec2=self.specializations[1].id.unwrap(),
			spec3=self.specializations[2].id.unwrap(),
			trait11=self.specializations[0].traits.unwrap()[0].unwrap(),
			trait12=self.specializations[0].traits.unwrap()[1].unwrap(),
			trait13=self.specializations[0].traits.unwrap()[2].unwrap(),
			trait21=self.specializations[1].traits.unwrap()[0].unwrap(),
			trait22=self.specializations[1].traits.unwrap()[1].unwrap(),
			trait23=self.specializations[1].traits.unwrap()[2].unwrap(),
			trait31=self.specializations[2].traits.unwrap()[0].unwrap(),
			trait32=self.specializations[2].traits.unwrap()[1].unwrap(),
			trait33=self.specializations[2].traits.unwrap()[2].unwrap(),
		)
	}
}

fn main() {
	let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
	let id = CharacterId::from("Johnny Vicious");
	let individual: Character = client.single(id).unwrap();

	// let equipidx = individual.active_equipment_tab.unwrap();
	// println!("Active Equipment: {}", equipidx);
	// println!("Equipment: {:?}", individual.equipment_tabs[equipidx].equipment);

	let buildidx = individual.active_build_tab.unwrap();
	let build = &individual.build_tabs[buildidx-1].build;
	// println!("Active Build: {}", buildidx);
	println!("Frontmatter:\n\n{}\n", build.to_frontmatter());
	println!("Markup:\n\n{}\n", build.to_markup());
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
	fn get_equipment() { 
		// relatively expensive
		let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
		let id = CharacterId::from("Johnny Vicious");
		let individual: Character = client.single(id).unwrap();
		assert_eq!(individual.core.name, "Johnny Vicious");
	}

	#[test]
	#[ignore]
	fn get_known_character() { 
		// relatively expensive
		let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
		let id = CharacterId::from("Johnny Vicious");
		let individual: Character = client.single(id).unwrap();
		assert_eq!(individual.core.name, "Johnny Vicious");
	}

	#[test]
	#[ignore]
	fn get_bulk_characters() {
		let client = Client::default().api_key("90791260-3DC7-D94C-8004-040CB45D645BD6E50684-1FFD-4169-A456-20F8AE7A22A2");
		let characters: Vec<Character> = client.all().unwrap();
		assert_eq!(characters.len(), 15);
	}
}
