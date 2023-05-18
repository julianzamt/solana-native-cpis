use crate::{instructions::*, state::*, stubs::*};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CounterInstruction::unpack(instruction_data)?;

        match instruction {
            CounterInstruction::Increase {
                number,
            } => {
                Self::process_increase(number, accounts, program_id)?;
            }
            CounterInstruction::LogCounter {} => {
                Self::process_log_counter(accounts, program_id)?;
            }
        }

        Ok(())
    }

    pub fn process_increase(number: u32, accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        msg!("process_increase ix...");
        let account_info_iter = &mut accounts.iter();

        let counter = next_account_info(account_info_iter)?;
        let signer = next_account_info(account_info_iter)?;
        let _system_program = next_account_info(account_info_iter)?;

        if counter.lamports() == 0 && *counter.owner == solana_program::system_program::id() {
            let rent = Rent::get()?;
            let (_, bump) = Pubkey::find_program_address(&[b"counter"], program_id);
            let space = Counter::LEN;
            let rent_minimum_balance = rent.minimum_balance(space);

            invoke_signed(
                &create_account(
                    &signer.key,
                    &counter.key,
                    rent_minimum_balance,
                    space as u64,
                    program_id,
                ),
                &[signer.clone(), counter.clone()],
                &[&[b"counter".as_ref(), &[bump]]],
            )?;
        }

        let mut counter_data = Counter::unpack_unchecked(&counter.try_borrow_data()?)?;

        increase::increase(number, &mut counter_data)?;

        Counter::pack(counter_data, &mut counter.try_borrow_mut_data()?)?;

        Ok(())
    }

    pub fn process_log_counter(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        msg!("process_log_counter ix...");
        let account_info_iter = &mut accounts.iter();

        let counter = next_account_info(account_info_iter)?;

        let counter_data = Counter::unpack_unchecked(&counter.try_borrow_data()?)?;

        msg!("Counter number: {}", counter_data.number);

        Counter::pack(counter_data, &mut counter.try_borrow_mut_data()?)?;

        Ok(())
    }
}
