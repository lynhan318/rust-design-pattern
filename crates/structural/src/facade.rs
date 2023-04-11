// Intent: facade hide all complex logics and abstract via an method, this methods will connect
// with other components and do the complex logic
// ┌───────────┐
// │  Client   ├──────────────────┐
// │           │                  │
// └───────────┘                  │
//                                │
//                                │
//                                │
//                                │
//                                │
//                                │
//   ┌─────────────────┬──────────▼─────────┬──────────────────┐
//   │                 │                    │                  │
//   │                 │                    │                  │
//   │                 │  Facade            │                  │
//   │                 │                    │                  │
//   │                 │                    ├──────────┐       │
//   │     ┌───────────┴────────┬───────────┘          │       │
//   │     │                    │                      │       │
//   │     │                    │                      │       │
//   │     │                    │                      │       │
//   │   ┌─▼───────┐            │                      │       │
//   │   │         │            │                ┌─────▼────┐  │
//   │   │  ObjectA│            │                │          │  │
//   │   │         │      ┌─────▼─────────┐      │          │  │
//   │   └─────────┘      │               │      │          │  │
//   │                    │ ObjectB       │      │Object    │  │
//   │                    │               │      │          │  │
//   │                    │               │      └──────────┘  │
//   │                    └───────────────┘                    │
//   │                                                         │
//   └─────────────────────────────────────────────────────────┘
//
//
//
struct Wallet {
    pub balance: u64,
}
impl Wallet {
    pub fn new(balance: u64) -> Self {
        Self { balance }
    }
    pub fn deposit(&mut self, amount: u64) -> Result<(), String> {
        println!("Wallet:: Deposit {}", &amount);
        self.balance += amount;
        Ok(())
    }

    pub fn withdrawal(&mut self, amount: u64) -> Result<(), String> {
        let valid_amount = self.balance > 0 && amount <= self.balance;
        if valid_amount {
            println!("Wallet:: Withdraw {}", &amount);
            self.balance -= amount;
            return Ok(());
        }
        Err("Invalid withdrawal amount".into())
    }
}

struct Account {
    pub account_id: u64,
    pub name: String,
}

impl Account {
    pub fn new(account_id: u64, name: String) -> Self {
        Self { account_id, name }
    }
    pub fn check(&self, account_id: u64) -> Result<(), String> {
        if self.account_id != account_id {
            println!("Account ID not matches");
            return Err("Acocunt ID not matches".into());
        }
        Ok(())
    }
}

struct Ledger;
impl Ledger {
    pub fn make_entry(&self, amount: u64, tx_type: String, account_id: u64) {
        println!(
            "Make ledger entry for account {} with transaction type {} amount {}",
            amount, tx_type, account_id
        );
    }
}

pub struct Notification;
impl Notification {
    pub fn send_wallet_credit_notification(&self) {
        println!("send wallet credit")
    }
    pub fn send_wallet_debit_notification(&self) {
        println!("Send wallet debit")
    }
}

struct SecurityCode {
    code: u64,
}
impl SecurityCode {
    pub fn new(code: u64) -> Self {
        SecurityCode { code }
    }
    pub fn check(&self, code: u64) -> Result<(), String> {
        if self.code != code {
            println!("Invalid code!");
            return Err("Invalid code".into());
        }
        Ok(())
    }
}

pub struct WalletFacade {
    wallet: Wallet,
    account: Account,
    security_code: SecurityCode,
    notification: Notification,
    ledger: Ledger,
}

impl WalletFacade {
    fn new(account_id: u64, name: String, code: u64) -> Self {
        Self {
            wallet: Wallet::new(0),
            account: Account::new(account_id, name),
            notification: Notification,
            ledger: Ledger,
            security_code: SecurityCode::new(code),
        }
    }

    fn add_money_to_wallet(
        &mut self,
        account_id: u64,
        amount: u64,
        code: u64,
    ) -> Result<(), String> {
        println!("Start add money to wallet");
        self.account.check(account_id)?;
        self.security_code.check(code)?;
        self.wallet.deposit(amount)?;
        self.notification.send_wallet_debit_notification();
        self.ledger.make_entry(amount, "deposit".into(), account_id);
        Ok(())
    }

    fn deduct_money_from_wallet(
        &mut self,
        account_id: u64,
        amount: u64,
        code: u64,
    ) -> Result<(), String> {
        println!("Start deduct money to wallet");
        self.account.check(account_id)?;
        self.security_code.check(code)?;
        self.wallet.deposit(amount)?;
        self.notification.send_wallet_debit_notification();
        self.ledger.make_entry(amount, "deposit".into(), account_id);
        Ok(())
    }
}
pub fn demo_facade() -> Result<(), String> {
    let mut ids = 0;
    let mut facade_wallet = WalletFacade::new(ids, "Kevin".into(), 12);
    facade_wallet.add_money_to_wallet(0, 10, 12)?;

    ids += 1;
    let mut facade_wallet = WalletFacade::new(ids, "Annie".into(), 13);
    facade_wallet.deduct_money_from_wallet(1, 10, 13)?;
    Ok(())
}
