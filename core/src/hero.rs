use crate::race::Race;

#[allow(dead_code)]
#[derive(Clone, Builder, Debug)]
#[builder(build_fn(skip))]
pub struct Hero {
    name: String,
    race: Race,
    abilities: Abilities,
}

impl Hero {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn abilities(&self) -> &Abilities {
        &self.abilities
    }
}

pub type AbilityPoints = u8;
#[derive(Copy, Clone, Debug, Default)]
pub struct Abilities {
    strength: AbilityPoints,
    dexterity: AbilityPoints,
    constitution: AbilityPoints,
    intelligence: AbilityPoints,
    wisdom: AbilityPoints,
    charisma: AbilityPoints,
}

impl Abilities {
    pub fn strength(&self) -> AbilityPoints {
        self.strength
    }
    pub fn dexterity(&self) -> AbilityPoints {
        self.dexterity
    }
    pub fn constitution(&self) -> AbilityPoints {
        self.constitution
    }
    pub fn intelligence(&self) -> AbilityPoints {
        self.intelligence
    }
    pub fn wisdom(&self) -> AbilityPoints {
        self.wisdom
    }
    pub fn charisma(&self) -> AbilityPoints {
        self.charisma
    }

    pub fn strength_mod(&self) -> i8 {
        Self::calc_mod(self.strength)
    }
    pub fn dexterity_mod(&self) -> i8 {
        Self::calc_mod(self.dexterity)
    }
    pub fn constitution_mod(&self) -> i8 {
        Self::calc_mod(self.constitution)
    }
    pub fn intelligence_mod(&self) -> i8 {
        Self::calc_mod(self.intelligence)
    }
    pub fn wisdom_mod(&self) -> i8 {
        Self::calc_mod(self.wisdom)
    }
    pub fn charisma_mod(&self) -> i8 {
        Self::calc_mod(self.charisma)
    }

    fn calc_mod(ability_points: AbilityPoints) -> i8 {
        ((ability_points as f32 - 10.0) / 2.0).floor() as i8
    }
}

impl std::ops::Add<&Race> for Abilities {
    type Output = Abilities;

    fn add(self, rhs: &Race) -> Self::Output {
        Abilities {
            strength: self.strength + rhs.strength_bonus,
            dexterity: self.dexterity + rhs.dexterity_bonus,
            constitution: self.constitution + rhs.constitution_bonus,
            intelligence: self.intelligence + rhs.intelligence_bonus,
            wisdom: self.wisdom + rhs.wisdom_bonus,
            charisma: self.charisma + rhs.charisma_bonus,
        }
    }
}

#[test]
fn test_mod_negative() {
    let params = vec![
        (9, -1),
        (8, -1),
        (7, -2),
        (6, -2),
        (5, -3),
        (4, -3),
        (3, -4),
        (2, -4),
        (1, -5),
    ];
    test_mod(&params);
}

#[test]
fn test_zero_mod() {
    test_mod(&vec![(10, 0), (11, 0)]);
}
#[test]
fn test_positive_mod() {
    let params = vec![
        (12, 1),
        (13, 1),
        (14, 2),
        (15, 2),
        (16, 3),
        (17, 3),
        (18, 4),
        (19, 4),
        (20, 5),
        (21, 5),
        (22, 6),
        (23, 6),
        (24, 7),
        (25, 7),
        (26, 8),
        (27, 8),
        (28, 9),
        (29, 9),
        (30, 10),
    ];
    test_mod(&params);
}

/// test helper method
#[allow(dead_code)]
fn test_mod(params: &[(AbilityPoints, i8)]) {
    let mut sut = Abilities::default();
    for (a_points, expected_mod) in params {
        sut.charisma = *a_points;
        assert_eq!(
            *expected_mod,
            sut.charisma_mod(),
            "wrong mod for {}",
            a_points
        );
    }
}

impl HeroBuilder {
    pub fn build(self) -> Result<Hero, HeroBuilderError> {
        let abilities = self
            .abilities
            .ok_or(HeroBuilderError::UninitializedField("abilities"))?;
        let race = self
            .race
            .ok_or(HeroBuilderError::UninitializedField("race"))?;

        Ok(Hero {
            name: self
                .name
                .ok_or(HeroBuilderError::UninitializedField("name"))?,
            abilities: abilities + &race,
            race: race,
        })
    }
}
