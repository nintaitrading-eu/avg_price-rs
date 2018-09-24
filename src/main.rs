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
    
    struct SharesPrice
    {
        shares: i32,
        price: f64,
    }

    let list = args.get_str("--list");
    let split_list = list.split(";");
    let mut vec: Vec<SharesPrice> = vec![];
    for sp in split_list
    {
        let mut split_sp = sp.split(",");
        let vec_tmp: Vec<&str> = split_sp.collect::<Vec<&str>>();
        let s: i32 = vec_tmp[0].parse::<i32>().unwrap();
        let p: f64 = vec_tmp[1].parse::<f64>().unwrap();
         
        let mut struct_sp = SharesPrice { shares: s, price: p}; 
        vec.push(struct_sp);
    }
    for (i, item) in vec.iter().enumerate()
    {
        println!("TEST2 :: vector element {} [{}, {}]", i + 1, item.shares, item.price);
    }
    // TODO: make the calculation
    // TODO: refactor
    //print!("Result = {}", );
}
