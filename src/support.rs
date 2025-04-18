// The most primitive representation of a blockchain block.
pub struct Block<Header, Extrinsic> {
      /// The block header contains metadata about the block
      pub header: Header,
      /// The extrinsic represent the state transitions to be executed in this block.
      pub extrinsics: Vec<Extrinsic>,
}

/// We are using an extremely simplified header which only contains the current block number.
/// On a real blockchain, you would expect to also find:
/// - parent block hash
/// - state root
/// - extrinsics root
/// - timestamp
/// - etc...

pub struct Header<BlockNumber> {
      /// The block number is a unique identifier for each block in the chain.
      pub block_number: BlockNumber,
}

/// This is an "extrinsic": literally an external message from outside of the blockchain.
/// This simplified version of an extrinsic tells us who is making the call and what the call is.
pub struct Extrinsic<Caller, Call> {
      /// The account that is making the call
      pub caller: Caller,
      /// The call that is being made
      pub call: Call,
}

/// The Result type for our runtime. When everything completes successfully, we return Ok.
/// Otherwise return a static error message.
pub type DispatchResult = Result<(), &'static str>;

/// A trait which allows us to dispatch an incoming extrinsic to the appropriate state transition function call.
pub trait Dispatch {
      /// The type used to identify the caller of the function
      type Caller;
      /// The state transition function call the caller is trying to access.
      type Call;

      /// This function takes a 'caller' and the 'call' they want to make, and returns a 'Result' based on the outcome of the func call
      fn dispatch(
            &mut self,
            caller: Self::Caller,
            call: Self::Call,
      ) -> DispatchResult;
}