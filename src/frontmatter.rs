use chrono::{Local};
use gw2lib::model::authenticated::characters::{BuildTemplate, Profession};

pub trait FrontmatterMarkup {
	fn to_frontmatter(&self) -> Option<String>;
}

impl FrontmatterMarkup for BuildTemplate {
	fn to_frontmatter(&self) -> Option<String>
	{
		let spec = match self.specializations[2].id.unwrap() {
			// HoT elites
			5 => Some(String::from("Druid")),
			7 => Some(String::from("Daredevil")),
			18 => Some(String::from("Berserker")),
			27 => Some(String::from("Dragonhunter")),
			34 => Some(String::from("Reaper")),
			40 => Some(String::from("Chronomancer")),
			43 => Some(String::from("Scrapper")),
			48 => Some(String::from("Tempest")),
			52 => Some(String::from("Herald")),
			
			// PoF elites
			55 => Some(String::from("Soulbeast")),
			56 => Some(String::from("Weaver")),
			57 => Some(String::from("Holosmith")),
			58 => Some(String::from("Deadeye")),
			59 => Some(String::from("Mirage")),
			60 => Some(String::from("Scourge")),
			61 => Some(String::from("Spellbreaker")),
			62 => Some(String::from("Firebrand")),
			63 => Some(String::from("Renegade")),

			// EoD elites
			64 => Some(String::from("Harbinger")),
			65 => Some(String::from("Willbender")),
			66 => Some(String::from("Virtuoso")),
			67 => Some(String::from("Catalyst")),
			68 => Some(String::from("Bladesworn")),
			69 => Some(String::from("Vindicator")),
			70 => Some(String::from("Mechanist")),
			71 => Some(String::from("Specter")),
			72 => Some(String::from("Untamed")),

			// core or no third spec
			_ => match self.profession.as_ref() {
				Some(Profession::Elementalist) => Some(String::from("Elementalist")), 
				Some(Profession::Engineer) => Some(String::from("Engineer")), 
				Some(Profession::Guardian) => Some(String::from("Guardian")), 
				Some(Profession::Mesmer) => Some(String::from("Mesmer")), 
				Some(Profession::Necromancer) => Some(String::from(format!("{:?}", self.profession.as_ref()))), 
				Some(Profession::Ranger) => Some(String::from("Ranger")), 
				Some(Profession::Revenant) => Some(String::from("Revenant")), 
				Some(Profession::Thief) => Some(String::from("Thief")), 
				Some(Profession::Warrior) => Some(String::from("Warrior")), 
				None => None
			}
		};
		let spec_lower = spec.as_ref().expect("Unknown profession/specialization in build!").to_ascii_lowercase();
		
		let desc = format!("{} Build", spec.as_ref()?);
		let prof = format!("{:?}", self.profession.as_ref()?);
		let prof_lower = format!("{:?}", self.profession.as_ref()?).to_ascii_lowercase();

		Some(format!(concat!(
				"title = \"{id}\"\n" ,
				"description = \"{desc}\"\n",
				"date = {today}\n",
				"draft = true\n",
				"template = \"build.html\"\n",
				"\n",
				"[taxonomies]\n",
				"tags = [\"CONDI_OR_POWER\",\"{prof_lower}\",\"{spec_lower}\",\"EXPANSION\",\"lowcog\",\"lowphys\",\"lowrep\"]\n",
				"authors = [\"YOURNAME\"]\n",
				"\n",
				"[extra]\n",
				"series = \"{prof_lower}\"\n",
				"tagline = \"SOMETHING SILLY\"\n",
				"keywords = \"Guild Wars 2, GW2, LI, {prof}, {spec}\"\n",
				"toc = true\n",
				"spec = \"{spec_lower}\"\n",
				"balance = \"January 2024\"",
			),
			id=self.name.as_ref()?,
			today=Local::now().date_naive().format("%Y-%m-%d").to_string(),
			desc=desc,
			prof_lower=prof_lower,
			prof=prof,
			spec_lower=spec_lower,
			spec=spec?,
		))
		
	}
}
