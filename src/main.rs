use fsrng::*;

fn main() {
    let game = String::from("demons_souls");
    let routes = data::load_routes(game).unwrap();
    let mut rng = gen_rng(11);

    let selected_route = get_route_selection(routes);
    let generated_route = determine_objective_order(selected_route, &mut rng);

    for objective_id in generated_route {
        println!("{}", objective_id);
    }
}
