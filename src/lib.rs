use gw2lib::{Client, Requester};
use gw2lib::model::authenticated::{
	characters::{Character, CharacterId, Profession},
};

pub mod frontmatter;
pub use frontmatter::FrontmatterMarkup;

pub mod armory;
pub use armory::ArmoryMarkup;

pub fn create_page(c: &Character, gear_arg: &String, build_arg: &String) -> Option<String> {

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
		build_frontmatter=build.to_frontmatter()?,
		gear=gear.to_markup()?,
		skills_and_traits=build.to_markup()?
	))
}

pub fn print_available_characters(key: &String) {
	let client = Client::default().api_key(key);
	client.ids::<Character, CharacterId>().unwrap()
		.into_iter()
		.for_each(|name| println!("- {name}"));
}
