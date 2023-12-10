use std::{
    env,
    process,
};

use scheduling::{
    util::{read_workload, show_metrics},
    schedulers,
    structs::{Process, ArrivalQueue},
};

mod scheduling;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage: [fifo|sjf|stcf|rr] workload_file");
        process::exit(0);
    }

    let algorithm: &str = &args[1];
    let workload_file: String = args[2].clone();

    let workload: Vec<ArrivalQueue> = read_workload(workload_file);

    match algorithm {
        "fifo" => show_metrics(schedulers::fifo(workload)),
        "sjf" => show_metrics(schedulers::sjf(workload)),
        "stcf" => show_metrics(schedulers::stcf(workload)),
        "rr" => {},
        _ => {
            println!("Error: Unknown algorithm: {algorithm}");
            println!("usage: [fifo|sjf|stcf|rr] workload_file");
            process::exit(1);
        }
    }
}
