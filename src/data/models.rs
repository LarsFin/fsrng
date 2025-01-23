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
pub struct Schema {
    pub file_version: String,
    pub flags: Vec<ConfigOption>,
    pub preferences: Vec<ConfigOption>,
    pub objectives: Vec<Objective>,
    pub routes: Vec<ConfigOption>,
}

#[derive(Deserialize)]
pub struct Objective {
    pub id: String,
    pub info: ObjectiveInfo,
    pub flag_checks: Option<Vec<FlagCheck>>,
    pub weighting: Option<HashMap<String, u64>>,
    pub routes: Option<Vec<String>>,
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
    pub flag_checks: Option<Vec<FlagCheck>>,
    pub conditions: Vec<Condition>,
}

#[derive(Deserialize)]
pub struct ConditionNode {
    pub objective_id: Option<String>,
    pub labels: Option<Vec<String>>,
    pub flag_checks: Option<Vec<FlagCheck>>,
}

#[derive(Deserialize)]
pub struct ConditionEnd;

#[derive(Serialize)]
pub struct Route {
    pub app_version: String,
    pub game_name: String,
    pub seed: u64,
    pub flags: Vec<BasicInfo>,
    pub preferences: Vec<BasicInfo>,
    pub ordered_objectives: Vec<ObjectiveInfo>,
}

#[derive(Deserialize, Clone)]
pub struct ConfigOption {
    pub id: String,
    pub info: BasicInfo,
}

#[derive(Deserialize)]
pub struct FlagCheck {
    pub clause: String,
    pub flag_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BasicInfo {
    pub name: String,
    pub description: String,
}
