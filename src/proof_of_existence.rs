// Proof of Existence Pallet uses the blockchain to provide a secure and immutable ledger that can be used to verify 
// the existence of a particular document, file, or piece of data at a specific point in time.

use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	/* TODO: Add a field `claims` which is a `BTreeMap` fom `T::Content` to `T::AccountId`. */
      claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
            Self {
                  claims: BTreeMap::new()
            }
	}

      // Get the owner (if any) of the claim.
      pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
            // TODO: Get the claim
            self.claims.get(claim)
      }

      // Create a new claim on behalf of the 'caller'.
      // This function will return an error if someone already has claimed that content.
      pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
            // TODO: Check that a 'claim' does not already exist. If so, return an error.
            // TODO: Insert the claim on behalf of 'caller'
            match self.get_claim(&claim) {
                Some(_) => Err("Claim already exists"),
                None => {
                  self.claims.insert(claim, caller);
                  Ok(())
                }
            }
      }

      // Revoke an existing claim on some content.
      // This function should only succeed if the caller is the owner of an existing claim.
      // It will return an error if the claim does not exist, or if the caller is not the owner.
      pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
            // TODO: Get the owner of the 'claim' to be revoked.
            // TODO: Check that the 'owner' matches the 'caller'.
            // TODO: If all checks pass, then 'remove' the claim.
            let claim_owner = self.get_claim(&claim).ok_or("Claim does not exist")?;

            if claim_owner != &caller {
                  return Err("Caller is not the owner of the claim");
            } 

            self.claims.remove(&claim);
            Ok(())
      }
}

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher, and not includes as a parameter of the call.
pub enum Call<T: Config> {
      /*
      TODO: Create variants for:
            - `CreateClaim`
            - `RevokeClaim`

            Remember that you only need to pass in the 'claim' data, as 'caller'
            information is passed in through the 'dispatch' logic.
       */
      CreateClaim { claim: T::Content },

      RevokeClaim { claim: T::Content },
}

/// Implementation of the dispatch logic, mapping from 'POECall' to the appropriate underlying function we want to execute.
impl<T: Config> crate::support::Dispatch for Pallet<T> {
      /*
      TODO: Implement 'crate::support::Dispatch' for the 'Pallet<T>'.
      
      In your 'dispatch' logic, match on 'call' and forward the 'caller' and 'claim' data to the appropriate function.
       */
      type Caller = T::AccountId;
      type Call = Call<T>;

      fn dispatch(
            &mut self,
            caller: Self::Caller,
            call: Self::Call,
      ) -> DispatchResult {
            match call {
                  Call::CreateClaim { claim } => {
                        self.create_claim(caller, claim)?;
                  }
                  Call::RevokeClaim { claim } => {
                        self.revoke_claim(caller, claim)?;
                  }
            }
            Ok(())
      }

}


#[cfg(test)]
mod test {
      struct TestConfig;

      impl super::Config for TestConfig {
            type Content = &'static str;
      }

      impl crate::system::Config for TestConfig {
            type AccountId = &'static str;
            type BlockNumber = u32;
            type Nonce = u32;
      }

      #[test]
      fn basic_proof_of_existence() {
            // Create a new instance of the Proof of Existence Module.
            /*
            TODO: Create an end to end test verifying the basic functionality of this pallet.
            - Check the initial state is as you expect.
            - Check that all the functions work successfully.
            - Check that all error conditions error as expected.
             */
            let mut proof_of_existence = super::Pallet::<TestConfig>::new();
            let _ = proof_of_existence.create_claim("alice", "my_document");

            assert_eq!(
                  proof_of_existence.get_claim(&"my_document"), Some(&"alice")
            );

            let res = proof_of_existence.revoke_claim("bob", "my_document");
            assert_eq!(res, Err("Caller is not the owner of the claim"));

            let res = proof_of_existence.create_claim("bob", "my_document");
            assert_eq!(res, Err("Claim already exists"));

            let res = proof_of_existence.revoke_claim("alice", "non existant");
            assert_eq!(res, Err("Claim does not exist"));
            
            let res = proof_of_existence.revoke_claim("alice", "my_document");
            assert_eq!(res, Ok(()));
            assert_eq!(proof_of_existence.get_claim(&"my_document"), None);
            
      }
}