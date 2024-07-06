use fsrng::*;

fn main() {
    let game = String::from("demons_souls");
    let mut routes = data::load_routes(game).unwrap();
    let mut rng = gen_rng(10);

    let selected_route = routes.items.remove(0);
    let generated_route = determine_objective_order(selected_route, &mut rng);

    for objective_id in generated_route {
        println!("{}", objective_id);
    }
}
