
fn main() {
    use bank::{User, Bank};

    // Create a new bank
    let mut my_bank = Bank::new("MyBank".to_string(), 0.05, 0.1);

    // Add some users to the bank
    let user1 = User {
        name: "Alice".to_string(),
        credit_line: 1000.0,
        balance: 500,
    };
    let user2 = User {
        name: "Bob".to_string(),
        credit_line: 2000.0,
        balance: -300,
    };
    my_bank.add_user(user1);
    my_bank.add_user(user2);

    // Perform some operations
    println!("Initial state of the bank: {:?}", my_bank);

    // Transfer funds
    if let Err(err) = my_bank.transfer_funds("Alice", "Bob", 200.0) {
        println!("Error transferring funds: {}", err);
    }

    // Accrue interest
    my_bank.accure_interest();
    

    // Print the final state of the bank
    println!("Final state of the bank: {:?}", my_bank);
}


mod bank {

    #[derive(Debug, Clone)]
    pub struct User {
        pub name: String,
        pub credit_line: f64,
        pub balance: i64,
    }
    #[derive(Debug)]
    pub struct Bank {
        pub name: String,
        //list of users
        users: Vec<User>,
        credit_interest: f64,
        debit_interest: f64,
    }

    impl Bank {
        pub fn new(name: String, credit_interest: f64, debit_interest: f64) -> Bank {
            Bank {
                name,
                users: Vec::new(),
                credit_interest,
                debit_interest,
            }
        }

        pub fn add_user(&mut self, user: User) {
            self.users.push(user);
        }

        pub fn get_user(&self, name: &str) -> Option<&User> {
            self.users.iter().find(|user| user.name == name)
            
        }

        pub fn calc_balance(&self, user: &User) -> f64 {
            let mut balance = user.balance as f64;
            if user.credit_line > 0.0 {
                balance += user.credit_line * self.credit_interest;
            }
            if user.balance < 0 {
                balance -= user.balance as f64 * self.debit_interest;
            }
            balance
        }

        pub fn transfer_funds(
            &mut self,
            from_user: &str,
            to_user: &str,
            amount: f64,
        ) -> Result<(), String> {
            let from_index = self
                .users
                .iter()
                .position(|user| user.name == from_user)
                .ok_or("From user not found")?;
            let to_index = self
                .users
                .iter()
                .position(|user| user.name == to_user)
                .ok_or("To user not found")?;

            let (from_user, to_user) = if from_index != to_index {
                let (left, right) = self.users.split_at_mut(std::cmp::max(from_index, to_index));
                if from_index < to_index {
                    (&mut left[from_index], &mut right[0])
                } else {
                    (&mut right[0], &mut left[to_index])
                }
            } else {
                return Err("Cannot transfer funds to the same user".to_string());
            };

            if from_user.balance < amount as i64 {
                return Err("Insufficient funds".to_string());
            }

            // Perform the transfer
            from_user.balance -= amount as i64;
            to_user.balance += amount as i64;

            Ok(())
        }

        pub fn accure_interest(&mut self) {
            for user in &mut self.users {
                let mut balance = user.balance as f64;
                if user.credit_line > 0.0 {
                    balance += user.credit_line * self.credit_interest;
                }
                if user.balance < 0 {
                    balance -= user.balance as f64 * self.debit_interest;
                }
                user.balance = balance as i64;
            }
        }

        pub fn merge_banks(&mut self, other: &Bank) {
            for user in &other.users {
                if let Some(existing_user) = self.get_user_mut(&user.name) {
                    existing_user.balance += user.balance;
                } else {
                    self.users.push(user.clone());
                }
            }
        }

        pub fn get_user_mut(&mut self, name: &str) -> Option<&mut User> {
            self.users.iter_mut().find(|user| user.name == name)
        }
    }

    impl Drop for Bank {
        fn drop(&mut self) {
            println!(
                "Bank '{}' is being destroyed. Cleaning up resources...",
                self.name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    
    use super::bank::User;
    use super::bank::Bank;

    #[test]
    fn test_bank() {
        let mut bank = Bank::new("Test Bank".to_string(), 0.05, 0.1);
        let user1 = User {
            name: "Alice".to_string(),
            credit_line: 1000.0,
            balance: 500,
        };
        let user2 = User {
            name: "Bob".to_string(),
            credit_line: 2000.0,
            balance: -300,
        };
        bank.add_user(user1);
        bank.add_user(user2);

        assert_eq!(bank.get_user("Alice").unwrap().name, "Alice");
        assert_eq!(bank.get_user("Bob").unwrap().name, "Bob");
    }

    #[test]
    fn test_calc_balance() {
        let mut bank = Bank::new("Test Bank".to_string(), 0.05, 0.1);
        let user = User {
            name: "Alice".to_string(),
            credit_line: 1000.0,
            balance: 500,
        };
        bank.add_user(user);
        let balance = bank.calc_balance(bank.get_user("Alice").unwrap());
        assert_eq!(balance, 500.0 + 1000.0 * 0.05);
    }

    #[test]
    fn test_transfer_funds() {
        let mut bank = Bank::new("Test Bank".to_string(), 0.05, 0.1);
        let user1 = User {
            name: "Alice".to_string(),
            credit_line: 1000.0,
            balance: 500,
        };
        let user2 = User {
            name: "Bob".to_string(),
            credit_line: 2000.0,
            balance: -300,
        };
        bank.add_user(user1);
        bank.add_user(user2);

        assert!(bank.transfer_funds("Alice", "Bob", 200.0).is_ok());
        assert_eq!(bank.get_user("Alice").unwrap().balance, 300);
        assert_eq!(bank.get_user("Bob").unwrap().balance, -100);
    }

    #[test]
    fn test_accure_interest() {
        let mut bank = Bank::new("Test Bank".to_string(), 0.05, 0.1);
        let user = User {
            name: "Alice".to_string(),
            credit_line: 1000.0,
            balance: 500,
        };
        bank.add_user(user);
        bank.accure_interest();
        assert_eq!(
            bank.get_user("Alice").unwrap().balance,
            (500.0 + 1000.0 * 0.05) as i64
        );
    }

    #[test]
    fn test_merge_banks() {
        let mut bank1 = Bank::new("Bank 1".to_string(), 0.05, 0.1);
        let mut bank2: Bank = Bank::new("Bank 2".to_string(), 0.05, 0.1);

        let user1 = User {
            name: "Alice".to_string(),
            credit_line: 1000.0,
            balance: 500,
        };
        let user2 = User {
            name: "Bob".to_string(),
            credit_line: 2000.0,
            balance: -300,
        };
        bank1.add_user(user1);
        bank2.add_user(user2);

        bank1.merge_banks(&bank2);
        assert_eq!(bank1.get_user("Alice").unwrap().balance, 500);
        assert_eq!(bank1.get_user("Bob").unwrap().balance, -300);
    }

    #[test]
    fn test_get_user_mut() {
        let mut bank = Bank::new("Test Bank".to_string(), 0.05, 0.1);
        let user = User {
            name: "Alice".to_string(),
            credit_line: 1000.0,
            balance: 500,
        };
        bank.add_user(user);
        let user_mut = bank.get_user_mut("Alice").unwrap();
        user_mut.balance += 100;
        assert_eq!(bank.get_user("Alice").unwrap().balance, 600);
    }

    #[test]
    fn test_drop_bank() {
        {
            let bank = Bank::new("Test Bank".to_string(), 0.05, 0.1);
            drop(bank)
        }
    }
}
