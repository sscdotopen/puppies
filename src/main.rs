extern crate puppies;

fn main() {

    let file: String = std::env::args().nth(1).unwrap().parse().unwrap();
    let num_users: usize = std::env::args().nth(2).unwrap().parse().unwrap();
    let num_items: usize = std::env::args().nth(3).unwrap().parse().unwrap();
    let pool_size: usize = std::env::args().nth(4).unwrap().parse().unwrap();

    puppies::incremental_indicators(file, num_users, num_items, pool_size);
}