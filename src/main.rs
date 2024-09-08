use account::Account;
use transaction::Transaction;
mod account;
mod transaction;
use std::error::Error;
use std::fs::File;
use std::collections::HashMap;
use std::io;
fn process_transactions(transactions: Vec<Transaction>) -> HashMap<u16, Account> {
    let mut accounts: HashMap<u16, Account> = HashMap::new();
    let mut disputed_transactions: HashMap<u32, (u16, f64)> = HashMap::new(); // tx -> (client, amount)

    for transaction in transactions {
        let account = accounts.entry(transaction.client).or_insert(Account {
            client: transaction.client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        });

        if account.locked {
            continue; // Skip any transactions for locked accounts.
        }

        match transaction.tx_type.as_str() {
            "deposit" => {
                if let Some(amount) = transaction.amount {
                    account.available += amount;
                    account.total += amount;
                }
            }
            "withdrawal" => {
                if let Some(amount) = transaction.amount {
                    if account.available >= amount {
                        account.available -= amount;
                        account.total -= amount;
                    }
                }
            }
            "dispute" => {
                if let Some(&(client, amount)) = disputed_transactions.get(&transaction.tx) {
                    if client == transaction.client {
                        account.available -= amount;
                        account.held += amount;
                    }
                }
            }
            "resolve" => {
                if let Some(&(client, amount)) = disputed_transactions.get(&transaction.tx) {
                    if client == transaction.client {
                        account.held -= amount;
                        account.available += amount;
                        disputed_transactions.remove(&transaction.tx);
                    }
                }
            }
            "chargeback" => {
                if let Some(&(client, amount)) = disputed_transactions.get(&transaction.tx) {
                    if client == transaction.client {
                        account.held -= amount;
                        account.total -= amount;
                        account.locked = true;
                        disputed_transactions.remove(&transaction.tx);
                    }
                }
            }
            _ => (),
        }
    }

    accounts
}
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = std::env::args().nth(1).expect("Please provide a CSV file.");
    let file = File::open(file_path)?;

    let mut rdr = csv::Reader::from_reader(file);
    let mut transactions = Vec::new();

    for result in rdr.deserialize() {
        let record: transaction::Transaction = result?;
        transactions.push(record);
    }

    let accounts = process_transactions(transactions);

    let mut wtr = csv::Writer::from_writer(io::stdout());
    for account in accounts.values() {
        wtr.serialize(account)?;
    }

    wtr.flush()?;
    Ok(())
}
