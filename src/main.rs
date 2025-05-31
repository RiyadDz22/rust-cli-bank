use std::collections::HashMap;

pub struct Account {
    pub name: String,
    pub balance: f32,
    pub debt: f32,
}

pub struct Accounts {
    pub list: HashMap<String, Account>,
}

impl Accounts {
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }
}

pub trait Actions {
    fn create_account(&mut self, name: String);
    fn deposit(&mut self, amount: f32, name: &str) -> Result<(), String>;
    fn withdraw(&mut self, amount: f32, name: &str) -> Result<(), String>;
    fn loan(&mut self, amount: f32, name: &str) -> Result<(), String>;
    fn list_aacounts(&self) {}
}

impl Actions for Accounts {
    fn create_account(&mut self, name: String) {
        if self.list.contains_key(&name) {
            println!("Account already exists!");
            return;
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
            Err(("invalid balance").to_string())
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
    fn list_aacounts(&self) {
        if self.list.is_empty() {
            println!("\n🔍 There is nothing to list. Please create some accounts.\n");
        } else {
            println!("\n📋 List of Accounts:\n");
            println!(
                "{:>2} | {:<20} | {:<10} | {}",
                "#", "Name", "Balance", "Debt"
            );
            println!("{}", "-".repeat(50));

            for (i, (_key, acc)) in self.list.iter().enumerate() {
                println!(
                    "{:>2}. {:<20} | {:<10.2} | {:.2}",
                    i + 1,
                    acc.name,
                    acc.balance,
                    acc.debt
                );
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
