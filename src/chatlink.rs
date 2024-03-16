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

		todo!("conversion of gw2lib BuildTemplate to chatr BuildTemplate");

		// Some(bt)
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
