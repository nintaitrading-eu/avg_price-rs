extern crate docopt;

use std::io;
use std::io::Write;
use docopt::Docopt;

const USAGE: &'static str = "
avg_price

Usage:
  avg_price --test=<astr>
  avg_price (-h | --help)
  avg_price --version

Options:
  -h --help  Show usage info.
  --version  Show version.
  --test=<astr>  Print the given test-string.
";

fn main()
{
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    println!("{:}", args.get_str("--test"));

    /*print!("S1: ");
    io::stdout().flush();
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1)
        .expect("Failed to read s1.");
    let s1: u32 = s1.trim().parse()
        .expect("Error: NaN.");
    println!("S1 = {}", s1);*/
}
