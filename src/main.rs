use fsrng::*;

fn main() {
    let meta = data::load_meta().unwrap();

    println!("Route Randomiser v{}", meta.app_version);

    let game = ask_game_selection(&meta.games);

    println!("Selected {}", game.name);

    let route_schema = load_route_schema(&game.file_name).unwrap();

    let route = ask_route_selection(&route_schema.routes);

    println!("Selected {}", route.info.name);

    let filtered_objectives = filter_objectives(&route.id, route_schema.objectives);

    let seed = ask_seed();

    println!("Using seed: {}", seed);

    let mut rng = gen_rng(seed);

    println!("Generating route...");

    let ordered_objectives = generate_ordered_objectives(&route.id, &filtered_objectives, &mut rng);

    let generated_route = build_generated_route(
        meta.app_version.clone(),
        game.name.clone(),
        route.info.clone(),
        seed,
        ordered_objectives
    );

    write_generated_route(&game.file_name, &route.id, generated_route).unwrap();

    println!("Route generated successfully!");
}
