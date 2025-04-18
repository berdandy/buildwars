use gw2lib::{Client, Requester};
use gw2lib::model::authenticated::characters::{Character, CharacterId, Profession};

pub mod frontmatter;
pub use frontmatter::FrontmatterMarkup;

pub mod chatlink;
pub use chatlink::ChatlinkMarkup;

pub mod aw2;
pub use aw2::Aw2Markup;

static USE_DEFAULT_EQUIPMENT_ON_CHAR: bool = true;

pub fn create_page(c: &Character, gear_arg: &String, build_arg: &String) -> Option<String> {
	let buildidx = build_arg.parse::<usize>().unwrap_or(c.active_build_tab.unwrap());
	let build = &c.build_tabs[buildidx-1].build;

	let gear = match gear_arg.parse::<usize>() {
		Ok(gearidx) => &c.equipment_tabs[gearidx-1].equipment,
		_ => match USE_DEFAULT_EQUIPMENT_ON_CHAR {
			true => &c.equipment,														// character equipment
			false => &c.equipment_tabs[c.active_equipment_tab.unwrap() - 1].equipment,	// active tab
		}
	};

	Some(format!(concat!(
			"+++\n",
			"{build_frontmatter}\n",
			"+++\n\n",
			"**DESCRIPTION**",
			"\n\n",
			"# Gearing\n\n",
			"{gear}\n\n",
			"# Build\n\n",
			"`{chatlink}`\n\n",
			"---\n\n",
			"{skills_and_traits}",
			"\n\n",
			"# Notes\n\n",
			"### Rotation\n\n",
			"**ROTATION AND BUILD NOTES**\n\n",
			"### Crowd Control\n\n",
			"**CC NOTES**\n\n",
			"# Video\n\n",
			"{{ youtube(id=\"**YOUTUBE_ID**\") }}",
			"\n\n",
			),
		build_frontmatter=build.to_frontmatter()?,
		gear=gear.to_markup()?,
		chatlink=match c.core.profession {
			Profession::Revenant | Profession::Ranger => String::from("CHATCODE"),
			_ => build.to_chatlink()?,
		},
		skills_and_traits=build.to_markup()?
	))
}

pub fn print_available_characters(key: &String) {
	let client = Client::default().api_key(key);
	client.ids::<Character, CharacterId>().unwrap()
		.into_iter()
		.for_each(|name| println!("\"{name}\""));
}

pub fn print_available_characters_detailed(key: &String) {
	let client = Client::default().api_key(key);
    for (i, c) in client.all::<Character, CharacterId>().unwrap()
		.into_iter()
        .enumerate()
    {
        println!("{}: \"{}\" ({:?} {:?} {:?})", i+1, c.core.name, c.core.gender, c.core.race, c.core.profession);
    }
}

