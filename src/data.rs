pub mod models;

pub fn load_meta(game: String) -> Result<models::Meta, Box<dyn std::error::Error>> {
    let path = format!("./data/{}/meta.json", game);
    let data = std::fs::read_to_string(path)?;
    let meta: models::Meta = serde_json::from_str(&data)?;
    Ok(meta)
}

pub fn load_objectives(game: String) -> Result<models::Objectives, Box<dyn std::error::Error>> {
    let path = format!("./data/{}/objectives.json", game);
    let data = std::fs::read_to_string(path)?;
    let objectives: models::Objectives = serde_json::from_str(&data)?;
    Ok(objectives)
}

pub fn load_routes(game: String) -> Result<models::Routes, Box<dyn std::error::Error>> {
    let path = format!("./data/{}/routes.json", game);
    let data = std::fs::read_to_string(path)?;
    let routes: models::Routes = serde_json::from_str(&data)?;
    Ok(routes)
}
