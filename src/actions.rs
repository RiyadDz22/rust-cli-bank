use crate::models::{Account, Accounts};

pub trait Actions {
    fn create_account(&mut self, name: String);
    fn deposit(&mut self, amount: f32, name: &str) -> Result<(), String>;
    fn withdraw(&mut self, amount: f32, name: &str) -> Result<(), String>;
    fn loan(&mut self, amount: f32, name: &str) -> Result<(), String>;
    fn account_balance(&self, name: &str);
}

impl Actions for Accounts {
    fn create_account(&mut self, name: String) {
        if self.list.contains_key(&name) {
            println!("Account already exists!");
        } else {
            let account = Account {
                name: name.clone(),
                balance: 0.0,
                debt: 0.0,
            };
            self.list.insert(name, account);
        }
    }

    fn deposit(&mut self, amount: f32, name: &str) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("invalid amount".to_string());
        }

        let account = self.list.get_mut(name).ok_or("account not found")?;

        if account.debt > 0.0 {
            if amount >= account.debt {
                let leftover = amount - account.debt;
                account.debt = 0.0;
                account.balance += leftover;
            } else {
                account.debt -= amount;
            }
        } else {
            account.balance += amount;
        }

        Ok(())
    }

    fn withdraw(&mut self, amount: f32, name: &str) -> Result<(), String> {
        let account = self.list.get_mut(name).ok_or("account not found")?;
        if account.balance >= amount {
            account.balance -= amount;
            Ok(())
        } else {
            Err("invalid balance".to_string())
        }
    }

    fn loan(&mut self, amount: f32, name: &str) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("loan amount must be positive".to_string());
        }

        let account = self.list.get_mut(name).ok_or("account not found")?;
        if account.debt + amount > 1000.0 {
            return Err("loan limit exceeded".to_string());
        }

        account.debt += amount;
        account.balance += amount;

        Ok(())
    }

    fn account_balance(&self, name: &str) {
        match self.list.get(name) {
            Some(account) => {
                println!("\n💼 Account Details for '{}':", name);
                println!("Balance: {:.2}", account.balance);
                println!("Debt: {:.2}\n", account.debt);
            }
            None => {
                println!("🚫 No account found with that name.");
            }
        }
    }
}
