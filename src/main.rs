mod utils;

fn main() {
    println!("Hello, world!");
    let dataset = utils::get_dataset("data/heart.csv");
    println!("{:?}", dataset);
}
