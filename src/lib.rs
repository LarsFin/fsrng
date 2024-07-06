use data::models::*;
use rand::Rng;
use rand_chacha::{ rand_core::SeedableRng, ChaCha8Rng };

pub mod data;

pub fn possible_objectives<'a>(
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

pub fn random_objective_id<'a>(
    objectives: &'a [&RouteObjective],
    rng: &mut ChaCha8Rng
) -> &'a String {
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

pub fn determine_objective_order(route: Route, rng: &mut ChaCha8Rng) -> Vec<String> {
    let mut remaining: Vec<RouteObjective> = route.objectives;
    let mut order: Vec<String> = Vec::new();

    while !remaining.is_empty() {
        // Determine the next objective in the order
        let possible = possible_objectives(&remaining, &order);
        let next_objective_id = random_objective_id(&possible, rng);
        order.push(next_objective_id.clone());

        // Remove the objective from the remaining list
        let index = remaining
            .iter()
            .position(|objective| objective.id == *next_objective_id)
            .unwrap();
        remaining.remove(index);
    }

    order
}
