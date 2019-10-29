use std::io;

fn main() {
    let mut input = String::new();
    let mut done = false; 
    while !done {
        io::stdin().read_line(&mut input)
            .ok()
            .expect("Couldn't read line");    
        println!("{}", input);
        
        if input.trim() == "quit" {
            done = true;
        }
    }
}
