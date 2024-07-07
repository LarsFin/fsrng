use data::write_output;
use fsrng::*;

fn main() {
    let games = data::load_game_options().unwrap();
    let game = get_game_selection(&games);

    let routes = data::load_routes(&game).unwrap();
    let loaded_objectives = data::load_objectives(&game).unwrap();
    let selected_route = get_route_selection(routes);

    let seed = get_seed();
    let mut rng = gen_rng(seed);

    let objective_ids = determine_objective_order(selected_route.objectives, &mut rng);

    let serialisable_route = build_serialisable_route(
        selected_route.info,
        seed,
        loaded_objectives.items,
        &objective_ids
    );

    let output_name = route_name(&game);
    write_output(output_name, serialisable_route).unwrap();
}
