use std::fs;
use std::process;
use super::structs::ArrivalQueue;
use super::structs::Process;

pub fn read_workload(file: String) -> Vec<ArrivalQueue> {
    let mut workload: Vec<ArrivalQueue> = Vec::new();
    let file_data = fs::read_to_string(file)
        .unwrap_or_else(|_| {
            eprintln!("Error: File doesn't exist");
            process::exit(1);
        });

    for line in file_data.lines() {
        let data: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap_or(-1)).collect();
        if data.len() > 2 || data.iter().all(|x| *x >= 0) {
            workload.push(ArrivalQueue { arrival: data[0], duration: data[1], process: None });
        } else {
            eprintln!("Error: File data not valid");
        }
    }  
    workload
}

#[allow(dead_code)]
// Show backwards for some reason?
pub fn show_workload(workload: &Vec<Process>) {
    let mut output = "Workload:\n".to_string();
    for x in workload {
        output.push_str(&format!("\t{} {}\n", x.arrival, x.duration));
    }
    println!("{output}");
}

pub fn show_processes(processes: &Vec<Process>) {
    let mut output: String = "Processes:\n".to_string();
    for p in processes {
        output.push_str(&format!(
            "\tarrival={}, duration={}, first_run={}, completion={}\n",
            p.arrival,
            p.duration,
            p.first_run.unwrap(),
            p.completion.unwrap()
        ))
    }
    println!("{output}");
}

fn avg_turnaround(processes: &Vec<Process>) -> f64 {
    // turn = complete - arrival
    let size = processes.len();
    let mut sum: f64 = 0.0;
    for p in processes {
        sum += (p.completion.unwrap_or_else(|| {
            eprintln!("Error: completion not properly set");
            process::exit(1);
        }) - p.arrival) as f64
    }
    sum/size as f64
}
fn avg_response(processes: &Vec<Process>) -> f64 {
    // response = first_run - arrival
    let size = processes.len();
    let mut sum: f64 = 0.0;
    for p in processes {
        sum += (p.first_run.unwrap_or_else(|| {
            eprintln!("Error: first_run not properly set");
            process::exit(1);
        }) - p.arrival) as f64
    }
    sum/size as f64
}

pub fn show_metrics(processes: Vec<Process>) {
    let avg_t: f64 = avg_turnaround(&processes);
    let avg_r: f64 = avg_response(&processes);
    show_processes(&processes);
    println!("Average Turnaround Time: {}\nAverage Response Time: {}", avg_t, avg_r);
}