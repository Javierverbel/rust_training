#[derive(Default, Debug)]

struct User {
    name: String,
    credit_line: u64,
    balance: i16,
}

struct Bank {
    users: Vec<User>,
    name: String,
    credit_interest: u64,
    debit_interest: u64,
}

impl Bank {
    fn calc_balance(&self) -> i64 {
        let mut bal = 0i64;
        for us in &self.users {
            bal += us.balance as i64;
        }
        bal
    }

    fn get_user(&self, user_name: String) -> Option<usize> {
        for u_idx in 0..&self.users.len() {
            if user_name == self.users[u_idx].name {
                return u_idx;
            }
        }
        None
    }

    fn transfer_funds(
        &self,
        origin_user_name: String,
        dest_user_name: String,
        amount: u64,
    ) -> Result<bool, &'static str> {
        let origin_user = 
        let destination_user = self.get_user(dest_user_name);

        let Some(origin_user) = self.get_user(origin_user_name) else {
            return Err("Origin user does not exists!");
        };

        if origin_user.is_none() {
            
        } else if destination_user.is_none() {
            return Err("Destination user does not exists!");
        } else if origin_user.unwrap().balance < amount as i16 {
            return Err("Origin user does not have enough founds!");
        } else {
            origin_user.unwrap().balance -= amount as i16;
            destination_user.unwrap().balance += amount as i16;
            return Ok(true);
        }
    }

    fn accrue_interest(&mut self) {
        todo!()
    }
}
