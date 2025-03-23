use std::{collections::BTreeMap, fmt::Result}; 
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
            let caller_balance = self.get_balance(&caller);
            let to_balance = self.get_balance(&to);
            /*TODO:
            - Get the balance of account 'caller'
            - Get the balance of account 'to'

            - Use safemath to calculate a 'new_caller_balance' 
            - Use safemath to calculate a 'new_to_balance'

            - Update the balance of account 'caller' to 'new_caller_balance'
            - Update the balance of account 'to' to 'new_to_balance'

             */
      }
}