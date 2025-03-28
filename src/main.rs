mod balances;
mod system; 

mod types {
    pub type AccountID = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
}

#[derive(Debug)]    
pub struct Runtime {
    /*TODO: Create a field 'system' which is of type 'system::Pallet'. */
    /*TODO: Create a field 'balances' which is of type 'balances::Pallet'. */
    system: system::Pallet<types::AccountID, types::BlockNumber, types::Nonce>,
    balances: balances::Pallet<types::AccountID, types::Balance>,
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet
    fn new() -> Self {
        /*TODO: Create a new 'Runtime' by creating new instances of 'system' and 'balances' */
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    // TODO: Create a mutable variable 'runtime' which is a new instance of 'Runtime'
    // TODO: Set the balance of 'Alice' to 100, allowing us to execute other transactions.
    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);


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


    println!("{:#?}", runtime); // This will print the state of the runtime after the transactions
}




#[test]
fn init_balances() {
    let mut pallet = balances::Pallet::new();

    assert_eq!(pallet.get_balance(&"Alice".to_string()), 0);
    pallet.set_balance(&"Alice".to_string(), 100);
    assert_eq!(pallet.get_balance(&"Alice".to_string()), 100);
    assert_eq!(pallet.get_balance(&"Bob".to_string()), 0);
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