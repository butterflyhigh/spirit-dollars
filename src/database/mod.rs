use std::{fs::{self, File}, io::Write, ops::Add, sync::{Arc, Mutex, PoisonError}};
use serde::{Deserialize, Serialize};

// i was posessed by the holy spirit and wrote this goofy ahh code

#[derive(Debug)]
pub struct Database {
    file: Arc<Mutex<File>>,
    data: Arc<Mutex<Data>>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Data {
    users: Vec<User>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id(pub String);

#[derive(Debug, Deserialize, Serialize, Clone)]
struct User {
    id: Id,
    debts: Vec<Debt>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Debt {
    owed_to: Id,
    amount: usize
}

impl Database {
    pub fn sync(&self) -> Result<(), std::io::Error> {
        match self.file.lock() {
            Ok(mut lock) => {
                let data = self.data.lock().unwrap().clone();
                lock.write(serde_json::to_string(&data).unwrap().as_bytes())?;
            }
            Err(e) => {
                panic!("{}", e)
            }
        }

        Ok(())
    }

    pub fn get_amount(&self, user_id: &Id, owed_to: &Id) -> Result<usize, usize> {
        let lock = self.data.lock().unwrap();
        let user_index = lock.users.binary_search_by_key(&user_id.0, |user| user.id.clone().0)?;
        let debt_index = lock.users[user_index].debts.binary_search_by_key(&owed_to.0, |owed| owed.owed_to.clone().0)?;

        Ok(lock.users[user_index].debts[debt_index].amount)
    }

    pub fn add_debt(&self, new_debt: &Debt, user_id: &Id) -> Result<(), PoisonError<()>> {
        match self.data.lock() {
            Ok(mut lock) => {
                // check if user in debt is already in database
                let res = lock.users.binary_search_by_key(&user_id.0, |user| user.id.0.clone());

                match res {
                    Ok(i) => {
                        let (j, added_debt) = match lock.users[i].debts.binary_search_by_key(&new_debt.owed_to.0, |debt| debt.owed_to.0.clone()) {
                            Ok(j) => { (j, lock.users[i].debts[i].to_owned() + new_debt.to_owned()) }, // call me lucia bc im making spaghetti
                            Err(j) => { (j, new_debt.to_owned()) },
                        };

                        lock.users[i].debts[j] = added_debt;
                        println!("1");
                    },
                    Err(i) => {
                        lock.users.insert(i, User {
                            id: user_id.to_owned(), debts: vec![new_debt.to_owned()]
                        });
                    },
                }
            }

            Err(e) => {
                panic!("{}", e);
            }
        }

        Ok(())
    }

}

impl Data {
    fn new() -> Self {
        Self {
            users: Vec::<User>::new()
        }
    }
}

impl Debt {
    pub fn new(owed_to: Id, amount: usize) -> Self {
        Self {
            owed_to,
            amount
        }
    }
}

impl Add for Debt {
    type Output = Debt;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.owed_to.0 == rhs.owed_to.0);

        Self::Output {
            owed_to: self.owed_to,
            amount: self.amount + rhs.amount
        }
    }
}

pub fn create_database(path: &str) -> Result<Database, std::io::Error> {
    let data = Data::new();
    let str_data = serde_json::to_string(&data).unwrap();
    let mut file = fs::File::create(path)?;
    file.write(&str_data.as_bytes())?;

    Ok(Database { file: Arc::new(Mutex::new(file)), data: Arc::new(Mutex::new(data)) })
}

pub fn open_database(path: &str) -> Result<Database, std::io::Error> {
    let data_str = fs::read_to_string(path)?;
    println!("{}", data_str);
    let data: Data = match serde_json::from_str(&data_str) {
        Ok(data) => data,
        Err(_) => {
            Data::new()
        }
    };
    let file = fs::File::create(path)?;
    let db = Database {
        file: Arc::new(Mutex::new(file)),
        data: Arc::new(Mutex::new(data))
    };

    Ok(db)
}
