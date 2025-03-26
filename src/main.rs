mod balances;
mod system; 

pub struct Runtime {
    /*TODO: Create a field 'system' which is of type 'system::Pallet'. */
    /*TODO: Create a field 'balances' which is of type 'balances::Pallet'. */
    system: system::Pallet,
    balances: balances::Pallet,
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
    println!("Hello, world!");
    let runtime = Runtime::new();
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