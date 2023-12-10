// use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
pub struct ArrivalQueue {
    pub arrival: i32,
    pub duration: i32,
    pub process: Option<Process>,
}

impl Ord for ArrivalQueue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.arrival == other.arrival {
            other.duration.cmp(&self.duration)
        } else {
            other.arrival.cmp(&self.arrival)
        }
    }
}
impl PartialOrd for ArrivalQueue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
pub struct DurationQueue {
    pub arrival: i32,
    pub duration: i32,
    pub process: Option<Process>,
}

impl Ord for DurationQueue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.duration == other.duration {
            other.arrival.cmp(&self.duration)
        } else {
            other.duration.cmp(&self.duration)
        }
    }
}
impl PartialOrd for DurationQueue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Process {
    pub arrival: i32,
    pub first_run: Option<i32>,
    pub duration: i32,
    pub completion: Option<i32>,
}

impl Process {
    pub fn to_process(queue: &ArrivalQueue) -> Process {
        Process {
            arrival: queue.arrival,
            first_run: None,
            duration: queue.duration,
            completion: None
        }
    }
}

// impl Ord for Process {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         other.arrival.cmp(&self.arrival)
//     }
// }
// impl PartialOrd for Process {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// pub fn temp() {
//     let vec_temp = vec![Process::new(2,1),Process::new(10,1),Process::new(1,1)];
//     let mut queue = BinaryHeap::from(vec_temp);
//     queue.pop();
//     println!("{:?}", queue.peek());
// }