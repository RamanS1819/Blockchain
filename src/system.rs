/*TODO: You might need to update your imports. */
use std::{collections::BTreeMap, ops::AddAssign};
use num::traits::{CheckedAdd, CheckedSub, Zero, One};

use crate::types::Balance;

/*TODO: Define the common types used in this pallet:
      - 'AccountID'
      - 'BlockNumber'
      - 'Nonce' 
*/
type AccountID = String;
type BlockNumber = u32;
type Nonce = u32;

#[derive(Debug)]
/// This is the System Pallet
/// It handles the system level functionality of the blockchain
pub struct Pallet<AccountID, BlockNumber, Nonce> {
      /// The current block number
      /*TODO: Create a field 'block_number' that stores a 'u32'.*/
      /// A map from an account to their nonce.
      /// The nonce is used to prevent replay attacks.
      /*TODO: Create a field 'nonce' that is a 'BTreeMap' from 'String' to 'u32'. */
      block_number: BlockNumber,
      nonce: BTreeMap<AccountID, Nonce>,   // (wallet, nonce)
}

impl <AccountID, BlockNumber, Nonce> Pallet<AccountID, BlockNumber, Nonce> 
where
      AccountID: Ord + Clone,
      BlockNumber: Zero + One + CheckedAdd + CheckedSub + Copy + AddAssign,
      Nonce: Ord + Clone + Copy + Zero + CheckedAdd + CheckedSub + One,
{
      pub fn new() -> Self {
            Self {
                  block_number: BlockNumber::zero(),   // start at block 0
                  nonce: BTreeMap::new(),
            }
      }

      /// Get the current block number
      pub fn block_number(&self) -> BlockNumber {
            /*TODO: Return the current block number. */
            self.block_number
      }

      /// This function increments the block number by 1
      /// It is called at the end of each block
      /// It should also increment the nonce of the system account
      pub fn increment_block_number(&mut self) {
            /*TODO: Increment the current block number by 1 */
            self.block_number += BlockNumber::one();     //[used unwrap: crashes if overflow(for purpose)]
      }

      // Increment the nonce of an account. This helps us keep track of how many transactions an account has made
      pub fn increment_nonce(&mut self, who: &AccountID) {
            /*TODO: Get the current nonce of 'who', and increment it by 1 */
            let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
            self.nonce.insert(who.clone(), nonce + Nonce::one());
      }

      pub fn get_nonce(&self, who: &AccountID) -> Nonce {
            *self.nonce.get(who).unwrap_or(&Nonce::zero())
      }
}


#[cfg(test)]
mod test {
      #[test]
      fn init_system() {
            /*TODO: Create a test which checks the following:
            - Increment the current block number.
            - Increment the nonce of 'alice'.
            - Check that the current block number is what we expect.
            - Check that the nonce of 'alice' is what we expect.
             */
            let system: super::Pallet<String, u32, u32> = super::Pallet::new();
            assert_eq!(system.block_number(), 0);
      }

      #[test]
      fn increment_block_number() {
            let mut system: super::Pallet<String, u32, u32> = super::Pallet::new();
            system.increment_block_number();
            assert_eq!(system.block_number(), 1);
      }

      #[test]
      fn increment_nonce() {
            let alice = String::from("alice");
            let mut system: super::Pallet<String, u32, u32> = super::Pallet::new();
            system.increment_nonce(&alice.clone());
            assert_eq!(system.get_nonce(&alice), 1);
      }
}
