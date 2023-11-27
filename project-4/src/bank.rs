use std::sync::{ Arc, Mutex };

pub struct Account {
    account_id: i32,
    balance: Arc<Mutex<i64>>,
}

pub struct Bank {
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
    pub fn record_fail(&self, message: String) {
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
    pub fn withdraw(&self, worker_id: i32, ledger_id: i32, account_id: i32, amount: i32) -> i32 {
        let mut succ = -1;
        let account = self.accounts.get(account_id as usize).unwrap();
        let mut balance_lock = account.balance.lock().unwrap();
        if amount as i64 <= *balance_lock {
            *balance_lock -= amount as i64;
            succ = 0;
            self.record_succ(format!("Worker {worker_id} completed ledger {ledger_id}: withdraw {amount} from account {account_id}"));
        } else {
            self.record_fail(format!("Worker {worker_id} failed to complete ledger {ledger_id}: withdraw {amount} from account {account_id}"));
        }

        succ
    }
    pub fn transfer(&self, worker_id: i32, ledger_id: i32, src_id: i32, dest_id: i32, amount: i32) -> i32 {
        let mut succ = -1;
        if src_id == dest_id {
            return succ;
        }

        // let greater = if src_id >= dest_id {src_id} else {dest_id};
        // let account = self.accounts.get(greater as usize).unwrap();
        // let mut greater_lock = account.balance.lock().unwrap();
        let account = self.accounts.get(src_id as usize).unwrap();
        let mut src_lock = account.balance.lock().unwrap();
        
        // let lesser = if src_id <= dest_id {src_id} else {dest_id};
        // let account = self.accounts.get(lesser as usize).unwrap();
        // let mut lesser_lock = account.balance.lock().unwrap();
        let account = self.accounts.get(dest_id as usize).unwrap();
        let mut dest_lock = account.balance.lock().unwrap();

        if amount as i64 <= *src_lock {
            succ = 0;
            *src_lock -= amount as i64;
            *dest_lock += amount as i64;
            self.record_succ(format!("Worker {worker_id} completed ledger {ledger_id}: transfer {amount} from account {src_id} to account {dest_id}"));
        } else {
            self.record_fail(format!("Worker {worker_id} failed to complete ledger {ledger_id}: transfer {amount} from account {src_id} to account {dest_id}"));
        }

        succ
    }
}