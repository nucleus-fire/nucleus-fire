use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Add, Mul, Sub};

// ═══════════════════════════════════════════════════════════════════════════
// MONEY
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money(Decimal);

impl Money {
    pub fn new(amount: Decimal) -> Self {
        Self(amount)
    }

    pub fn zero() -> Self {
        Self(dec!(0))
    }

    pub fn amount(&self) -> Decimal {
        self.0
    }

    pub fn from_f64(_: f64) -> ! {
        panic!("Floats are strictly forbidden in Vault engine. Use Decimal.");
    }

    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    pub fn is_positive(&self) -> bool {
        self.0 > dec!(0)
    }

    pub fn is_negative(&self) -> bool {
        self.0 < dec!(0)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == dec!(0)
    }
}

impl Add for Money {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for Money {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Mul<Decimal> for Money {
    type Output = Self;
    fn mul(self, rhs: Decimal) -> Self {
        Self(self.0 * rhs)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ACCOUNTS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    Asset,     // Cash, Receivables (Normal Debit)
    Liability, // Payables, loans (Normal Credit)
    Equity,    // Owner's equity (Normal Credit)
    Revenue,   // Sales (Normal Credit)
    Expense,   // Salaries (Normal Debit)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub account_type: AccountType,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// TRANSACTIONS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub account_id: String,
    /// Debit (+) or Credit (-)
    pub amount: Money,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub description: String,
    pub entries: Vec<LedgerEntry>,
    pub date: String, // ISO date
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl Transaction {
    /// Create a new transaction builder
    pub fn builder(id: impl Into<String>, description: impl Into<String>) -> TransactionBuilder {
        TransactionBuilder {
            id: id.into(),
            description: description.into(),
            entries: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Validate that the transaction is balanced (sum = 0)
    pub fn is_balanced(&self) -> bool {
        let sum: Decimal = self.entries.iter().map(|e| e.amount.amount()).sum();
        sum == dec!(0)
    }
}

pub struct TransactionBuilder {
    id: String,
    description: String,
    entries: Vec<LedgerEntry>,
    metadata: HashMap<String, String>,
}

impl TransactionBuilder {
    pub fn debit(mut self, account_id: &str, amount: Money) -> Self {
        // Debits are positive for Assets/Expenses
        self.entries.push(LedgerEntry {
            account_id: account_id.to_string(),
            amount: amount.abs(),
            description: None,
        });
        self
    }

    pub fn credit(mut self, account_id: &str, amount: Money) -> Self {
        // Credits are negative
        self.entries.push(LedgerEntry {
            account_id: account_id.to_string(),
            amount: Money::new(-amount.amount().abs()),
            description: None,
        });
        self
    }

    pub fn entry(mut self, account_id: &str, amount: Money) -> Self {
        self.entries.push(LedgerEntry {
            account_id: account_id.to_string(),
            amount,
            description: None,
        });
        self
    }

    pub fn metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(self) -> Result<Transaction, String> {
        let tx = Transaction {
            id: self.id,
            description: self.description,
            entries: self.entries,
            date: chrono::Utc::now().to_rfc3339(),
            metadata: self.metadata,
        };

        if !tx.is_balanced() {
            return Err("Transaction is not balanced (Debits != Credits)".to_string());
        }

        Ok(tx)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// LEDGER
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Default)]
pub struct Ledger {
    accounts: HashMap<String, Account>,
    transactions: Vec<Transaction>,
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_account(&mut self, account: Account) {
        self.accounts.insert(account.id.clone(), account);
    }

    pub fn record(&mut self, transaction: Transaction) -> Result<(), String> {
        // Verify accounts exist
        for entry in &transaction.entries {
            if !self.accounts.contains_key(&entry.account_id) {
                return Err(format!("Account not found: {}", entry.account_id));
            }
        }

        if !transaction.is_balanced() {
            return Err("Transaction is not balanced".to_string());
        }

        self.transactions.push(transaction);
        Ok(())
    }

    /// Calculate current balance for an account
    pub fn balance(&self, account_id: &str) -> Money {
        let mut total = dec!(0);

        for tx in &self.transactions {
            for entry in &tx.entries {
                if entry.account_id == account_id {
                    total += entry.amount.amount();
                }
            }
        }

        Money::new(total)
    }

    /// Get trial balance (all accounts)
    pub fn trial_balance(&self) -> HashMap<String, Money> {
        let mut balances = HashMap::new();

        for id in self.accounts.keys() {
            balances.insert(id.clone(), self.balance(id));
        }

        balances
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_money_ops() {
        let a = Money::new(dec!(100));
        let b = Money::new(dec!(50));
        assert_eq!((a + b).amount(), dec!(150));
        assert_eq!((a - b).amount(), dec!(50));
        assert_eq!((a * dec!(2)).amount(), dec!(200));
    }

    #[test]
    fn test_transaction_balancing() {
        let tx = Transaction::builder("tx1", "Sale")
            .debit("cash", Money::new(dec!(100)))
            .credit("revenue", Money::new(dec!(100)))
            .build();

        assert!(tx.is_ok());

        let tx = Transaction::builder("tx2", "Bad")
            .debit("cash", Money::new(dec!(100)))
            .credit("revenue", Money::new(dec!(90))) // Unbalanced
            .build();

        assert!(tx.is_err());
    }

    #[test]
    fn test_ledger_flow() {
        let mut ledger = Ledger::new();

        ledger.create_account(Account {
            id: "cash".into(),
            name: "Cash".into(),
            account_type: AccountType::Asset,
            metadata: HashMap::new(),
        });

        ledger.create_account(Account {
            id: "sales".into(),
            name: "Sales".into(),
            account_type: AccountType::Revenue,
            metadata: HashMap::new(),
        });

        let tx = Transaction::builder("t1", "Sale of Goods")
            .debit("cash", Money::new(dec!(100)))
            .credit("sales", Money::new(dec!(100)))
            .build()
            .unwrap();

        ledger.record(tx).unwrap();

        assert_eq!(ledger.balance("cash").amount(), dec!(100));
        assert_eq!(ledger.balance("sales").amount(), dec!(-100)); // Credits are negative
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// COMPATIBILITY HELPER
// ═══════════════════════════════════════════════════════════════════════════

pub struct Vault;

impl Vault {
    pub fn transfer(from: &str, to: &str, amount: Money) -> (LedgerEntry, LedgerEntry) {
        let debit = LedgerEntry {
            account_id: to.to_string(),
            amount: amount.abs(),
            description: Some(format!("Transfer from {}", from)),
        };

        // Credit is negative
        let credit_val = Money::new(-amount.amount().abs());
        let credit = LedgerEntry {
            account_id: from.to_string(),
            amount: credit_val,
            description: Some(format!("Transfer to {}", to)),
        };

        (debit, credit)
    }
}
