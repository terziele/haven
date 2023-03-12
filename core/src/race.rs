use crate::hero::AbilityPoints;

#[derive(Clone, Debug)]
pub struct Race {
    pub name: String,
    pub strength_bonus: AbilityPoints,
    pub dexterity_bonus: AbilityPoints,
    pub constitution_bonus: AbilityPoints,
    pub intelligence_bonus: AbilityPoints,
    pub wisdom_bonus: AbilityPoints,
    pub charisma_bonus: AbilityPoints,
}

impl Race {
    pub fn builder() -> RaceBuilder {
        RaceBuilder::default()
    }
}

#[derive(Default)]
pub struct RaceBuilder {
    name: Option<String>,
    strength_bonus: Option<AbilityPoints>,
    dexterity_bonus: Option<AbilityPoints>,
    constitution_bonus: Option<AbilityPoints>,
    intelligence_bonus: Option<AbilityPoints>,
    wisdom_bonus: Option<AbilityPoints>,
    charisma_bonus: Option<AbilityPoints>,
}

impl RaceBuilder {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    pub fn with_strength_bonus(mut self, strength_bonus: AbilityPoints) -> Self {
        self.strength_bonus = Some(strength_bonus);
        self
    }
    pub fn with_dexterity_bonus(mut self, dexterity_bonus: AbilityPoints) -> Self {
        self.dexterity_bonus = Some(dexterity_bonus);
        self
    }
    pub fn with_constitution_bonus(mut self, constitution_bonus: AbilityPoints) -> Self {
        self.constitution_bonus = Some(constitution_bonus);
        self
    }
    pub fn with_intelligence_bonus(mut self, intelligence_bonus: AbilityPoints) -> Self {
        self.intelligence_bonus = Some(intelligence_bonus);
        self
    }
    pub fn with_wisdom_bonus(mut self, wisdom_bonus: AbilityPoints) -> Self {
        self.wisdom_bonus = Some(wisdom_bonus);
        self
    }
    pub fn with_charisma_bonus(mut self, charisma_bonus: AbilityPoints) -> Self {
        self.charisma_bonus = Some(charisma_bonus);
        self
    }

    pub fn build(self) -> Race {
        Race {
            name: self.name.expect("name"),
            strength_bonus: self.strength_bonus.expect("strength_bonus"),
            dexterity_bonus: self.dexterity_bonus.expect("dexterity_bonus"),
            constitution_bonus: self.constitution_bonus.expect("constitution_bonus"),
            intelligence_bonus: self.intelligence_bonus.expect("intelligence_bonus"),
            wisdom_bonus: self.wisdom_bonus.expect("wisdom_bonus"),
            charisma_bonus: self.charisma_bonus.expect("charisma_bonus"),
        }
    }
}
