use std::{ fs, io };
use std::path::Path;

pub mod models;

pub fn load_meta() -> Result<models::Meta, Box<dyn std::error::Error>> {
    load_data(String::from("./data/meta.json"))
}

pub fn load_schema(file_name: &String) -> Result<models::Schema, Box<dyn std::error::Error>> {
    let path = format!("./data/schemas/{}.json", file_name);
    load_data(path)
}

pub fn load_game_options() -> Result<Vec<String>, io::Error> {
    let read_dir = fs::read_dir("./data")?;
    let mut games: Vec<String> = Vec::new();

    for entry in read_dir {
        let dir = entry?;

        if dir.file_type()?.is_dir() {
            let file_name = dir.file_name();
            match file_name.to_str() {
                Some(name) => games.push(String::from(name)),
                None => {
                    continue;
                }
            }
        }
    }

    Ok(games)
}

fn load_data<T>(path: String) -> Result<T, Box<dyn std::error::Error>>
    where T: serde::de::DeserializeOwned
{
    let data = std::fs::read_to_string(path)?;
    let deserialized: T = serde_json::from_str(&data)?;
    Ok(deserialized)
}

pub fn write_output<T>(name: String, serialisable_data: T) -> Result<(), std::io::Error>
    where T: serde::ser::Serialize
{
    if !Path::new("./output").exists() {
        fs::create_dir("./output")?;
    }

    let path = format!("./output/{}.json", name);
    let serialised_data = serde_json::to_string_pretty(&serialisable_data)?;
    std::fs::write(path, serialised_data)
}
