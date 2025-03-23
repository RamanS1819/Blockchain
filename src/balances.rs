use std::collections::BTreeMap;
pub struct Pallet {
      balances: BTreeMap<String, u128>,
} 

// function to create a new instance of the Pallet struct from outside
impl Pallet {
      pub fn new() -> Self {
            Self {
                  balances: BTreeMap::new()
            }
      }

      /// function to set the balance of an account 'who' to some 'amount'
      pub fn set_balance(&mut self, who: &String, amount: u128) {
            /*insert 'amount' into the BTreeMap under 'who' */
            self.balances.insert(who.clone(), amount);
      }

      /// function to get the balance of an account 'who'
      /// returns the balance of 'who' if it exists, otherwise returns 0
      pub fn get_balance(&self, who: &String) -> u128 {
            /* get the balance of 'who' from the BTreeMap */
            *self.balances.get(who).unwrap_or(&0)
      }

      /// function to transfer 'amount' from one account to another
      /// This function verifies that the sender has enough balance to transfer
      /// and then transfers the 'amount' from the sender to the receiver
      /// and that no mathematical overflow occurs
      pub fn transfer(
            &mut self,
            caller: String,
            to: String,
            amount: u128,
      ) -> Result<(), &'static str> {
            /*TODO:
            - Get the balance of account 'caller'
            - Get the balance of account 'to'

            - Use safemath to calculate a 'new_caller_balance' 
            - Use safemath to calculate a 'new_to_balance'

            - Update the balance of account 'caller' to 'new_caller_balance'
            - Update the balance of account 'to' to 'new_to_balance'

            */
            let caller_balance = self.get_balance(&caller);
            let to_balance = self.get_balance(&to);

            let new_caller_balance = caller_balance
                  .checked_sub(amount)
                  .ok_or("Insufficient balance")?;

            let new_to_balance = to_balance
                  .checked_add(amount)
                  .ok_or("Overflow while transferring")?;

            self.set_balance(&caller, new_caller_balance);
            self.set_balance(&to, new_to_balance);

            Ok(())
            
      }
}

#[cfg(test)]
mod test {
      #[test]
      fn init_balances() {
            let mut pallet = super::Pallet::new();
        
            assert_eq!(pallet.get_balance(&"Alice".to_string()), 0);
            pallet.set_balance(&"Alice".to_string(), 100);
            assert_eq!(pallet.get_balance(&"Alice".to_string()), 100);
            assert_eq!(pallet.get_balance(&"Bob".to_string()), 0);
      }
        
      #[test]
      fn transfer_balance() {
            let alice = "alice".to_string();
            let bob = "bob".to_string();
              /*TODO: Create a test that checks the following:
              - That 'alice' cannot transfer funds she does not have
              - That 'alice' can successfully transfer funds to 'bob'
              - That the balance of 'alice' and 'bob' is updated correctly after the transfer
               */
            let mut pallet = super::Pallet::new();

            pallet.set_balance(&"alice".to_string(), 100);

            let _ = pallet.transfer(alice.clone(), bob.clone(), 90);

            assert_eq!(pallet.get_balance(&alice), 10);
            assert_eq!(pallet.get_balance(&bob), 90);
      }

      #[test]
      fn transfer_balance_insufficient() {
            let alice = "alice".to_string();
            let bob = "bob".to_string();

            let mut pallet = super::Pallet::new();

            pallet.set_balance(&"alice".to_string(), 100);

            let result = pallet.transfer(alice.clone(), bob.clone(), 110);

            assert_eq!(result, Err("Insufficient balance"));
            assert_eq!(pallet.get_balance(&alice), 100);
            assert_eq!(pallet.get_balance(&bob), 0);
      }

      #[test]
      fn transfer_balance_overflow() {
            let alice = "alice".to_string();
            let bob = "bob".to_string();

            let mut pallet = super::Pallet::new();

            pallet.set_balance(&"alice".to_string(), 100);
            pallet.set_balance(&"bob".to_string(), u128::MAX);

            let result = pallet.transfer(alice.clone(), bob.clone(), 1);

            assert_eq!(result, Err("Overflow while transferring"));
            assert_eq!(pallet.get_balance(&alice), 100);
            assert_eq!(pallet.get_balance(&bob), u128::MAX);
      }
}