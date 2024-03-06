use itertools::Itertools;

use gw2lib::{Client, Requester};
use gw2lib::model::items::{Item, ItemId, Details};
use gw2lib::model::items::itemstats::{ItemStat, StatsId};
use gw2lib::model::game_mechanics::pets::Pet;
use gw2lib::model::authenticated::{
	characters::{BuildTemplate, Skillset, TraitLine, Equip, Slot, Stats, PetId, BuildPets},
};

pub trait ArmoryMarkup {
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
			"{pets}",
			"{legends}",
			"{skills}",
			"<div ",
			  "data-armory-embed='specializations' ",
			  "data-armory-ids='{spec1},{spec2},{spec3}' ",
			  "{traitline1} {traitline2} {traitline3}",
			">",
			"</div>"),
			pets=match &self.pets {
				Some(pets) => pets.to_markup()?,
				_ => String::from("")
			},
			legends=match &self.legends {
				// Some(legends) => legends.to_markup()?,
				_ => String::from("")
			},
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

impl ArmoryMarkup for PetId {
	fn to_markup(&self) -> Option<String>
	{
		// Some(format!("PETID({})", self))
		let client = Client::default();
		let result = client.single::<Pet, PetId>(*self);
		match result {
			Ok(pet) => Some(format!("{}", pet.name)),
			_ => None
		}
	}
}

impl ArmoryMarkup for BuildPets {
	fn to_markup(&self) -> Option<String>
	{
		Some(format!("Pets: {}, {}\n\n", self.terrestrial[0]?.to_markup()?, self.terrestrial[1]?.to_markup()?))
	}
}
