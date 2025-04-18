#[derive(Default, Debug, Clone)]

pub struct User {
    name: String,
    credit_line: u64,
    balance: i64,
}
impl User {
    fn new(name: String, init_balance: i64) -> User {
        User {
            name,
            credit_line: 0,
            balance: init_balance,
        }
    }

    fn update_balance(&mut self, amount: i64) {
        self.balance += amount;
    }

    fn set_credit_line(&mut self, amount_credit: u64) {
        self.credit_line = amount_credit;
    }
}

#[derive(Default, Debug)]
pub struct Bank {
    users: Vec<User>,
    name: String,
    credit_interest: u64,
    debit_interest: u64,
}

impl Bank {
    pub fn new(bank_name: String) -> Bank {
        Bank {
            users: Vec::new(),
            name: bank_name,
            credit_interest: 0,
            debit_interest: 0,
        }
    }

    pub fn create_user(&mut self, user_name: String, init_balance: i64) {
        let user = User::new(user_name, init_balance);
        self.add_user(user);
    }

    // pub fn update_user_balance(&mut self, user_name: &str){
    //     get_user_idx(&self, user_name: &str)
    // }

    fn calc_balance(&self) -> i64 {
        let mut bal = 0i64;
        for us in &self.users {
            bal += us.balance as i64;
        }
        bal
    }

    fn get_user_idx(&self, user_name: &str) -> Option<usize> {
        (0..self.users.len()).find(|&u_idx| user_name == self.users[u_idx].name)
    }

    fn transfer_funds(
        &mut self,
        origin_user_name: &str,
        dest_user_name: &str,
        amount: u64,
    ) -> Result<bool, String> {
        let Some(origin_user_idx) = self.get_user_idx(origin_user_name) else {
            return Err(format!("{origin_user_name} (origin) user does not exists!"));
        };

        let Some(destination_user_idx) = self.get_user_idx(dest_user_name) else {
            return Err(format!(
                "{dest_user_name} (destination) user does not exists!"
            ));
        };

        match self.users[origin_user_idx].balance >= amount as i64 {
            true => {
                self.users[origin_user_idx].update_balance(-(amount as i64));
                self.users[destination_user_idx].update_balance(amount as i64);
                Ok(true)
            }
            false => Err(format!(
                "{origin_user_name} user does not have enough founds!"
            )),
        }
    }

    fn accrue_interest(&mut self) {
        for user_idx in 0..self.users.len() {
            let user = &mut self.users[user_idx];
            if user.balance >= 0 {
                user.balance += (self.debit_interest as i64) * user.balance;
            } else {
                user.balance += (self.credit_interest as i64) * user.balance;
            }
        }
    }

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn merge_bank(&mut self, bank: Bank) {
        for user_origin in bank.users.iter() {
            match self.get_user_idx(&user_origin.name) {
                None => self.add_user(user_origin.clone()),
                Some(user_idx_in_dest) => {
                    self.users[user_idx_in_dest].balance += user_origin.balance
                }
            }
        }
    }
}
