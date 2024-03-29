use std::io;

fn main() {
    let mut done = false; 
    while !done {

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("Couldn't read line");    
        println!("{}", input);

        if input.trim() == "quit" {
            done = true;
        }
    }
}
