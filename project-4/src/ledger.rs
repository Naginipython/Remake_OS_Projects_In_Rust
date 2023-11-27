use std::{
    fs,
    error::Error,
    sync::{Arc, Mutex},
    process,
    thread,
    collections::VecDeque,
};
use crate::bank::{self, Bank};

#[derive(Debug)]
struct Ledger {
    acc: i32,
    other: i32,
    amount: i32,
    mode: i32,
    ledger_id: i32,
}


pub fn init_bank(num_workers: i32, file: String) {
    let mut ledger: VecDeque<Ledger> = VecDeque::new();
    let bank = bank::init(10);
    if let Err(e) = load_ledger(file, &mut ledger) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    // println!("{:?}", bank.print_account());
    let mut workers = Vec::new();
    let ledger = Arc::new(Mutex::new(ledger));

    for id in 0..num_workers {
        let l = Arc::clone(&ledger);
        let worker = thread::spawn(move || worker(id, l, &bank));
        workers.push(worker);
    }
    for worker in workers {
        worker.join().unwrap();
    }
}

/**
    This function takes a file with lines of 4 peices of data, and places it into a Ledger object
 */
fn load_ledger(file: String, ledger: &mut VecDeque<Ledger>) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(file)?;
    let mut id = 0;
    for line in content.lines() {
        let data: Vec<i32> = line.split_whitespace()
            .map(|i| i.parse::<i32>().unwrap_or(-1))
            .collect();
        if data.len() != 4 { Err("Data contained more than 5 elements")? }
        if !data.iter().all(|i| *i >= 0) { Err("Data contains invalid data")? }
        ledger.push_back(Ledger {
            acc: data[0],
            other: data[1],
            amount: data[2],
            mode: data[3],
            ledger_id: id,
        });
        id+=1;
    }
    Ok(())
}

fn worker(worker_id: i32, ledger: Arc<Mutex<VecDeque<Ledger>>>, bank: &Bank) {
    let mut size = i32::MAX;
    while size != 0 {
        // lock and modify
        let mut l = ledger.lock().unwrap();
        size = (*l).len() as i32;
        let item;
        if size != 0 {
            item = (*l).pop_front().unwrap();
        } else { break; }
        // unlock
        drop(l);
        
        // println!("{worker_id}");
        // println!("{:?}", item);
        match item.mode {
            0 => bank.deposit(worker_id, item.ledger_id, item.acc, item.amount),
            1 => {}
            2 => {}
            _ => {}
        }
    }
}