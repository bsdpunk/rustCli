use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Program Name {}", &args[0]);
    println!("Arguement 1 {}", query);
    println!("Arguement 2 {}", filename);
}
