use support::Dispatch;

mod balances;
mod system; 
mod support;
mod proof_of_existence;

mod types {
    use crate::support;

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    // TODO: Define a concrete 'Extrinsic' type using 'AccountId' and 'RuntimeCall'
    // TODO: Define a concrete 'Header' type using 'BlockNumber'
    // TODO: Define a concrete 'Block' type using 'Header' and 'Extrinsic'
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}

//removed after macros introduced for runtime
/*
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}
*/

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}


#[derive(Debug)]    
#[macros::runtime]
pub struct Runtime {
    /*TODO: Create a field 'system' which is of type 'system::Pallet'. */
    /*TODO: Create a field 'balances' which is of type 'balances::Pallet'. */
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
    proof_of_existence: proof_of_existence::Pallet<Runtime>,
}


// removed after adding runtime macros
/*
impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet
    fn new() -> Self {
        /*TODO: Create a new 'Runtime' by creating new instances of 'system' and 'balances' */
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new(),
        }
    }

    // Execute a block of extrinsics. Increments the block number.
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        /* TODO:
        - Increment the block number in the system
        - Check that the block number of the incoming block matches the current block number, or return an error.
        - Iterate over the extrinsics in the block...
                 - Increment the nonce of the caller
                 - Dispatch the extrinsic using the 'caller' and the 'call' contained in the extrinsic error and capturing the result
                 - You can extend the error message to include information like the block number and extrinsic number.        
         */
        self.system.increment_block_number();

        if (self.system.block_number() != block.header.block_number) {
            return Err("Block number does not match");
        }

        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.increment_nonce(&caller);
            let _ = self.dispatch(caller, call).map_err(|e| 
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number:{}\n\tError: {}",
                    block.header.block_number, i, e
                )
            );  
        }

        Ok(())
    }
}


impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    // Dispatch a call on behalf of the caller. Increment the nonce of the caller.
    // Dispatch allows us to identify which underlying module call we want to execute.
    // Note that we extract the 'caller' from the extrinsic, and use that information
    // to determine who we are executing the call on behalf of.

    fn dispatch(
            &mut self,
            caller: Self::Caller,
            runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances(call) => {
                // Call the dispatch function of the balances pallet
                self.balances.dispatch(caller, call)?;
            },

            RuntimeCall::ProofOfExistence(call) => {
                // Call the dispatch function of the proof_of_existence pallet
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}
 */

fn main() {
    // TODO: Create a mutable variable 'runtime' which is a new instance of 'Runtime'
    // TODO: Set the balance of 'Alice' to 100, allowing us to execute other transactions.
    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);
/////////////////////////////////
    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer { to: bob.clone(), amount: 30 }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer { to: charlie.clone(), amount: 20 }),
            },
        ],
    };

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { claim: "my_document" }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { claim: "bob_document" }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("wrong block execution");
    runtime.execute_block(block_2).expect("wrong block execution");


    //////////OR/////////
/*  

    // Start emulating a block
    // TODO: Increment the block number in system
    // TODO: Assert the block number is what we expect.
    runtime.system.increment_block_number();

    assert_eq!(runtime.system.block_number(), 1);


    // First transaction
    // TODO: Increment the nonce of 'Alice' in system
    /*  TODO: Execute a transfer from 'Alice' to 'Bob' of 30 tokens.
     - The transfer _could_ return an error. We should use 'map_err' to print the error if there is one.
     - We should capture the result of the transfer in an unused variable like '_result'.
    */
    runtime.system.increment_nonce(&alice);

    let _ = runtime.balances
        .transfer(alice.clone(), bob.clone(), 30)
        .map_err(|e| println!("Error: {:?}", e));


    // Second transaction
    // TODO: Increment the nonce of 'Alice' in system again.
    // TODO: Execute another balance transfer, this time from 'Alice' to 'Charlie' of 20 tokens.
    runtime.system.increment_nonce(&alice);

    let _ = runtime.balances
        .transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e| println!("Error: {:?}", e));
*/

    println!("{:#?}", runtime); // This will print the state of the runtime after the transactions
}




#[test]
fn init_balances() {
    let mut balances: balances::Pallet<Runtime> = balances::Pallet::new();

    assert_eq!(balances.get_balance(&"Alice".to_string()), 0);
    balances.set_balance(&"Alice".to_string(), 100);
    assert_eq!(balances.get_balance(&"Alice".to_string()), 100);
    assert_eq!(balances.get_balance(&"Bob".to_string()), 0);
}

#[test]
fn fail_test() {
    assert_eq!(2, 2);
}




















/*
let mut map = BTreeMap::new();
    map.insert("Alice", 100);
    // assert_eq!(map.get(&"Alice"), Some(&100));           -----> this is pattern matching
    // assert_eq!(map.get(&"Bob"), None);                   -----> doing so much pattern matching is not good
    //////or//////
    assert_eq!(map.get(&"Alice").unwrap_or(&0), &100);      //-----> this is better instead
    

    let maybe_value = map.get(&"Bob");
    match maybe_value {
        Some(value) => {
            println!("Alice's balance is {}", value);
        },
        None => {
            println!("This user does not exist and have no balance");
        }
    }
 */