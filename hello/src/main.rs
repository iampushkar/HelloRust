use hello::greet;
use rand::Rng;

fn main() {
    greet();
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1..=100);
    println!("Random Num : {}", num);
}
