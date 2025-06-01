use std::{collections::HashMap, io::stdin};

/// Represents a user's account
pub struct Account {
    pub name: String,
    pub balance: f32,
    pub debt: f32,
}

/// Holds all user accounts in a map
pub struct Accounts {
    pub list: HashMap<String, Account>,
}

impl Accounts {
    /// Create a new `Accounts` instance with an empty list
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }
}

/// Defines actions you can perform on an account
pub trait Actions {
    fn create_account(&mut self, name: String);
    fn deposit(&mut self, amount: f32, name: &str) -> Result<(), String>;
    fn withdraw(&mut self, amount: f32, name: &str) -> Result<(), String>;
    fn loan(&mut self, amount: f32, name: &str) -> Result<(), String>;
    fn account_balance(&self, name: &str);
}

impl Actions for Accounts {
    /// Creates a new account if it doesn't already exist
    fn create_account(&mut self, name: String) {
        if self.list.contains_key(&name) {
            println!("⚠️ Account already exists!");
            return;
        }
        let account = Account {
            name: name.clone(),
            balance: 0.0,
            debt: 0.0,
        };
        self.list.insert(name, account);
    }

    /// Deposit funds into the account, paying off debt first
    fn deposit(&mut self, amount: f32, name: &str) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("❌ Invalid deposit amount".to_string());
        }

        let account = self.list.get_mut(name).ok_or("❌ Account not found")?;

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

    /// Withdraw funds from the account if balance allows
    fn withdraw(&mut self, amount: f32, name: &str) -> Result<(), String> {
        let account = self.list.get_mut(name).ok_or("❌ Account not found")?;
        if amount <= 0.0 {
            return Err("❌ Withdrawal amount must be positive".to_string());
        }
        if account.balance >= amount {
            account.balance -= amount;
            Ok(())
        } else {
            Err("❌ Insufficient balance".to_string())
        }
    }

    /// Take a loan up to a maximum of 1000
    fn loan(&mut self, amount: f32, name: &str) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("❌ Loan amount must be positive".to_string());
        }

        let account = self.list.get_mut(name).ok_or("❌ Account not found")?;

        if account.debt + amount > 1000.0 {
            return Err("❌ Loan limit exceeded (max 1000)".to_string());
        }

        account.debt += amount;
        account.balance += amount;

        Ok(())
    }

    /// Print account balance and debt
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

/// Helper to take string input from the user
fn take_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

/// Convert string input to f32, returns error if invalid
fn str_to_float(num: &str) -> Result<f32, String> {
    num.trim()
        .parse::<f32>()
        .map_err(|_| "❌ Please enter a valid number.".to_string())
}

/// CLI Entry point
fn main() {
    let mut account = Accounts::new();

    loop {
        println!(
            "🏦 Welcome to Rust CLI Bank!

Choose an option:
1 - Create an account
2 - Deposit into your account
3 - Withdraw from account
4 - Take a loan
5 - View account balance
6 - Exit
"
        );

        let choice = take_input("Enter your choice (1-6): ");

        match choice.as_str() {
            "1" => {
                // Clear screen
                print!("\x1B[2J\x1B[1;1H");
                let name = take_input("Enter the name of the account:");
                account.create_account(name);
            }
            "2" => {
                print!("\x1B[2J\x1B[1;1H");
                let str_amount = take_input("Enter deposit amount:");
                match str_to_float(&str_amount) {
                    Ok(amount) => {
                        let name = take_input("Enter account name:");
                        if let Err(err) = account.deposit(amount, &name) {
                            println!("{err}");
                        }
                    }
                    Err(err) => println!("{err}"),
                }
            }
            "3" => {
                print!("\x1B[2J\x1B[1;1H");
                let str_amount = take_input("Enter withdrawal amount:");
                match str_to_float(&str_amount) {
                    Ok(amount) => {
                        let name = take_input("Enter account name:");
                        if let Err(err) = account.withdraw(amount, &name) {
                            println!("{err}");
                        }
                    }
                    Err(err) => println!("{err}"),
                }
            }
            "4" => {
                print!("\x1B[2J\x1B[1;1H");
                let str_amount = take_input("Enter loan amount (max 1000):");
                match str_to_float(&str_amount) {
                    Ok(amount) => {
                        let name = take_input("Enter account name:");
                        if let Err(err) = account.loan(amount, &name) {
                            println!("{err}");
                        }
                    }
                    Err(err) => println!("{err}"),
                }
            }
            "5" => {
                print!("\x1B[2J\x1B[1;1H");
                let name = take_input("Enter account name:");
                account.account_balance(&name);
            }
            "6" => {
                println!("👋 Exiting... Bye!");
                break;
            }
            _ => println!("❌ Invalid option. Please enter a number from 1 to 6."),
        }
    }
}
