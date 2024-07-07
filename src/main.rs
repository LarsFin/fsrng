use fsrng::*;

fn main() {
    let games = data::load_game_options().unwrap();
    let game = get_game_selection(&games);

    let routes = data::load_routes(game).unwrap();
    let mut rng = gen_rng(11);

    let selected_route = get_route_selection(routes);
    let generated_route = determine_objective_order(selected_route, &mut rng);

    for objective_id in generated_route {
        println!("{}", objective_id);
    }
}
