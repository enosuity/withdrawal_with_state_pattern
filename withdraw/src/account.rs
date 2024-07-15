#![allow(unused)]

use std::vec;

use crate::state::{Accepted, Processing};
use crate::types::UserType;
use crate::{Withdraw, ZERO};
use crate::transaction::{Mode, TransactionHistory, TransactionType};

#[derive(Debug)]
pub(super)  struct Account {
    pub name: String,
    pub account_type: UserType,
    pub balance: Option<f64>,
    pub history: Vec<TransactionHistory>,
}

impl Account {
  pub(super) fn new() -> Self {
        Account {
            name: String::new(),
            account_type: UserType::Admin,
            balance: None,
            history: vec![],
        }
    }

    pub(super) fn build(name: &str, holder: UserType, bal: f64) -> Self {
        Account {
            name: name.to_string(),
            account_type: holder,
            balance: Some(bal),
            history: Vec::new(),
        }
    }

    fn is_admin(&self) -> bool {
        match self.account_type {
            UserType::Admin => return  true,
            _ => return false
        }
    }

    pub(super) fn approve <'a>(&mut self, withdraw: &'a mut Withdraw) -> Result<(), &'a str> {
      if self.is_admin() {
        withdraw.state = Some(
          Box::new(Accepted {})
        );

        withdraw.state = Some(
          Box::new(Processing {})
        );

        // transfer fee to Admin
        let charge = withdraw.consume_fee();
        
        if withdraw.from.balance.unwrap() >= (withdraw.amount) {
          println!("Withdrawal is accepted successfully.");
          withdraw.do_transaction(self);
        } else {
          println!("Withdrawal is rejected ");
          return Err("In-sufficient Balance !"); 
        }
        

        Ok(())
      } else {
        Err("Unauthorized permission!. Only Admin can approve the request.")
      }
      
    }

    pub(super) fn transaction(&mut self, amount: f64) -> Result<(), &'static str> {
      // check sufficient balance in withdraw case
      if amount < ZERO {
        if amount * -1.0 > self.balance.unwrap() {
          return Err("Insufficeint Balance!");
        }
      }

      let mut wallet_balance = if let Some(bal) = self.balance {
        bal
      } else {
        0.0
      };

      wallet_balance += amount;
      self.balance = Some(wallet_balance);

      Ok(())
    }

    pub(super) fn get_balance(&self) -> f64 {
      if let Some(balance) = self.balance {
        balance
      } else {
        0.0
      }
    }

    pub(super) fn add_transaction_history(&mut self, from: &str, to: &str, amount: f64, trans_type: TransactionType, mode: Mode) {
      let mut histories = self.history.clone();
      histories.push(
        TransactionHistory {
          from: from.to_string(),
          to: to.to_string(),
          amount,
          transaction_type: trans_type,
          mode
        }
      );

      self.history = histories;
    }



}
