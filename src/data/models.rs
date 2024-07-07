use serde::{ Deserialize, Serialize };

#[derive(Deserialize)]
pub struct Meta {
    pub version: String,
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

#[derive(Deserialize, Serialize)]
pub struct RouteInfo {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Condition {
    Branch(ConditionBranch),
    Node(ConditionNode),
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

#[derive(Serialize)]
pub struct GeneratedRoute {
    pub info: RouteInfo,
    pub seed: u64,
    pub objectives: Vec<Objective>,
}
