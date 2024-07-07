use data::models::*;
use rand::Rng;
use rand_chacha::{ rand_core::SeedableRng, ChaCha8Rng };
use std::time::SystemTime;

pub mod data;

fn possible_objectives<'a>(
    remaining: &'a [RouteObjective],
    completed: &[String]
) -> Vec<&'a RouteObjective> {
    let mut possible: Vec<&RouteObjective> = Vec::new();

    for route_objective in remaining {
        if let Some(condition) = &route_objective.condition {
            if condition_met(condition, completed) {
                possible.push(route_objective);
            }
        } else {
            possible.push(route_objective);
        }
    }

    possible
}

fn condition_met(condition: &Condition, completed: &[String]) -> bool {
    match condition {
        Condition::Branch(branch) => condition_branch_met(branch, completed),
        Condition::Node(node) => condition_node_met(node, completed),
    }
}

fn condition_node_met(condition: &ConditionNode, completed: &[String]) -> bool {
    if condition.status == "completed" {
        return completed.iter().any(|objective_id| objective_id == &condition.objective_id);
    }

    false
}

fn condition_branch_met(condition: &ConditionBranch, completed: &[String]) -> bool {
    if condition.clause == "all" {
        return condition.conditions.iter().all(|condition| condition_met(condition, completed));
    }

    if condition.clause == "any" {
        return condition.conditions.iter().any(|condition| condition_met(condition, completed));
    }

    false
}

pub fn gen_rng(seed: u64) -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(seed)
}

fn random_objective_id<'a>(objectives: &'a [&RouteObjective], rng: &mut ChaCha8Rng) -> &'a String {
    let total = objectives
        .iter()
        .map(|objective| objective.weight)
        .sum::<u32>();

    let mut roll = rng.gen_range(0..total);
    let mut i = 0;

    loop {
        let objective = &objectives[i];
        if roll < objective.weight {
            return &objective.id;
        }

        roll -= objective.weight;
        i += 1;
    }
}

pub fn determine_objective_order(
    mut objectives: Vec<RouteObjective>,
    rng: &mut ChaCha8Rng
) -> Vec<String> {
    let mut order: Vec<String> = Vec::new();

    while !objectives.is_empty() {
        // Determine the next objective in the order
        let possible = possible_objectives(&objectives, &order);
        let next_objective_id = random_objective_id(&possible, rng);
        order.push(next_objective_id.clone());

        // Remove the objective from the remaining list
        let index = objectives
            .iter()
            .position(|objective| objective.id == *next_objective_id)
            .unwrap();
        objectives.remove(index);
    }

    order
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

pub fn get_route_selection(mut routes: Routes) -> Route {
    let choices: Vec<String> = routes.items
        .iter()
        .map(|route| route.info.name.clone())
        .collect();
    let choice = get_selection_index(String::from("Select a route"), &choices);
    routes.items.remove(choice - 1)
}

pub fn get_game_selection(games: &[String]) -> String {
    let choice = get_selection_index(String::from("Select a game"), games);
    games[choice - 1].clone()
}

pub fn build_serialisable_route(
    info: RouteInfo,
    mut objectives: Vec<Objective>,
    objective_ids: &[String]
) -> GeneratedRoute {
    let mut ordered_objectives: Vec<Objective> = Vec::new();

    for objective_id in objective_ids {
        for (i, objective) in objectives.iter().enumerate() {
            if objective.id == *objective_id {
                ordered_objectives.push(objectives.remove(i));
                break;
            }
        }
    }

    GeneratedRoute {
        info,
        objectives: ordered_objectives,
    }
}

pub fn route_name(game: &String) -> String {
    // TODO: handle unwrap case here, though this would be very rare
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    format!("{}_{}", game, timestamp)
}
