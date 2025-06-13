// use borsh::{BorshSerialize, BorshDeserialize};
// use solana_program::{
//     account_info::{
//         AccountInfo, next_account_info
//     },
//     entrypoint::ProgramResult, entrypoint,
//     pubkey::Pubkey,
//     msg,
// };


// entrypoint!(counter_contact);

// #[derive(BorshDeserialize, BorshSerialize)]
// enum InstructionType{
//     Increment(u32),
//     Decrement(u32),
// }


// // AccountInfo.data is in binary so we have to deserialize it to assumed struct of our program choice 

// pub fn counter_contact( 
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],  // what all accounts you might be intracting with in this program 
//     instruction_data: &[u8] // instruction incapsulate functions and arguments inside  
//     // this instructions are deserialize using borsh
// )-> ProgramResult {
//     // unrwrap vs ? (unwrap - panic, ? -> propogate error backwards)
//     let acc = next_account_info(&mut accounts.iter())? ;

//     let instruction_type = InstructionType::try_from_slice(instruction_data);

//     match instruction_type {
//         InstructionType::Increment(data) => acc.data +=1,
//         InstructionType::Decrement(data) => acc.data +=1
//     }
// }

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::{self, ProgramResult}, pubkey::Pubkey
};


entrypoint!(counter_program);

#[derive(BorshDeserialize, BorshSerialize)]
enum InstructionType{
    Increment(u32),
    Decrement(u32)
}

#[derive(BorshDeserialize, BorshSerialize)]
struct Counter{
    val: u32
}

pub fn counter_program(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;
    
    let instruction_type = InstructionType::try_from_slice(&instruction_data);
    let mut counter = Counter::try_from_slice(&acc.data.borrow())?;
    match instruction_type {
        InstructionType::Increment(val) => {
            counter.val += val;
        },
        InstructionType::Decrement(val) => {
            counter.val += val;
        }
    }
    counter.serialize(&mut *acc.data.borrow_mut());
    Ok(())
}