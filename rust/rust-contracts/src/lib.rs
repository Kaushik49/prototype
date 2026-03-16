use borsh::{BorshDeserialize,BorshSerialize}; 
use solana_program::{//next account info iterates over accountInfo array
    account_info::{AccountInfo, next_account_info}, 
    entrypoint::{ProgramResult}, 
    msg, 
    pubkey::Pubkey,
    entrypoint // macro to specify the function call in solana
};

#[derive(BorshDeserialize,BorshSerialize)]

// try o pattern match instruction given by the user
enum CounterInstruction {
    Increment(u32), 
    Decrement(u32)
}

#[derive(BorshDeserialize,BorshSerialize)]
// you need to derserialize again for writing back to the account info

// what are we storing in the blockchain , we defin the state variable
struct Counter {
    count: u32
}



// macro for invoking counter_contract
entrypoint!(counter_contract);

// takes three things a input 
pub fn counter_contract(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    instruction_data: &[u8] // 0 , means that add otherwise if 10001, then subtract
)-> ProgramResult {
    // get the first account and access it inside the account variable inside the function
    // this is actually the data account 
    //? propogating the error variant if the account result error
    let acc = next_account_info(&mut accounts.iter())?;// unwrap means if there is right thing passed then it will execute, otherwise it will panic
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;
    match CounterInstruction::try_from_slice(instruction_data)? {
        CounterInstruction::Increment(value) => {
            counter_data.count += value; 
        },
        CounterInstruction::Decrement(value)=> {
            counter_data.count -= value;
        }
    }
    counter_data.serialize(&mut *acc.data.borrow_mut())?;
    msg!("hi there ");
    return Ok(());


}
