pub mod models;

pub fn load_meta(game: String) -> Result<models::Meta, Box<dyn std::error::Error>> {
    let path = format!("./data/{}/meta.json", game);
    load_data(path)
}

pub fn load_objectives(game: String) -> Result<models::Objectives, Box<dyn std::error::Error>> {
    let path = format!("./data/{}/objectives.json", game);
    load_data(path)
}

pub fn load_routes(game: String) -> Result<models::Routes, Box<dyn std::error::Error>> {
    let path = format!("./data/{}/routes.json", game);
    load_data(path)
}

fn load_data<T>(path: String) -> Result<T, Box<dyn std::error::Error>>
    where T: serde::de::DeserializeOwned
{
    let data = std::fs::read_to_string(path)?;
    let deserialized: T = serde_json::from_str(&data)?;
    Ok(deserialized)
}
