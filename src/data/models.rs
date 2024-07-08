use std::collections::HashMap;

use serde::{ Deserialize, Serialize };

#[derive(Deserialize)]
pub struct Meta {
    pub app_version: String,
    pub games: Vec<Game>,
}

#[derive(Deserialize)]
pub struct Game {
    pub name: String,
    pub file_name: String,
}

#[derive(Deserialize)]
pub struct RouteSchema {
    pub file_version: String,
    pub routes: Vec<Route>,
    pub objectives: Vec<Objective>,
}

#[derive(Deserialize)]
pub struct Route {
    pub id: String,
    pub info: RouteInfo,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RouteInfo {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct Objective {
    pub id: String,
    pub info: ObjectiveInfo,
    pub excluded_routes: Vec<String>,
    pub weighting: HashMap<String, u64>,
    pub condition: Option<Condition>,
}

pub struct WeightedObjective {
    pub id: String,
    pub weight: u64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ObjectiveInfo {
    pub name: String,
    pub location: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Condition {
    Branch(ConditionBranch),
    Node(ConditionNode),
    End(ConditionEnd),
}

#[derive(Deserialize)]
pub struct ConditionBranch {
    pub clause: String,
    pub excluded_routes: Vec<String>,
    pub conditions: Vec<Condition>,
}

#[derive(Deserialize)]
pub struct ConditionNode {
    pub objective_id: String,
    pub excluded_routes: Vec<String>,
}

#[derive(Serialize)]
pub struct RouteInstance {
    pub info: RouteInfo,
    pub ordered_objectives: Vec<ObjectiveInfo>,
}

#[derive(Deserialize)]
pub struct ConditionEnd;

#[derive(Serialize)]
pub struct GeneratedRoute {
    pub app_version: String,
    pub game_name: String,
    pub info: RouteInfo,
    pub seed: u64,
    pub ordered_objectives: Vec<ObjectiveInfo>,
}
