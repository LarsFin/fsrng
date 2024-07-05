use data::models::*;
use rand::Rng;
use rand_chacha::{ rand_core::SeedableRng, ChaCha8Rng };

pub mod data;

pub fn possible_objectives<'a>(
    remaining: &'a [RouteObjective],
    completed: &[Objective]
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

fn condition_met(condition: &Condition, completed: &[Objective]) -> bool {
    match condition {
        Condition::Branch(branch) => condition_branch_met(branch, completed),
        Condition::Node(node) => condition_node_met(node, completed),
    }
}

fn condition_node_met(condition: &ConditionNode, completed: &[Objective]) -> bool {
    if condition.status == "completed" {
        return condition.objectives
            .iter()
            .all(|objective_id| {
                completed.iter().any(|objective| objective.id == *objective_id)
            });
    }

    // TODO: handle "not_completed" status

    false
}

fn condition_branch_met(condition: &ConditionBranch, completed: &[Objective]) -> bool {
    if condition.clause == "and" {
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
