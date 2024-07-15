#[derive(Debug, Clone)]
pub(super) enum TransactionType {
  Deposit,
  Withdraw,
  Fee,
}

#[derive(Debug, Clone)]
pub(super) enum Mode {
  Credit, 
  Debit,
}

#[derive(Debug, Clone)]
pub(super) struct TransactionHistory {
  pub from: String,
  pub to: String,
  pub amount: f64,
  pub transaction_type: TransactionType,
  pub mode: Mode,
}