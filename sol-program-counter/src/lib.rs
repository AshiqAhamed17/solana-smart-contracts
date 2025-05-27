use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{self, next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey
};

#[derive(BorshSerialize, BorshDeserialize)]

// Like Function declearation , two function Increment and Decrement. which will be implemented in the match case of InstructionType
enum InstructionType {
    Increment(u32),
    Decrement(u32)
}

#[derive(BorshSerialize, BorshDeserialize)]
struct Counter {
    count: u32,
}

entrypoint!(counter_contract);
pub fn counter_contract (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    
    let acc = next_account_info(&mut accounts.iter())?;
    let instruction_type = InstructionType::try_from_slice(instruction_data)?;
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;

    match instruction_type {

        // Think of it like Increment function
        InstructionType::Increment(value) => {
            msg!("Increment!!");
            counter_data.count += value;
        },

        // Think of it like Decrement function
        InstructionType::Decrement(value) => {
            msg!("Decrement!!");
            counter_data.count -= value;
        }
    }

    counter_data.serialize(&mut *acc.data.borrow_mut())?;
    msg!("Contract successfully executed!");

    Ok(())

}