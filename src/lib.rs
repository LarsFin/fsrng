use std::collections::HashMap;

use data::models::*;
use interface::*;

use rand::Rng;
use rand_chacha::{ rand_core::SeedableRng, ChaCha8Rng };

pub mod data;
pub mod interface;

pub fn ask_game_selection(games: &[Game]) -> &Game {
    let choices: Vec<String> = games
        .iter()
        .map(|game| game.name.clone())
        .collect();
    let choice = ask_option_selection(String::from("Select game"), &choices);
    &games[choice]
}

pub trait Informable: Clone {
    fn info(self) -> BasicInfo;
}

impl Informable for ConfigOption {
    fn info(self) -> BasicInfo {
        self.info
    }
}

pub fn ask_selection<T: Informable>(question: String, selections: &[T]) -> Option<T> {
    if selections.is_empty() {
        return None;
    }

    let choices: Vec<String> = selections
        .iter()
        .map(|selection| selection.clone().info().name)
        .collect();

    let selection = ask_option_selection(question, &choices);

    Some(selections[selection].clone())
}

pub fn ask_selections<T: Informable>(question: String, selections: &[T]) -> Vec<T> {
    if selections.is_empty() {
        return Vec::new();
    }

    let choices: Vec<String> = selections
        .iter()
        .map(|selection| selection.clone().info().name)
        .collect();

    let decisions = ask_multiple_selection(question, &choices);

    decisions
        .iter()
        .map(|decision| selections[*decision].clone())
        .collect()
}

pub fn ask_seed() -> u64 {
    match
        ask_optional_positive_number(
            &String::from("Enter a seed for the RNG (leave blank for random)")
        )
    {
        Some(seed) => seed,
        None => rand::thread_rng().gen(),
    }
}

pub fn load_schema(file_name: &String) -> Result<Schema, Box<dyn std::error::Error>> {
    data::load_schema(file_name)
}

fn possible_objectives_ids(
    flags: &[ConfigOption],
    objectives: &[Objective],
    completed: &[String]
) -> Vec<String> {
    let mut possible_objective_ids: Vec<String> = Vec::new();

    for objective in objectives {
        if completed.contains(&objective.id) {
            continue;
        }

        if let Some(condition) = &objective.condition {
            if !resolve_condition(flags, condition, completed, objectives.len()) {
                continue;
            }
        }

        possible_objective_ids.push(objective.id.clone());
    }

    possible_objective_ids
}

fn resolve_condition(
    flags: &[ConfigOption],
    condition: &Condition,
    completed: &[String],
    total_objective_count: usize
) -> bool {
    match condition {
        Condition::Branch(branch) =>
            resolve_branch(flags, branch, completed, total_objective_count),
        Condition::Node(node) => resolve_node(flags, node, completed),
        Condition::End(_) => completed.len() == total_objective_count - 1,
    }
}

fn resolve_branch(
    flags: &[ConfigOption],
    branch: &ConditionBranch,
    completed: &[String],
    total_objective_count: usize
) -> bool {
    if let Some(flag_checks) = &branch.flag_checks {
        if !check_flags(flag_checks, flags) {
            return false;
        }
    }

    if branch.clause == "all" {
        return branch.conditions
            .iter()
            .all(|condition| resolve_condition(flags, condition, completed, total_objective_count));
    }

    if branch.clause == "any" {
        return branch.conditions
            .iter()
            .any(|condition| resolve_condition(flags, condition, completed, total_objective_count));
    }

    // TODO: probably a nicer way to do this
    if branch.clause == "any_two" {
        let mut count = 0;
        branch.conditions.iter().for_each(|condition| {
            if resolve_condition(flags, condition, completed, total_objective_count) {
                count += 1;
            }
        });
        return count >= 2;
    }

    false
}

fn resolve_node(flags: &[ConfigOption], node: &ConditionNode, completed: &[String]) -> bool {
    if let Some(flag_checks) = &node.flag_checks {
        if !check_flags(flag_checks, flags) {
            return false;
        }
    }

    if let Some(objective_id) = &node.objective_id {
        completed.contains(objective_id)
    } else {
        // no objective_id indicates no objective to check, so return true
        true
    }
}

pub fn gen_rng(seed: u64) -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(seed)
}

pub fn filter_objectives(flags: &[ConfigOption], objectives: Vec<Objective>) -> Vec<Objective> {
    let mut filtered_objectives: Vec<Objective> = Vec::new();

    for objective in objectives {
        // apparently let is unstable when followed by && that relies on optional value, an issue on GH exists
        if let Some(flag_checks) = &objective.flag_checks {
            if !check_flags(flag_checks, flags) {
                continue;
            }
        }

        filtered_objectives.push(objective);
    }

    filtered_objectives
}

pub fn check_flags(flag_checks: &[FlagCheck], flags: &[ConfigOption]) -> bool {
    let flag_ids: Vec<String> = flags
        .iter()
        .map(|filter| filter.id.clone())
        .collect();

    for flag_check in flag_checks {
        match flag_check.clause.as_str() {
            "any" => {
                if !flag_check.flag_ids.iter().any(|flag_id| flag_ids.contains(flag_id)) {
                    return false;
                }
            }
            "all" => {
                if !flag_check.flag_ids.iter().all(|flag_id| flag_ids.contains(flag_id)) {
                    return false;
                }
            }
            "none" => {
                if flag_check.flag_ids.iter().any(|flag_id| flag_ids.contains(flag_id)) {
                    return false;
                }
            }
            _ => {
                println!("Unknown flag check clause '{}'", flag_check.clause);
            }
        }
    }

    return true;
}

pub fn generate_ordered_objectives(
    flags: &[ConfigOption],
    preferences: &[ConfigOption],
    objectives: &[Objective],
    rng: &mut ChaCha8Rng
) -> Vec<ObjectiveInfo> {
    let mut completed: Vec<String> = Vec::new();
    let mut ordered_objectives: Vec<ObjectiveInfo> = Vec::new();

    while completed.len() < objectives.len() {
        let possible_objective_ids = possible_objectives_ids(flags, objectives, &completed);

        if possible_objective_ids.is_empty() {
            panic!(
                "Could not resolve remaining objectives: {:?}",
                remaining_objective_ids(&completed, objectives)
            );
        }

        let weighted_objectives = build_weighted_objectives(
            preferences,
            &possible_objective_ids,
            objectives
        );
        let next_objective_id = random_weighted_objective(&weighted_objectives, rng);

        completed.push(next_objective_id.clone());

        let next_objective = objectives
            .iter()
            .find(|objective| objective.id == next_objective_id)
            .unwrap();

        ordered_objectives.push(next_objective.info.clone());
    }

    ordered_objectives
}

fn remaining_objective_ids(
    completed_objective_ids: &[String],
    all_objectives: &[Objective]
) -> Vec<String> {
    all_objectives
        .iter()
        .filter(|objective| !completed_objective_ids.contains(&objective.id))
        .map(|objective| objective.id.clone())
        .collect()
}

fn build_weighted_objectives(
    preferences: &[ConfigOption],
    objective_ids: &[String],
    objectives: &[Objective]
) -> Vec<WeightedObjective> {
    let mut weighted_objectives: Vec<WeightedObjective> = Vec::new();

    for objective_id in objective_ids {
        let objective = objectives
            .iter()
            .find(|objective| objective.id == *objective_id)
            .unwrap();

        weighted_objectives.push(WeightedObjective {
            id: objective.id.clone(),
            weight: get_weight(preferences, &objective.weighting),
        });
    }

    weighted_objectives
}

fn get_weight(
    preferences: &[ConfigOption],
    optional_weighting: &Option<HashMap<String, u64>>
) -> u64 {
    if let Some(weighting) = optional_weighting {
        for preference in preferences {
            if let Some(weight) = weighting.get(&preference.id) {
                return *weight;
            }
        }
    }

    1
}

fn random_weighted_objective(
    weighted_objectives: &[WeightedObjective],
    rng: &mut ChaCha8Rng
) -> String {
    let total = weighted_objectives
        .iter()
        .map(|weighted_objective| weighted_objective.weight)
        .sum::<u64>();

    let mut roll = rng.gen_range(0..total);
    let mut i = 0;

    loop {
        let weighted_objective = &weighted_objectives[i];
        if roll < weighted_objective.weight {
            return weighted_objective.id.clone();
        }

        roll -= weighted_objective.weight;
        i += 1;
    }
}

pub fn build_generated_route(
    app_version: String,
    game_name: String,
    seed: u64,
    flags: Vec<ConfigOption>,
    preferences: Vec<ConfigOption>,
    ordered_objectives: Vec<ObjectiveInfo>
) -> Route {
    Route {
        app_version,
        game_name,
        seed,
        flags: flags
            .iter()
            .map(|flag| flag.info.clone())
            .collect(),
        preferences: preferences
            .iter()
            .map(|preference| preference.info.clone())
            .collect(),
        ordered_objectives,
    }
}

pub fn write_route(game_id: &String, generated_route: Route) -> Result<(), std::io::Error> {
    let time_sig = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let file_name = format!("{}-{}", time_sig, game_id);
    data::write_output(file_name, generated_route)
}

fn get_selection_index(question: String, choices: &[String]) -> usize {
    println!("{}", question);

    let mut input = String::new();

    for (i, choice) in choices.iter().enumerate() {
        println!("{}. {}", i + 1, choice);
    }

    // TODO: handle panics here, reask selection. Fails if input is not a number
    // or if the number is out of bounds
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

pub fn get_seed() -> u64 {
    println!("Enter a seed for the RNG (leave blank for random):");

    let mut input = String::new();

    // TODO: handle panic here
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim().is_empty() {
        println!("Using random seed");
        return rand::thread_rng().gen();
    }

    // TODO: handle panic here
    input.trim().parse().unwrap()
}

pub fn get_game_selection(games: &[String]) -> String {
    let choice = get_selection_index(String::from("Select a game"), games);
    games[choice - 1].clone()
}
