use gw2lib::model::authenticated::characters::BuildTemplate as Gw2libTemplate;
use gw2lib::model::authenticated::characters::Profession;
use chatr::BuildTemplate as ChatrTemplate;

pub trait ToChatr {
    fn to_chatr(&self) -> Option<ChatrTemplate>;
}

impl ToChatr for Gw2libTemplate {
    fn to_chatr(&self) -> Option<ChatrTemplate>
    {
        let mut bt = ChatrTemplate::default();
        bt.profession = match self.profession.as_ref()? {
			Profession::Guardian => 1,
			Profession::Warrior => 2,
			Profession::Engineer => 3,
			Profession::Ranger => 4,
			Profession::Thief => 5,
			Profession::Elementalist => 6,
			Profession::Mesmer => 7,
			Profession::Necromancer => 8,
			Profession::Revenant => 9,
		};

		let skills: [u16 ; 5] = [
			self.skills.heal.unwrap().try_into().unwrap(),
			self.skills.utilities[0].unwrap().try_into().unwrap(),
			self.skills.utilities[1].unwrap().try_into().unwrap(),
			self.skills.utilities[2].unwrap().try_into().unwrap(),
			self.skills.elite.unwrap().try_into().unwrap(),
		];
		bt.set_palette_ids_from_skill_ids(skills);

		let specs = [
			self.specializations[0].id.unwrap() as u8,
			self.specializations[1].id.unwrap() as u8,
			self.specializations[2].id.unwrap() as u8,
		];

		let traits = [
			self.specializations[0].traits.unwrap()[0].unwrap(),
			self.specializations[0].traits.unwrap()[1].unwrap(),
			self.specializations[0].traits.unwrap()[2].unwrap(),
			self.specializations[1].traits.unwrap()[0].unwrap(),
			self.specializations[1].traits.unwrap()[1].unwrap(),
			self.specializations[1].traits.unwrap()[2].unwrap(),
			self.specializations[2].traits.unwrap()[0].unwrap(),
			self.specializations[2].traits.unwrap()[1].unwrap(),
			self.specializations[2].traits.unwrap()[2].unwrap(),
		];
		bt.set_spec_and_trait_indexes_from_ids(specs, traits);

		Some(bt)
    }
}

pub trait ChatlinkMarkup {
	fn to_chatlink(&self) -> Option<String>;
}

impl ChatlinkMarkup for Gw2libTemplate {
	fn to_chatlink(&self) -> Option<String>
	{
		Some(self.to_chatr()?.to_decorated_chatcode())
	}
}
