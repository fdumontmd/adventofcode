// stragegy: model the game, then generate spell plans based on maximum cost of mana (i.e. generate all plans that cost less
// then n, for each n), then iterate over these plans and see which one first wins.
use std::collections::BinaryHeap;
use std::rc::Rc;


#[derive(Clone, Copy, Debug)]
struct Fighter {
    hit_points: u32,
    damage: u32,
    armor: u32,
    mana: u32,
}

impl Fighter {
    fn hit(&mut self, damage: u32) {
        let damage = if damage > self.armor {
            damage - self.armor
        } else {
            1
        };

        if self.hit_points >= damage {
            self.hit_points -= damage;
        } else {
            self.hit_points = 0;
        }
    }

    fn is_defeated(&self) -> bool {
        self.hit_points == 0 
    }

    fn new_round(&mut self) {
        self.armor = 0;
    }
}

// need to differentiate between immediate spells and effect spells (with duration)
trait Spell {
    fn active(&self) -> bool;
    fn cost(&self) -> u32;
    fn cast(&self, player: &mut Fighter, boss: &mut Fighter) -> Result<(), ()>{
        if player.mana >= self.cost() {
            player.mana -= self.cost();
            self.apply_immediate(player, boss);
            Ok(())
        } else {
            Err(())
        }
    }

    fn apply_immediate(&self, _player: &mut Fighter, _boss: &mut Fighter) {}

    fn effect(&mut self, _player: &mut Fighter, _boss: &mut Fighter) {}
}

struct MagicMissile();

impl Default for MagicMissile {
    fn default() -> Self {
        MagicMissile()
    }
}

impl Spell for MagicMissile {
    fn active(&self) -> bool { false }
    fn cost(&self) -> u32 {
        53
    }
    fn apply_immediate(&self, _player: &mut Fighter, boss: &mut Fighter) {
        boss.hit(4);
    }
}

struct Drain();

impl Default for Drain {
    fn default() -> Self {
        Drain()
    }
}

impl Spell for Drain {
    fn active(&self) -> bool { false }
    fn cost(&self) -> u32 {
        73
    }
    fn apply_immediate(&self, player: &mut Fighter, boss: &mut Fighter) {
        boss.hit(2);
        player.hit_points += 2;
    }
}

struct Shield{
    duration: u32,
}

impl Default for Shield {
    fn default() -> Self {
        Shield {
            duration: 6,
        }
    }
}

impl Spell for Shield {
    fn active(&self) -> bool {
        self.duration > 0
    }
    fn cost(&self) -> u32 {
        113
    }
    fn effect(&mut self, player: &mut Fighter, _boss: &mut Fighter) {
        self.duration -= 1;
        player.armor += 7;
    }
}

struct Poison {
    duration: u32,
}

impl Default for Poison {
    fn default() -> Self {
        Poison {
            duration: 6,
        }
    }
}

impl Spell for Poison {
    fn active(&self) -> bool {
        self.duration > 0
    }
    fn cost(&self) -> u32 {
        173
    }
    fn effect(&mut self, _player: &mut Fighter, boss: &mut Fighter) {
        self.duration -= 1;
        boss.hit(3);
    }
}

struct Recharge {
    duration: u32,
}

impl Default for Recharge {
    fn default() -> Self {
        Recharge {
            duration: 5,
        }
    }
}

impl Spell for Recharge {
    fn active(&self) -> bool {
        self.duration > 0
    }
    fn cost(&self) -> u32 {
        229
    }
    fn effect(&mut self, player: &mut Fighter, _boss: &mut Fighter) {
        self.duration -= 1;
        player.mana += 101;
    }
}

type BattlePlan = Vec<Box<Spell>>;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Outcome {
    Victory,
    Loss,
    Undecided,
}

fn battle(plan: &mut BattlePlan, player: Fighter, boss: Fighter) -> Outcome {
    let mut player = player;
    let mut boss = boss;
    let mut active_spells: Vec<&mut Box<Spell>> = Vec::new();

    for spell in plan {
        // Player turn
        player.new_round();
        for spell in &mut active_spells {
            spell.effect(&mut player, &mut boss);
        }

        if spell.cast(&mut player, &mut boss).is_err() {
            return Outcome::Loss;
        }

        if boss.is_defeated() {
            return Outcome::Victory;
        }

        active_spells = active_spells.into_iter().filter(|s| s.active()).collect();

        active_spells.push(spell);

        // Boss turn
        player.new_round();
        for spell in &mut active_spells {
            spell.effect(&mut player, &mut boss);
        }

        if boss.is_defeated() {
            return Outcome::Victory;
        }

        player.hit(boss.damage);

        if player.is_defeated() {
            return Outcome::Loss;
        }

        active_spells = active_spells.into_iter().filter(|s| s.active()).collect();
    }

    return Outcome::Undecided;
}

const MONSTER: Fighter = Fighter {
    hit_points: 71,
    damage: 10,
    armor: 0,
    mana: 0,
};



fn main() {
    println!("Hello, world!");
}

#[test]
fn test_plan_1() {
    let mut plan: BattlePlan = vec![:
        Box::new(Poison::default()),
        Box::new(MagicMissile::default()),
    ];

    let player = Fighter {
        hit_points: 10,
        damage: 0,
        armor: 0,
        mana: 250,
    };

    let boss = Fighter {
        hit_points: 13,
        damage: 8,
        armor: 0,
        mana: 0,
    };

    assert_eq!(battle(&mut plan, player, boss), Outcome::Victory);
}

#[test]
fn test_plan_2() {
    let mut plan: BattlePlan = vec![
        Box::new(Recharge::default()),
        Box::new(Shield::default()),
        Box::new(Drain::default()),
        Box::new(Poison::default()),
        Box::new(MagicMissile::default()),
    ];

    let player = Fighter {
        hit_points: 10,
        damage: 0,
        armor: 0,
        mana: 250,
    };

    let boss = Fighter {
        hit_points: 14,
        damage: 8,
        armor: 0,
        mana: 0,
    };

    assert_eq!(battle(&mut plan, player, boss), Outcome::Victory);
}
