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
    pub routes: Vec<_Route>,
    pub objectives: Vec<_Objective>,
}

#[derive(Deserialize)]
pub struct _Route {
    pub id: String,
    pub info: _RouteInfo,
}

#[derive(Deserialize, Serialize)]
pub struct RouteInfo {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct _Objective {
    pub id: String,
    pub info: _ObjectiveInfo,
    pub excluded_routes: Vec<String>,
    pub weighting: HashMap<String, u64>,
    pub condition: Option<_Condition>,
}

pub struct _WeightedObjective {
    pub id: String,
    pub weight: u64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct _ObjectiveInfo {
    pub name: String,
    pub location: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum _Condition {
    Branch(_ConditionBranch),
    Node(_ConditionNode),
    End(_ConditionEnd),
}

#[derive(Deserialize)]
pub struct _ConditionBranch {
    pub clause: String,
    pub excluded_routes: Vec<String>,
    pub conditions: Vec<_Condition>,
}

#[derive(Deserialize)]
pub struct _ConditionNode {
    pub objective_id: String,
    pub excluded_routes: Vec<String>,
}

#[derive(Serialize)]
pub struct _RouteInstance {
    pub info: _RouteInfo,
    pub ordered_objectives: Vec<_ObjectiveInfo>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct _RouteInfo {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct _ConditionEnd;

#[derive(Serialize)]
pub struct _GeneratedRoute {
    pub app_version: String,
    pub game_name: String,
    pub info: _RouteInfo,
    pub seed: u64,
    pub ordered_objectives: Vec<_ObjectiveInfo>,
}

#[derive(Deserialize)]
pub struct Objectives {
    pub items: Vec<Objective>,
}

#[derive(Deserialize, Serialize)]
pub struct Objective {
    pub name: String,
    pub id: String,
    pub labels: Vec<String>,
    pub location: String,
}

#[derive(Deserialize)]
pub struct Routes {
    pub items: Vec<Route>,
}

#[derive(Deserialize)]
pub struct Route {
    pub id: String,
    pub info: RouteInfo,
    pub objectives: Vec<RouteObjective>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Condition {
    Branch(ConditionBranch),
    Node(ConditionNode),
    End(ConditionEnd),
}

#[derive(Deserialize)]
pub struct RouteObjective {
    pub id: String,
    pub weight: u32,
    pub labels: Vec<String>,
    pub condition: Option<Condition>,
}

#[derive(Deserialize)]
pub struct ConditionBranch {
    pub clause: String,
    pub conditions: Vec<Condition>,
}

#[derive(Deserialize)]
pub struct ConditionNode {
    pub status: String,
    pub objective_id: String,
}

#[derive(Deserialize)]
pub struct ConditionEnd;

#[derive(Serialize)]
pub struct GeneratedRoute {
    pub info: RouteInfo,
    pub seed: u64,
    pub objectives: Vec<Objective>,
}
