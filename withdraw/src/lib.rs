#![allow(unused)]
mod state;
use state::{Initiate, State, Submit};

mod types;
use types::UserType;

mod account;
use account::Account;
mod transaction;
use transaction::{ Mode, TransactionType };

use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, error::Error, ops::{Div, Mul}, rc::Rc};

const FEE_PERCENT: f64 = 1.0;
const ZERO: f64 = 0.0;

struct Withdraw<'a> {
    amount: f64,
    state: Option<Box<dyn State>>,
    from: &'a mut Account,
    to: &'a mut Account,    
} 

impl<'a> Withdraw<'a> {
    fn build(from: &'a mut Account, to: &'a mut Account, amount: f64) -> Self {
        Withdraw { 
            amount,
            from,
            to,
            state: Some(Box::new(Initiate { }))
        }
    }

    fn consume_fee(&self) -> f64 {
        self.amount.mul(FEE_PERCENT).div(100_f64)
    }

    fn submit(&mut self) -> Result<(), &'static str> {
        println!("A withdrawal request of ${} is {}", self.amount, self.state.as_ref().unwrap().state());
        println!("Required Fee: {}", self.consume_fee());

        if self.amount <= ZERO {
           return Err("Withdraw can not be submitted. Invalid amount!");
        }

        self.state = Some(
            Box::new(Submit {})
        );

        Ok(())
    }

    pub fn do_transaction(&mut self, admin:  &mut Account)  {
        let charge = self.consume_fee();

        let net_debit = self.amount - charge;
        
        // Subtract Amount from Account holder
        {
            
            let from_acc = self.from.borrow_mut();
            from_acc.transaction(self.amount * -1.0).map_err(|error| {
                println!("{error}");
            });

            from_acc.add_transaction_history(
                &from_acc.name.clone(),
                &self.to.name,
                self.amount,
                TransactionType::Withdraw,
                Mode::Debit
            );
        }   
        

        // Add Amount to Account holder
        {
            let to_acc = self.to.borrow_mut();
            to_acc.transaction(net_debit).map_err(|error| {
                println!("{error}");
            });

            to_acc.add_transaction_history(
                &self.from.name.clone(),
                &to_acc.name.clone(),
                self.amount,
                TransactionType::Deposit,
                Mode::Credit
            );

        }

        // Fee transfer to Admin
        {
            admin.transaction(charge).map_err(|error| {
                println!("{error}");
            });

            admin.add_transaction_history(
                &self.from.name.clone(),
                &admin.name.clone(),
                charge,
                TransactionType::Deposit,
                Mode::Credit
            );
        }
        
        

    }

    
}

pub fn execute() {
    let mut ram = Account::build("Ram", UserType::User, 250f64);
    let mut shyam = Account::build("Shyam", UserType::User, 0f64);


    let mut withdraw = Withdraw::build(
        &mut ram,
        &mut shyam,
         50 as f64
    );

    let charge = withdraw.consume_fee();  

    withdraw.submit().map_err(|err| {
        println!("Error: {}", err);
    });

    println!("withdraw: {:#?}", withdraw.state.as_ref().unwrap().state());

    let mut admin = Account::new();
    // let admin = Account::build(UserType::User, 0.0);

    admin.approve(&mut withdraw).map_err(|err| {
        println!("Error: {}", err);
    }); 

    println!("Ram balance: {}", ram.get_balance());
    println!("Ram's transactions => {:?}", ram.history);
    println!("Shyam balance: {}", shyam.get_balance());
    println!("Shyam's transactions => {:?}", shyam.history);

    println!("Admin Balance: {}", admin.get_balance());
    println!("Admin's transactions => {:?}", admin.history);

    let mut goldy = Account::build("goldy", UserType::User, 10.0);

    let mut another_withdraw = Withdraw::build(&mut ram, &mut  goldy, 70f64);

    another_withdraw.submit();

    admin.approve(&mut another_withdraw).map_err(|rr| {
        println!("{rr}")
    });

    println!("\nRam balance: {}", ram.get_balance());
    println!("Ram's transactions => {:?}", ram.history);
     
}