use gw2lib::model::authenticated::characters::{BuildTemplate, Profession};

pub trait FrontmatterMarkup {
	fn to_frontmatter(&self) -> Option<String>;
}

impl FrontmatterMarkup for BuildTemplate {
	fn to_frontmatter(&self) -> Option<String>
	{
		let spec = match self.specializations[2].id.unwrap() {
			// HoT elites
			5 => Some(String::from("druid")),
			7 => Some(String::from("daredevil")),
			18 => Some(String::from("berserker")),
			27 => Some(String::from("dragonhunter")),
			34 => Some(String::from("reaper")),
			40 => Some(String::from("chronomancer")),
			43 => Some(String::from("scrapper")),
			48 => Some(String::from("tempest")),
			52 => Some(String::from("herald")),
			
			// PoF elites
			55 => Some(String::from("soulbeast")),
			56 => Some(String::from("weaver")),
			57 => Some(String::from("holosmith")),
			58 => Some(String::from("deadeye")),
			59 => Some(String::from("mirage")),
			60 => Some(String::from("scourge")),
			61 => Some(String::from("spellbreaker")),
			62 => Some(String::from("firebrand")),
			63 => Some(String::from("renegade")),

			// EoD elites
			64 => Some(String::from("harbinger")),
			65 => Some(String::from("willbender")),
			66 => Some(String::from("virtuoso")),
			67 => Some(String::from("catalyst")),
			68 => Some(String::from("bladesworn")),
			69 => Some(String::from("vindicator")),
			70 => Some(String::from("mechanist")),
			71 => Some(String::from("specter")),
			72 => Some(String::from("untamed")),

			// core or no third spec
			_ => match self.profession.as_ref() {
				Some(Profession::Elementalist) => Some(String::from("elementalist")), 
				Some(Profession::Engineer) => Some(String::from("engineer")), 
				Some(Profession::Guardian) => Some(String::from("guardian")), 
				Some(Profession::Mesmer) => Some(String::from("mesmer")), 
				Some(Profession::Necromancer) => Some(String::from("necromancer")), 
				Some(Profession::Ranger) => Some(String::from("ranger")), 
				Some(Profession::Revenant) => Some(String::from("revenant")), 
				Some(Profession::Thief) => Some(String::from("thief")), 
				Some(Profession::Warrior) => Some(String::from("warrior")), 
				None => None
			}
		};
		Some(format!("spec: {}", spec?))
	}
}
