use std::sync::{ Arc, Mutex };

#[derive(Debug)]
pub struct Account {
    account_id: i32,
    balance: Arc<Mutex<i64>>,
}

#[derive(Debug)]
pub struct Bank {
    num: Arc<Mutex<i32>>,
    num_succ: Arc<Mutex<i32>>,
    num_fail: Arc<Mutex<i32>>,
    accounts: Vec<Account>,
}

pub fn init(n: i32) -> Bank {
    let mut accounts: Vec<Account> = Vec::new();
    for i in 0..n {
        accounts.push(Account {
            account_id: i,
            balance: Arc::new(Mutex::new(0)),
        });
    }
    Bank {
        num: Arc::new(Mutex::new(0)),
        num_succ: Arc::new(Mutex::new(0)),
        num_fail: Arc::new(Mutex::new(0)),
        accounts: accounts,
    }
}

impl Bank {
    pub fn print_account(&self) {
        for i in &self.accounts {
            let balance_lock = i.balance.lock().unwrap();
            println!("ID# {} | {}", i.account_id, balance_lock);
        }

        println!(
            "Success: {} Fails: {}", 
            self.num_succ.lock().unwrap(), 
            self.num_fail.lock().unwrap(),
        );
    }
    pub fn record_succ(&self, message: String) {
        let mut succ_lock = self.num_succ.lock().unwrap();
        println!("{message}");
        *succ_lock+=1;
    }
    pub fn record_fail(&self, message: &str) {
        let mut fail_lock = self.num_fail.lock().unwrap();
        println!("{message}");
        *fail_lock+=1;
    }
    pub fn deposit(&self, worker_id: i32, ledger_id: i32, account_id: i32, amount: i32) {
        let account = self.accounts.get(account_id as usize).unwrap();
        let mut balance_lock = account.balance.lock().unwrap();
        *balance_lock += amount as i64;
        self.record_succ(format!("Worker {worker_id} completed ledger {ledger_id}: deposit {} into account {account_id}", *balance_lock));
    }
    pub fn withdraw(&self, worker_id: i32, ledger_id: i32, account_id: i32, amount: i32) {
        
    }
    pub fn transfer(&self, worker_id: i32, ledger_id: i32, src_id: i32, dest_id: i32, amount: i32) {
        
    }
}

impl Clone for Bank {
    fn clone(&self) -> Self {
        Bank {
            num: Arc::new(Mutex::new(*self.num.lock().unwrap())),
            num_succ: Arc::new(Mutex::new(*self.num_succ.lock().unwrap())),
            num_fail: Arc::new(Mutex::new(*self.num_fail.lock().unwrap())),
            accounts: self.accounts.clone(),
        }
    }
}
impl Clone for Account {
    fn clone(&self) -> Self {
        Account {
            account_id: self.account_id,
            balance: Arc::new(Mutex::new(*self.balance.lock().unwrap())),
        }
    }
}