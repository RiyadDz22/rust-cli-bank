mod actions;
mod models;
mod utils;

use actions::Actions;
use models::*;
use utils::{str_to_float, take_input};

fn main() {
    let mut account = Accounts::new();
    loop {
        println!(
            "Welcome to rust cli bank!
            
    Choose what you want to perform:
    1 - create an account
    2 - deposite into your account
    3 - withdraw from account
    4 - take a loan
    5 - account balance and status
    6 - exit
    "
        );
        let choice = take_input("enter your choice (1, 2, 3, 4, 5): ");
        if choice == "6" {
            println!("exiting...bye!");
            break;
        } else {
            match choice.as_str() {
                "1" => {
                    print!("\x1B[2J\x1B[1;1H");
                    let name = take_input("enter the name of the account: ");
                    account.create_account(name);
                }
                "2" => {
                    print!("\x1B[2J\x1B[1;1H");
                    let str_amount = take_input("enter the amount you want to deposite : ");
                    let amount = str_to_float(&str_amount).expect("invalid number");
                    let name =
                        take_input("enter the name of the account you want to deposite to: ");
                    account.deposit(amount, &name).expect("invalid deposit");
                }
                "3" => {
                    print!("\x1B[2J\x1B[1;1H");
                    let str_amount = take_input("enter the amount you want to withdraw : ");
                    let amount = str_to_float(&str_amount).expect("invalid number");
                    let name =
                        take_input("enter the name of the account you want to withdraw from: ");
                    account.withdraw(amount, &name).expect("invalid withdraw");
                }
                "4" => {
                    print!("\x1B[2J\x1B[1;1H");
                    let str_amount = take_input("enter the loan amount (max 1000): ");
                    let amount = str_to_float(&str_amount).expect("invalid number");
                    let name = take_input("enter the name of the account : ");
                    account.loan(amount, &name).expect("invalid loan");
                }
                "5" => {
                    print!("\x1B[2J\x1B[1;1H");
                    let name = take_input("enter the name of the account : ");
                    account.account_balance(&name);
                }
                _ => println!("Invalid option."),
            }
        }
    }
}
