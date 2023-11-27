use std::env;
use std::process;

pub mod bank;
mod ledger;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <num_of_threads> <ledger_file>\n", args[0]);
        process::exit(1);
    }
    //using clone isn't ideal, but since this is a smaller program, it's fine. [Fixed in NEW WAY]
    let num = args[1].parse::<i32>().unwrap_or(-1);
    let ledger = args[2].clone();
    ledger::init_bank(num, ledger);
}
