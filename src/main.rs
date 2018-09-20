use std::io;
use std::io::Write;

fn main()
{
    print!("S1: ");
    io::stdout().flush();
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1)
        .expect("Failed to read s1.");
    println!("S1 = {}", s1);
}
