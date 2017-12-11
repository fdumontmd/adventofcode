#[derive(Copy, Clone, Debug)]
struct Fighter {
    hit_points: u32,
    damage: u32,
    armor: u32,
}

const MONSTER: Fighter = Fighter{
    hit_points: 109,
    damage: 8,
    armor: 2,
};

#[derive(Eq,PartialEq, Debug)]
enum Outcome {
    Victory,
    Loss,
}

#[derive(Copy, Clone, Debug)]
struct Stats {
    name: &'static str,
    cost: u32,
    damage: u32,
    armor: u32,
}

#[derive(Copy, Clone, Debug)]
struct Weapon(Stats);
#[derive(Copy, Clone, Debug)]
struct Armor(Stats);
#[derive(Copy, Clone, Debug)]
struct Ring(Stats);

impl Weapon {
    fn new(name: &'static str, cost: u32, damage: u32) -> Self {
        Weapon(Stats{ name, cost, damage, armor: 0})
    }
}

impl Armor {
    fn new(name: &'static str, cost: u32, armor: u32) -> Self {
        Armor(Stats{ name, cost, damage: 0, armor })
    }
}

impl Ring {
    fn new(name: &'static str, cost: u32, damage: u32, armor: u32) -> Self {
        Ring(Stats{name, cost, damage, armor})
    }
}

#[derive(Debug)]
struct Setup {
    price: u32,
    weapon: Weapon,
    armor: Armor,
    left_ring: Ring,
    right_ring: Ring,
}

struct SetupData {
    weapons: Vec<Weapon>,
    armors: Vec<Armor>,
    rings: Vec<Ring>,
}

impl SetupData {
    fn new() -> Self {
        SetupData {
            weapons: vec![
                Weapon::new("Dagger", 8, 4),
                Weapon::new("Shortsword", 10, 5),
                Weapon::new("Warhammer", 25, 6),
                Weapon::new("Longsword", 40, 7),
                Weapon::new("Greataxe", 74, 8),
            ],
            armors: vec![
                Armor::new("no armor", 0, 0),
                Armor::new("Leather", 13, 1),
                Armor::new("Chainmail", 31, 2),
                Armor::new("Splintmail", 53, 3),
                Armor::new("Bandedmail", 75, 4),
                Armor::new("Platemail", 102, 5),
            ],
            rings: vec![
                Ring::new("no ring", 0, 0, 0),
                Ring::new("no ring", 0, 0, 0),
                Ring::new("Damage +1", 25, 1, 0),
                Ring::new("Damage +2", 50, 2, 0),
                Ring::new("Damage +3", 100, 3, 0),
                Ring::new("Defense +1", 20, 0, 1),
                Ring::new("Defense +2", 40, 0, 2),
                Ring::new("Defense +3", 80, 0, 3),
            ],
        }
    }

    fn iter(&self) -> SetupIterator {
        SetupIterator {
            data: self,
            weapon_idx: 0,
            armor_idx: 0,
            ring_1_idx: 0,
            ring_2_idx: 1,
        }
    }
}

struct SetupIterator<'a> {
    data: &'a SetupData,
    weapon_idx: usize,
    armor_idx: usize,
    ring_1_idx: usize,
    ring_2_idx: usize,
}

impl<'a> SetupIterator<'a> {
    fn next_idx(&mut self) {
        if self.weapon_idx < self.data.weapons.len() {
            self.ring_2_idx += 1;
            if self.ring_2_idx >= self.data.rings.len() {
                self.ring_1_idx += 1;
                self.ring_2_idx = self.ring_1_idx + 1;
            }
            if self.ring_1_idx >= self.data.rings.len() - 1 {
                self.armor_idx += 1;
                self.ring_1_idx = 0;
                self.ring_2_idx = 1;
            }
            if self.armor_idx >= self.data.armors.len() {
                self.weapon_idx += 1;
                self.armor_idx = 0;
                self.ring_1_idx = 0;
                self.ring_2_idx = 1;
            }
        }
    }
}

impl<'a> Iterator for SetupIterator<'a> {
    type Item = Setup;
    fn next(&mut self) -> Option<Self::Item> {
        if self.weapon_idx >= self.data.weapons.len() {
            None
        } else {
            let mut setup = Setup {
                price: 0,
                weapon: self.data.weapons[self.weapon_idx],
                armor: self.data.armors[self.armor_idx],
                right_ring: self.data.rings[self.ring_1_idx],
                left_ring: self.data.rings[self.ring_2_idx],
            };
            setup.price = setup.weapon.0.cost
                + setup.armor.0.cost
                + setup.right_ring.0.cost
                + setup.left_ring.0.cost;
            self.next_idx();
            Some(setup)
        }
    }
}

impl Fighter {
    fn new(setup: &Setup) -> Self {
        let damage = setup.weapon.0.damage
            + setup.left_ring.0.damage
            + setup.right_ring.0.damage;
        let armor = setup.armor.0.armor
            + setup.left_ring.0.armor
            + setup.right_ring.0.armor;
        Fighter{
            hit_points: 100,
            damage: damage,
            armor: armor,
        }
    }

    fn fight(&self, other: &Fighter) -> Outcome {
        let mut damage_self = 0;
        let mut damage_other = 0;

        loop {
            damage_other += if self.damage <= other.armor {
                1
            } else {
                self.damage - other.armor
            };

            if damage_other >= other.hit_points {
                return Outcome::Victory;
            }

            damage_self += if other.damage <= self.armor {
                1
            } else {
                other.damage - self.armor
            };

            if damage_self >= self.hit_points {
                return Outcome::Loss;
            }
        }
    }
}

fn main() {
    let setup_data = SetupData::new();

    let results = setup_data.iter().map(|s| {
        let f = Fighter::new(&s);
        match f.fight(&MONSTER) {
            Outcome::Victory => Some(s.price),
            Outcome::Loss => None,
        }
    }).filter(|o| o.is_some())
    .collect::<Option<Vec<u32>>>().unwrap();
    println!("Cheapest victory: {}", results.iter().min().unwrap());

    let results = setup_data.iter().map(|s| {
        let f = Fighter::new(&s);
        match f.fight(&MONSTER) {
            Outcome::Loss => Some(s.price),
            Outcome::Victory => None,
        }
    }).filter(|o| o.is_some())
        .collect::<Option<Vec<u32>>>().unwrap();
    println!("Most expensive loss: {}", results.iter().max().unwrap());
}

#[test]
fn test() {
    let monster = Fighter{
        hit_points: 12,
        damage: 7,
        armor: 2,
    };
    let you = Fighter{
        hit_points: 8,
        damage: 5,
        armor: 5,
    };

    assert_eq!(Outcome::Victory, you.fight(&monster));
}
