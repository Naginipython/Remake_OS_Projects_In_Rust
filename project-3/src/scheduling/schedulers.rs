use std::collections::BinaryHeap;
use crate::Process;

use super::{util::show_workload, structs::ArrivalQueue};
// use crate::show_workload;

fn remove_time(wl: BinaryHeap<ArrivalQueue>, runtime: i32) -> BinaryHeap<ArrivalQueue> {
    let mut temp = wl.into_vec();
    for w in temp.iter_mut() {
        let new_t = w.arrival - runtime;
        if new_t < 0 {
            w.arrival = 0;
        }
    }
    BinaryHeap::from(temp)
}

pub fn fifo(workload: Vec<ArrivalQueue>) -> Vec<Process> {
    let mut wl = BinaryHeap::from(workload);
    let mut complete: Vec<Process> = Vec::new();
    let mut runtime = 0;
    while !wl.is_empty() {
        let mut p: Process = Process::to_process(&wl.pop().unwrap());
        p.first_run = Some(runtime);
        runtime += p.duration;
        p.completion = Some(runtime);

        complete.push(p);
    }
    complete
}

pub fn sjf(mut workload: Vec<ArrivalQueue>) -> Vec<Process> {
    // Init processes in workload
    for w in workload.iter_mut() {
        let p = Process::to_process(&w);
        w.process = Some(p);
    }
    let mut wl = BinaryHeap::from(workload);
    let mut complete: Vec<Process> = Vec::new();
    let mut runtime = 0;
    while !wl.is_empty() {
        let mut p: Process = wl.pop().unwrap().process.unwrap();
        p.first_run = Some(runtime);
        runtime += p.duration;
        p.completion = Some(runtime);

        // remove time from wl
        wl = remove_time(wl, runtime);

        complete.push(p);
    }
    complete
}

pub fn stcf(mut workload: Vec<ArrivalQueue>) -> Vec<Process> {
    // Init processes in workload
    for w in workload.iter_mut() {
        let p = Process::to_process(&w);
        w.process = Some(p);
    }
    let mut wl = BinaryHeap::from(workload);
    let mut complete: Vec<Process> = Vec::new();
    let mut runtime = 0;

    while !wl.is_empty() {
        let mut queue = wl.pop().unwrap();
        let p = queue.process.as_mut().unwrap();
        if p.first_run == None {
            p.first_run = Some(runtime);
        }
        queue.duration -= 1;
        runtime += 1;
        wl = remove_time(wl, runtime+1);
        
        // remove time from wl
        if queue.duration != 0 {
            wl.push(queue);
        } else {
            p.completion = Some(runtime);
            complete.push(queue.process.unwrap());
        }
    }

    complete
}