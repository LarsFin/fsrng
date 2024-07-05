use data::models::*;
use fsrng::*;

fn main() {
    let game = String::from("demons_souls");
    let routes = data::load_routes(game).unwrap();
    let completed: &Vec<Objective> = &Vec::new();
    let possible = possible_objectives(&routes.items[0].objectives, &completed);

    let mut rng = gen_rng(10);

    for _ in 0..5 {
        println!("{}", random_objective_id(&possible, &mut rng));
    }
}
