use rand::Rng;

const D6: Dice = Dice(6);

pub struct Dice(u8);

impl Dice {
    pub fn roll(&self) -> u8 {
        rand::thread_rng().gen_range(1..self.0 + 1)
    }
}

#[test]
fn test_d6() {
    let roll = D6.roll();
    assert!(roll <= 6);
    assert!(roll > 0);
}
