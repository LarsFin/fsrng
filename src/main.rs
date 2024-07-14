use fsrng::*;

fn main() {
    let meta = data::load_meta().unwrap();

    println!("Route Randomiser v{}", meta.app_version);

    let game = ask_game_selection(&meta.games);

    println!("Selected {}", game.name);

    let schema = load_schema(&game.file_name).unwrap();

    let filters = ask_selections(String::from("Select filters."), &schema.filters);
    let flags = ask_selections(String::from("Select flags."), &schema.flags);
    let preferences = ask_selections(String::from("Select preferences."), &schema.preferences);

    let filtered_objectives = filter_objectives(&filters, schema.objectives);

    let seed = ask_seed();

    println!("Using seed: {}", seed);

    let mut rng = gen_rng(seed);

    println!("Generating route...");

    let ordered_objectives = generate_ordered_objectives(
        &filters,
        &flags,
        &preferences,
        &filtered_objectives,
        &mut rng
    );

    let generated_route = build_generated_route(
        meta.app_version.clone(),
        game.name.clone(),
        seed,
        filters,
        flags,
        preferences,
        ordered_objectives
    );

    write_route(&game.file_name, generated_route).unwrap();

    println!("Route generated successfully!");
}
