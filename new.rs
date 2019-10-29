use ::std::*;

/// # Performance
/// Allocates a new `String` every time it is called.
///
/// # Panic
/// Panics if it fails to read from `Stdin`
fn input (message: &'_ impl fmt::Display) -> String
{
    print!("{}", message);
    let mut ret = String::new();
    io::stdin::().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main () {
println!("{}", input());
}
