#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount{ balance: initial_balance }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 { self.balance += amount; }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount <= self.balance { self.balance -= amount; }
    }
    
    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        assert_eq!(BankAccount::new(5.0).balance(), 5.0);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut acc = BankAccount::new(5.0);
        acc.deposit(8.0);
        assert_eq!(acc.balance(), 5.0 + 8.0);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut acc = BankAccount::new(5.0);
        acc.withdraw(2.0);
        acc.withdraw(6.0);
        assert_eq!(acc.balance(), 5.0 - 2.0);
    }

    // Add more tests here
}

