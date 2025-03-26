/*TODO: You might need to update your imports. */
use std::collections::BTreeMap;

/// This is the System Pallet
/// It handles the system level functionality of the blockchain
pub struct Pallet {
      /// The current block number
      /*TODO: Create a field 'block_number' that stores a 'u32'.*/
      /// A map from an account to their nonce.
      /// The nonce is used to prevent replay attacks.
      /*TODO: Create a field 'nonce' that is a 'BTreeMap' from 'String' to 'u32'. */
      block_number: u32,
      nonce: BTreeMap<String, u32>,   // (wallet, nonce)
}

impl Pallet {
      pub fn new() -> Self {
            Self {
                  block_number: 0,
                  nonce: BTreeMap::new(),
            }
      }

      /// Get the current block number
      pub fn block_number(&self) -> u32 {
            /*TODO: Return the current block number. */
            self.block_number
      }

      /// This function increments the block number by 1
      /// It is called at the end of each block
      /// It should also increment the nonce of the system account
      pub fn increment_block_number(&mut self) {
            /*TODO: Increment the current block number by 1 */
            self.block_number = self.block_number.checked_add(1).unwrap();     //[used unwrap: crashes if overflow(for purpose)]
      }

      // Increment the nonce of an account. This helps us keep track of how many transactions an account has made
      pub fn increment_nonce(&mut self, who: &String) {
            /*TODO: Get the current nonce of 'who', and increment it by 1 */
            let nonce = self.nonce.get(who).unwrap_or(&0);
            self.nonce.insert(who.clone(), nonce + 1);
      }

      pub fn get_nonce(&self, who: &String) -> u32 {
            *self.nonce.get(who).unwrap_or(&0)
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
            let system = super::Pallet::new();
            assert_eq!(system.block_number(), 0);
      }

      #[test]
      fn increment_block_number() {
            let mut system = super::Pallet::new();
            system.increment_block_number();
            assert_eq!(system.block_number(), 1);
      }

      #[test]
      fn increment_nonce() {
            let alice = String::from("alice");
            let mut system = super::Pallet::new();
            system.increment_nonce(&alice.clone());
            assert_eq!(system.get_nonce(&alice), 1);
      }
}
