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

pub fn ask_route_selection(routes: &[Route]) -> &Route {
    let choices: Vec<String> = routes
        .iter()
        .map(|route| route.info.name.clone())
        .collect();
    let choice = ask_option_selection(String::from("Select route"), &choices);
    &routes[choice]
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

pub fn load_route_schema(file_name: &String) -> Result<RouteSchema, Box<dyn std::error::Error>> {
    data::load_route_schema(file_name)
}

fn possible_objectives_ids(
    route_id: &String,
    objectives: &[Objective],
    completed: &[String]
) -> Vec<String> {
    let mut possible_objective_ids: Vec<String> = Vec::new();

    for objective in objectives {
        if completed.contains(&objective.id) {
            continue;
        }

        if let Some(condition) = &objective.condition {
            if !resolve_condition(route_id, condition, completed, objectives.len()) {
                continue;
            }
        }

        possible_objective_ids.push(objective.id.clone());
    }

    possible_objective_ids
}

fn resolve_condition(
    route_id: &String,
    condition: &Condition,
    completed: &[String],
    total_objective_count: usize
) -> bool {
    match condition {
        Condition::Branch(branch) =>
            resolve_branch(route_id, branch, completed, total_objective_count),
        Condition::Node(node) => resolve_node(route_id, node, completed),
        Condition::End(_) => completed.len() == total_objective_count - 1,
    }
}

fn resolve_branch(
    route_id: &String,
    branch: &ConditionBranch,
    completed: &[String],
    total_objective_count: usize
) -> bool {
    // TODO: figure out refactor here as all condition types have excluded routes
    if branch.excluded_routes.contains(route_id) {
        return false;
    }

    if branch.clause == "all" {
        return branch.conditions
            .iter()
            .all(|condition|
                resolve_condition(route_id, condition, completed, total_objective_count)
            );
    }

    if branch.clause == "any" {
        return branch.conditions
            .iter()
            .any(|condition|
                resolve_condition(route_id, condition, completed, total_objective_count)
            );
    }

    false
}

fn resolve_node(route_id: &String, node: &ConditionNode, completed: &[String]) -> bool {
    // TODO: figure out refactor here as all condition types have excluded routes
    if node.excluded_routes.contains(route_id) {
        return false;
    }

    completed.contains(&node.objective_id)
}

pub fn gen_rng(seed: u64) -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(seed)
}

pub fn filter_objectives(route_id: &String, objectives: Vec<Objective>) -> Vec<Objective> {
    let mut filtered_objectives: Vec<Objective> = Vec::new();

    for objective in objectives {
        if objective.excluded_routes.contains(route_id) {
            continue;
        }

        filtered_objectives.push(objective);
    }

    filtered_objectives
}

pub fn generate_ordered_objectives(
    route_id: &String,
    objectives: &[Objective],
    rng: &mut ChaCha8Rng
) -> Vec<ObjectiveInfo> {
    let mut completed: Vec<String> = Vec::new();
    let mut ordered_objectives: Vec<ObjectiveInfo> = Vec::new();

    while completed.len() < objectives.len() {
        let possible_objective_ids = possible_objectives_ids(route_id, objectives, &completed);
        let weighted_objectives = build_weighted_objectives(
            route_id,
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

fn build_weighted_objectives(
    route_id: &String,
    objective_ids: &[String],
    objectives: &[Objective]
) -> Vec<WeightedObjective> {
    let default_weight: u64 = 1;
    let mut weighted_objectives: Vec<WeightedObjective> = Vec::new();

    for objective_id in objective_ids {
        let objective = objectives
            .iter()
            .find(|objective| objective.id == *objective_id)
            .unwrap();

        weighted_objectives.push(WeightedObjective {
            id: objective.id.clone(),
            weight: objective.weighting.get(route_id).unwrap_or(&default_weight).clone(),
        });
    }

    weighted_objectives
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
    info: RouteInfo,
    seed: u64,
    ordered_objectives: Vec<ObjectiveInfo>
) -> GeneratedRoute {
    GeneratedRoute {
        app_version,
        game_name,
        info,
        seed,
        ordered_objectives,
    }
}

pub fn write_generated_route(
    game_id: &String,
    route_id: &String,
    generated_route: GeneratedRoute
) -> Result<(), std::io::Error> {
    let time_sig = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let file_name = format!("{}-{}-{}", game_id, route_id, time_sig);
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
