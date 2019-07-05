extern crate docopt;

use docopt::Docopt;

const VERSION: &'static str = "0.1.0";
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

    if args.get_bool("--version")
    {
        println!("avg_price v{}", VERSION);
        std::process::exit(0);
    }
    
    struct SharesPrice
    {
        shares: i32,
        price: f64
    }

    let mut vec: Vec<SharesPrice> = vec![];
    for sp in args.get_str("--list").split(";")
    {
        let vec_tmp: Vec<&str> = sp.split(",").collect::<Vec<&str>>();
        let s: i32 = vec_tmp[0].parse::<i32>().unwrap();
        let p: f64 = vec_tmp[1].parse::<f64>().unwrap();
        let struct_sp = SharesPrice { shares: s, price: p}; 
        vec.push(struct_sp);
    }
    let mut enumerator: f64 = 0.0;
    let mut denominator: i32 = 0;
    let mut shares_total: i32 = 0;
    for (_i, item) in vec.iter().enumerate()
    {
        enumerator = enumerator + (item.shares as f64) * item.price;
        denominator = denominator + item.shares;
        shares_total = shares_total + item.shares;
    }
    let avg_value = enumerator/(denominator as f64);
    println!("Result = {}/{} = {}", enumerator, denominator, avg_value);
    println!("Value = {}*{} = {}", (shares_total as f64), avg_value, (shares_total as f64) * avg_value);
}
