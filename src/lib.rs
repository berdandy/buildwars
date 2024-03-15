use gw2lib::{Client, Requester};
use gw2lib::model::authenticated::{
	characters::{Character, CharacterId},
};

pub mod frontmatter;
pub use frontmatter::FrontmatterMarkup;

pub mod aw2;
pub use aw2::Aw2Markup;

pub fn create_page(c: &Character, gear_arg: &String, build_arg: &String) -> Option<String> {
	let gearidx = gear_arg.parse::<usize>().unwrap_or(c.active_equipment_tab.unwrap());
	let buildidx = build_arg.parse::<usize>().unwrap_or(c.active_build_tab.unwrap());

	let gear = &c.equipment_tabs[gearidx-1].equipment;
	let build = &c.build_tabs[buildidx-1].build;

	Some(format!(concat!(
			"+++\n",
			"{build_frontmatter}\n",
			"+++\n",
			"\n\n",
			"DESCRIPTION",
			"\n\n",
			"## Gearing\n\n",
			"{gear}",
			"\n\n",
			"---\n",
			"\n\n",
			"`CHATLINK`", // #TODO
			"\n\n",
			"{skills_and_traits}",
			"\n\n",
			"## Notes\n\n",
			"INSERT NOTES HERE\n\n",
			"## Crowd Control\n\n",
			"INSERT CC NOTES HERE\n\n",
			"## Video\n\n",
			"{{ youtube(id=\"YOUTUBE_ID\") }}",
			"\n\n",
			),
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
