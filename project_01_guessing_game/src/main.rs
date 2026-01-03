use std::io;
use rand::Rng;
fn main() {
    let mut fruit: String = String::new();
    let guess_list = ["banana","apple","mango","orange","grapes"];
    let mut rng = rand::rng();
    let random_index = rng.random_range(0..guess_list.len());
    let random_fruit = guess_list[random_index];
    println!("Random fruit: {}", random_fruit);
    loop {
        println!("Guess the fruit:");
        io::stdin()
            .read_line(&mut fruit)
            .expect("Failed to read line");
        let fruit = fruit.trim();
        if fruit == random_fruit {
            println!("You win!");
            break;
        }
        println!("Try again!")
    }
}
