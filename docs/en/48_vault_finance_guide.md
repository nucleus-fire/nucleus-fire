# Finance & Ledger Guide

Nucleus Vault provides detailed financial primitives and double-entry bookkeeping capabilities to ensure data integrity.

## Quick Start

```rust
use nucleus_std::vault::{Money, Ledger, Account, AccountType, Transaction};
use rust_decimal_macros::dec;

// 1. Setup Ledger
let mut ledger = Ledger::new();

// 2. Create Accounts
ledger.create_account(Account {
    id: "cash".into(),
    name: "Cash".into(),
    account_type: AccountType::Asset,
    metadata: Default::default(),
});

ledger.create_account(Account {
    id: "revenue".into(),
    name: "Sales Revenue".into(),
    account_type: AccountType::Revenue,
    metadata: Default::default(),
});

// 3. Record Transaction (Double-entry enforced)
let tx = Transaction::builder("tx_1", "Consulting Services")
    .debit("cash", Money::new(dec!(500.00)))
    .credit("revenue", Money::new(dec!(500.00)))
    .build()?;

ledger.record(tx)?;

// 4. Check Balance
assert_eq!(ledger.balance("cash").amount(), dec!(500.00));
```

## Money Primitive

`Money` wraps `Decimal` to prevent floating point errors. It supports standard arithmetic.

```rust
let cost = Money::new(dec!(19.99));
let tax = cost * dec!(0.10);
let total = cost + tax;
```

## Double-Entry Accounting

### Accounts

Accounts are categorized by type:
- `Asset` (e.g., Cash, Receivables) - Normal Balance: Debit (+)
- `Liability` (e.g., Loans, Payables) - Normal Balance: Credit (-)
- `Equity` (e.g., Owner's Capital) - Normal Balance: Credit (-)
- `Revenue` (e.g., Sales) - Normal Balance: Credit (-)
- `Expense` (e.g., Rent, Salaries) - Normal Balance: Debit (+)

### Transactions

Transactions MUST be balanced (Debits = Credits). The builder API helps construct valid transactions.

```rust
// A split transaction
let tx = Transaction::builder("tx_2", "Office Supplies")
    .credit("cash", Money::new(dec!(100.00)))      // Paid $100
    .debit("supplies", Money::new(dec!(90.00)))    // Expense $90
    .debit("tax_expense", Money::new(dec!(10.00))) // Tax $10
    .build()?;
```

### Ledger

The `Ledger` struct holds accounts and verified transactions.

```rust
// Get balance of specific account
let cash = ledger.balance("cash");

// Get trial balance of ALL accounts
let trial = ledger.trial_balance();
for (id, balance) in trial {
    println!("{}: {}", id, balance.amount());
}
```

## Helpers

For quick operations without a full ledger, you can use the `Vault` helper:

```rust
use nucleus_std::vault::Vault;

let (debit, credit) = Vault::transfer("Alice", "Bob", Money::new(dec!(50)));
// Returns raw LedgerEntry structs you can store manually
```
