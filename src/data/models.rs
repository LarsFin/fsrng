use std::collections::HashMap;

use serde::{ Deserialize, Serialize };

/**
 * TODO
 * Hopefully last change...
 * A Route is the finished generation, not a part of the schema.
 * Instead, there are flags which are used to filter objectives and their conditions
 */

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
pub struct Schema {
    pub file_version: String,
    pub filters: Vec<Filter>,
    pub preferences: Vec<Preference>,
    pub objectives: Vec<Objective>,
}

#[derive(Deserialize)]
pub struct Objective {
    pub id: String,
    pub info: ObjectiveInfo,
    pub labels: Vec<String>,
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
    pub labels: Option<Vec<String>>,
    pub conditions: Vec<Condition>,
}

#[derive(Deserialize)]
pub struct ConditionNode {
    pub objective_id: String,
    pub labels: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct ConditionEnd;

#[derive(Serialize)]
pub struct Route {
    pub app_version: String,
    pub game_name: String,
    pub seed: u64,
    pub filters: Vec<BasicInfo>,
    pub preferences: Vec<BasicInfo>,
    pub ordered_objectives: Vec<ObjectiveInfo>,
}

#[derive(Deserialize, Clone)]
pub struct Filter {
    pub id: String,
    pub info: BasicInfo,
    pub clause: String,
    pub labels: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct Preference {
    pub id: String,
    pub info: BasicInfo,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BasicInfo {
    pub name: String,
    pub description: String,
}
