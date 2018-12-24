use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
enum AttackType {
    Bludgeoning = 1,
    Slashing,
    Cold,
    Fire,
    Radiation,
}

#[derive(Eq, Debug)]
struct Group {
    id: i32,
    units: i32,
    hit_points: i32,
    damage: i32,
    attack_type: AttackType,
    initiative: u32,
    immune_to: Vec<AttackType>,
    weak_to: Vec<AttackType>,
}

impl Group {
    fn effective_power(&self) -> i32 {
        return self.units * self.damage;
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Group) -> Ordering {
        if self.effective_power() != other.effective_power() {
            return self.effective_power().cmp(&other.effective_power());
        }
        return self.initiative.cmp(&other.initiative);
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Group) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Group {
    fn eq(&self, other: &Group) -> bool {
        self.id == other.id
    }
}

fn get_immune_system_army(boost: i32) -> Vec<Group> {
    vec![
        Group{id: 1, units: 2086, hit_points: 11953, damage: 46 + boost, attack_type: AttackType::Cold, initiative: 13, immune_to: vec![], weak_to: vec![]},
        Group{id: 1, units: 329, hit_points: 3402, damage: 90 + boost, attack_type: AttackType::Slashing, initiative: 1, immune_to: vec![], weak_to: vec![AttackType::Bludgeoning]},
        Group{id: 1, units: 414, hit_points: 7103 , damage: 170 + boost, attack_type: AttackType::Radiation, initiative: 4, immune_to: vec![AttackType::Radiation], weak_to: vec![AttackType::Bludgeoning]},
        Group{id: 1, units: 2205, hit_points: 7118, damage: 26 + boost, attack_type: AttackType::Radiation, initiative: 18, immune_to: vec![AttackType::Cold], weak_to: vec![AttackType::Fire]},
        Group{id: 1, units: 234, hit_points: 9284, damage: 287 + boost, attack_type: AttackType::Radiation, initiative: 12, immune_to: vec![AttackType::Cold, AttackType::Fire], weak_to: vec![AttackType::Slashing]},
        Group{id: 1, units: 6460, hit_points: 10804, damage: 15 + boost, attack_type: AttackType::Slashing, initiative: 11, immune_to: vec![], weak_to: vec![AttackType::Fire]},
        Group{id: 1, units: 79, hit_points: 1935, damage: 244 + boost, attack_type: AttackType::Radiation, initiative: 8, immune_to: vec![], weak_to: vec![]},
        Group{id: 1, units: 919, hit_points: 2403, damage: 22 + boost, attack_type: AttackType::Slashing, initiative: 2, immune_to: vec![], weak_to: vec![AttackType::Fire]},
        Group{id: 1, units: 172, hit_points: 1439, damage: 69 + boost, attack_type: AttackType::Slashing, initiative: 3, immune_to: vec![AttackType::Cold, AttackType::Fire], weak_to: vec![AttackType::Slashing]},
        Group{id: 1, units: 1721, hit_points: 2792, damage: 13 + boost, attack_type: AttackType::Cold, initiative: 16, immune_to: vec![], weak_to: vec![AttackType::Radiation, AttackType::Fire]},
    ]
}

fn get_infection_army() -> Vec<Group> {
    vec![
        Group{id: 1, units: 1721, hit_points: 29925, damage: 34, attack_type: AttackType::Radiation, initiative: 5, immune_to: vec![AttackType::Slashing], weak_to: vec![AttackType::Cold, AttackType::Radiation]},
        Group{id: 1, units: 6351, hit_points: 21460, damage: 6, attack_type: AttackType::Slashing, initiative: 15, immune_to: vec![], weak_to: vec![AttackType::Cold]},
        Group{id: 1, units: 958, hit_points: 48155, damage: 93, attack_type: AttackType::Radiation, initiative: 7, immune_to: vec![], weak_to: vec![AttackType::Bludgeoning]},
        Group{id: 1, units: 288, hit_points: 41029, damage: 279, attack_type: AttackType::Cold, initiative: 20, immune_to: vec![AttackType::Bludgeoning], weak_to: vec![AttackType::Radiation]},
        Group{id: 1, units: 3310, hit_points: 38913, damage: 21, attack_type: AttackType::Radiation, initiative: 19, immune_to: vec![], weak_to: vec![]},
        Group{id: 1, units: 3886, hit_points: 16567, damage: 7, attack_type: AttackType::Cold, initiative: 9, immune_to: vec![AttackType::Bludgeoning, AttackType::Cold], weak_to: vec![]},
        Group{id: 1, units: 39, hit_points: 7078, damage: 300, attack_type: AttackType::Bludgeoning, initiative: 14, immune_to: vec![], weak_to: vec![]},
        Group{id: 1, units: 241, hit_points: 40635, damage: 304, attack_type: AttackType::Fire, initiative: 6, immune_to: vec![], weak_to: vec![AttackType::Cold]},
        Group{id: 1, units: 7990, hit_points: 7747, damage: 1, attack_type: AttackType::Radiation, initiative: 10, immune_to: vec![AttackType::Fire], weak_to: vec![]},
        Group{id: 1, units: 80, hit_points: 30196, damage: 702, attack_type: AttackType::Bludgeoning, initiative: 17, immune_to: vec![], weak_to: vec![AttackType::Fire]},
    ]
}

fn calculate_damage(attacker: &Group, defender: &Group) -> i32 {
    if defender.immune_to.contains(&attacker.attack_type) {
        return 0;
    }

    let damage = attacker.units * attacker.damage;
    if defender.weak_to.contains(&attacker.attack_type) {
        return damage * 2;
    }

    return damage;
}

fn target_selection(attackers: &Vec<Group>, defenders: &Vec<Group>) -> HashMap<usize, usize> {

    let mut attacked_groups = HashSet::new();
    let mut pairing = HashMap::new();
    for attacker_idx in 0..attackers.len() {
        // group eliminated
        if attackers[attacker_idx].units <= 0 {
            continue;
        }

        let mut max_damage = i32::min_value();
        let mut enemy_idx = usize::max_value();

        for defender_idx in 0..defenders.len() {
            // group eliminated
            if defenders[defender_idx].units <= 0 {
                continue;
            }

            if !attacked_groups.contains(&defender_idx) {
                let damage = calculate_damage(&attackers[attacker_idx], &defenders[defender_idx]);
                if max_damage == i32::min_value() ||
                    damage > max_damage ||
                    (damage == damage && defenders[defender_idx].effective_power() > defenders[enemy_idx].effective_power()) ||
                    (damage == damage && defenders[defender_idx].effective_power() == defenders[enemy_idx].effective_power() && defenders[defender_idx].initiative > defenders[enemy_idx].initiative) {
                    max_damage = damage;
                    enemy_idx = defender_idx;
                }
            }
        }

        if max_damage > 0 {
            pairing.insert(attacker_idx, enemy_idx);
            attacked_groups.insert(enemy_idx);
        }
    }

    return pairing;
}

fn attack(attacker: &Group, defender: &mut Group) {
    let damage = calculate_damage(attacker, defender);
    if damage > 0 {
        let mut  units_lost = damage / defender.hit_points;
        if units_lost > defender.units {
            units_lost = defender.units;
        }
        defender.units -= units_lost;

        // println!("unit: {} attacked unit: {} dealing {} damage. Defender lost {} units.", attacker.id, defender.id, damage, units_lost);
    }
}

fn has_active_units(army: &Vec<Group>) -> bool {
    return army.iter().any(|g| g.units > 0);
}

fn num_active_units(army: &Vec<Group>) -> i32 {
    return army.iter().map(|g| g.units).sum();
}

fn max_initiative(army: &Vec<Group>) -> u32 {
    return army.iter().map(|g| g.initiative).max().unwrap();
}

fn fight(boost: i32) -> (i32, i32) {
    let mut immune_system_army = get_immune_system_army(boost);
    let mut infection_army = get_infection_army();

    let max_initiative = std::cmp::max(max_initiative(&immune_system_army), max_initiative(&infection_army));

    while has_active_units(&immune_system_army) && has_active_units(&infection_army) {
        immune_system_army.sort_by(|g1, g2| g1.cmp(&g2).reverse());
        infection_army.sort_by(|g1, g2| g1.cmp(&g2).reverse());

        let immune_system_targets = target_selection(&mut immune_system_army, &infection_army);
        let infection_targets = target_selection(&mut infection_army, &immune_system_army);

        for initiative in (1..=max_initiative).rev() {
            for k in immune_system_targets.keys() {
                let i = *k;
                if immune_system_army[i].initiative == initiative && immune_system_army[i].units > 0 {
                    attack(&immune_system_army[i], &mut infection_army[immune_system_targets[&i]]);
                }
            }

            for k in infection_targets.keys() {
                let i = *k;
                if infection_army[i].initiative == initiative && infection_army[i].units > 0 {
                    attack(&infection_army[i], &mut immune_system_army[infection_targets[&i] as usize]);
                }
            }
        }
    }

    return (num_active_units(&immune_system_army), num_active_units(&infection_army));
}

fn problem_1() {
    let (immune_system_active_units, infection_army_active_units) = fight(0);
    println!("{}", immune_system_active_units + infection_army_active_units);
}

fn problem_2() {
    // 38 - manual binary search (note that no one wins for 37 and 36)
    let (immune_system_active_units, infection_army_active_units) = fight(38);
    println!("immune system army active units: {}, infection army active units: {}", immune_system_active_units, infection_army_active_units);
}

fn main() {
    problem_1();
    problem_2();
}
