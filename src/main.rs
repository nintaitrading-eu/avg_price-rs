extern crate docopt;

use std::io;
use std::io::Write;
use docopt::Docopt;

const USAGE: &'static str = "
avg_price

Usage:
  avg_price --list=<astr>
  avg_price (-h | --help)
  avg_price --version

Options:
  -h --help  Show usage info.
  --version  Show version.
  --list=<astr>  List of shares with prices in the form \"s1,p1;s2,p2\"
";

fn main()
{
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args); // TODO: Remove after DEBUG
    println!("Given list: {:}", args.get_str("--list")); // TODO: Remove after DEBUG

    io::stdout().flush();
    let mut list = args.get_str("--list");
    // TODO: Parse string: s1,p1
    // TODO: Parse element s1 p1 en multiply
    // TODO: calculate average
    //print!("Result = {}", );
    //let s1: u32 = s1.trim().parse()
    //    .expect("Error: NaN.");
}
