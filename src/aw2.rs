use itertools::Itertools;

use gw2lib::{Client, Requester};
use gw2lib::model::items::{Item, ItemId, Details, WeaponDetails, ArmorDetails, InfixUpgrade};
use gw2lib::model::items::itemstats::{ItemStat, StatsId};
use gw2lib::model::game_mechanics::pets::{Pet, PetId};
// use gw2lib::model::game_mechanics::legends::Legend;
use gw2lib::model::authenticated::{
	characters::{BuildTemplate, Profession, Skillset, TraitLine, Equip, Slot, Stats, BuildPets, LegendSlots, LegendId },
};

pub trait Aw2Markup {
	fn to_markup(&self) -> Option<String>;
}

// note: this only really works in the context of a full BuildTemplate
impl Aw2Markup for TraitLine {
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

impl Aw2Markup for Skillset {
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

impl Aw2Markup for BuildTemplate {
	fn to_markup(&self) -> Option<String>
	{
		if self.profession == Some(Profession::Revenant) {
			eprintln!("Warning: revenant legends not implemented. Skills will likely be wrong");
		}
		
		/*
		let legends = [
			self.legends?[0].clone()?.to_markup()?,
			self.legends?[1].clone()?.to_markup()?
		];

		let fixed_legends = legends
			.iter()
			.map(|legend| match &legend[..] {
				"Fire" => String::from("Legend1"),
				"Water" => String::from("Legend2"),
				"Air" => String::from("Legend3"),
				"Earth" => String::from("Legend4"),
				"Deathshroud" => String::from("Legend6"),
				_ => match &self.specializations[2].id {
					Some(63) => String::from("Legend5"),
					Some(69) => String::from("Legend7"),	// doesn't exist in API
					_ => String::from(""),
				}
			}).collect();
		*/

		Some(format!(concat!(
			"{pets}",
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

impl Aw2Markup for InfixUpgrade {
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

impl Aw2Markup for WeaponDetails {
	fn to_markup(&self) -> Option<String>
    {
        match &self.upgrades.infix_upgrade {
            // Zojja's Hammer, etc
			Some(upg) => Some(format!("INFIX {} {:?}", upg.to_markup()?, self._type)),

            // Legendary with stat
			_ => Some(format!("LEGENDARY {:?}", self._type))
        }
    }
}

impl Aw2Markup for ArmorDetails {
	fn to_markup(&self) -> Option<String>
    {
        match &self.upgrades.infix_upgrade {
            // Zojja's Greaves, etc
			Some(upg) => Some(format!("INFIX {} {:?}", upg.to_markup()?, self._type)),

            // Legendary with stat
			//_ => Some(format!("{:?}", self._type))
            
            // debug:
			_ => {

                /*
                let client = Client::default();
                let result = client.single::<ItemStat, ItemId>(self.id);

                Some(format!("UNKNOWN ARMOR<<<{:?} {:?}>>>", self, result.unwrap()))
                */
                Some(format!("UNKNOWN ARMOR<<<{:?}>>>", self))
            }
        }
    }
}

impl Aw2Markup for ItemId
{
	fn to_markup(&self) -> Option<String>
	{
		let client = Client::default();
		let result = client.single::<Item, ItemId>(*self);
		match result {
			Ok(item) => match item.details {
				Details::Weapon(w) => w.to_markup(),
				Details::Armor(a) => a.to_markup(),
				_ => Some(item.name),
			}
			_ => None
		}
	}
}

impl Aw2Markup for Vec<ItemId>
{
	fn to_markup(&self) -> Option<String>
	{
		Some(self.iter().flat_map(|e| e.to_markup()).join(", "))
	}
}

impl Aw2Markup for Vec<Equip>
{
	fn to_markup(&self) -> Option<String>
	{
		Some(self.iter().flat_map(|e| e.to_markup()).join("\n"))
	}
}

/*
// generic
impl<T> Aw2Markup for Vec<T>
	where T: Aw2Markup
{
	fn to_markup(&self) -> Option<String>
	{
		Some(self.iter().flat_map(|e| e.to_markup()).join(", "))
	}
}
*/

impl Aw2Markup for Stats {
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

impl Aw2Markup for Equip {
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

			(Some(Slot::WeaponA1), Some(s), Some(u))	=> Some(format!("- Weapon A1: {} {}, {}", s.to_markup()?, self.id.to_markup()?, u.to_markup()?)),
			(Some(Slot::WeaponA2), Some(s), Some(u))	=> Some(format!("- Weapon A2: {} {}, {}", s.to_markup()?, self.id.to_markup()?, u.to_markup()?)),
			(Some(Slot::WeaponB1), Some(s), Some(u))	=> Some(format!("- Weapon B1: {} {}, {}", s.to_markup()?, self.id.to_markup()?, u.to_markup()?)),
			(Some(Slot::WeaponB2), Some(s), Some(u))	=> Some(format!("- Weapon B2: {} {}, {}", s.to_markup()?, self.id.to_markup()?, u.to_markup()?)),

			(Some(Slot::Relic), _, _)		=> Some(format!("- Relic: {}", self.id.to_markup()?)),

            // TODO: aquatic gear?
			(Some(Slot::HelmAquatic),_,_) => None,
			(Some(Slot::WeaponAquaticA),_,_) => None,
			(Some(Slot::WeaponAquaticB),_,_) => None,
            
            // fixed stat gear (named ascended, like Zojja's, etc)
			(Some(Slot::WeaponA1), _, Some(u))	=> Some(format!("- Weapon A1: {}, {}", self.id.to_markup()?, u.to_markup()?)),
			(Some(Slot::WeaponA2), _, Some(u))	=> Some(format!("- Weapon A2: {}, {}", self.id.to_markup()?, u.to_markup()?)),
			(Some(Slot::WeaponB1), _, Some(u))	=> Some(format!("- Weapon B1: {}, {}", self.id.to_markup()?, u.to_markup()?)),
			(Some(Slot::WeaponB2), _, Some(u))	=> Some(format!("- Weapon B2: {}, {}", self.id.to_markup()?, u.to_markup()?)),

            // TODO: non-legendary selectable stat
			(Some(Slot::WeaponA1), _, _)	    => Some(format!("- Weapon A1: {} DBG:{:?}", self.id.to_markup()?, self)),

			(Some(Slot::Boots), _, _)		=> Some(format!("- ??? Boots, {:?}, DBG:{:?}", self.id.to_markup()?, self)),
			(Some(Slot::Gloves), _, _)		=> Some(format!("- ??? Gloves, {:?}, DBG:{:?}", self.id.to_markup()?, self)),

            // degenerate cases
			(Some(slot), _, Some(u))		=> Some(format!("- UnknownStat {:?}, {} DBG:{:?}", slot, u.to_markup()?, self)),
			(Some(slot), _, _)				=> Some(format!("- Unknown {:?} DBG:{:?}", slot, self)),

			// (None,_,_) => None,
			(None,_,_) => Some(format!("? MYSTERY: {:?}", self)),	// <-- replace None(,_,_) line above to expose api wierdness instead of silent ignore
		}?)
	}
}

impl Aw2Markup for PetId {
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

impl Aw2Markup for BuildPets {
	fn to_markup(&self) -> Option<String>
	{
		Some(format!("Pets: {}, {}\n\n", self.terrestrial[0]?.to_markup()?, self.terrestrial[1]?.to_markup()?))
	}
}

impl Aw2Markup for LegendId {
	fn to_markup(&self) -> Option<String>
	{
		Some(format!("LEGEND({})", self))
/*
		let client = Client::default();
		let result = client.single::<Pet, PetId>(*self);
		match result {
			Ok(pet) => Some(format!("{}", pet.name)),
			_ => None
		}
*/
	}
}

impl Aw2Markup for LegendSlots {
	fn to_markup(&self) -> Option<String>
	{
		let legend1 = self[0].clone()?.to_markup()?;
		println!(">> {:?}", legend1);
		let legend2 = self[1].clone()?.to_markup()?;
		println!(">> >> {:?}", legend2);
		Some(format!("Legends: {legend1}, {legend2}\n\n"))
	}
}

