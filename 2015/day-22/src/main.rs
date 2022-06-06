#[macro_use]
extern crate log;
extern crate env_logger;

use std::rc::Rc;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Victory,
    Loss,
    Undecided,
}

#[derive(Debug)]
struct Player {
    hit_points: i32,
    armor: i32,
    damage: i32,
    mana: i32,
}

impl Player {
    fn new(hit_points: i32, damage: i32, mana: i32) -> Self {
        Player {
            hit_points,
            armor: 0,
            damage,
            mana,
        }
    }
    fn is_dead(&self) -> bool {
        self.hit_points <= 0
    }

    fn hit_by(&mut self, damage: i32) {
        let damage = (damage - self.armor).max(1);
        self.hit_points -= damage;
    }
}

trait Spell {
    fn cost(&self) -> i32;
    fn cast(&mut self, caster: &mut Player, opponent: &mut Player) -> Result<(), ()>;
    fn effect(&mut self, caster: &mut Player, opponent: &mut Player);
    fn is_active(&self) -> bool;
    fn display_name(&self) -> &'static str;
}

struct ActiveSpells(Vec<Box<Spell>>);

impl ActiveSpells {
    fn new() -> Self {
        ActiveSpells(Vec::new())
    }

    fn effects(&mut self, player: &mut Player, boss: &mut Player) {
        for spell in &mut self.0 {
            spell.effect(player, boss);
        }
        // NLL would help
        let v: Vec<Box<Spell>> = self.0.drain(..).filter(|s| s.is_active()).collect();
        self.0 = v;
    }

    fn cast(&mut self, mut spell: Box<Spell>, player: &mut Player, boss: &mut Player) -> Result<(), ()> {
        for s in &self.0 {
            if spell.display_name() == s.display_name() {
                return Err(());
            }
        }
        match spell.cast(player, boss) {
            Err(_) => Err(()),
            Ok(_) => {
                if spell.is_active() {
                    self.0.push(spell);
                }
                Ok(())
            }
        }
    }
}

fn battle(mut player: Player, mut boss: Player, battle_plan: Vec<fn () -> Box<Spell>>) -> Outcome {
    let mut active_spells = ActiveSpells::new();
    for g in battle_plan {
        info!("-- Player turn --");
        info!("- Player: {:?}", player);
        info!("- Boss: {:?}", boss);
        active_spells.effects(&mut player, &mut boss);
        match active_spells.cast(g(), &mut player, &mut boss) {
            Err(_) => return Outcome::Loss,
            _ => {}
        }

        if boss.is_dead() {
            return Outcome::Victory;
        }

        // boss round
        info!("-- Boss turn --");
        info!("- Player: {:?}", player);
        info!("- Boss: {:?}", boss);
        active_spells.effects(&mut player, &mut boss);
        if boss.is_dead() {
            return Outcome::Victory;
        }

        player.hit_by(boss.damage);
        if player.is_dead() {
            return Outcome::Loss;
        }
    }
    Outcome::Undecided
}

fn battle_hard(mut player: Player, mut boss: Player, battle_plan: Vec<fn () -> Box<Spell>>) -> Outcome {
    let mut active_spells = ActiveSpells::new();
    for g in battle_plan {
        info!("-- Player turn --");
        info!("- Player: {:?}", player);
        info!("- Boss: {:?}", boss);
        player.hit_by(1);
        active_spells.effects(&mut player, &mut boss);
        match active_spells.cast(g(), &mut player, &mut boss) {
            Err(_) => return Outcome::Loss,
            _ => {}
        }

        if boss.is_dead() {
            return Outcome::Victory;
        }

        // boss round
        info!("-- Boss turn --");
        info!("- Player: {:?}", player);
        info!("- Boss: {:?}", boss);
        active_spells.effects(&mut player, &mut boss);
        if boss.is_dead() {
            return Outcome::Victory;
        }

        player.hit_by(boss.damage);
        if player.is_dead() {
            return Outcome::Loss;
        }
    }
    Outcome::Undecided
}

struct MagicMissile();

impl MagicMissile {
    fn generate() -> Box<Spell> {
        Box::new(MagicMissile())
    }
    fn cost() -> i32 {
        53
    }
}

impl Spell for MagicMissile {
    fn display_name(&self) -> &'static str {
        "MagicMissile"
    }
    fn cast(&mut self, caster: &mut Player, opponent: &mut Player) -> Result<(), ()> {
        if caster.mana >= self.cost() {
            info!("Player casts Magic Missile");
            caster.mana -= self.cost();
            opponent.hit_by(4);
            Ok(())
        } else {
            Err(())
        }
    }
    fn effect(&mut self, _caster: &mut Player, _opponent: &mut Player) {}
    fn is_active(&self) -> bool {
        false
    }
    fn cost(&self) -> i32 {
        MagicMissile::cost()
    }
}

struct Drain();

impl Drain {
    fn generate() -> Box<Spell> {
        Box::new(Drain())
    }
    fn cost() -> i32 {
        73
    }
}

impl Spell for Drain {
    fn display_name(&self) -> &'static str {
        "Drain"
    }
    fn cast(&mut self, caster: &mut Player, opponent: &mut Player) -> Result<(), ()> {
        if caster.mana >= self.cost() {
            info!("Player casts Drain");
            caster.mana -= self.cost();
            opponent.hit_by(2);
            caster.hit_points += 2;
            Ok(())
        } else {
            Err(())
        }
    }
    fn effect(&mut self, _caster: &mut Player, _opponent: &mut Player) {}
    fn is_active(&self) -> bool {
        false
    }
    fn cost(&self) -> i32 {
        Drain::cost()
    }
}

struct Shield(u8);

impl Shield {
    fn generate() -> Box<Spell> {
        Box::new(Shield(6))
    }
    fn cost() -> i32 {
        113
    }
}

impl Spell for Shield {
    fn display_name(&self) -> &'static str {
        "Shield"
    }
    fn cast(&mut self, caster: &mut Player, _opponent: &mut Player) -> Result<(), ()> {
        if caster.mana >= self.cost() {
            info!("Player casts Shield");
            caster.mana -= self.cost();
            caster.armor += 7;
            Ok(())
        } else {
            Err(())
        }
    }

    fn effect(&mut self, caster: &mut Player, _opponent: &mut Player) {
        // Shield appear to have different behaviour from other spells: its effect starts when cast, so casting
        // turn counts as effect turn 
        if self.is_active() {
            self.0 -= 1;
            if self.0 == 0 {
                info!("Shield wears off");
                caster.armor -= 7;
            }
        }
    }

    fn is_active(&self) -> bool {
        self.0 > 0
    }

    fn cost(&self) -> i32 {
        Shield::cost()
    }
}

struct Poison(u8);

impl Poison {
    fn generate() -> Box<Spell> {
        Box::new(Poison(6))
    }
    fn cost() -> i32 {
        173
    }
}

impl Spell for Poison {
    fn display_name(&self) -> &'static str {
        "Poison"
    }
    fn cast(&mut self, caster: &mut Player, _opponent: &mut Player) -> Result<(), ()> {
        if caster.mana >= self.cost() {
            caster.mana -= self.cost();
            info!("Player casts Poison");
            Ok(())
        } else {
            Err(())
        }
    }

    fn effect(&mut self, _caster: &mut Player, opponent: &mut Player) {
        if self.is_active() {
            info!("Poison effect for {}", self.0);
            opponent.hit_by(3);
            self.0 -= 1;
        }
    }
    fn is_active(&self) -> bool {
        self.0 > 0
    }
    fn cost(&self) -> i32 {
        Poison::cost()
    }
}

struct Recharge(u8);

impl Recharge {
    fn generate() -> Box<Spell> {
        Box::new(Recharge(5))
    }
    fn cost() -> i32 {
        229
    }
}

impl Spell for Recharge {
    fn display_name(&self) -> &'static str {
        "Recharge"
    }
    fn cast(&mut self, caster: &mut Player, _opponent: &mut Player) -> Result<(), ()> {
        if caster.mana >= self.cost() {
            info!("Player casts Recharge");
            caster.mana -= self.cost();
            Ok(())
        } else {
            Err(())
        }
    }
    fn effect(&mut self, caster: &mut Player, _opponent: &mut Player) {
        if self.is_active() {
            info!("Recharge effect for {}", self.0);
            caster.mana += 101;
            self.0 -= 1;
        }
    }
    fn is_active(&self) -> bool {
        self.0 > 0
    }
    fn cost(&self) -> i32 {
        Recharge::cost()
    }
}

struct PlanSearch {
    plans: BinaryHeap<Rc<PlanProposal>>,
}

impl PlanSearch {
    fn new() -> Self {
        let mut plans = BinaryHeap::new();
        plans.push(Rc::new(PlanProposal::new(MagicMissile::cost(), MagicMissile::generate)));
        plans.push(Rc::new(PlanProposal::new(Drain::cost(), Drain::generate)));
        plans.push(Rc::new(PlanProposal::new(Shield::cost(), Shield::generate)));
        plans.push(Rc::new(PlanProposal::new(Poison::cost(), Poison::generate)));
        plans.push(Rc::new(PlanProposal::new(Recharge::cost(), Recharge::generate)));
        PlanSearch {
            plans,
        }
    }

    fn test(&mut self, player: Player, boss: Player) -> Option<i32> {
        let pp = self.plans.pop().expect("Plan empty?");
        info!("New plan ------------------------------------");
        match battle(player, boss, pp.generate_plan()) {
            Outcome::Loss => None,
            Outcome::Victory => {
                for gs in pp.generate_plan().into_iter() {
                    let s = gs();
                    println!("Casting {}", s.display_name());
                }
                Some(pp.total_cost)
            }
            Outcome::Undecided => {
                self.plans.push(Rc::new(PlanProposal::extend(&pp, MagicMissile::cost(), MagicMissile::generate)));
                self.plans.push(Rc::new(PlanProposal::extend(&pp, Drain::cost(), Drain::generate)));
                self.plans.push(Rc::new(PlanProposal::extend(&pp, Shield::cost(), Shield::generate)));
                self.plans.push(Rc::new(PlanProposal::extend(&pp, Poison::cost(), Poison::generate)));
                self.plans.push(Rc::new(PlanProposal::extend(&pp, Recharge::cost(), Recharge::generate)));
                None
            }
        }
    }

    fn test_hard(&mut self, player: Player, boss: Player) -> Option<i32> {
        let pp = self.plans.pop().expect("Plan empty?");
        info!("New plan ------------------------------------");
        match battle_hard(player, boss, pp.generate_plan()) {
            Outcome::Loss => None,
            Outcome::Victory => {
                for gs in pp.generate_plan().into_iter() {
                    let s = gs();
                    println!("Casting {}", s.display_name());
                }
                Some(pp.total_cost)
            }
            Outcome::Undecided => {
                self.plans.push(Rc::new(PlanProposal::extend(&pp, MagicMissile::cost(), MagicMissile::generate)));
                self.plans.push(Rc::new(PlanProposal::extend(&pp, Drain::cost(), Drain::generate)));
                self.plans.push(Rc::new(PlanProposal::extend(&pp, Shield::cost(), Shield::generate)));
                self.plans.push(Rc::new(PlanProposal::extend(&pp, Poison::cost(), Poison::generate)));
                self.plans.push(Rc::new(PlanProposal::extend(&pp, Recharge::cost(), Recharge::generate)));
                None
            }
        }
    }
}

#[derive(Eq)]
struct PlanProposal {
    total_cost: i32,
    spell_generator: fn () -> Box<Spell>,
    head: Option<Rc<PlanProposal>>,
}

impl Ord for PlanProposal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.total_cost.cmp(&self.total_cost)
    }
}

impl PartialOrd for PlanProposal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.total_cost.cmp(&self.total_cost))
    }
}

impl PartialEq for PlanProposal {
    fn eq(&self, other: &Self) -> bool {
        let ps = self as *const PlanProposal;
        let po = other as *const PlanProposal;
        ps == po
    }
}

impl PlanProposal {
    fn new(spell_cost: i32, spell_generator: fn () -> Box<Spell>) -> Self {
        PlanProposal {
            total_cost: spell_cost,
            spell_generator,
            head: None,
        }
    }

    fn extend(pp: &Rc<PlanProposal>, spell_cost: i32, spell_generator: fn () -> Box<Spell>) -> Self {
        PlanProposal {
            total_cost: pp.total_cost + spell_cost,
            spell_generator,
            head: Some(pp.clone()),
        }
    }
    fn generate_plan(&self) -> Vec<fn () -> Box<Spell>> {
        let mut v = Vec::new();

        let mut curr = self;
        v.push(curr.spell_generator);
        while let Some(ref pp) = curr.head {
            curr = &*pp;
            v.push(curr.spell_generator);
        }

        v.reverse();
        v
    }
}

fn main_1() {
    env_logger::init();

    let mut bp: Vec<fn () -> Box<Spell>> = Vec::new();
    bp.push(Shield::generate);
    bp.push(Recharge::generate);
    bp.push(Poison::generate);
    bp.push(Recharge::generate);
    bp.push(Poison::generate);
    bp.push(Poison::generate);
    bp.push(Shield::generate);
    bp.push(MagicMissile::generate);
    bp.push(Poison::generate);
    bp.push(MagicMissile::generate);

    let player = Player::new(50, 0, 500);
    let boss = Player::new(71, 10, 0);

    println!("{:?}", battle(player, boss, bp));
}

fn main() {
    env_logger::init();

    let mut ps = PlanSearch::new();

    loop {
        let player = Player::new(50, 0, 500);
        let boss = Player::new(71, 10, 0);
        match ps.test(player, boss) {
            Some(pp) => {
                println!("Minimum cost victory: {}", pp);
                break;
            }
            _ => {}
        }
    }

    let mut ps = PlanSearch::new();

    loop {
        let player = Player::new(50, 0, 500);
        let boss = Player::new(71, 10, 0);
        match ps.test_hard(player, boss) {
            Some(pp) => {
                println!("Minimum cost victory (hard mode): {}", pp);
                break;
            }
            _ => {}
        }
    }
}

#[test]
fn test() {
    let mut bp: Vec<fn () -> Box<Spell>> = Vec::new();
    bp.push(Poison::generate);
    bp.push(MagicMissile::generate);

    let player = Player::new(10, 0, 250);
    let boss = Player::new(13, 8, 0);

    assert_eq!(battle(player, boss, bp), Outcome::Victory);

    let mut bp: Vec<fn () -> Box<Spell>> = Vec::new();
    bp.push(Recharge::generate);
    bp.push(Shield::generate);
    bp.push(Drain::generate);
    bp.push(Poison::generate);
    bp.push(MagicMissile::generate);
    let player = Player::new(10, 0, 250);
    let boss = Player::new(14, 8, 0);

    assert_eq!(battle(player, boss, bp), Outcome::Victory);
}
