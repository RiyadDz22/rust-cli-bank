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
